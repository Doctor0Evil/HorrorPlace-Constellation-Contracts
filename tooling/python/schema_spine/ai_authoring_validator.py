"""
AI authoring envelope validator for CHAT_DIRECTOR-compatible constellations.

Responsibilities:
- Validate ai-authoring request and response JSON against their schemas.
- Validate any attached prism-style envelope metadata.
- Enforce lightweight, envelope-only rules:
  - Required fields (intent, objectKind, targetRepo, tier, schemaRef, referencedIds).
  - One-file-per-request discipline (exactly one primary artifact per response).
  - Basic provenance completeness (agent, session, target repo/path).
  - Invariants / metrics fields exist and are within canonical spine ranges.
  - Manifest/tier declarations are self-consistent at the envelope level.

This module is designed for fast, cheap checks prior to invoking the Rust CLI.
It MUST NOT attempt full invariant or manifest validation.

Use validate_request_envelope() and validate_response_envelope() or the CLI entrypoint.
"""

from __future__ import annotations

import json
import sys
from dataclasses import dataclass, asdict
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple

from jsonschema import Draft202012Validator, RefResolver  # type: ignore


REPO_ROOT_SENTINELS = {".git", ".github"}

CORE_SCHEMAS_DIR = Path("schemas") / "core"
SPINE_INDEX_NAME = "schema-spine-index-v1.json"

AI_REQUEST_SCHEMA_NAME = "ai-authoring-request-v1.json"
AI_RESPONSE_SCHEMA_NAME = "ai-authoring-response-v1.json"
PRISM_META_SCHEMA_NAME = "prismMeta.v1.json"


@dataclass
class Diagnostic:
    severity: str  # "error" or "warning"
    code: str
    message: str
    json_pointer: Optional[str] = None
    field: Optional[str] = None
    value: Optional[Any] = None
    context: Optional[Dict[str, Any]] = None


@dataclass
class ValidationResult:
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


def _build_validator(schema: Dict[str, Any], base_uri: Optional[str] = None) -> Draft202012Validator:
    resolver = RefResolver(base_uri=base_uri or schema.get("$id") or "", referrer=schema)
    return Draft202012Validator(schema, resolver=resolver)


def _load_core_schema(repo_root: Path, name: str) -> Optional[Dict[str, Any]]:
    path = repo_root / CORE_SCHEMAS_DIR / name
    if not path.is_file():
        return None
    return _load_json(path)


def _load_spine_index(repo_root: Path) -> Optional[Dict[str, Any]]:
    path = repo_root / CORE_SCHEMAS_DIR / SPINE_INDEX_NAME
    if not path.is_file():
        return None
    return _load_json(path)


def _validate_against_schema(
    obj: Dict[str, Any],
    validator: Draft202012Validator,
) -> List[Diagnostic]:
    diags: List[Diagnostic] = []
    for error in validator.iter_errors(obj):
        pointer = "/".join(
            [""] + list(error.absolute_path)
        ) if error.absolute_path else ""
        field = str(error.path[-1]) if error.path else None
        diags.append(
            Diagnostic(
                severity="error",
                code="SCHEMA_VALIDATION_ERROR",
                message=error.message,
                json_pointer=pointer or None,
                field=field,
                value=error.instance,
                context={"validator": error.validator, "validator_value": error.validator_value},
            )
        )
    return diags


def _require_fields(
    obj: Dict[str, Any],
    required: List[str],
    prefix_pointer: str = "",
) -> List[Diagnostic]:
    diags: List[Diagnostic] = []
    for field in required:
        if field not in obj:
            diags.append(
                Diagnostic(
                    severity="error",
                    code="MISSING_REQUIRED_FIELD",
                    message=f"Missing required field '{field}'",
                    json_pointer=f"{prefix_pointer}/{field}" if prefix_pointer else f"/{field}",
                    field=field,
                )
            )
    return diags


def _lookup_invariant_band(
    spine_index: Dict[str, Any],
    invariant_key: str,
    contract_family: Optional[str] = None,
    tier: Optional[str] = None,
) -> Optional[Tuple[Optional[float], Optional[float]]]:
    catalog = spine_index.get("invariants_catalog") or spine_index.get("invariantsCatalog")
    if not isinstance(catalog, list):
        return None

    for entry in catalog:
        if entry.get("key") != invariant_key:
            continue
        if contract_family is not None and entry.get("contract_family") not in (
            contract_family,
            None,
        ):
            continue
        if tier is not None and entry.get("tier") not in (tier, None):
            continue
        return entry.get("min"), entry.get("max")

    return None


def _lookup_metric_band(
    spine_index: Dict[str, Any],
    metric_key: str,
    contract_family: Optional[str] = None,
    tier: Optional[str] = None,
) -> Optional[Tuple[Optional[float], Optional[float]]]:
    catalog = spine_index.get("metrics_catalog") or spine_index.get("metricsCatalog")
    if not isinstance(catalog, list):
        return None

    for entry in catalog:
        if entry.get("key") != metric_key:
            continue
        if contract_family is not None and entry.get("contract_family") not in (
            contract_family,
            None,
        ):
            continue
        if tier is not None and entry.get("tier") not in (tier, None):
            continue
        return entry.get("min"), entry.get("max")

    return None


def _enforce_numeric_band(
    obj: Dict[str, Any],
    pointer_base: str,
    key: str,
    value: Any,
    min_val: Optional[float],
    max_val: Optional[float],
    code_prefix: str,
) -> Optional[Diagnostic]:
    if not isinstance(value, (int, float)):
        return Diagnostic(
            severity="error",
            code=f"{code_prefix}_NON_NUMERIC",
            message=f"Field '{key}' must be numeric",
            json_pointer=f"{pointer_base}/{key}",
            field=key,
            value=value,
        )
    if min_val is not None and value < min_val:
        return Diagnostic(
            severity="error",
            code=f"{code_prefix}_BELOW_MIN",
            message=f"Field '{key}' value {value} is below canonical minimum {min_val}",
            json_pointer=f"{pointer_base}/{key}",
            field=key,
            value=value,
        )
    if max_val is not None and value > max_val:
        return Diagnostic(
            severity="error",
            code=f"{code_prefix}_ABOVE_MAX",
            message=f"Field '{key}' value {value} is above canonical maximum {max_val}",
            json_pointer=f"{pointer_base}/{key}",
            field=key,
            value=value,
        )
    return None


def _validate_invariants_and_metrics_block(
    obj: Dict[str, Any],
    spine_index: Optional[Dict[str, Any]],
    contract_family: Optional[str],
    tier: Optional[str],
    invariants_field: str,
    metrics_field: str,
    pointer_base: str,
) -> List[Diagnostic]:
    """
    Envelope-only band checks against the spine.

    Expects:
    - obj[invariants_field] as an object mapping invariant keys to numeric values.
    - obj[metrics_field] as an object mapping metric keys to numeric values or ranges.
    """
    diags: List[Diagnostic] = []

    invariants = obj.get(invariants_field)
    metrics = obj.get(metrics_field)

    if invariants is None:
        diags.append(
            Diagnostic(
                severity="error",
                code="MISSING_INVARIANTS_BLOCK",
                message=f"Envelope missing '{invariants_field}' block",
                json_pointer=f"{pointer_base}/{invariants_field}",
                field=invariants_field,
            )
        )
    if metrics is None:
        diags.append(
            Diagnostic(
                severity="error",
                code="MISSING_METRICS_BLOCK",
                message=f"Envelope missing '{metrics_field}' block",
                json_pointer=f"{pointer_base}/{metrics_field}",
                field=metrics_field,
            )
        )

    if spine_index is None:
        return diags

    if isinstance(invariants, dict):
        for key, value in invariants.items():
            band = _lookup_invariant_band(spine_index, key, contract_family, tier)
            if band is None:
                continue
            min_val, max_val = band
            diag = _enforce_numeric_band(
                obj=invariants,
                pointer_base=f"{pointer_base}/{invariants_field}",
                key=key,
                value=value,
                min_val=min_val,
                max_val=max_val,
                code_prefix="INVARIANT_BAND",
            )
            if diag is not None:
                diags.append(diag)

    if isinstance(metrics, dict):
        for key, value in metrics.items():
            band = _lookup_metric_band(spine_index, key, contract_family, tier)
            if band is None:
                continue

            pointer = f"{pointer_base}/{metrics_field}/{key}"
            min_val, max_val = band

            if isinstance(value, list) and len(value) == 2:
                low, high = value
                diag_low = _enforce_numeric_band(
                    obj=metrics,
                    pointer_base=f"{pointer_base}/{metrics_field}",
                    key=f"{key}[0]",
                    value=low,
                    min_val=min_val,
                    max_val=max_val,
                    code_prefix="METRIC_BAND",
                )
                if diag_low is not None:
                    diags.append(diag_low)
                diag_high = _enforce_numeric_band(
                    obj=metrics,
                    pointer_base=f"{pointer_base}/{metrics_field}",
                    key=f"{key}[1]",
                    value=high,
                    min_val=min_val,
                    max_val=max_val,
                    code_prefix="METRIC_BAND",
                )
                if diag_high is not None:
                    diags.append(diag_high)
                if isinstance(low, (int, float)) and isinstance(high, (int, float)) and low > high:
                    diags.append(
                        Diagnostic(
                            severity="error",
                            code="METRIC_RANGE_INVERTED",
                            message=f"Metric '{key}' range [{low}, {high}] has min > max",
                            json_pointer=pointer,
                            field=key,
                            value=value,
                        )
                    )
            else:
                diag = _enforce_numeric_band(
                    obj=metrics,
                    pointer_base=f"{pointer_base}/{metrics_field}",
                    key=key,
                    value=value,
                    min_val=min_val,
                    max_val=max_val,
                    code_prefix="METRIC_BAND",
                )
                if diag is not None:
                    diags.append(diag)

    return diags


def validate_request_envelope(
    obj: Dict[str, Any],
    repo_root: Optional[Path] = None,
) -> ValidationResult:
    """
    Validate an ai-authoring-request envelope.

    Checks:
    - Schema validation against ai-authoring-request-v1.json.
    - Required logical fields: intent, objectKind, targetRepo, tier, schemaRef, referencedIds.
    - Basic invariants / metrics block presence and band checks against the spine (if available).
    - Coherence between tier declarations in envelope and any embedded manifest hints (shallow).
    """
    if repo_root is None:
        repo_root = _find_repo_root(Path(__file__).resolve())

    req_schema = _load_core_schema(repo_root, AI_REQUEST_SCHEMA_NAME)
    spine_index = _load_spine_index(repo_root)

    diags: List[Diagnostic] = []

    if req_schema is None:
        diags.append(
            Diagnostic(
                severity="error",
                code="MISSING_REQUEST_SCHEMA",
                message=f"Could not find {AI_REQUEST_SCHEMA_NAME} under schemas/core/",
            )
        )
        return ValidationResult(diagnostics=diags)

    validator = _build_validator(req_schema, base_uri=req_schema.get("$id"))
    diags.extend(_validate_against_schema(obj, validator))

    required_fields = [
        "intent",
        "objectKind",
        "targetRepo",
        "tier",
        "schemaRef",
        "referencedIds",
    ]
    diags.extend(_require_fields(obj, required_fields))

    contract_family = obj.get("objectKind")
    tier = obj.get("tier") if isinstance(obj.get("tier"), str) else None

    diags.extend(
        _validate_invariants_and_metrics_block(
            obj=obj,
            spine_index=spine_index,
            contract_family=contract_family,
            tier=tier,
            invariants_field="intendedInvariants",
            metrics_field="intendedMetrics",
            pointer_base="",
        )
    )

    manifest_tier = obj.get("manifestTier")
    if isinstance(manifest_tier, str) and tier is not None and manifest_tier != tier:
        diags.append(
            Diagnostic(
                severity="error",
                code="TIER_MISMATCH",
                message=f"Declared tier '{tier}' does not match manifestTier '{manifest_tier}'",
                json_pointer="/manifestTier",
                field="manifestTier",
                value=manifest_tier,
            )
        )

    return ValidationResult(diagnostics=diags)


def _extract_primary_artifacts(resp: Dict[str, Any]) -> List[Dict[str, Any]]:
    """
    Extract primary artifact(s) from a response envelope.

    Expected patterns (examples):
    - resp["artifact"] as a single object.
    - resp["artifacts"] as an array where a field like isPrimary marks the main one.
    """
    artifacts: List[Dict[str, Any]] = []

    artifact = resp.get("artifact")
    if isinstance(artifact, dict):
        artifacts.append(artifact)

    artifacts_array = resp.get("artifacts")
    if isinstance(artifacts_array, list):
        primaries = [
            a for a in artifacts_array
            if isinstance(a, dict) and a.get("isPrimary") is True
        ]
        if primaries:
            artifacts.extend(primaries)
        else:
            artifacts.extend(a for a in artifacts_array if isinstance(a, dict))

    return artifacts


def _validate_prism_meta(prism_meta: Dict[str, Any]) -> List[Diagnostic]:
    """
    Envelope-only checks for prism-style metadata.

    This function does not rely on the full prisma schema; it just enforces
    essential provenance and routing fields.
    """
    diags: List[Diagnostic] = []

    required = [
        "prismId",
        "sessionId",
        "agentId",
        "agentProfileId",
        "targetRepo",
        "path",
        "tier",
    ]
    diags.extend(_require_fields(prism_meta, required, prefix_pointer="/prismMeta"))

    return diags


def validate_response_envelope(
    obj: Dict[str, Any],
    repo_root: Optional[Path] = None,
) -> ValidationResult:
    """
    Validate an ai-authoring-response envelope plus its primary artifact.

    Checks:
    - Schema validation against ai-authoring-response-v1.json (if present).
    - Exactly one primary artifact (one-file-per-request discipline).
    - Prism metadata completeness (if present).
    - Invariants and metrics of the primary artifact are present and within bands.
    - Tier and manifestTier coherence at envelope level.
    """
    if repo_root is None:
        repo_root = _find_repo_root(Path(__file__).resolve())

    resp_schema = _load_core_schema(repo_root, AI_RESPONSE_SCHEMA_NAME)
    spine_index = _load_spine_index(repo_root)

    diags: List[Diagnostic] = []

    if resp_schema is not None:
        validator = _build_validator(resp_schema, base_uri=resp_schema.get("$id"))
        diags.extend(_validate_against_schema(obj, validator))

    artifacts = _extract_primary_artifacts(obj)
    if len(artifacts) == 0:
        diags.append(
            Diagnostic(
                severity="error",
                code="NO_PRIMARY_ARTIFACT",
                message="Response envelope does not contain any primary artifact",
            )
        )
    elif len(artifacts) > 1:
        diags.append(
            Diagnostic(
                severity="error",
                code="MULTIPLE_PRIMARY_ARTIFACTS",
                message="Response envelope contains more than one primary artifact; expected exactly one",
            )
        )

    prism_meta = obj.get("prismMeta")
    if isinstance(prism_meta, dict):
        diags.extend(_validate_prism_meta(prism_meta))

    tier = obj.get("tier") if isinstance(obj.get("tier"), str) else None
    manifest_tier = obj.get("manifestTier")
    if isinstance(manifest_tier, str) and tier is not None and manifest_tier != tier:
        diags.append(
            Diagnostic(
                severity="error",
                code="TIER_MISMATCH",
                message=f"Response tier '{tier}' does not match manifestTier '{manifest_tier}'",
                json_pointer="/manifestTier",
                field="manifestTier",
                value=manifest_tier,
            )
        )

    if artifacts:
        primary = artifacts[0]
        contract_family = primary.get("objectKind") or obj.get("objectKind")
        diags.extend(
            _validate_invariants_and_metrics_block(
                obj=primary,
                spine_index=spine_index,
                contract_family=contract_family,
                tier=tier,
                invariants_field="invariantBindings",
                metrics_field="metricTargets",
                pointer_base="/artifact",
            )
        )

    return ValidationResult(diagnostics=diags)


def _print_diagnostics(result: ValidationResult) -> None:
    """
    Emit diagnostics as JSONL to stdout (machine-readable).
    """
    for d in result.diagnostics:
        obj = asdict(d)
        compact = {k: v for k, v in obj.items() if v is not None}
        sys.stdout.write(json.dumps(compact, ensure_ascii=False) + "\n")


def main(argv: Optional[List[str]] = None) -> int:
    """
    CLI entrypoint.

    Intended usage:
        python -m schemaspine.aiauthoringvalidator request path/to/request.json
        python -m schemaspine.aiauthoringvalidator response path/to/response.json
    """
    if argv is None:
        argv = sys.argv[1:]

    if len(argv) < 2 or argv[0] not in ("request", "response"):
        sys.stderr.write(
            "Usage: python -m schemaspine.aiauthoringvalidator "
            "(request|response) path/to/envelope.json\n"
        )
        return 2

    mode = argv[0]
    path = Path(argv[1])

    if not path.is_file():
        sys.stderr.write(f"Envelope file not found: {path}\n")
        return 2

    try:
        obj = _load_json(path)
    except Exception as exc:
        sys.stderr.write(f"Failed to read envelope JSON: {exc}\n")
        return 2

    repo_root = _find_repo_root(Path.cwd())

    if mode == "request":
        result = validate_request_envelope(obj, repo_root=repo_root)
    else:
        result = validate_response_envelope(obj, repo_root=repo_root)

    _print_diagnostics(result)
    has_errors = any(d.severity == "error" for d in result.diagnostics)
    return 1 if has_errors else 0


if __name__ == "__main__":
    raise SystemExit(main())
