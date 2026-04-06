"""
schemaspine: thin helpers for loading the schema spine and invariant/metric catalogs.

This module is intentionally small and read-only. It:

- Knows where core schemas and spine JSON files live inside the repo.
- Loads and normalizes the schema spine index and invariant / metric spines.
- Exposes simple query helpers for CLI tools and CI workflows.
- Provides a light self-check against CHAT_DIRECTOR's Rust expectations.

It does NOT implement full schema validation, invariant enforcement, or manifest rules.
Those remain in the Rust crate and in dedicated Python modules such as
spineindexbuilder.py, registrylinter.py, and aiauthoringvalidator.py.
"""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple

# ---------------------------------------------------------------------------
# Repository layout assumptions
# ---------------------------------------------------------------------------

_THIS_DIR = Path(__file__).resolve().parent
# tooling/python/schemaspine/__init__.py -> tooling/python/schemaspine -> tooling/python -> tooling -> repo root
_REPO_ROOT = _THIS_DIR.parents[3]

SCHEMAS_ROOT: Path = _REPO_ROOT / "schemas" / "core"
SPINE_INDEX_PATH: Path = SCHEMAS_ROOT / "schema-spine-index-v1.json"
INVARIANTS_SPINE_PATH: Path = SCHEMAS_ROOT / "invariants-spine.v1.json"
METRICS_SPINE_PATH: Path = SCHEMAS_ROOT / "entertainment-metrics-spine.v1.json"


# ---------------------------------------------------------------------------
# Low-level JSON loaders
# ---------------------------------------------------------------------------

def _load_json(path: Path) -> Any:
    with path.open("r", encoding="utf-8") as f:
        return json.load(f)


def load_spine_index(path: Optional[Path] = None) -> Dict[str, Any]:
    """
    Load the schema spine index (schema-spine-index-v1.json) as a Python dict.
    """
    spine_path = path or SPINE_INDEX_PATH
    return _load_json(spine_path)


def load_invariants_spine(path: Optional[Path] = None) -> Dict[str, Any]:
    """
    Load invariants-spine.v1.json, which defines invariant names, abbreviations,
    ranges, categories, and tier overrides.
    """
    inv_path = path or INVARIANTS_SPINE_PATH
    return _load_json(inv_path)


def load_metrics_spine(path: Optional[Path] = None) -> Dict[str, Any]:
    """
    Load entertainment-metrics-spine.v1.json, which defines entertainment metrics,
    their target bands, and applicability to contract families.
    """
    met_path = path or METRICS_SPINE_PATH
    return _load_json(met_path)


def load_schema_by_id(schema_id: str, spine_index: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
    """
    Resolve a JSON Schema by its canonical $id using the spine index, then load it.

    CLI tools should call this instead of hardcoding paths.
    """
    index = spine_index or load_spine_index()
    schemas = index.get("schemas", {})

    entry = schemas.get(schema_id)
    if entry is None:
        raise KeyError(f"Schema id not found in spine index: {schema_id}")

    rel_path = entry.get("path")
    if not rel_path:
        raise KeyError(f"Schema entry for {schema_id} is missing 'path'")

    schema_path = (SCHEMAS_ROOT / rel_path).resolve()
    if not schema_path.is_file():
        raise FileNotFoundError(f"Schema file not found for {schema_id}: {schema_path}")

    return _load_json(schema_path)


# ---------------------------------------------------------------------------
# High-level spine accessors
# ---------------------------------------------------------------------------

def list_contract_families(spine_index: Optional[Dict[str, Any]] = None) -> List[str]:
    """
    Return the list of contract families (objectKind values) known to the spine.

    For v1 this should include:
    - moodContract
    - eventContract
    - regionContractCard
    - seedContractCard
    """
    index = spine_index or load_spine_index()
    families = index.get("contractFamilies") or index.get("contract_families") or []
    return [entry.get("objectKind") for entry in families if entry.get("objectKind")]


def get_invariant_spec(name: str, invariants_spine: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
    """
    Look up a single invariant specification by its name or abbreviation
    (e.g., 'CIC', 'AOS', 'DET', 'LSG', 'SHCI').
    """
    inv = invariants_spine or load_invariants_spine()
    catalog = inv.get("invariants", [])

    for spec in catalog:
        if spec.get("name") == name or spec.get("abbreviation") == name:
            return spec

    raise KeyError(f"Invariant not found: {name}")


def get_metric_spec(name: str, metrics_spine: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
    """
    Look up a single metric specification by its name or abbreviation
    (e.g., 'UEC', 'EMD', 'STCI', 'CDL', 'ARR').
    """
    met = metrics_spine or load_metrics_spine()
    catalog = met.get("entertainmentMetrics") or met.get("metrics") or []

    for spec in catalog:
        if spec.get("name") == name or spec.get("abbreviation") == name:
            return spec

    raise KeyError(f"Metric not found: {name}")


def required_invariants_for(object_kind: str, spine_index: Optional[Dict[str, Any]] = None) -> List[str]:
    """
    Return the set of invariant abbreviations required for a given contract
    family (objectKind), according to the spine index.
    """
    index = spine_index or load_spine_index()
    families = index.get("contractFamilies") or index.get("contract_families") or []

    for entry in families:
        if entry.get("objectKind") == object_kind:
            required = entry.get("requiredInvariants") or entry.get("required_invariants") or []
            return list(required)

    raise KeyError(f"Contract family not found in spine: {object_kind}")


def required_metrics_for(object_kind: str, spine_index: Optional[Dict[str, Any]] = None) -> List[str]:
    """
    Return the set of metric abbreviations required for a given contract
    family (objectKind), according to the spine index.
    """
    index = spine_index or load_spine_index()
    families = index.get("contractFamilies") or index.get("contract_families") or []

    for entry in families:
        if entry.get("objectKind") == object_kind:
            required = entry.get("requiredMetrics") or entry.get("required_metrics") or []
            return list(required)

    raise KeyError(f"Contract family not found in spine: {object_kind}")


def safe_default_bands(
    object_kind: str,
    tier: str,
    spine_index: Optional[Dict[str, Any]] = None,
) -> Dict[str, Tuple[float, float]]:
    """
    Return a mapping name -> (min, max) representing a safe default band for
    invariants/metrics for the given contract family and tier.

    Mirrors the Rust safe_defaults helper and reads from the spine index.
    """
    index = spine_index or load_spine_index()
    defaults = index.get("safeDefaults") or index.get("safe_defaults") or {}

    key = f"{object_kind}:{tier}"
    entry = defaults.get(key)
    if entry is None:
        fallback = defaults.get(object_kind)
        if fallback is None:
            raise KeyError(f"No safe defaults found for {object_kind} at tier {tier}")
        entry = fallback

    result: Dict[str, Tuple[float, float]] = {}
    for name, band in entry.items():
        if isinstance(band, (list, tuple)) and len(band) == 2:
            result[name] = (float(band[0]), float(band[1]))
    return result


# ---------------------------------------------------------------------------
# Self-check against Rust expectations
# ---------------------------------------------------------------------------

# These sets should mirror what the Rust crate expects to see in the spines.
_EXPECTED_FAMILIES = {
    "moodContract",
    "eventContract",
    "regionContractCard",
    "seedContractCard",
}

_EXPECTED_INVARIANTS = {
    "CIC",
    "AOS",
    "DET",
    "LSG",
    "SHCI",
}

_EXPECTED_METRICS = {
    "UEC",
    "EMD",
    "STCI",
    "CDL",
    "ARR",
}


def run_self_check() -> None:
    """
    Perform a lightweight self-check:

    - Confirms all expected contract families are present.
    - Confirms all expected invariants and metrics are present in their spines.

    Raises RuntimeError on failure so CLI wrappers can convert into non-zero exit codes.
    """
    spine_index = load_spine_index()
    inv_spine = load_invariants_spine()
    met_spine = load_metrics_spine()

    families = set(list_contract_families(spine_index))
    missing_families = _EXPECTED_FAMILIES - families
    if missing_families:
        raise RuntimeError(f"Missing contract families in spine index: {sorted(missing_families)}")

    inv_names = set()
    for spec in inv_spine.get("invariants", []):
        abbr = spec.get("abbreviation")
        if abbr:
            inv_names.add(abbr)
    missing_inv = _EXPECTED_INVARIANTS - inv_names
    if missing_inv:
        raise RuntimeError(f"Missing invariants in invariants spine: {sorted(missing_inv)}")

    met_names = set()
    for spec in met_spine.get("entertainmentMetrics", met_spine.get("metrics", [])):
        abbr = spec.get("abbreviation")
        if abbr:
            met_names.add(abbr)
    missing_met = _EXPECTED_METRICS - met_names
    if missing_met:
        raise RuntimeError(f"Missing metrics in metrics spine: {sorted(missing_met)}")


__all__ = [
    "SCHEMAS_ROOT",
    "SPINE_INDEX_PATH",
    "INVARIANTS_SPINE_PATH",
    "METRICS_SPINE_PATH",
    "load_spine_index",
    "load_invariants_spine",
    "load_metrics_spine",
    "load_schema_by_id",
    "list_contract_families",
    "get_invariant_spec",
    "get_metric_spec",
    "required_invariants_for",
    "required_metrics_for",
    "safe_default_bands",
    "run_self_check",
]
