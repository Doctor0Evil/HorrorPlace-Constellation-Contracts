"""
Registry-aware NDJSON linter for CHAT_DIRECTOR-compatible constellations.

Responsibilities:
- Read NDJSON registry files (regions, events, personas, styles, etc.)
- Determine the appropriate JSON Schema for each line (via schemaref or mapping)
- Validate each record with JSON Schema Draft 2020-12
- Enforce ID patterns (e.g., KIND-CODE-SEQ) per registry kind
- Build in-memory indices for cross-reference checks
- Enforce cross-repo/tier/phase rules consistent with manifest/registry helpers
- Emit machine-readable diagnostics and non-zero exit on any error

This module is side-effect free on import. Use lint_registries() or main().
"""

from __future__ import annotations

import json
import re
import sys
from dataclasses import dataclass, asdict
from pathlib import Path
from typing import Any, Dict, Iterable, List, Optional, Tuple

from jsonschema import Draft202012Validator, RefResolver  # type: ignore


REPO_ROOT_SENTINELS = {".git", ".github"}

SCHEMAS_REGISTRY_DIR = Path("schemas") / "registry"

# Registry file naming conventions can be adapted or extended:
DEFAULT_REGISTRY_GLOBS = [
    "registry/*.ndjson",
    "registry/**/*.ndjson",
]

# Simple ID pattern: KIND-CODE-SEQ (example: REGION-ARAL-0001)
ID_PATTERN = re.compile(r"^[A-Z]+-[A-Z0-9]+-[0-9]{4,}$")


@dataclass
class Diagnostic:
    severity: str  # "error" or "warning"
    registry_file: str
    line_number: int
    code: str
    message: str
    json_pointer: Optional[str] = None
    field: Optional[str] = None
    value: Optional[Any] = None
    context: Optional[Dict[str, Any]] = None


@dataclass
class RegistryRef:
    registry_file: str
    line_number: int
    id: str
    kind: str
    tier: Optional[str]
    repo: Optional[str]
    phase: Optional[int]
    raw: Dict[str, Any]


@dataclass
class LintResult:
    diagnostics: List[Diagnostic]


def _find_repo_root(start: Path) -> Path:
    current = start
    while current != current.parent:
        if any((current / s).exists() for s in REPO_ROOT_SENTINELS):
            return current
        current = current.parent
    return start


def _load_json(path: Path) -> Any:
    with path.open("r", encoding="utf-8") as f:
        return json.load(f)


def _iter_registry_paths(repo_root: Path, globs: Optional[Iterable[str]] = None) -> List[Path]:
    patterns = list(globs) if globs is not None else DEFAULT_REGISTRY_GLOBS
    paths: List[Path] = []
    for pattern in patterns:
        paths.extend(sorted((repo_root / pattern).glob()))
    # Remove duplicates while preserving order
    seen = set()
    unique_paths: List[Path] = []
    for p in paths:
        if p not in seen and p.is_file():
            seen.add(p)
            unique_paths.append(p)
    return unique_paths


def _load_registry_schemas(repo_root: Path) -> Dict[str, Dict[str, Any]]:
    """
    Load all registry JSON Schemas under schemas/registry.

    Returns mapping from schemaref prefix or logical kind to schema dict.

    Example mapping keys:
    - "https://horror.place/schemas/registry-events.v1.json"
    - "events"
    """
    dir_path = repo_root / SCHEMAS_REGISTRY_DIR
    if not dir_path.is_dir():
        return {}

    schemas: Dict[str, Dict[str, Any]] = {}
    for schema_path in sorted(dir_path.glob("*.json")):
        schema = _load_json(schema_path)
        schema_id = schema.get("$id") or schema.get("id")
        name_key = schema_path.stem  # e.g. "registry-events.v1"
        logical_key = name_key.replace("registry-", "").split(".")[0]
        if schema_id:
            schemas[str(schema_id)] = schema
        schemas[logical_key] = schema

    return schemas


def _build_validator(schema: Dict[str, Any], base_uri: Optional[str] = None) -> Draft202012Validator:
    resolver = RefResolver(base_uri=base_uri or schema.get("$id") or "", referrer=schema)
    return Draft202012Validator(schema, resolver=resolver)


def _schema_for_record(
    record: Dict[str, Any],
    registry_file: Path,
    schemas: Dict[str, Dict[str, Any]],
) -> Optional[Dict[str, Any]]:
    """
    Resolve the appropriate schema object for this record.

    Strategy:
    - If record.schemaref exists and matches a known schema id/prefix, use that.
    - Otherwise, infer from registry filename logical key (events, regions, etc.).
    """
    schemaref = record.get("schemaref")

    if isinstance(schemaref, str):
        # Exact match
        if schemaref in schemas:
            return schemas[schemaref]

        # Prefix match (schemaref starts with canonical id)
        matching_ids = [sid for sid in schemas.keys() if schemaref.startswith(sid)]
        if matching_ids:
            # Prefer the longest match
            selected = max(matching_ids, key=len)
            return schemas[selected]

    # Fallback: infer from filename
    logical = registry_file.stem  # regions.minimal -> "regions"
    logical = logical.split(".")[0]
    if logical in schemas:
        return schemas[logical]

    return None


def _validate_structural(
    record: Dict[str, Any],
    validator: Draft202012Validator,
) -> List[Diagnostic]:
    diagnostics: List[Diagnostic] = []
    for error in validator.iter_errors(record):
        pointer = "/".join(
            [""] + list(error.absolute_path)
        ) if error.absolute_path else ""
        field = str(error.path[-1]) if error.path else None
        diagnostics.append(
            Diagnostic(
                severity="error",
                registry_file="",
                line_number=0,
                code="SCHEMA_VALIDATION_ERROR",
                message=error.message,
                json_pointer=pointer or None,
                field=field,
                value=error.instance,
                context={"validator": error.validator, "validator_value": error.validator_value},
            )
        )
    return diagnostics


def _extract_id_and_kind(record: Dict[str, Any]) -> Tuple[Optional[str], Optional[str]]:
    """
    Extract a registry ID and logical kind from the record.

    Expected:
    - id: "REGION-ARAL-0001"
    - kind: "region" or similar (optional; can be inferred from id prefix)
    """
    rec_id = record.get("id")
    kind = record.get("kind")
    if isinstance(rec_id, str) and not kind:
        prefix = rec_id.split("-", 1)[0]
        kind = prefix.lower()
    if not isinstance(rec_id, str):
        rec_id = None
    if not isinstance(kind, str):
        kind = None
    return rec_id, kind


def _enforce_id_pattern(
    rec_id: str,
    registry_file: Path,
    line_number: int,
) -> Optional[Diagnostic]:
    if not ID_PATTERN.match(rec_id):
        return Diagnostic(
            severity="error",
            registry_file=str(registry_file),
            line_number=line_number,
            code="INVALID_ID_PATTERN",
            message=f"id '{rec_id}' does not match KIND-CODE-SEQ pattern",
            json_pointer="/id",
            field="id",
            value=rec_id,
        )
    return None


def _extract_tier_repo_phase(record: Dict[str, Any]) -> Tuple[Optional[str], Optional[str], Optional[int]]:
    """
    Extract tier, repo, phase fields (if present) to support trust/phase checks.

    These names may be aligned with the Rust manifest types.
    """
    tier = record.get("tier")
    if not isinstance(tier, str):
        tier = None

    repo = record.get("repo") or record.get("targetRepo")
    if not isinstance(repo, str):
        repo = None

    phase_val = record.get("phase")
    if isinstance(phase_val, int):
        phase = phase_val
    else:
        phase = None

    return tier, repo, phase


def _index_registry_record(
    record: Dict[str, Any],
    registry_file: Path,
    line_number: int,
    index: Dict[str, RegistryRef],
) -> None:
    rec_id, kind = _extract_id_and_kind(record)
    tier, repo, phase = _extract_tier_repo_phase(record)

    if not rec_id or not kind:
        return

    ref = RegistryRef(
        registry_file=str(registry_file),
        line_number=line_number,
        id=rec_id,
        kind=kind,
        tier=tier,
        repo=repo,
        phase=phase,
        raw=record,
    )
    index[rec_id] = ref


def _collect_references(record: Dict[str, Any]) -> List[Tuple[str, str]]:
    """
    Collect cross-reference fields as (field_name, referenced_id).

    This is intentionally generic and can be extended to match your schemas:
    - regionId
    - styleId / styleid
    - seedId
    - personaId
    - policyId
    - contractSource
    - referencedIds (array)
    """
    results: List[Tuple[str, str]] = []

    candidate_scalar_fields = [
        "regionId",
        "styleId",
        "styleid",
        "seedId",
        "personaId",
        "policyId",
        "contractSource",
    ]
    for field in candidate_scalar_fields:
        value = record.get(field)
        if isinstance(value, str):
            results.append((field, value))

    ref_ids = record.get("referencedIds")
    if isinstance(ref_ids, list):
        for v in ref_ids:
            if isinstance(v, str):
                results.append(("referencedIds[]", v))

    return results


def _trust_and_phase_compatible(
    src: RegistryRef,
    dst: RegistryRef,
) -> bool:
    """
    Enforce basic trust/tier/phase boundaries.

    Strategy (placeholder logic; tune to match Rust crate rules):
    - Disallow references from higher trust (vault/private) to lower (public) tiers.
    - Disallow references from earlier phases to later phases if your doctrine requires forward-only dependencies.
    """
    # Tier gate example: treat tier strings lexically, but you likely want a mapping.
    if src.tier and dst.tier and src.tier != dst.tier:
        # Example: "vault" cannot depend on "standard" if that is considered illegal.
        # For now, only forbid if src tier appears "more sensitive" than dst in a simple ordering.
        hierarchy = ["standard", "mature", "vault", "lab", "research"]
        try:
            src_idx = hierarchy.index(src.tier)
            dst_idx = hierarchy.index(dst.tier)
            if src_idx > dst_idx:
                return False
        except ValueError:
            # Unknown tiers -> do not block, but they could be logged by callers.
            pass

    # Phase gate example: forbid references from lower phase to higher phase.
    if src.phase is not None and dst.phase is not None and src.phase < dst.phase:
        return False

    return True


def _lint_cross_references(
    registry_file: Path,
    line_number: int,
    record: Dict[str, Any],
    src_ref: Optional[RegistryRef],
    index: Dict[str, RegistryRef],
) -> List[Diagnostic]:
    diagnostics: List[Diagnostic] = []
    refs = _collect_references(record)
    if not refs:
        return diagnostics

    for field_name, ref_id in refs:
        target = index.get(ref_id)
        if target is None:
            diagnostics.append(
                Diagnostic(
                    severity="error",
                    registry_file=str(registry_file),
                    line_number=line_number,
                    code="MISSING_REFERENCE",
                    message=f"Referenced id '{ref_id}' not found in any registry index",
                    json_pointer=f"/{field_name}",
                    field=field_name,
                    value=ref_id,
                )
            )
            continue

        if src_ref is not None and not _trust_and_phase_compatible(src_ref, target):
            diagnostics.append(
                Diagnostic(
                    severity="error",
                    registry_file=str(registry_file),
                    line_number=line_number,
                    code="ILLEGAL_TRUST_OR_PHASE_REFERENCE",
                    message=(
                        f"Reference from {src_ref.id} (tier={src_ref.tier}, phase={src_ref.phase}) "
                        f"to {target.id} (tier={target.tier}, phase={target.phase}) "
                        "violates trust/phase boundary rules"
                    ),
                    json_pointer=f"/{field_name}",
                    field=field_name,
                    value=ref_id,
                    context={
                        "source": asdict(src_ref),
                        "target": asdict(target),
                    },
                )
            )

    return diagnostics


def lint_registries(
    repo_root: Optional[Path] = None,
    registry_paths: Optional[List[Path]] = None,
) -> LintResult:
    """
    Main linter entrypoint.

    - Loads all registry schemas.
    - Builds validators per schema.
    - First pass: schema validation + ID pattern checks + index build.
    - Second pass: cross-reference and trust/phase checks.

    Returns a LintResult containing all diagnostics.
    """
    if repo_root is None:
        repo_root = _find_repo_root(Path(__file__).resolve())

    schemas = _load_registry_schemas(repo_root)
    validators: Dict[str, Draft202012Validator] = {}
    for key, schema in schemas.items():
        validators[key] = _build_validator(schema, base_uri=schema.get("$id"))

    if registry_paths is None:
        registry_paths = _iter_registry_paths(repo_root)

    diagnostics: List[Diagnostic] = []
    index: Dict[str, RegistryRef] = {}

    # First pass: structural validation and indexing
    for registry_path in registry_paths:
        schema_cache: Dict[str, Draft202012Validator] = {}
        with registry_path.open("r", encoding="utf-8") as f:
            for line_number, raw_line in enumerate(f, start=1):
                line = raw_line.strip()
                if not line:
                    continue
                try:
                    record = json.loads(line)
                except json.JSONDecodeError as exc:
                    diagnostics.append(
                        Diagnostic(
                            severity="error",
                            registry_file=str(registry_path),
                            line_number=line_number,
                            code="INVALID_JSON",
                            message=f"Invalid JSON in NDJSON line: {exc}",
                            field=None,
                            value=line,
                        )
                    )
                    continue

                schema_obj = _schema_for_record(record, registry_path, schemas)
                if schema_obj is None:
                    diagnostics.append(
                        Diagnostic(
                            severity="error",
                            registry_file=str(registry_path),
                            line_number=line_number,
                            code="MISSING_SCHEMA",
                            message="No registry schema found for this record (schemaref or filename mapping missing)",
                            json_pointer=None,
                            field="schemaref",
                            value=record.get("schemaref"),
                        )
                    )
                    continue

                # Use schema id or logical key for validator cache.
                cache_key = schema_obj.get("$id") or schema_obj.get("id") or registry_path.stem
                validator = schema_cache.get(cache_key)
                if validator is None:
                    validator = _build_validator(schema_obj, base_uri=schema_obj.get("$id"))
                    schema_cache[cache_key] = validator

                schema_diags = _validate_structural(record, validator)
                for d in schema_diags:
                    d.registry_file = str(registry_path)
                    d.line_number = line_number
                diagnostics.extend(schema_diags)

                if schema_diags:
                    continue  # Skip non-structurally valid entries

                rec_id, _ = _extract_id_and_kind(record)
                if rec_id:
                    id_diag = _enforce_id_pattern(rec_id, registry_path, line_number)
                    if id_diag is not None:
                        diagnostics.append(id_diag)

                _index_registry_record(record, registry_path, line_number, index)

    # Second pass: cross-reference checks
    for registry_path in registry_paths:
        with registry_path.open("r", encoding="utf-8") as f:
            for line_number, raw_line in enumerate(f, start=1):
                line = raw_line.strip()
                if not line:
                    continue
                try:
                    record = json.loads(line)
                except json.JSONDecodeError:
                    continue  # Already reported in first pass

                rec_id, kind = _extract_id_and_kind(record)
                src_ref = index.get(rec_id) if rec_id is not None and kind is not None else None

                ref_diags = _lint_cross_references(
                    registry_file=registry_path,
                    line_number=line_number,
                    record=record,
                    src_ref=src_ref,
                    index=index,
                )
                diagnostics.extend(ref_diags)

    return LintResult(diagnostics=diagnostics)


def _print_diagnostics(result: LintResult) -> None:
    """
    Emit machine-readable diagnostics (JSONL) to stdout.

    Each line is a JSON object representing a Diagnostic.
    """
    for d in result.diagnostics:
        obj = asdict(d)
        # Drop None fields for brevity.
        compact = {k: v for k, v in obj.items() if v is not None}
        sys.stdout.write(json.dumps(compact, ensure_ascii=False) + "\n")


def main(argv: Optional[List[str]] = None) -> int:
    """
    CLI entrypoint.

    Intended to be wrapped by hpc-lint-registry.py, which will parse
    arguments and maybe limit which registries are linted.

    Usage (direct):
        python -m schemaspine.registrylinter
    """
    repo_root = _find_repo_root(Path.cwd())
    result = lint_registries(repo_root=repo_root, registry_paths=None)
    _print_diagnostics(result)
    has_errors = any(d.severity == "error" for d in result.diagnostics)
    return 1 if has_errors else 0


if __name__ == "__main__":
    raise SystemExit(main())
