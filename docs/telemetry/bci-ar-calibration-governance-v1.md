---
invariants_used:
  - CIC
  - AOS
  - RRM
  - LSG
  - DET
  - SHCI
metrics_used:
  - UEC
  - EMD
  - STCI
  - CDL
  - ARR
tiers:
  - tier3-research
deadledger_surface:
  - zkpproof_schema
  - verifiers_registry
  - bcistate_proof
  - bundle_attestation
---

# BCI–AR Calibration Governance v1 (Tier 3)

This document specializes the existing BCI and Ten‑Module governance stack for mixed‑reality (MR/AR) horror calibration, NDJSON replay, and DET‑aware safety caps without creating a separate governance track. It defines Tier 3 contracts that describe what “AR BCI horror calibration + NDJSON replay + safety caps” is allowed to do across all engines, while Rust/Lua tools and Unity/Unreal stubs act as mechanical consumers of canonical schemas.

## 1. Role of Tier 3 in BCI–AR work

Tier 3 remains research‑lab only and schema‑first: it defines what tools, traces, and calibration contracts are allowed to exist and how they must look, not how any engine implements them. Schemas and contracts live in HorrorPlace‑Constellation‑Contracts; implementations live in vault repos such as Death‑Engine, Neural‑Resonance‑Lab, Redacted‑Chronicles, and Spectral‑Foundry, with Tier 2 as a pure consumer via CHAT_DIRECTOR, PCG, and engine facades.

For BCI–AR horror, Tier 3 explicitly owns:

- AR‑aware BCI feature and metrics envelope schemas as v2 IDs in the existing telemetry spine (refining, not replacing, bci‑feature‑envelope and bci‑metrics‑envelope).
- Governance contracts for BCI calibration experiments, DET‑aware caps, and NDJSON telemetry / replay tools.
- A BCI calibration profile contract that encodes fear thresholds, UEC/EMD/ARR targets, and DET safety bands per cohort and AR condition, not per build or engine.

This scope is “Module 6 Telemetry + BCI extension”: BCI–AR calibration contracts are part of the telemetry/policy layer, aligned with existing modules rather than a new doctrine.

## 2. Canonical BCI–AR envelope schemas

Tier 3 BCI–AR governance standardizes three envelopes in the schema spine. All are defined as v2 IDs in the existing telemetry namespace so current v1 engine stubs remain valid while AR calibration tools move to v2.

### 2.1 AR‑aware feature envelope (bci-feature-envelope-v2)

**Schema path**

- `schemas/telemetry/bci-feature-envelope-v2.json`

**Relationship to v1**

- Extends the existing `bci-feature-envelope-v1` feature family without breaking it.
- Keeps the rawish, per‑window container semantics: derived features only, no raw waveforms, no PII, `additionalProperties: false`.

**Key fields**

- Core fields (unchanged): `schemaVersion`, `windowId`, `sessionId`, `timestamp`, `samplingRateHz`, `windowDurationSec`, `channelLayout`, and `features[]` with band powers, arousal/valence, and related feature fields.
- Device/signal profile:

  - `signalProfileId`: references a separate signal profile definition (sampling rate, filters, artifact handling).
  - `deviceProfileId`: references a device/headset profile (channels, form factor, vendor).

- AR context (opaque, engine‑agnostic):

  - `arContext.mode`: enum such as `"AR-HMD"`, `"Mobile-AR"`, `"MR-Desktop"`.
  - `arContext.anchorType`: enum such as `"world-locked"`, `"head-locked"`, `"hand-anchored"`.
  - `arContext.visualLoadHint`: coarse classification of visual complexity, e.g. `"low"`, `"medium"`, `"high"`.

These AR fields are tags for calibration and replay only and must not carry world geometry or narrative content; engines interpret them only via higher‑level contracts.

### 2.2 AR metrics envelope (bci-metrics-envelope-v2)

**Schema path**

- `schemas/telemetry/bci-metrics-envelope-v2.json`

**Role**

- Bridge from per‑window features into BCI‑derived entertainment metrics and safety estimates.
- Treats BCI as an additional observation channel for existing bands (UEC, EMD, STCI, CDL, ARR) plus a few derived safety variables.

**Key fields**

- Core linkage: `windowId`, `sessionId`, `timestamp`, `source.featureEnvelopeId?`, `signalProfileId`, `calibrationProfileId`, `mappingProfileId`.
- Metrics (all normalized):

  - `metrics.uecBand`, `metrics.emdBand`, `metrics.stciBand`, `metrics.cdlBand`, `metrics.arrBand` in [0,1], referencing the entertainment metrics spine.
  - Optionally `metrics.fearBand` if needed as a derived scalar, still in [0,1] and documented as a mapping into UEC/EMD/ARR, not a new primary metric.

- Safety block (DET‑aware, derived only):

  - `safety.detEstimate`: estimate of the current dread exposure on the canonical DET scale [0,10] from the invariants spine, never redefining DET itself.
  - `safety.overloadFlag`: boolean indicating sustained DET / fearband beyond calibrated safe band.
  - `safety.underEngagedFlag`: boolean indicating sustained boredom/flatness.
  - `safety.exposureDose`: cumulative exposure dose over a session or segment, in a normalized range, used for Dead‑Ledger policies and intensity caps.

- Transform metadata:

  - `transform.method`: enum (`"ema"`, `"kalman"`, `"static"`, `"ml-classifier"`, `"other"`).
  - `transform.smoothingAlpha?`: EMA alpha in [0,1].
  - `transform.modelId?`: reference into a Tier‑3 model registry.
  - `transform.featureMappingVersion?`: reference to a mapping spec.

Metrics envelopes remain downstream of feature envelopes and upstream of contractCards and Dead‑Ledger verifiers. All scalar ranges are enforced by JSON Schema and CI based on the invariants/metrics spine.

### 2.3 Calibration profile envelope (bci-calibration-profile-envelope-v1)

**Schema path**

- `schemas/telemetry/bci-calibration-profile-envelope-v1.json`

**Scope and placement**

- Tier‑3 research only; instances live in lab repos such as Neural‑Resonance‑Lab and Redacted‑Chronicles, not in public Horror.Place or Dead‑Ledger.
- Describes calibration for cohorts and AR conditions (per‑user or per‑group), never per build.

**Key fields**

- Identifiers and context:

  - `profileId`: stable ID for this calibration profile.
  - `cohortId`: anonymized cohort identifier or pseudonymous subject grouping.
  - `deviceProfileId`: link to headset/sensor configuration.
  - `arMode`: enum mirroring `arContext.mode` for the cohort.
  - `mappingProfileId`: reference to the mapping functions or strategy family used in this calibration.
  - `experimentConfigId`: link to an experiment configuration (tasks, segments) defined elsewhere.

- Observed metric distributions:

  - `observedMetrics.uecBand`, `observedMetrics.emdBand`, `observedMetrics.arrBand`, etc., each described with summary statistics (e.g., mean, variance, percentile bands).
  - `observedSafety.detEstimateDistribution`, `overloadRate`, `recoveryTime` fields capturing DET and safety behavior.

- Recommended caps and bands:

  - `recommended.detBand`: [minDet, maxDet] recommended for this cohort and AR mode.
  - `recommended.uecBand`, `recommended.emdBand`, `recommended.arrBand`: target bands for “good” horror engagement.
  - `recommended.overloadThresholds`: conditions that should trigger intensity reduction, gating, or content changes.

- Governance metadata:

  - `tier`: must be `"tier3-research"` for all calibration profiles.
  - `labId`: lab/project identifier.
  - `ethicsReviewId?`: reference to internal ethics/comfort review.
  - `createdAt`, `updatedAt`, `schemaVersion`.

Calibration profiles are used to drive mapping and safety decisions but are never visible to runtime engines directly; engines see only metrics envelopes and BCI‑intensity policies derived from them.

## 3. NDJSON telemetry and replay contracts

Existing NDJSON tiers (raw, redacted, summary, events) are extended for BCI–AR calibration via synthetic trace schemas and a replay‑tool contract. Synthetic traces themselves belong in Spectral‑Foundry’s schema spine, while the replay contract remains in Constellation‑Contracts.

### 3.1 Synthetic trace schemas

**Schema paths (Spectral‑Foundry)**

- `schemas/telemetry/synthetic-bci-feature-trace-v1.json`
- `schemas/telemetry/synthetic-bci-metrics-trace-v1.json`
- `schemas/telemetry/synthetic-calibration-expectations-v1.json`

**Synthetic feature trace**

Each NDJSON line represents a synthetic time window of BCI features plus AR context, with:

- `syntheticTraceId`: ID of the overall trace.
- `tickIndex` and `dt`: discrete time index and timestep.
- `seed`: deterministic seed used to generate or shuffle this trace.
- `calibrationProfileId`, `mappingProfileId`, `signalProfileId`: explicit links into Tier‑3 contracts.
- Embedded `bci-feature-envelope-v2` object inline or by reference, with the same constraints on PII and raw data.

**Synthetic metrics trace**

Each NDJSON line represents the corresponding metrics output, with:

- The same linking fields as the feature trace (`syntheticTraceId`, `tickIndex`, `seed`, `calibrationProfileId`, etc.).
- Embedded `bci-metrics-envelope-v2` object (or a reduced form that still includes all safety fields and metric bands).
- Optional engine state snapshot: `engineState` block containing normalized tension, region/seed IDs, and intensity caps as seen by the engine.

**Synthetic calibration expectations**

Each line outlines the expected behavior for a synthetic trace under a given mapping profile:

- `expectationId`: ID for the expectation record.
- `syntheticTraceId`, `mappingProfileId`, `calibrationProfileId`.
- Expected trajectories for UEC, EMD, ARR, and DET over time: ranges, trend descriptions, and thresholds.
- Acceptable bounds for overload events, recovery times, and CDL behavior.

These schemas use `additionalProperties: false` and never contain PII or raw waveforms. “Synthetic” in this context means “safe for GitHub and research replay”; they can be entirely simulated or derived / anonymized from empirical data.

### 3.2 NDJSON replay tool contract

**Schema path (Constellation‑Contracts)**

- `schemas/tools/bci-ndjson-replay-contract-v1.json`

**Purpose**

Defines the capabilities and obligations of any NDJSON replay runner (Rust CLIs, Unity/Unreal editor tools, lab harnesses) so they can be validated uniformly.

**Key fields**

- Identity and scope:

  - `toolId`, `version`, `maintainedBy`.
  - `supportedSchemas`: list of telemetry schema IDs (e.g., `bci-feature-envelope-v2`, `synthetic-bci-metrics-trace-v1`).

- Inputs and modes:

  - `inputs.featureTrace`: supported or required.
  - `inputs.metricsTrace`: supported or required.
  - `playbackMode`: supported modes (`"real-time"`, `"fixed-step"`, `"fast-forward"`).
  - `orderingGuarantees`: deterministic playback rules for equal timestamps / tick indices.

- Safety and policy alignment:

  - `safety.applyDetCaps`: boolean, must be true for Tier‑3 tools.
  - `safety.clampBehavior`: documentation of how out‑of‑policy metrics or DET values are clamped, logged, and surfaced.
  - `policyIntegration`: which BCI‑intensity policies and Dead‑Ledger profiles must be consulted during replay.

- Outputs:

  - `outputs.metricsTrace`: whether the tool emits post‑clamp metrics traces.
  - `outputs.deviationReports`: summary of differences between expected and observed metrics for calibration profile refinement.

Replay tools are validated by CI and lab harnesses against this schema and must be referenced from lab repos’ metadata; AI‑generated code for replay tools targets this contract, not ad‑hoc behavior.

## 4. BCI calibration protocols and DET‑aware caps

Tier 3 treats calibration as a contract combining experiment configuration, calibration profile, and BCI‑intensity policy. Raw features and live EEG remain out of scope for governance; only derived envelopes and policies are visible.

### 4.1 Experiment configuration and calibration pairing

Experiment configuration and calibration profiles define:

- Task structure:

  - Segments such as calm baseline, AR scare sequence, recovery phases.
  - AR conditions (mask overlays, hallucination anchoring, environmental density) referenced via AR context tags, not engine‑specific details.

- Feature selection:

  - Which feature bands and modalities contribute to fearband, arousal, and overload estimates (within feature envelopes).

- Governance expectations:

  - How UEC/EMD/ARR and DET are expected to move during each segment for a “valid” calibration run.

Calibration runs log:

- Feature envelopes (`bci-feature-envelope-v2`) per window.
- Candidate metrics envelopes (`bci-metrics-envelope-v2`) per window.
- Segment‑level deltas for UEC/EMD/ARR and DET.

Per‑user baselines, normalization parameters, and fear thresholds are derived from these logs under a BCI‑intensity‑policy schema; the derived calibration profile must respect DET and intensity caps.

### 4.2 BCI‑intensity policy and DET caps

A dedicated policy schema binds DET and entertainment metrics to allowed BCI ranges:

**Schema path**

- `schemas/policy/bci-intensity-policy-v1.json`

**Key fields**

- `id`, `schemaVersion`, `tier` (Tier‑3 or promotion target).
- `detRange`: global DET bounds allowed for this policy (subset of canonical DET [0,10]).
- Metric guardrails:

  - Allowed bands for UEC/EMD/ARR; conditions for overload and under‑engagement.
  - Rules for how BCI‑derived metrics can bias or adjust existing metrics before hitting contractCards and Dead‑Ledger.

- Overload / exposure constraints:

  - Maximum contiguous exposure time above certain DET/UEC/EMD thresholds.
  - Required recovery behavior and caps on cumulative exposureDose per session.

Dead‑Ledger policies and BCI safeguard verifiers continue to operate **only** on derived metrics (fearband, overloadFlag, exposureDose, UEC/EMD/ARR, DET estimates) and never on raw features or device profiles. Any mapping or calibration change that attempts to widen DET or overload limits must be expressed as a policy document change and validated via existing ZKP/safeguard verifiers.

### 4.3 Metrics envelope safety extensions

The bci‑metrics envelope’s safety block captures the variables needed by BCI‑intensity policies and Dead‑Ledger verifiers:

- `safety.detEstimate` and `safety.exposureDose` are computed by mapping engines according to calibration profiles and policies.
- `safety.flags` can include:

  - `flags.overload`: sustained DET/UEC above policy threshold.
  - `flags.burnout`: signs of habituation or fatigue (e.g., declining UEC/ARR).
  - `flags.calibrationDrift`: deviation between expected and observed metrics.

DET’s canonical definition remains in the invariants/metrics spine; calibration and metrics envelopes only estimate and constrain it.

## 5. Rust mapping engine and Lua AI‑playbook hooks

Rust remains the canonical mapping/EMA/calibration engine; Lua acts as the runtime facade; Unity/Unreal stubs mirror these APIs without modifying the semantics. Tier 3 governance pins these surfaces so AI‑chat code generation is mechanical and schema‑constrained.

### 5.1 Rust crate family and responsibilities

Tier 3 defines interfaces (not implementations) for a family of Rust crates that must conform to the BCI envelope schemas and policies:

- `bciconverter`:

  - Converts external EEG/BCI formats into `bci-feature-envelope-v2` NDJSON streams under strict anonymization and schema constraints.
  - Runs only in lab / ingestion contexts, not in game runtime.

- `bciema`:

  - Applies EMA and other smoothing to features; maps them into provisional metrics envelopes (`bci-metrics-envelope-v2`) given a calibration profile.

- `bcicalibration`:

  - Maintains calibration profiles in memory; updates them based on calibration runs and synthetic trace comparisons.
  - Computes recommended DET and metrics caps that must be recorded back into calibration profiles.

All crates expose a narrow C ABI or FFI boundary usable by engine vaults; only this ABI is considered part of the governance layer.

### 5.2 FFI surface for engines and lab harnesses

Tier 3 pins a minimal FFI/API contract, described semantically here (actual headers live in engine repos, but must follow this shape):

- `apply_ema_and_calibration(featureEnvelopeJson) -> metricsEnvelopeJson`:

  - Input: JSON string or struct validated against `bci-feature-envelope-v2`.
  - Output: JSON string or struct validated against `bci-metrics-envelope-v2`.

- `update_calibration_state(calibrationProfileJson, metricsEnvelopeJson) -> updatedCalibrationProfileJson`:

  - For lab only; used to refine calibration profiles offline.

- `evaluate_policy(metricsEnvelopeJson, policyJson) -> metricsEnvelopeJsonWithSafety`:

  - Applies BCI‑intensity policy; clamps values and annotates safety flags.

All FFI calls must treat schemas as authoritative contracts; no ad‑hoc fields are allowed. Engines must never bypass these functions to adjust BCI metrics directly.

### 5.3 Lua modules as the sole BCI runtime facade

Lua modules in Death‑Engine provide the only runtime access to BCI for engine scripts:

- `hpcbciimport.lua`:

  - Reads NDJSON files (typically from synthetic traces or offline conversions).
  - Uses a Rust JSON Schema validator to verify each line against `bci-feature-envelope-v2` and `bci-metrics-envelope-v2`.
  - Forwards valid envelopes to mapping and calibration FFI.

- `hpcbciadapter.lua`:

  - Calls into the Rust FFI (`apply_ema_and_calibration`, `evaluate_policy`).
  - Converts metrics envelopes into normalized values and flags, then exposes them via:

    - `H.BCI.current_metrics()` – returns UEC/EMD/ARR bands, DET estimate, overload/under‑engaged flags, exposureDose.
    - `H.BCI.apply_to_director(directorState)` – optional helper that applies clamped metrics to CHAT_DIRECTOR, PCG budgets, and horror exposure budgets.

Lua code must treat these APIs as read‑only views of BCI; any attempt to manipulate DET or overload beyond policy must be blocked by CI and runtime checks.

### 5.4 AI‑playbook constraints for BCI codegen

An accompanying AI‑playbook (separate doc) must state:

- AI‑generated BCI code may only:

  - Target the envelope schemas defined here.
  - Use the Rust/Lua FFI functions described above.
  - Read BCI metrics via `H.BCI.current_metrics()` and adjust content only via existing metrics/invariants channels.

- AI‑generated code must not:

  - Talk directly to BrainFlow, LSL, or device SDKs.
  - Introduce new BCI fields without corresponding schema updates in Constellation‑Contracts.
  - Bypass DET caps, overload flags, or Dead‑Ledger policy verifiers.

CI lints in engine and lab repos enforce these rules via static analysis and schema validation.

## 6. Synthetic trace libraries and automated calibration refinement

Tier 3 uses synthetic trace libraries to evaluate and refine calibration profiles and mapping functions before any promotion to lower tiers.

### 6.1 Synthetic BCI trace library

A synthetic trace library governed by the schemas in Section 3 contains NDJSON traces that encode:

- BCI features and metrics (via envelopes).
- AR context tags (mode, anchor type, load hints).
- Engine state (region/seed IDs, tension, UEC/EMD/ARR bands, intensity caps).
- Ground truth events or labels (e.g., scripted scare occurrences, “safe” intervals, subjective feedback if available).

These traces are stored and versioned in Spectral‑Foundry / lab repos and serve as canonical inputs for evaluation and regression tests.

### 6.2 Rust replay tools and evaluation loops

Rust replay tools, built on the replay contract, take:

- Synthetic traces (`synthetic-bci-feature-trace-v1`, `synthetic-bci-metrics-trace-v1`).
- Mapping profiles and calibration profiles.
- BCI‑intensity policies.

They then:

- Run deterministic replays to generate predicted metrics envelopes and policy‑clamped outputs.
- Compare observed vs expected trajectories (UEC/EMD/ARR, DET, overload rates).
- Emit deviation logs and aggregate statistics (e.g., “mutation‑gain” signals such as deltaUEC, deltaARR, deltaCDL, overload avoidances).

Only calibration profiles and mapping profiles that show consistent uplift in UEC/ARR and bounded DET/CDL across traces can be promoted from Tier‑3 lab status to Tier‑2/Tier‑1 defaults, under Module 9 governance.

## 7. Unity/Unreal stub alignment

Unity and Unreal are symmetric consumers of normalized BCI metrics; their stubs must mirror each other and treat BCI‑intensity policies as authoritative.

### 7.1 Engine‑agnostic BCI state structs

A minimal, shared BCI state interface (documented in public API stubs) mirrors metrics envelopes:

- Struct fields:

  - `fearIndex` (if used), `uec`, `emd`, `arr`, `stci`, `cdl`.
  - `detEstimate`, `overloadFlag`, `underEngagedFlag`, `exposureDose`.

Engines receive these values from Lua/Rust, not from raw EEG or device APIs. Any engine‑specific fields must be derived locally from these normalized values, not from new BCI inputs.

### 7.2 BCI intensity state machine alignment

Both engines implement an identical BCI‑intensity state machine described by a shared contract (e.g., `schemas/telemetry/bcicore-intensity-envelope-v1.json`):

- Maps BCI metrics and safety flags into discrete states (e.g., CALM, FOCUSED, TENSE, OVERLOADED).
- Defines hysteresis and transition rules.
- Feeds into contractCards, selector policies, and Dead‑Ledger via existing mechanisms.

Divergence between Unity and Unreal implementations is treated as a test bug and must be resolved by aligning with the shared contract and replay tests.

### 7.3 AR‑specific behavior and caps

For AR‑specific behaviors—visual tunnels, spatialized whispers, haptics—BCI influence is expressed only as normalized parameters (0–1) capped by BCI‑intensity policies:

- E.g., “AR tunnel strength” in [0,1] computed from `fearIndex` and `detEstimate` but clamped by policy.
- “Whisper spatialization strength” in [0,1] derived from ARR and UEC observations.

BCI cannot introduce new, uncapped axes of intensity; it can only modulate existing policy‑bound parameters.

## 8. Entertainment metrics and user‑state modeling

BCI‑derived metrics are treated strictly as additional observations for existing entertainment metrics and invariants, not as new metrics.

### 8.1 Observational role of BCI metrics

Calibration profiles define how:

- BCI feature patterns (fearband, arousal, overload) map into estimates of UEC/EMD/ARR per player, per AR mode.
- DET estimates are derived and bounded for safety.

These estimates are fed into existing metrics envelopes and used alongside non‑BCI observations (e.g., in‑game behavior, session progress) to track user state.

### 8.2 NDJSON telemetry and mis‑alignment detection

Death‑Engine emits NDJSON session metrics envelopes that include:

- UEC/EMD/ARR/STCI/CDL bands.
- DET and exposureDose.
- Region/seed IDs, selector patterns, persona modes.

Joined with BCI traces, these logs allow Tier‑3 analysis to detect mis‑alignment between intended and observed fear trajectories (e.g., chronic DET overshoot, CDL burnout, ARR collapse). Governance modules can then propose changes to:

- BCI‑intensity policies.
- ContractCards (caps, bands).
- Selector patterns or mapping profiles.

### 8.3 Optimization objectives

Tier 3 BCI calibration research optimizes only against:

- Entertainment metrics and invariants (UEC, EMD, ARR, STCI, CDL, DET, CIC, AOS, etc.).
- Safety and ethics guardrails encoded in policies and Dead‑Ledger verifiers.

Raw performance measures (FPS, latency) are logged for engineering purposes but are not Tier‑3 optimization objectives; they belong to other modules.

---

This document is the governance anchor for BCI–AR calibration in the VM‑constellation: envelope schemas, NDJSON trace and replay contracts, BCI‑intensity policies, and Rust/Lua/engine API surfaces defined here are the only lanes Tier‑3 work and AI‑chat code generation may use for BCI‑related MR horror.```
