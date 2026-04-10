---
invariants_used:
  - CIC
  - MDI
  - AOS
  - RRM
  - FCF
  - SPR
  - RWF
  - DET
  - HVF
  - LSG
  - SHCI
metrics_used:
  - UEC
  - EMD
  - STCI
  - CDL
  - ARR
tiers:
  - standard
  - mature
  - research
deadledger_surface:
  - zkpproof_schema
  - bundle_attestation
  - agent_attestation
---

# Systemic Timing Stack v1

This document defines the systemic timing stack for the HorrorPlace VM-constellation, specifying how timing signals TMI, RPI, ADI, and STI are derived from existing invariants and entertainment metrics, how they are exposed through the canonical H.* API surface, and how their behavior is validated via NDJSON telemetry. It is a protocol description only; concrete proofs live in Dead-Ledger zkpproof envelopes.

## 1. Position in the Spine

Systemic timing lives strictly above the geo-historical invariant spine and the entertainment metrics layer.

- Invariants describe world structure and history: CIC, MDI, AOS, RRM, FCF, SPR, RWF, DET, HVF, LSG, SHCI.
- Entertainment metrics describe the player’s experiential state: UEC, EMD, STCI, CDL, ARR.
- Systemic timing signals are **derived, session-scoped** functions of invariants, metrics, and event timelines. They are not new base invariants and must not be added to the invariant spine schemas.

Each timing signal is defined per session and, where applicable, per region or window in a session-experience-envelope.

- TMI: Timing Momentum Index – how strongly the session is currently “pushing forward” in terms of event cadence and DET exposure.
- RPI: Recovery Phase Index – how strongly the session is currently in a recovery or decompression phase.
- ADI: Anticipation Dissonance Index – how far the player’s expectations (from recent ARR/UEC history) diverge from what the system is actually doing.
- STI: Shock Timing Index – how “primed” the system is to deliver a high-impact Surprise.Event!, given current DET, LSG, and metric history.

These signals exist to make timing decisions explainable in CIC/DET/LSG/UEC/ARR space and to provide simple scalars that Lua and Rust code, as well as AI-generated code, can read without re-implementing timing analysis.

## 2. Canonical Engine API Surface

The engine_api_surface_v1.json schema (defined elsewhere in this repo) must declare the canonical, engine-agnostic timing API under the H.* namespace.

For systemic timing, the required entries are:

- H.TMI(session_id, region_id | null) -> number in [0, 1]
- H.RPI(session_id, region_id | null) -> number in [0, 1]
- H.ADI(session_id, region_id | null) -> number in [0, 1]
- H.STI(session_id, region_id | null) -> number in [0, 1]

All four functions:

- Are pure from the caller’s perspective: they do not trigger events; they only read telemetry and cached invariants.
- Return normalized values in [0, 1], clamped by the helper implementation.
- Are monotonically non-decreasing in their main “pressure” arguments (detailed below) for fixed context, up to clamping.

Each engine (Godot, Unreal, custom Death-Engine) must provide an adapter that implements these signatures, but the engine_api_surface_v1.json does not require all functions to be implemented on every platform. Capability discovery is handled via the capability spine.

### 2.1 Capability Spine Binding

The capability spine document (capabilities.json) must list systemic timing capabilities with stable IDs, for example:

- capability_id: timing.TMI.v1
  - function: H.TMI
  - inputs: session_id, region_id?
  - range: [0, 1]
- capability_id: timing.RPI.v1
  - function: H.RPI
  - inputs: session_id, region_id?
  - range: [0, 1]
- capability_id: timing.ADI.v1
  - function: H.ADI
  - inputs: session_id, region_id?
  - range: [0, 1]
- capability_id: timing.STI.v1
  - function: H.STI
  - inputs: session_id, region_id?
  - range: [0, 1]

Authoring contracts (ai-safe-authoring-contract-v1, discovery-contract-v1, pacing-contract, session-experience-envelope) use capabilitySpineRef to declare which version of the timing stack they are written against.

## 3. Mathematical Definitions

All definitions below are intentionally simple and monotone to make them easy to implement, verify, and tune empirically. They are expressed as normalized functions over a recent time window W (e.g., last 5–10 minutes of session time), sampled from NDJSON telemetry.

Let:

- t be current session time.
- W be a fixed window length in minutes (e.g., 10), chosen per product but recorded in telemetry.
- Events(t) be the set of horror events in [t − W, t], tagged with DET and type.
- DET(t) be the player’s recent Dread Exposure Threshold trajectory, normalized to [0, 1] by dividing by 10.
- UEC(t), ARR(t) be the latest values of UEC and ARR, in [0, 1].
- N(t) be the count of events in Events(t).
- N_high(t) be the count of events in Events(t) whose DET >= det_high_threshold (e.g., 0.7 normalized).
- idle_time(t) be the total minutes in [t − W, t] without any events.
- expected_events(t) be a smoothed expected event count for the window, derived from pacing-contract or session-experience-envelope.

The concrete parameter values (window length, thresholds, smoothing constants) are owned by configuration and tracked in telemetry as formula parameters.

### 3.1 Timing Momentum Index (TMI)

Intuition: TMI is high when many events are happening recently, with high DET, especially in high-LSG tiles, and ARR is not collapsing to zero.

We define:

1. Event density term:
   - density(t) = clamp01(N(t) / N_ref)
   - N_ref is a reference number of events per window (e.g., targetEventsPerWindow from pacing-contract).

2. High-DET intensity term:
   - intensity(t) = clamp01(N_high(t) / max(1, N_ref_high))
   - N_ref_high is a reference number of high-DET events per window.

3. Liminal weighting term:
   - liminal_mean(t) = average over Events(t) of LSG at each event’s region/tile, normalized in [0, 1].
   - liminal_weight(t) = liminal_mean(t).

4. ARR stability term:
   - arr_stability(t) = clamp01(ARR(t) / arr_target)
   - arr_target is the desired ARR band midpoint from session-experience-envelope.

Then:

- raw_TMI(t) = w_d * density(t) + w_i * intensity(t) + w_l * liminal_weight(t) + w_a * arr_stability(t)

with weights w_d, w_i, w_l, w_a >= 0 and w_d + w_i + w_l + w_a = 1. Finally:

- TMI(t) = clamp01(raw_TMI(t))

Monotonicity properties:

- For fixed configuration and metrics, increasing N(t) or N_high(t) cannot decrease TMI(t).
- For fixed event counts, increasing LSG at event locations cannot decrease TMI(t).
- For fixed others, increasing ARR(t) up to arr_target cannot decrease TMI(t); beyond arr_target the effect is bounded by clamp01.

### 3.2 Recovery Phase Index (RPI)

Intuition: RPI is high when the system is in a deliberate cooldown: low recent DET, low event rate, and rising ARR.

Define:

1. Quietness term:
   - quiet_fraction(t) = clamp01(idle_time(t) / W)

2. Low-DET dominance term:
   - det_mean(t) = mean over Events(t) of normalized DET, or 0 if no events.
   - low_det_term(t) = 1 − det_mean(t)

3. ARR recovery term:
   - arr_recovery(t) = clamp01((ARR(t) − ARR_min_recent) / max(ε, ARR_target − ARR_min_recent))
   - ARR_min_recent is the minimum ARR observed in [t − W, t].
   - ARR_target is the target ARR band midpoint.
   - ε is a small constant to avoid division by zero.

Then:

- raw_RPI(t) = v_q * quiet_fraction(t) + v_l * low_det_term(t) + v_a * arr_recovery(t)
- RPI(t) = clamp01(raw_RPI(t))

Monotonicity properties:

- For fixed others, increasing idle_time(t) cannot decrease RPI(t).
- For fixed others, decreasing det_mean(t) cannot decrease RPI(t).
- For fixed others, increasing ARR(t) (for ARR(t) >= ARR_min_recent) cannot decrease RPI(t).

TMI and RPI are not forced to sum to 1; they are independent indicators.

### 3.3 Anticipation Dissonance Index (ADI)

Intuition: ADI measures mismatch between how much resolution the player expects versus what the system is providing. High ADI means “the system is zigging where the player expects a zag”.

We approximate “expectation” using short-window trends of ARR and UEC compared to a pacing baseline.

Define:

1. Expected resolution term:
   - baseline_arr(t) = ARR target band midpoint for the current experience window.
   - expected_resolution(t) = clamp01((baseline_arr(t) − ARR(t)) / baseline_arr(t))
   - When ARR is below baseline, expected_resolution is high: the player “expects” more closure.

2. Delivered resolution term:
   - delivered_resolution(t) = fraction of events in [t − W, t] that ended with clear resolution, based on event tags. This is in [0, 1].

3. Dissonance term:
   - dissonance(t) = |expected_resolution(t) − delivered_resolution(t)|

4. UEC tension term:
   - tension(t) = clamp01(UEC(t) / uec_target)
   - uec_target is the target UEC band midpoint.

Then:

- raw_ADI(t) = d_d * dissonance(t) + d_u * tension(t)
- ADI(t) = clamp01(raw_ADI(t))

Monotonicity properties:

- For fixed delivered_resolution(t), increasing the gap between baseline_arr(t) and ARR(t) (i.e., lowering ARR) cannot decrease ADI(t).
- For fixed dissonance, increasing UEC(t) up to uec_target cannot decrease ADI(t).

### 3.4 Shock Timing Index (STI)

Intuition: STI is the system’s readiness to fire a high-impact Surprise.Event! given that the player has recovered enough, local invariants justify it, and anticipation has been built.

We define:

1. Local dread readiness term:
   - det_peak(t) = normalized maximum DET in [t − W, t].
   - dread_ready(t) = det_peak(t).

2. Liminal readiness term:
   - lsg_peak(t) = maximum LSG seen in [t − W, t].
   - liminal_ready(t) = lsg_peak(t).

3. Recovery gating term:
   - rpi_term(t) = RPI(t). Shock should not fire if RPI is very low; we use RPI as a gate.

4. ADI contribution term:
   - adi_term(t) = ADI(t). High anticipation dissonance suggests a good moment to subvert expectations.

5. SHCI consistency term:
   - shci_local(t) = mean SHCI over candidate loci for Surprise.Events! in the current region.
   - shci_term(t) = shci_local(t), ensuring shocks remain history-coherent.

Then:

- base_STI(t) = s_d * dread_ready(t) + s_l * liminal_ready(t) + s_a * adi_term(t) + s_s * shci_term(t)
- gated_STI(t) = base_STI(t) * g(rpi_term(t))

where g(x) is a monotone gate in [0, 1], for example g(x) = clamp01((x − rpi_min) / (1 − rpi_min)) for some rpi_min threshold (e.g., 0.2). Finally:

- STI(t) = clamp01(gated_STI(t))

Monotonicity properties:

- For fixed others, increasing det_peak(t) cannot decrease STI(t).
- For fixed others, increasing lsg_peak(t) cannot decrease STI(t).
- For fixed others, increasing ADI(t) cannot decrease STI(t).
- For fixed base_STI, increasing RPI(t) cannot decrease STI(t).

## 4. Lua Helper Contracts

The canonical Lua helper module (e.g., engine/systemic_timing.lua) must implement H.TMI, H.RPI, H.ADI, and H.STI using the definitions above, with the following constraints:

- Inputs:
  - session_id (string) and optional region_id (string) must be used to select telemetry slices.
  - The module reads only from:
    - Invariant helpers: H.DET, H.LSG, H.SHCI, and optionally H.SPR/RWF for derived relationships.
    - Telemetry helpers: H.Telemetry.getUEC, getARR, and event logs for the session.
    - Config: timing-formula configuration (weights, thresholds, window length, IDs).

- Output:
  - Each function returns a number in [0, 1].
  - All intermediate calculations that may leave [0, 1] must be wrapped in a clamp01 function.

- Monotonicity:
  - Functions must respect the monotonicity properties described in section 3. Small floating-point deviations are acceptable; gross violations are not.
  - The implementation must avoid non-monotone operations (e.g., oscillatory trigonometric functions) over the main pressure variables.

- Versioning:
  - Each formula implementation must have a formula_id (for example: timing.TMI.v1.linear) stored in configuration.
  - The helper must report its formula_id to telemetry for each systemic-timing-formula-run.v1 record.

Engines may choose to implement these helpers in Rust and expose them via FFI to Lua; the contract above still applies at the API boundary.

## 5. Telemetry: systemic-timing-formula-run.v1

The systemic-timing-formula-run.v1 schema (defined in schemas/telemetry/systemic-timing-formula-run.v1.json) records the runtime behavior of the timing stack for empirical validation and tuning.

Each NDJSON line represents a single evaluation of the timing formulas for a particular session and time tick.

### 5.1 Required Fields (Conceptual)

- schemaFamily: "systemic-timing-formula-run"
- schemaVersion: semantic version string
- sessionId: string (matches session-experience-envelope.sessionId)
- regionId: string (optional; "*" or null for global)
- tickId: string (monotonically increasing per session)
- tSessionMinutes: number (minutes since session start)
- formulaConfigId: string (ID for the configuration bundle used)
- tmi:
  - value: number in [0, 1]
  - formulaId: string
- rpi:
  - value: number in [0, 1]
  - formulaId: string
- adi:
  - value: number in [0, 1]
  - formulaId: string
- sti:
  - value: number in [0, 1]
  - formulaId: string
- invariantsSlice:
  - detMean: number in [0, 1]
  - detPeak: number in [0, 1]
  - lsgMean: number in [0, 1]
  - lsgPeak: number in [0, 1]
  - shciLocal: number in [0, 1]
- metricsSlice:
  - uec: number in [0, 1]
  - arr: number in [0, 1]
  - stci: number in [0, 1]
  - cdl: number in [0, 1]
- eventStats:
  - windowMinutes: number
  - eventCount: integer
  - highDetEventCount: integer
  - deliveredResolutionFraction: number in [0, 1]
  - idleMinutes: number
- decisions:
  - firedHighImpactEvent: boolean
  - suppressedHighImpactEvent: boolean
  - notes: optional string (for debugging only)

The real JSON schema enforces numeric ranges, required fields, and string patterns, matching the style of existing telemetry schemas in this repo.

### 5.2 Use in Validation

Offline analyzers read systemic-timing-formula-run.v1 streams and:

- Check that all TMI, RPI, ADI, STI values are within [0, 1].
- Check approximate monotonicity properties by comparing changes in the main pressure variables with changes in the timing signals.
- Correlate STI spikes with actual firedHighImpactEvent decisions to estimate calibration quality.
- Compare observed timing behavior against the bands and budgets declared in:
  - session-experience-envelope windows (for example, bounds on high-DET events per window).
  - pacing-contract (for example, maxHighDetEventsPer10Min).

Violations (for example, systematic overuse of shocks at high STI, or shocks fired at low STI) are recorded as analysis artifacts and may become new constraints in future schema revisions or Dead-Ledger proof envelopes.

## 6. Contracts and Authoring Constraints

### 6.1 session-experience-envelope

The session-experience-envelope schema may reference timing bands implicitly but must not include TMI/RPI/ADI/STI as first-class fields. Instead:

- Windows define bands for DET, UEC, EMD, STCI, CDL, ARR.
- Timing behavior is monitored via systemic-timing-formula-run.v1 and checked against these bands.

Authoring guidance:

- A session-experience-envelope may include qualitative hints for timing (for example, “slow build”, “rupture”), but all numeric constraints are expressed in terms of DET/metrics and event counts.

### 6.2 pacing-contract

The pacing-contract schema is where timing behavior is made explicit in contracts. It may:

- Define targetEventsPerMinute bands and maxHighDetEventsPer10Min, as already specified.
- Optionally define advisory bands for TMI, RPI, ADI, STI per window, expressed as functions of DET and metric bands but not as spine fields.

Example:

- Instead of storing “TMI_min” directly, store a rule: “if DET_band and event density bands are satisfied, TMI must be >= 0.6”; this rule can be enforced by an analyzer using systemic-timing-formula-run telemetry.

### 6.3 AI Authoring and CHAT_DIRECTOR

CHAT_DIRECTOR and AI authoring envelopes must respect the following:

- AI-generated code in Rust or Lua that uses timing must:
  - Declare engine_api_surface_v1.json as its capabilitySpineRef.
  - Use only H.TMI, H.RPI, H.ADI, H.STI for timing signals, not internal event logs or ad-hoc time calculations.
- Authoring contracts must not propose new timing primitives; they can only configure formula parameters and reference existing timing signals by capability ID.

CI rules:

- Static analysis ensures AI-authored Lua/Rust uses H.* timing functions instead of custom ones.
- Validation ensures that any configuration changes to timing formulas are within declared bounds (for example, weights remain in [0, 1] and sum to 1).

## 7. Dead-Ledger and Proof Envelopes

For high-intensity tiers (for example, Tier3Vault), systemic timing behavior may be subject to Dead-Ledger attestation.

- Dead-Ledger proof envelopes may:
  - Attest that a given formulaConfigId has been evaluated on representative telemetry and satisfies timing constraints with respect to session-experience-envelope and pacing-contract bands.
  - Provide anonymous aggregate evidence (for example, distributions of STI at which shocks fired) without exposing raw player telemetry.

- Contracts carry an opaque deadledgerref pointing to such an envelope; runtime systems never inspect proofs directly.

This keeps timing behavior within the same governance model as other high-impact mechanics, while preserving privacy and engine-agnostic design.

---

End of Systemic Timing Stack v1.
