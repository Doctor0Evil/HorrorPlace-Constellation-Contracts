## 1. Purpose and scope

This spec defines Tier 3 governance rules for **AR BCI calibration** across the VM‑constellation, focusing on telemetry schemas, NDJSON traces, and AI‑chat API hooks rather than engine internals. It binds BCI feature envelopes, derived metrics, and calibration profiles to DET‑aware safety caps and entertainment metrics (UEC, EMD, ARR) in a way that Unity/Unreal/C++/C# stubs can consume mechanically. AR specifics (world‑locked overlays, mixed‑reality focus) are handled as schema‑level tags and context metadata, not as per‑engine behavior. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

***

## 2. Canonical envelopes for AR BCI

### 2.1 `bci-feature-envelope-v2`

- Lives under: `schemas/bci/bci-feature-envelope-v2.json`.  
- Extends the existing v1 feature envelope with:
  - `arContext` block (e.g., `mode: enum["AR-HMD","Mobile-AR"]`, `anchorType`, `visualLoadHint`) as opaque enums/strings.  
  - `signalProfileId`, `deviceProfileId` for reproducible replay and cross‑device study. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/1c3e38a9-000b-42d6-bb93-f005b8cfad2f/1-should-the-research-prioriti-tQnn6sdDQ06XDNmNoVKx.g.md)
- Remains “rawish”: per‑window, minimally processed features; no raw waveforms; no PII; `additionalProperties: false`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

### 2.2 `bci-metrics-envelope-v2`

- Lives under: `schemas/bci/bci-metrics-envelope-v2.json`.  
- Derived from feature envelopes; maps into canonical entertainment metrics and safety:
  - `metrics.uecBand, emdBand, stciBand, cdlBand, arrBand` in \[0,1\], referencing the entertainment metrics spine. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/1c3e38a9-000b-42d6-bb93-f005b8cfad2f/1-should-the-research-prioriti-tQnn6sdDQ06XDNmNoVKx.g.md)
  - `safety.detEstimate` in \[0,10\], referencing invariants DET; `safety.overloadFlag`, `safety.underEngagedFlag`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
  - `calibrationProfileId`, `mappingProfileId`, `signalProfileId` for traceability. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/1c3e38a9-000b-42d6-bb93-f005b8cfad2f/1-should-the-research-prioriti-tQnn6sdDQ06XDNmNoVKx.g.md)
- Captures transformation metadata only (`method`, `smoothingAlpha`, `modelId`); mapping math stays in Rust/Lua/engine code. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

### 2.3 `bci-calibration-profile-envelope-v1`

- Lives under: `schemas/bci/bci-calibration-profile-envelope-v1.json`.  
- Tier‑3 research‑only contract describing **how a cohort’s BCI signals map into metrics**:
  - Identifiers: `profileId`, `labId`, `deviceProfileId`, `arContextProfileId`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
  - Target bands: desired `uecBand`, `emdBand`, `arrBand` ranges and DET exposure bands for calibration scenarios. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)
  - Limits: hard caps for `fearband`, `overloadFlag` criteria, exposure dose per session.  
  - Validity metadata: `createdAt`, `version`, `dataSources`, `ethicsReviewId`.  

All three envelopes must declare stable `$id` values and reuse core invariants/metrics definitions via `$ref` to the schema spine. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

***

## 3. Synthetic traces and NDJSON replay governance

### 3.1 Synthetic trace schemas

Define **lab‑side** NDJSON schemas in the same repo:

- `schemas/telemetry/synthetic-bci-feature-trace-v1.json`  
  - Each line: full `bci-feature-envelope-v2` plus `syntheticTraceId`, `seed`, `replayProfileId`, `traceKind: enum["calibration","stress-test","safety-regression"]`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/1c3e38a9-000b-42d6-bb93-f005b8cfad2f/1-should-the-research-prioriti-tQnn6sdDQ06XDNmNoVKx.g.md)

- `schemas/telemetry/synthetic-bci-metrics-trace-v1.json`  
  - Each line: `bci-metrics-envelope-v2` plus:
    - `expectedBandTrajectoryId` (link to design spec),  
    - `engineTarget: enum["Unity","Unreal","Rust-Harness"]`,  
    - `tickIndex`, `dt` for deterministic time stepping. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

- `schemas/telemetry/synthetic-calibration-expectations-v1.json`  
  - High‑level expectations per trace:
    - Expected distributions for UEC/EMD/ARR, allowed DET range, max overloadFlag count, etc. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)
    - Maps to research outcomes, not engine code.

All NDJSON schemas enforce `additionalProperties: false` and forbid PII; traces are purely synthetic or anonymized. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

### 3.2 Replay tool contract

Add a governance document for replay tools:

- `schemas/tools/bci-ndjson-replay-contract-v1.json` describing the capabilities a replay runner must implement:
  - Inputs: which NDJSON schemas it accepts (`bci-feature`, `bci-metrics`, or both).  
  - Determinism: requirement for seed‑based, repeatable playback (fixed `dt`, no hidden randomness). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
  - Controls: `playbackMode` (real‑time, fixed‑step, fast‑forward), `safetyClamp` behavior when traces violate current policy/DET caps.  
  - Outputs: required telemetry of **what was actually delivered** to the engine (post‑clamp metrics) for audit and lab‑to‑engine feedback. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)

Rust CLIs, Unity editor tools, and Unreal plugins must declare conformance to this schema in their own metadata, but the governance lives here. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)

***

## 4. DET‑aware safety and calibration rules

The spec should codify DET‑first safety for all AR BCI calibration:

- **DET as hard cap**: any BCI‑derived attempt to raise horror intensity must be clamped inside the DET range allowed by the active policy and region/seed contract cards. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)
- **Calibration bands**: calibration profiles define target **comfort bands** for UEC/EMD/ARR and acceptable DET drift, explicitly separating:
  - Exploration range (Tier 3 lab) vs. deployment range (Tier 2 production). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)
- **User‑state modeling**: metrics envelopes must support fields for fatigue / habituation flags (derived from temporal patterns of UEC/ARR/overload), but raw modeling lives outside schemas. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/1c3e38a9-000b-42d6-bb93-f005b8cfad2f/1-should-the-research-prioriti-tQnn6sdDQ06XDNmNoVKx.g.md)

The document should also state that any change to BCI mapping that widens DET or overload limits requires a **policyEnvelope** update and Dead‑Ledger‑compatible proofs, not just code changes. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

***

## 5. Rust and Lua API hooks for AI‑chat

To keep AI‑chat generation deterministic and constrained, the spec pins a narrow, language‑agnostic surface that all BCI‑related code must target. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)

### 5.1 Rust mapping engine expectations

The document references an engine‑facing crate (e.g., `bcimappingengine`) and defines required functions semantically (not full signatures):

- `load_calibration_profile(profile_id | path)`  
  - Loads a `bci-calibration-profile-envelope-v1` and enforces DET/metric ranges embedded in it.  
- `ingest_feature_envelope(bci-feature-envelope-v2)`  
  - Validates against schema, applies EMA/calibration, and returns a `bci-metrics-envelope-v2`.  
- `apply_safety_and_contracts(metricsEnvelope, contractSnapshot)`  
  - Clamps metrics to policy/region/seed constraints before exposing them to engines. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)

Unity/Unreal bindings are required to call only these functions (through C/FFI), never to re‑implement mapping logic. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

### 5.2 Lua `H.` API hooks

The governance spec pins the **Lua‑visible API** that AI‑chat must use:

- `H.BCI.load_calibration(profile_id)` – Tier‑3 only, calls into Rust. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- `H.BCI.push_feature_window(feature_env_table)` – accepts already‑validated feature envelopes from NDJSON replay or live streams.  
- `H.BCI.current_metrics()` – returns normalized bands and safety flags:
  - `uec, emd, arr, stci, cdl, detEstimate, overloadFlag, underEngagedFlag`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/1c3e38a9-000b-42d6-bb93-f005b8cfad2f/1-should-the-research-prioriti-tQnn6sdDQ06XDNmNoVKx.g.md)
- `H.BCI.apply_to_director(directorState)` – optional helper that applies DET/metric clamps consistently, consuming only the above fields.

AI‑chat tools generating Lua must call into `H.BCI.*` and **must not** invent ad‑hoc JSON parsing or alternative BCI APIs; CI can enforce this. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

### 5.3 AI‑chat governance hooks

The doc should add a short section for the **AI‑authoring lint** layer:

- Any AI‑generated BCI code or NDJSON must:
  - Declare which BCI schemas it targets (`bci-feature-envelope-v2`, `bci-metrics-envelope-v2`, etc.). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)
  - Use only the Rust/Lua surfaces declared above (checked via static linting/regex in CI).  
  - Never bypass DET or overload flags; attempts to do so are rejected at pre‑commit. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

***

## 6. Lab‑to‑engine feedback loop

Finally, the spec ties telemetry back into governance:

- Requires **aggregation schemas** that fold BCI metrics + UEC/EMD/ARR back into calibration profile refinement, using NDJSON summary streams aligned with the existing entertainment metrics telemetry plan. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)
- Specifies that calibration profiles are updated only through **documented schema updates** (new `bci-calibration-profile-envelope` instances), never through opaque code changes, so changes remain auditable and replayable. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)
