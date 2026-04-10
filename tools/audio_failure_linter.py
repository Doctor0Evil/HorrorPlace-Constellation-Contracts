#!/usr/bin/env python3
import json
import sys
from pathlib import Path
from typing import Any, Dict, List, Tuple

def load_json(path: Path) -> Dict[str, Any]:
    with path.open("r", encoding="utf-8") as fh:
        return json.load(fh)

def approx_in_band(value: float, band: Dict[str, float]) -> bool:
    return band["min"] <= value <= band["max"]

def tile_matches_invariant_signature(tile: Dict[str, Any],
                                     signature: Dict[str, Dict[str, float]]) -> bool:
    for key, band in signature.items():
        v = tile.get("invariants", {}).get(key)
        if v is None:
            return False
        if not approx_in_band(float(v), band):
            return False
    return True

def metrics_match_signature(metrics: Dict[str, float],
                            signature: Dict[str, Dict[str, float]]) -> bool:
    for key, band in signature.items():
        v = metrics.get(key)
        if v is None:
            return False
        if not approx_in_band(float(v), band):
            return False
    return True

def load_ndjson(path: Path) -> List[Dict[str, Any]]:
    rows: List[Dict[str, Any]] = []
    with path.open("r", encoding="utf-8") as fh:
        for line in fh:
            line = line.strip()
            if not line:
                continue
            try:
                rows.append(json.loads(line))
            except json.JSONDecodeError:
                continue
    return rows

def lint_region_tiles(region_tiles: List[Dict[str, Any]],
                      atlas: Dict[str, Any]) -> List[Dict[str, Any]]:
    violations: List[Dict[str, Any]] = []
    entries = atlas.get("entries", [])
    for tile in region_tiles:
        tile_role = tile.get("tileRole")
        region_id = tile.get("regionId")
        tile_id = tile.get("tileId")
        metrics = tile.get("metrics", {})
        for entry in entries:
            if tile_role not in entry.get("tileRoles", []):
                continue
            inv_sig = entry.get("invariantSignature", {})
            met_sig = entry.get("metricSignature", {})
            if inv_sig and not tile_matches_invariant_signature(tile, inv_sig):
                continue
            if met_sig and not metrics_match_signature(metrics, met_sig):
                continue
            violations.append({
                "regionId": region_id,
                "tileId": tile_id,
                "failureId": entry["failureId"],
                "category": entry["category"],
                "label": entry.get("label", "")
            })
    return violations

def main() -> None:
    if len(sys.argv) < 4:
        print("usage: audio_failure_linter.py ATLAS_JSON INVARIANT_MAP_NDJSON OUTPUT_JSON",
              file=sys.stderr)
        sys.exit(1)

    atlas_path = Path(sys.argv[1])
    ndjson_path = Path(sys.argv[2])
    out_path = Path(sys.argv[3])

    atlas = load_json(atlas_path)
    tiles = load_ndjson(ndjson_path)

    violations = lint_region_tiles(tiles, atlas)

    with out_path.open("w", encoding="utf-8") as fh:
        json.dump({"violations": violations}, fh, indent=2)

    if violations:
        print(f"audio_failure_linter: {len(violations)} violations found", file=sys.stderr)
        sys.exit(1)
    print("audio_failure_linter: no violations found")
    sys.exit(0)

if __name__ == "__main__":
    main()
