"""
Schema spine index builder for CHAT_DIRECTOR-compatible constellations.

Responsible for:
- Walking schemas/core/**/*.json
- Extracting canonical metadata from each JSON Schema
- Loading invariants- and entertainment-metrics spine JSON
- Emitting a normalized schema-spine-index-v1.json

This module is intentionally side-effect free on import.
Use `build_and_write_index()` or the module CLI entrypoint.
"""

from __future__ import annotations

import json
import sys
from dataclasses import dataclass, asdict, field
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple


REPO_ROOT_SENTINELS = {".git", ".github"}
CORE_SCHEMAS_DIR = Path("schemas") / "core"
SPINE_OUTPUT_NAME = "schema-spine-index-v1.json"
INVARIANTS_SPINE_NAME = "invariants-spine.v1.json"
METRICS_SPINE_NAME = "entertainment-metrics-spine.v1.json"


@dataclass
class TierApplicability:
    tier: str
    applicable: bool = True


@dataclass
class SchemaEntry:
    schema_id: str
    file_path: str
    title: Optional[str] = None
    object_kind: Optional[str] = None
    contract_family: Optional[str] = None
    tier_applicability: List[TierApplicability] = field(default_factory=list)
    invariants: List[str] = field(default_factory=list)
    metrics: List[str] = field(default_factory=list)


@dataclass
class InvariantBand:
    key: str
    contract_family: Optional[str]
    tier: Optional[str]
    min: Optional[float]
    max: Optional[float]


@dataclass
class MetricBand:
    key: str
    contract_family: Optional[str]
    tier: Optional[str]
    min: Optional[float]
    max: Optional[float]


@dataclass
class SchemaSpineIndex:
    version: str
    schemas: List[SchemaEntry]
    invariants_catalog: List[InvariantBand]
    metrics_catalog: List[MetricBand]


def _find_repo_root(start: Path) -> Path:
    current = start
    while current != current.parent:
        if any((current / s).exists() for s in REPO_ROOT_SENTINELS):
            return current
        current = current.parent
    return start


def _iter_schema_files(core_dir: Path) -> List[Path]:
    if not core_dir.is_dir():
        return []
    return sorted(core_dir.rglob("*.json"))


def _load_json(path: Path) -> Any:
    with path.open("r", encoding="utf-8") as f:
        return json.load(f)


def _extract_schema_id_and_title(schema: Dict[str, Any]) -> Tuple[Optional[str], Optional[str]]:
    raw_id = schema.get("$id") or schema.get("id")
    title = schema.get("title")
    return raw_id, title


def _extract_object_kind(schema: Dict[str, Any]) -> Tuple[Optional[str], Optional[str]]:
    """
    Extract object kind and contract family from extension fields.

    Expected patterns (best-effort):
    - x-objectKind: "regionContractCard"
    - x-contractFamily: "region"
    """
    object_kind = schema.get("x-objectKind")
    contract_family = schema.get("x-contractFamily")

    # Fallback heuristics for contract_family if not provided explicitly.
    if contract_family is None and isinstance(object_kind, str):
        if object_kind.endswith("ContractCard"):
            contract_family = object_kind.replace("ContractCard", "").lower()
        elif object_kind.endswith("Contract"):
            contract_family = object_kind.replace("Contract", "").lower()

    return object_kind, contract_family


def _extract_tier_applicability(schema: Dict[str, Any]) -> List[TierApplicability]:
    """
    Extract tier applicability from custom annotations.

    Expected shapes (examples):
    - "x-tierApplicability": ["standard", "mature"]
    - "x-tierApplicability": [{"tier": "standard", "applicable": true}, ...]
    """
    tiers_raw = schema.get("x-tierApplicability")
    result: List[TierApplicability] = []

    if tiers_raw is None:
        return result

    if isinstance(tiers_raw, list):
        for item in tiers_raw:
            if isinstance(item, str):
                result.append(TierApplicability(tier=item, applicable=True))
            elif isinstance(item, dict):
                tier = item.get("tier")
                applicable = bool(item.get("applicable", True))
                if isinstance(tier, str):
                    result.append(TierApplicability(tier=tier, applicable=applicable))
    elif isinstance(tiers_raw, str):
        result.append(TierApplicability(tier=tiers_raw, applicable=True))

    return result


def _load_invariants_spine(path: Path) -> List[InvariantBand]:
    """
    Load invariants-spine.v1.json and convert to catalog entries.

    Expected minimal structure (example, flexible by design):

    {
      "version": "v1",
      "invariants": [
        {
          "key": "CIC",
          "contractFamilies": [
            {
              "family": "region",
              "tiers": [
                {"tier": "standard", "min": 0.0, "max": 1.0}
              ]
            }
          ]
        },
        ...
      ]
    }
    """
    if not path.is_file():
        return []

    data = _load_json(path)
    invariants = data.get("invariants", [])
    catalog: List[InvariantBand] = []

    for inv in invariants:
        key = inv.get("key")
        families = inv.get("contractFamilies") or []
        if not isinstance(key, str):
            continue
        if not families:
            catalog.append(
                InvariantBand(
                    key=key,
                    contract_family=None,
                    tier=None,
                    min=inv.get("min"),
                    max=inv.get("max"),
                )
            )
            continue

        for fam in families:
            family_name = fam.get("family")
            tiers = fam.get("tiers") or []
            if not tiers:
                catalog.append(
                    InvariantBand(
                        key=key,
                        contract_family=family_name,
                        tier=None,
                        min=fam.get("min"),
                        max=fam.get("max"),
                    )
                )
                continue
            for tier_entry in tiers:
                catalog.append(
                    InvariantBand(
                        key=key,
                        contract_family=family_name,
                        tier=tier_entry.get("tier"),
                        min=tier_entry.get("min"),
                        max=tier_entry.get("max"),
                    )
                )

    return catalog


def _load_metrics_spine(path: Path) -> List[MetricBand]:
    """
    Load entertainment-metrics-spine.v1.json and convert to catalog entries.

    Expected minimal structure (example, flexible by design):

    {
      "version": "v1",
      "metrics": [
        {
          "key": "UEC",
          "contractFamilies": [
            {
              "family": "region",
              "tiers": [
                {"tier": "standard", "min": 0.0, "max": 1.0}
              ]
            }
          ]
        },
        ...
      ]
    }
    """
    if not path.is_file():
        return []

    data = _load_json(path)
    metrics = data.get("metrics", [])
    catalog: List[MetricBand] = []

    for metric in metrics:
        key = metric.get("key")
        families = metric.get("contractFamilies") or []
        if not isinstance(key, str):
            continue
        if not families:
            catalog.append(
                MetricBand(
                    key=key,
                    contract_family=None,
                    tier=None,
                    min=metric.get("min"),
                    max=metric.get("max"),
                )
            )
            continue

        for fam in families:
            family_name = fam.get("family")
            tiers = fam.get("tiers") or []
            if not tiers:
                catalog.append(
                    MetricBand(
                        key=key,
                        contract_family=family_name,
                        tier=None,
                        min=fam.get("min"),
                        max=fam.get("max"),
                    )
                )
                continue
            for tier_entry in tiers:
                catalog.append(
                    MetricBand(
                        key=key,
                        contract_family=family_name,
                        tier=tier_entry.get("tier"),
                        min=tier_entry.get("min"),
                        max=tier_entry.get("max"),
                    )
                )

    return catalog


def _extract_invariants_and_metrics_for_schema(
    schema: Dict[str, Any],
    contract_family: Optional[str],
    invariants_catalog: List[InvariantBand],
    metrics_catalog: List[MetricBand],
) -> Tuple[List[str], List[str]]:
    """
    Derive which invariant and metric keys are relevant to a given schema.

    Strategy:
    - Prefer explicit annotations:
        - x-invariants: ["CIC", "AOS", ...]
        - x-metrics: ["UEC", "ARR", ...]
    - Otherwise, if a contract_family is known, infer from catalogs by family.
    """
    invariant_keys: List[str] = []
    metric_keys: List[str] = []

    x_invariants = schema.get("x-invariants")
    x_metrics = schema.get("x-metrics")

    if isinstance(x_invariants, list):
        invariant_keys = [k for k in x_invariants if isinstance(k, str)]
    if isinstance(x_metrics, list):
        metric_keys = [k for k in x_metrics if isinstance(k, str)]

    if not invariant_keys and contract_family:
        invariant_keys = sorted(
            {
                band.key
                for band in invariants_catalog
                if band.contract_family == contract_family
            }
        )

    if not metric_keys and contract_family:
        metric_keys = sorted(
            {
                band.key
                for band in metrics_catalog
                if band.contract_family == contract_family
            }
        )

    return invariant_keys, metric_keys


def build_index(repo_root: Optional[Path] = None) -> SchemaSpineIndex:
    """
    Build an in-memory SchemaSpineIndex from the concrete schemas and spines.

    This function does not write to disk; use build_and_write_index() instead
    when integrating with CLI or CI workflows.
    """
    if repo_root is None:
        repo_root = _find_repo_root(Path(__file__).resolve())

    core_dir = repo_root / CORE_SCHEMAS_DIR

    invariants_spine_path = core_dir / INVARIANTS_SPINE_NAME
    metrics_spine_path = core_dir / METRICS_SPINE_NAME

    invariants_catalog = _load_invariants_spine(invariants_spine_path)
    metrics_catalog = _load_metrics_spine(metrics_spine_path)

    schema_files = _iter_schema_files(core_dir)

    schema_entries: List[SchemaEntry] = []

    for path in schema_files:
        try:
            schema = _load_json(path)
        except Exception:
            # Non-schema JSON files in core/ are ignored but could be logged by callers.
            continue

        schema_id, title = _extract_schema_id_and_title(schema)
        if not schema_id:
            continue

        object_kind, contract_family = _extract_object_kind(schema)
        tier_applicability = _extract_tier_applicability(schema)
        invariants, metrics = _extract_invariants_and_metrics_for_schema(
            schema=schema,
            contract_family=contract_family,
            invariants_catalog=invariants_catalog,
            metrics_catalog=metrics_catalog,
        )

        relative_path = str(path.relative_to(repo_root))

        entry = SchemaEntry(
            schema_id=schema_id,
            file_path=relative_path,
            title=title,
            object_kind=object_kind,
            contract_family=contract_family,
            tier_applicability=tier_applicability,
            invariants=invariants,
            metrics=metrics,
        )
        schema_entries.append(entry)

    index = SchemaSpineIndex(
        version="schema-spine-index-v1",
        schemas=schema_entries,
        invariants_catalog=invariants_catalog,
        metrics_catalog=metrics_catalog,
    )
    return index


def write_index(index: SchemaSpineIndex, repo_root: Optional[Path] = None) -> Path:
    """
    Serialize a SchemaSpineIndex to schemas/core/schema-spine-index-v1.json
    under the given repo root.
    """
    if repo_root is None:
        repo_root = _find_repo_root(Path(__file__).resolve())

    core_dir = repo_root / CORE_SCHEMAS_DIR
    core_dir.mkdir(parents=True, exist_ok=True)

    output_path = core_dir / SPINE_OUTPUT_NAME

    # Convert dataclasses to plain dicts, including nested ones.
    def _convert(obj: Any) -> Any:
        if hasattr(obj, "__dataclass_fields__"):
            return {k: _convert(v) for k, v in asdict(obj).items()}
        if isinstance(obj, list):
            return [_convert(x) for x in obj]
        if isinstance(obj, dict):
            return {k: _convert(v) for k, v in obj.items()}
        return obj

    payload = _convert(index)

    with output_path.open("w", encoding="utf-8") as f:
        json.dump(payload, f, indent=2, sort_keys=True)

    return output_path


def build_and_write_index(repo_root: Optional[Path] = None) -> Path:
    """
    Convenience function used by CLI wrappers and CI.

    - Builds the in-memory index from current schemas and spines
    - Writes schema-spine-index-v1.json under schemas/core/
    - Returns the output path
    """
    index = build_index(repo_root=repo_root)
    return write_index(index=index, repo_root=repo_root)


def main(argv: Optional[List[str]] = None) -> int:
    """
    Minimal CLI entrypoint for direct module invocation:

        python -m schemaspine.spineindexbuilder

    The Python-level hpc-generate-spine-index.py wrapper should call
    build_and_write_index() and handle exit codes or logging as needed.
    """
    try:
        build_and_write_index()
    except Exception as exc:
        sys.stderr.write(f"[spineindexbuilder] error: {exc}\n")
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
