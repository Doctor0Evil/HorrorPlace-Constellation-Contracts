#!/usr/bin/env python3
import json
import math
import sys
from pathlib import Path
from typing import Any, Dict, List, Tuple


CHARter_ARR_MIN = 0.05  # Charter-aligned global minimum ARR floor (normalized)


def load_json(path: Path) -> Dict[str, Any]:
    with path.open("r", encoding="utf-8") as fh:
        return json.load(fh)


def find_ritual_files(root: Path) -> List[Path]:
    if not root.exists():
        return []
    return [p for p in root.rglob("*.json") if p.is_file()]


def get_stage_metric_bands(ritual: Dict[str, Any]) -> Dict[str, Dict[str, float]]:
    bands = ritual.get("stageMetricBands") or ritual.get("stage_metric_bands") or {}
    out: Dict[str, Dict[str, float]] = {}
    for stage in ["probe", "evidence", "confrontation", "aftermath", "residual"]:
        band = bands.get(stage, {})
        if isinstance(band, dict):
            out[stage] = band
    return out


def get_stage_intensity(ritual: Dict[str, Any]) -> Dict[str, Dict[str, Any]]:
    env = ritual.get("stageIntensityEnvelopes") or ritual.get("stage_intensity_envelopes") or {}
    out: Dict[str, Dict[str, Any]] = {}
    for stage in ["probe", "evidence", "confrontation", "aftermath", "residual"]:
        band = env.get(stage, {})
        if isinstance(band, dict):
            out[stage] = band
    return out


def approx_ge(a: float, b: float, eps: float = 1e-9) -> bool:
    return a + eps >= b


def lint_monotone_stci_cdl(
    ritual_id: str,
    bands: Dict[str, Dict[str, float]],
) -> List[str]:
    """
    Enforce STCI/CDL monotonicity up to confrontation:
    Probe <= Evidence <= Confrontation (for both minima).
    """
    errors: List[str] = []
    stages = ["probe", "evidence", "confrontation"]

    stci_vals: List[Tuple[str, float]] = []
    cdl_vals: List[Tuple[str, float]] = []

    for stage in stages:
        band = bands.get(stage) or {}
        stci_min = band.get("STCI_min")
        cdl_min = band.get("CDL_min")
        if isinstance(stci_min, (int, float)):
            stci_vals.append((stage, float(stci_min)))
        if isinstance(cdl_min, (int, float)):
            cdl_vals.append((stage, float(cdl_min)))

    # Only lint if we have values for all three stages.
    if len(stci_vals) == 3:
        for (s_prev, v_prev), (s_next, v_next) in zip(stci_vals, stci_vals[1:]):
            if not approx_ge(v_next, v_prev):
                errors.append(
                    f"[{ritual_id}] STCI_min monotonicity violated: "
                    f"{s_prev}={v_prev} -> {s_next}={v_next} (expected non-decreasing)."
                )

    if len(cdl_vals) == 3:
        for (s_prev, v_prev), (s_next, v_next) in zip(cdl_vals, cdl_vals[1:]):
            if not approx_ge(v_next, v_prev):
                errors.append(
                    f"[{ritual_id}] CDL_min monotonicity violated: "
                    f"{s_prev}={v_prev} -> {s_next}={v_next} (expected non-decreasing)."
                )

    return errors


def lint_probe_intensity_non_zero(
    ritual_id: str,
    intensity_env: Dict[str, Dict[str, Any]],
) -> List[str]:
    """
    Enforce non-zero Probe intensity when a ritual declares non-null intensity profiles.
    If a Probe envelope exists, audio/visual mins must be > 0.
    """
    errors: List[str] = []

    probe = intensity_env.get("probe") or {}
    if not probe:
        return errors

    audio = probe.get("audioIntensity") or {}
    visual = probe.get("visualIntensity") or {}

    audio_min = audio.get("min")
    visual_min = visual.get("min")

    if isinstance(audio_min, (int, float)) and audio_min <= 0.0:
        errors.append(
            f"[{ritual_id}] Probe audioIntensity.min={audio_min} "
            f"must be > 0 when a Probe intensity profile is defined."
        )

    if isinstance(visual_min, (int, float)) and visual_min <= 0.0:
        errors.append(
            f"[{ritual_id}] Probe visualIntensity.min={visual_min} "
            f"must be > 0 when a Probe intensity profile is defined."
        )

    return errors


def lint_arr_min_charter(
    ritual_id: str,
    bands: Dict[str, Dict[str, float]],
) -> List[str]:
    """
    Enforce Charter-aligned ARR minima across all stages.
    ARR_min must be >= global Charter minimum when defined.
    """
    errors: List[str] = []

    for stage, band in bands.items():
        arr_min = band.get("ARR_min")
        if not isinstance(arr_min, (int, float)):
            continue
        val = float(arr_min)
        if val + 1e-9 < CHARter_ARR_MIN:
            errors.append(
                f"[{ritual_id}] {stage} ARR_min={val} is below Charter minimum "
                f"{CHARter_ARR_MIN}."
            )

    return errors


def lint_file(path: Path) -> List[str]:
    try:
        ritual = load_json(path)
    except Exception as exc:
        return [f"[{path}] Failed to parse JSON: {exc}"]

    ritual_id = ritual.get("ritualId") or ritual.get("ritual_id") or str(path)
    bands = get_stage_metric_bands(ritual)
    intensity_env = get_stage_intensity(ritual)

    errors: List[str] = []
    errors.extend(lint_monotone_stci_cdl(ritual_id, bands))
    errors.extend(lint_probe_intensity_non_zero(ritual_id, intensity_env))
    errors.extend(lint_arr_min_charter(ritual_id, bands))

    return errors


def main() -> None:
    if len(sys.argv) < 2:
        print(
            "Usage: lint-seed-ritual-stage-bands.py <rituals_root_dir>",
            file=sys.stderr,
        )
        sys.exit(1)

    root = Path(sys.argv[1])
    files = find_ritual_files(root)

    if not files:
        print(f"No ritual JSON files found under {root}")
        sys.exit(0)

    all_errors: List[str] = []
    for f in files:
        errs = lint_file(f)
        if errs:
            for e in errs:
                print(f"{f}: {e}")
            all_errors.extend(errs)

    if all_errors:
        print(
            f"lint-seed-ritual-stage-bands: {len(all_errors)} violation(s) detected.",
            file=sys.stderr,
        )
        sys.exit(1)

    print("lint-seed-ritual-stage-bands: all Seed-Ritual bands passed.")


if __name__ == "__main__":
    main()
