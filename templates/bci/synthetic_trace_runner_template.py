#!/usr/bin/env python3
"""
Template: Synthetic EEG feature trace runner for Horror$Place.

Behavior:
- Loads a SyntheticEEGFeatureTrace (synthetic-eeg-feature-trace-v1) and expectations.
- Feeds frames into a headless HorrorDirector-like mapping function.
- Asserts on tension, policy caps, and cooldown events.

This template is backend-agnostic and focuses on wiring tests, not runtime engines.
"""

import json
from dataclasses import dataclass
from typing import Any, Dict, List, Optional, Tuple


@dataclass
class HorrorPolicyCaps:
    max_tension: float
    max_rate_of_change: float
    min_cooldown_duration: int


@dataclass
class TraceFrame:
    index: int
    eeg: Dict[str, Any]


@dataclass
class DirectorState:
    tension: float = 0.0
    cooldown_active: bool = False
    cooldown_remaining: int = 0


def load_synthetic_trace(path: str) -> Dict[str, Any]:
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def extract_frames(trace: Dict[str, Any]) -> List[TraceFrame]:
    frames = []
    raw_frames = trace.get("frames", [])
    for i, frame in enumerate(raw_frames):
        frames.append(TraceFrame(index=i, eeg=frame))
    return frames


def load_policy_caps(trace: Dict[str, Any]) -> HorrorPolicyCaps:
    policy = trace.get("policy_caps", {})
    return HorrorPolicyCaps(
        max_tension=float(policy.get("max_tension", 0.8)),
        max_rate_of_change=float(policy.get("max_rate_of_change", 0.2)),
        min_cooldown_duration=int(policy.get("min_cooldown_duration", 5)),
    )


def compute_tension_from_eeg(eeg: Dict[str, Any]) -> float:
    """
    Example mapping from EEGFeatureContract-compatible payload to tension.

    Replace with:
    - The same mapping logic your Unity or Unreal HorrorDirector uses.
    """
    composite = eeg.get("composite", {})
    horror_ctx = eeg.get("horror_context", {})

    stress = float(composite.get("stress", 0.0))
    cic = float(horror_ctx.get("CIC", 0.0))

    baseline = 0.1
    tension = baseline + 0.8 * stress + 0.4 * cic
    return max(0.0, min(1.0, tension))


def step_director(state: DirectorState, eeg: Dict[str, Any], caps: HorrorPolicyCaps) -> DirectorState:
    """
    Headless HorrorDirector step that:
    - Computes target tension from EEG.
    - Applies cooldown logic and caps.
    """
    previous_tension = state.tension
    target_tension = compute_tension_from_eeg(eeg)

    if state.cooldown_active:
        target_tension = min(target_tension, previous_tension)
        state.cooldown_remaining -= 1
        if state.cooldown_remaining <= 0:
            state.cooldown_active = False

    tension_delta = max(-caps.max_rate_of_change, min(caps.max_rate_of_change, target_tension - previous_tension))
    new_tension = previous_tension + tension_delta
    new_tension = max(0.0, min(caps.max_tension, new_tension))

    horror_ctx = eeg.get("horror_context", {})
    mdi = float(horror_ctx.get("MDI", 0.0))
    if mdi >= 0.9 and not state.cooldown_active:
        state.cooldown_active = True
        state.cooldown_remaining = caps.min_cooldown_duration

    state.tension = new_tension
    return state


def assert_policy_invariants(
    history: List[DirectorState],
    caps: HorrorPolicyCaps,
    expectations: Dict[str, Any],
) -> List[str]:
    errors: List[str] = []

    for i in range(1, len(history)):
        prev = history[i - 1]
        cur = history[i]

        if cur.tension > caps.max_tension + 1e-6:
            errors.append(f"Frame {i}: tension {cur.tension:.3f} exceeds max_tension {caps.max_tension:.3f}")

        rate = cur.tension - prev.tension
        if abs(rate) > caps.max_rate_of_change + 1e-6:
            errors.append(
                f"Frame {i}: rate of change {rate:.3f} exceeds max_rate_of_change {caps.max_rate_of_change:.3f}"
            )

    expected_cooldowns = int(expectations.get("min_cooldown_events", 0))
    observed_cooldowns = sum(1 for s in history if s.cooldown_active)
    if expected_cooldowns > 0 and observed_cooldowns < expected_cooldowns:
        errors.append(
            f"Cooldown events: expected at least {expected_cooldowns}, observed {observed_cooldowns}"
        )

    return errors


def run_trace(trace_path: str) -> Tuple[bool, List[str]]:
    trace = load_synthetic_trace(trace_path)
    frames = extract_frames(trace)
    caps = load_policy_caps(trace)
    expectations = trace.get("expectations", {})

    state = DirectorState()
    history: List[DirectorState] = []

    for frame in frames:
        eeg = frame.eeg
        state = step_director(state, eeg, caps)
        history.append(DirectorState(tension=state.tension,
                                     cooldown_active=state.cooldown_active,
                                     cooldown_remaining=state.cooldown_remaining))

    errors = assert_policy_invariants(history, caps, expectations)
    return len(errors) == 0, errors


def main() -> None:
    import argparse

    parser = argparse.ArgumentParser(description="Synthetic EEG feature trace runner template.")
    parser.add_argument("trace", help="Path to synthetic-eeg-feature-trace-v1 JSON file.")
    args = parser.parse_args()

    success, errors = run_trace(args.trace)
    if success:
        print("[synthetic_trace_runner] All invariants satisfied.")
    else:
        print("[synthetic_trace_runner] Invariant violations:")
        for err in errors:
            print(f"  - {err}")
        raise SystemExit(1)


if __name__ == "__main__":
    main()
