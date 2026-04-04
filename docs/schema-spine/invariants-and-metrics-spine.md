# Invariants and Metrics Spine

This document catalogs the canonical invariants and entertainment metrics defined in the HorrorPlace constellation, their semantic meanings, valid ranges, and usage contexts. All definitions are normative and enforced by `schemas/core/invariants-spine.v1.json` and `schemas/core/entertainment-metrics-spine.v1.json`.

## Invariants

Invariants are unbreakable rules derived from historical, geographical, or metaphysical constraints. They govern what *can* and *cannot* occur within a region, event, or persona.

### CIC: Collective Invariant of Consequence
- **Type**: `number` (float)
- **Range**: `[0.0, 1.0]`
- **Semantics**: Probability-weighted measure of historical impact. Higher values indicate regions/events where collective trauma or consequence is more likely to manifest.
- **Usage**: Required in `regionContractCard`, `seedContractCard`. Used by `HorrorPlace-Black-Archivum` for trauma inference, `HorrorPlace-Atrocity-Seeds` for PCG weighting.
- **Validation**: Must be derived from normalized historical data; never hardcoded. Telemetry tracks CIC drift via `runtime-drift-event.v1.json`.

### AOS: Actualized Omission Severity
- **Type**: `number` (float)
- **Range**: `[0.0, 10.0]`
- **Semantics**: Intensity of unresolved historical trauma or omission. Represents the "pressure" of unacknowledged events seeking expression.
- **Usage**: Required in `regionContractCard`. Drives spectral entity behavior in `HorrorPlace-Spectral-Foundry`.
- **Validation**: Must correlate with archival trauma data; values >8.0 require vault-tier gating and `deadledgerref` attestation.

### DET: Dynamic Event Tension
- **Type**: `number` (float)
- **Range**: `[0.0, 10.0]` (integer steps preferred)
- **Semantics**: Real-time tension level for an event or encounter. Governs pacing, escalation, and player agency constraints.
- **Usage**: Optional in `regionContractCard`, required in `event-contract.v1.json`. Consumed by runtime engines for dynamic difficulty adjustment.
- **Validation**: Must be mutable at runtime but bounded; telemetry logs DET transitions for ARR calculation.

### SHCI: Spectral-Historical Consistency Index
- **Type**: `number` (float)
- **Range**: `[0.0, 1.0]`
- **Semantics**: Measure of alignment between a spectral entity's behavior and its historical source material. SHCI≈1.0 indicates high-fidelity historical coupling.
- **Usage**: Required in `persona-contract.v1.json`, `HorrorPlace-Spectral-Foundry` outputs.
- **Validation**: Computed from telemetry (UEC, EMD) and archival alignment; values <0.3 trigger review in `HorrorPlace-Process-Gods-Research`.

## Entertainment Metrics

Metrics quantify player experience, system performance, and narrative efficacy. They are captured in telemetry envelopes and feed back into contract refinement.

### UEC: User Engagement Continuity
- **Type**: `number` (float)
- **Range**: `[0.0, 100.0]`
- **Semantics**: Session-level measure of engagement stability. High UEC indicates sustained immersion; low UEC signals disengagement or confusion.
- **Telemetry**: Captured in `session-metrics-envelope.v1.json` at 30-second intervals.
- **Usage**: Required in `policyEnvelope`, `regionContractCard`. Drives adaptive content delivery in `HorrorPlace-Orchestrator`.

### EMD: Experience Modulation Delta
- **Type**: `number` (float)
- **Range**: `[-1.0, 1.0]`
- **Semantics**: Rate of change in player emotional valence. Positive = increasing tension/engagement; negative = release/boredom.
- **Telemetry**: Derived from BCI/fMRI data in `HorrorPlace-Redacted-Chronicles` (lab-tier only).
- **Usage**: Optional in `policyEnvelope`; used by `HorrorPlace-Neural-Resonance-Lab` for haptic/audio tuning.

### STCI: Spectral Threat Confidence Interval
- **Type**: `object`
- **Properties**:
  - `lower`: `number` [0.0, 1.0]
  - `upper`: `number` [0.0, 1.0]
  - `confidence`: `number` [0.0, 1.0]
- **Semantics**: Statistical confidence interval for perceived threat level from spectral entities. Wider intervals indicate uncertainty in threat modeling.
- **Usage**: Required in `event-contract.v1.json`, consumed by runtime threat-assessment systems.
- **Validation**: `lower <= upper`; `confidence` must reflect sample size from telemetry.

### CDL: Cognitive Dissonance Level
- **Type**: `number` (float)
- **Range**: `[0.0, 10.0]`
- **Semantics**: Measure of narrative or mechanical contradiction experienced by the player. High CDL can enhance horror but risks confusion.
- **Telemetry**: Inferred from player choice patterns, pause frequency, and replay behavior.
- **Usage**: Optional in `seedContractCard`; monitored by `HorrorPlace-Obscura-Nexus` for experimental style validation.

### ARR: Adaptive Resonance Ratio
- **Type**: `number` (float)
- **Range**: `[0.0, 1.0]`
- **Semantics**: Ratio of player adaptation to system adaptation. ARR≈1.0 indicates perfect synchrony; ARR≈0.0 indicates mismatch.
- **Calculation**: `ARR = min(UEC_normalized, DET_normalized) / max(UEC_normalized, DET_normalized)`
- **Usage**: Required in `policyEnvelope` for vault-tier constellations; drives `HorrorPlace-Liminal-Continuum` agent-sharing decisions.

## Cross-Field Constraints

Certain invariants and metrics interact via logical constraints enforced by schema validation:

1. **CIC × AOS → DET Cap**: `DET <= min(10, AOS + (CIC * 5))`
   - Prevents tension escalation beyond historical plausibility.
2. **SHCI × UEC → ARR Floor**: `ARR >= SHCI * (UEC / 100)`
   - Ensures high-fidelity entities maintain engagement.
3. **CDL × STCI → Review Threshold**: If `CDL > 7.0 AND STCI.upper > 0.8`, flag for `HorrorPlace-Process-Gods-Research` review.
   - Prevents overwhelming dissonance in high-threat scenarios.

These constraints are encoded in `schemas/core/invariants-spine.v1.json` via `allOf`/`if-then` JSON Schema keywords and validated by `hpc-validate-schema.py --mode ai-authoring`.

## Telemetry Integration

All metrics are captured in strictly typed envelopes:

- `session-metrics-envelope.v1.json`: Per-session aggregates of UEC, EMD, CDL.
- `runtime-drift-event.v1.json`: Flags invariant violations (e.g., DET out of range) with context.

Telemetry flows:
```
Runtime Engine → session-metrics-envelope → Horror.Place-Orchestrator → spine index update → contract refinement
```

## Versioning and Deprecation

- Invariants/metrics are versioned with their defining schema (e.g., `invariants-spine.v1.json`).
- Deprecation follows the policy in `docs/overview/versioning-and-stability.md`.
- Deprecated fields retain `"deprecated": true` and `"deprecationNote"` in the schema; tools emit warnings but do not fail.

## Related Documents

- `schemas/core/invariants-spine.v1.json`: Canonical invariant definitions.
- `schemas/core/entertainment-metrics-spine.v1.json`: Canonical metric definitions.
- `schemas/telemetry/session-metrics-envelope.v1.json`: Telemetry capture format.
- `docs/tooling/prismMeta-and-agentProfiles.md`: How invariants/metrics bind to AI authoring.
