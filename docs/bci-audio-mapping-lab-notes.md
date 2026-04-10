# BCI Audio Mapping Lab Notes

This note describes how to author BCI audio mapping profiles against the HorrorMappingConfig.BCI.v1 and audio RTPC mapping families, and how Kotlin ranking jobs should consume the resulting telemetry summaries.

The focus here is on BCI → audio mappings that drive RTPC channels such as `pressure`, `whisperDensity`, and `ritualMotif` using canonical BCI metrics and invariants, under strict schema and safety constraints.

---

## 1. Authoring BCI Audio Mapping Profiles

### 1.1. Inputs and outputs

A BCI audio mapping profile is a `HorrorMappingConfig.BCI.v1` profile whose `channel` outputs are audio RTPCs rather than gameplay fields like `sprintMul` or `tunnelRadius`.

Typical inputs:

- BCI metrics from `bci-metrics-envelope-v2.json`:  
  - `fearIndex`, `calmScore`, `stressScore`, `attentionFocus`.  
  - Canonical entertainment metrics: `UEC`, `EMD`, `STCI`, `CDL`, `ARR`.  
- Invariants from the horror context snapshot:  
  - `CIC`, `AOS`, `LSG`, `HVF`, `SHCI`, `DET`.

Typical outputs:

- RTPC channels controlled by audio mapping families:  
  - `pressure`, `whisperDensity`, `hissLevel`, `staticBed`, `ritualMotif`.  
- Optional “meta” channels for lab use (gated by tier and schema).

Each mapping entry in a profile:

- Selects one or more inputs (metrics and invariants).  
- Chooses a mapping family (`linear`, `sigmoid`, `hysteresis`, `metricaware`, etc.).  
- Declares parameters, safety bounds, and output range (`yMin`, `yMax`).  
- Includes optional gating conditions on UEC/ARR/CDL and BCI intensity mode.

### 1.2. Family selection guidelines

Use mapping families conservatively:

- `linear` (`0xPKLIN`):  
  - Good for low‑to‑mid range changes in `pressure` or `hissLevel`.  
  - Keep gain `|a|` within policy bounds so that `|Δy|` stays smooth under typical metric changes.  
- `sigmoid` (`0xPKSIG`):  
  - Good for softly entering higher tension bands based on `fearIndex` or `UEC`.  
  - Use `k` (steepness) and `x0` (midpoint) to avoid hair‑trigger jumps.  
- `hysteresis` (`0xPKHYS`):  
  - Good for stabilizing mode changes (e.g., switching between low and mid tension soundscapes).  
  - Use narrow but nonzero hysteresis loops; make loop width a function of `DET` and `SHCI` when appropriate.  
- `metricaware` / generic multi‑input families:  
  - Blend multiple metrics and invariants to drive channels like `whisperDensity`.  
  - Respect sign conventions (e.g., CDL weight negative when mapping to tension, ARR weight positive or neutral).

In all cases:

- Clamp outputs to \[0, 1] or the RTPC band specified by the mood contract.  
- Provide `maxSlopePerSec` or similar velocity caps in the profile’s `safety` block.  
- Use `disableWhenOverloadFlag` and `allowedIntensityModes` to protect overloaded states.

### 1.3. Profile structure for audio mappings

A typical `HorrorMappingConfig.BCI.v1` profile for audio might look like:

- `profileId`: `bci.audio.profile.subdued_dread.v1`.  
- `conditions`:
  - `minCIC`, `maxCIC` selecting corridor/threshold regions.  
  - `allowedIntensityModes`: `CALM`, `FOCUSED`.  
  - `disableWhenOverloadFlag`: `true`.  
- `mappings`:
  - `pressure`:
    - Family: `metricaware` (weights on `fearIndex`, `UEC`, `CIC`).  
    - Output range: within mood’s `pressure` band (e.g., 0.15–0.40).  
    - Gating: only active if `UEC` in a configured band.  
  - `whisperDensity`:
    - Family: `sigmoid` on `fearIndex`.  
    - Soft threshold: starts rising at moderate `fearIndex`, saturates below overload.  
  - `hissLevel`:
    - Family: `linear` on `attentionFocus`, moderated by `AOS`.  
- `safety`:
  - `maxSlopePerSec` for each channel.  
  - `exposureCaps` on time spent above certain thresholds.

Authoring practice:

- Anchor `yMin` and `yMax` to the mood’s RTPC bands (e.g., from `moodcontract.subdued_dread.v1`).  
- Use invariants like `CIC` and `AOS` to shift mapping behavior by region type rather than hard‑coding region IDs.  
- For combat profiles (e.g., `bci.audio.profile.combat_nightmare.v1`), require intensity modes including `TENSE` and stricter overload gating.

### 1.4. Safety and policy conformance

Profiles must satisfy:

- Schema bounds on parameters (`a`, `b`, `k`, `x0`, weights norms).  
- Differential inequality safety envelopes (e.g., tension growth and cooldown bounds) enforced in CI.  
- BCI intensity policy constraints (caps on exposure, overload, high tension time).

In authoring terms:

- Treat `a`, `k`, weights, and oscillatory amplitudes as constrained knobs with known safe ranges.  
- Use lab tiers (`tier = research`) for experimental families (oscillatory, noise‑augmented) and keep production audio profiles on linear/logistic / hysteresis families until validated.

---

## 2. Telemetry and Summary Objects

### 2.1. Per‑frame telemetry

The Rust mapping engine must emit per‑frame audio mapping telemetry (e.g., `audio-rtpc-mapping-telemetry-frame-v1`) with fields:

- Context:
  - `sessionId`, `frameTime`, `regionClass`, `personaId`, `bciPhase`.  
  - `moodId`, `profileId`, `familyId`.  
- Inputs:
  - `metricsBefore` and `metricsAfter` (UEC, EMD, STCI, CDL, ARR).  
  - `bciInputs` (`fearIndex`, `calmScore`, `overloadFlag`, etc.).  
  - `invariantsSnapshot` (CIC, AOS, LSG, DET, SHCI, etc.).  
- Outputs:
  - `rtpcOutputs` map (e.g., `pressure`, `whisperDensity`, `hissLevel`, `ritualMotif`).  
- Safety:
  - Any internal flags where slope or exposure caps were applied or clamped.

These per‑frame records are the raw material for lab analysis and ranking.

### 2.2. Summary objects

Kotlin ranking jobs should consume summary NDJSON (e.g., `audio-rtpc-mapping-summary-v1`) keyed by:

- `(profileId, regionClass, personaId, bciPhaseBand)`.

Each summary object should include:

- Aggregated metrics:
  - `E[ΔUEC]`, `E[ΔARR]`, `E[ΔEMD]`, `E[ΔSTCI]`, `E[ΔCDL]`.  
  - `Pr(overload)` and overload severity estimates.  
  - Time fractions in different BCI intensity modes (CALM, FOCUSED, TENSE, OVERLOADED).  
- RTPC behavior:
  - Distribution summaries for each channel (mean, variance, percentiles, fraction of time above thresholds).  
  - Velocity statistics (distribution of `|Δrtpc/Δt|`).  
- Failure Atlas flags:
  - Indicators informed by the Audio Failure Atlas, such as:
    - `flat_corridor`: near‑zero RTPC variance in regions expecting subtle tension.  
    - `lifeless_firefight`: low `pressure` and `whisperDensity` despite high DET in combat.  
    - `jump_scare_spam`: frequent large spikes and overload in mid‑intensity contexts.  
- Coverage:
  - Number of sessions and frames contributing to the summary.  
  - Distribution of region classes and persona states.

---

## 3. Kotlin Ranking Jobs

### 3.1. Context‑aware scoring

Kotlin ranking jobs should rank profiles in a context‑aware manner:

- For each context `(regionClass, personaId, bciPhaseBand)`:

  - Filter summary objects to that context.  
  - Compute a composite score `S(profile, context)` that rewards:

    - Positive uplift in UEC and ARR within desired bands.  
    - Appropriate STCI and CDL behavior (suspense and confusion levels in their intended ranges).  
    - Low overload probability and adherence to BCI intensity policy.

  - Penalize:

    - High overload risk or DET over‑exposure.  
    - Failure patterns flagged by the Audio Failure Atlas.

Profiles are then:

- Ranked per context.  
- Decorated with descriptive labels (“good corridor tension”, “strong combat uplift”, “mid‑intensity overload risk”) based on these metrics and flags.

### 3.2. Using failure patterns

Kotlin must interpret summary flags:

- `flat_corridor`:

  - RTPC variance and metric deltas below thresholds in corridor regions.  
  - Profiles with repeated flat corridor flags should be down‑ranked or rejected for corridor contexts.

- `lifeless_firefight`:

  - Underdriven RTPC levels and low STCI/EMD uplift in high‑DET combat.  
  - Profiles should be demoted or excluded from combat contexts.

- `jump_scare_spam`:

  - Excessive large jumps and peaks; high overload risk in mid‑intensity contexts.  
  - Profiles should be confined to experimental tiers or demoted entirely from relevant contexts.

These flags are not optional metadata; they should feed directly into selection logic (e.g., profiles with any failure flag cannot be candidates for “safe default” attestation in that context).

### 3.3. Evolution and promotion

Over time:

- Profiles start in lab tiers (`tier = research`, narrow context).  
- Kotlin jobs gather data, compute `S(profile, context)`, and check failure flags.  
- Orchestrator and Dead‑Ledger use these results to:

  - Promote profiles to `mature` or `standard` tiers for specific contexts.  
  - Extend context coverage where performance is good and safe.  
  - Revoke or down‑tier profiles if new data reveals failure patterns or safety issues.

Authoring teams should:

- Treat every profile as a hypothesis about how BCI and invariants should shape audio.  
- Use summary objects and rankings as the empirical evidence to keep or discard those hypotheses.  
- Iterate on parameter sets within the existing family codes instead of introducing new families.

---

## 4. Practical Lab Workflow

1. **Design phase**  
   - Choose one or more mapping families and initial parameters for each audio RTPC.  
   - Ensure inputs are limited to canonical BCI metrics and invariants.  
   - Set `yMin`, `yMax`, and velocity caps aligned with mood RTPC bands and BCI policy.

2. **Schema and safety validation**  
   - Validate the profile JSON against `HorrorMappingConfig.BCI.v1`.  
   - Run mapping safety CI (differential inequality checks, Lipschitz and slope bounds).  
   - Reject or adjust profiles that fail numeric safety gates.

3. **Data collection**  
   - Deploy the profile in lab or limited live sessions.  
   - Collect per‑frame mapping telemetry and aggregate into summary objects.

4. **Ranking and decision**  
   - Run Kotlin ranking jobs to compute `S(profile, context)` and failure flags.  
   - Decide whether to:
     - Promote the profile for certain contexts.  
     - Retune parameters and re‑test.  
     - Retire the profile if performance is consistently poor or unsafe.

By following this loop, BCI audio mapping remains a contract‑bound, telemetry‑driven process, with Rust enforcing safety, Kotlin providing quantitative ranking, and schemas in this repository anchoring the entire pipeline.
