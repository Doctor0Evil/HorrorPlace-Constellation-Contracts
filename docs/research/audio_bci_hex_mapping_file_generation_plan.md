# Audio–BCI Hex Mapping: File‑Generation Plan for VM‑Constellation

This document defines a concrete file‑generation plan for 10–20 research‑grade artifacts spanning Neural‑Resonance‑Lab, Redacted‑Chronicles, Horror.Place, Spectral‑Foundry, Orchestrator, and Dead‑Ledger. Each item is a discrete, trackable file to be generated and wired into the VM‑constellation, enabling empirical evaluation of hex‑coded mapping families for audio and cross‑modal horror systems.

The plan assumes:

- Public schemas in Horror.Place remain stable and conservative.  
- Lab‑tier schemas and specs live in private repositories (Tier‑2/Tier‑3).  
- Rust enforces safety and mutation logic.  
- Unreal/C++ and Lua act as thin, engine‑specific consumers.  
- Kotlin tooling performs offline analysis.  
- Dead‑Ledger receives only attestations, never raw biomarker data.

***

## 1. Lab Telemetry and Hex‑Curve Manifests (Neural‑Resonance‑Lab, Redacted‑Chronicles)

### 1.1 Telemetry Schema – Audio Mapping Frames

**Target repo:** HorrorPlace‑Neural‑Resonance‑Lab  
**Type:** JSON Schema (lab‑only)  

**Scope:**  
Define the schema for per‑frame audio mapping telemetry records capturing:

- Identity: `family_code`, `family_modality`, `parameter_vector` (compact encoding).  
- Context: `region_class`, invariant snapshot (`CIC`, `AOS`, `MDI`, `RRM`, `HVF`, `LSG`, `SHCI`, `DET`, `RWF`).  
- BCI inputs: normalized `bci_fear_index`, `bci_attention_focus`, `bci_startle_spike`, `bci_cognitive_load`, `bci_visual_overload_score`, `bci_heart_sync_ratio`, `bci_breath_phase`.  
- RTPC outputs: `dl_pressure_lf`, `dl_whisper_send`, `dl_stereo_width`, `dl_noise_grain`, `dl_pulse_depth`, etc.  
- Metrics before/after: `UEC`, `ARR`, `CDL`, `EMD`, `STCI` and their deltas.  
- Temporal snapshot: `activation_id`, `activation_phase`, `timestamp`, `player_state_phase`.

**Goal:**  
Provide a high‑resolution ground truth for mapping‑family behavior, suitable for mutation‑gain and go‑point analysis.

***

### 1.2 Telemetry Schema – Audio Mapping Event Summaries

**Target repo:** HorrorPlace‑Neural‑Resonance‑Lab  
**Type:** JSON Schema (lab‑only)  

**Scope:**  
Define per‑experiment summary records including:

- Experiment metadata: `session_id`, `player_id` (pseudonymous), `style_id`, `persona_mode`.  
- Aggregated metrics: mean and max `dl_pressure_lf`, `dl_whisper_send`, `dl_stereo_width`, `dl_noise_grain`.  
- Metric outcomes: average and peak `UEC`, `ARR`, `CDL`, `EMD`, `STCI` over the run.  
- Safety stats: `comfort_cap_breaches`, `overload_avoidance_events`, `abandonment_events`.  
- Mapping breakdown: counts per `family_code`, per `region_class`, per `player_state_phase`.

**Goal:**  
Support fast comparative analysis of families and parameter sets without scanning frame‑level logs.

***

### 1.3 Hex Curve Manifest Schema

**Target repo:** HorrorPlace‑Neural‑Resonance‑Lab  
**Type:** JSON Schema  

**Scope:**  
Formalize the “hex curve manifest” as a data format. Each entry describes:

- Identity: `family_code`, `name`, `version`, `modality`, `target_rtpc_set`.  
- Input semantics: `allowed_invariants`, `allowed_metrics`, `allowed_bci_fields`, `input_domain_note`.  
- Parameterization: `param_vector`, `param_bounds`, `shape_class`, `qualitative_shape_note`.  
- Safety envelope: `max_output_change_per_second`, `max_output_range`, `overload_response_mode`, `safety_tiers_allowed`.  
- Routing constraints: `region_class_constraints`, `persona_interactions`, `modal_context`.  
- Evaluation hooks: `primary_eval_metrics`, `target_metric_band`, `telemetry_tags`, `deadledger_attestation_hint`.  
- Implementation indirection: `impl_binding_rust`, `impl_binding_engine`, `impl_binding_lab`.

**Goal:**  
Provide a manifest language that all labs, engines, and governance layers can read without exposing implementation or biomarker detail.

***

### 1.4 Hex Curve Manifest – Initial Family Set

**Target repo:** HorrorPlace‑Neural‑Resonance‑Lab  
**Type:** JSON or NDJSON data file  

**Scope:**  
Populate the manifest schema with an initial library of families, including at minimum:

- Pressure: `0xPKLIN`, `0xPKSIG`, `0xPKHYS`, `0xPKOSC`, `0xPKPIECE`, `0xPKNOISE`.  
- Whisper: families for whisper send curves (sigmoid, piecewise, hysteresis).  
- Width: families for stereo width curves (linear, sigmoid, hysteresis).  
- Noise: noise‑augmented variants for micro‑textures.

Each entry fills out identity, inputs, param_vector/bounds, safety, routing, and evaluation hooks.

**Goal:**  
Make the mapping family space explicit and machine‑readable for Rust, tools, and governance.

***

### 1.5 BCI Research Input Schema

**Target repo:** HorrorPlace‑Neural‑Resonance‑Lab  
**Type:** JSON Schema  

**Scope:**  
Define normalized BCI research fields used by mapping families:

- Canonical fields: `bci_fear_index`, `bci_attention_focus`, `bci_startle_spike`, `bci_cognitive_load`, `bci_visual_overload_score`, `bci_heart_sync_ratio`, `bci_breath_phase`, `bci_anticipation_level`.  
- Normalization rules: ranges, sampling rates, missing‑data handling.  
- Coupling hints: optional tags describing qualitative relationships to invariants/metrics (e.g., attention vs UEC).

**Goal:**  
Anchor all new BCI‑derived field_names in a single lab‑tier schema, avoiding proliferation of untracked inputs.

***

### 1.6 Audio Mapping Telemetry Schema – Privacy‑Scrubbed Summary

**Target repo:** HorrorPlace‑Redacted‑Chronicles  
**Type:** JSON Schema  

**Scope:**  
Define a summary schema that carries only:

- Aggregated metrics and safety stats per `family_code`, `region_class`, `tier`.  
- No raw BCI fields, no player identifiers, no per‑frame detail.

**Goal:**  
Enable cross‑repo analysis and Dead‑Ledger attestations while preserving privacy.

***

## 2. Rust Mapping Engine and Adapters (Neural‑Resonance‑Lab)

### 2.1 Rust Mapping Engine Spec

**Target repo:** HorrorPlace‑Neural‑Resonance‑Lab  
**Type:** Design document  

**Scope:**  
Describe:

- Data structures: `MappingFamily`, `InputSample`, `RtpcFrame`.  
- Family registry: load manifest, index by `family_code` and version.  
- Core API: `set_active_family(family_code, variant_id)`, `tick(sample) -> RtpcFrame`.  
- Safety enforcement: application of `max_output_change_per_second`, `max_output_range`, overload behavior, DET scaling.  
- Mutation‑gain logic: how param_vector is perturbed in research builds; logging patterns for parent params and metric deltas.

**Goal:**  
Provide a stable contract for the Rust implementation backing all engines.

***

### 2.2 Rust Mapping Engine Prototype (Research‑Only)

**Target repo:** HorrorPlace‑Neural‑Resonance‑Lab  
**Type:** Rust module  

**Scope:**  
Implement a minimal but complete engine:

- Load manifest entries into a registry.  
- Evaluate at least the initial family set (linear, sigmoid, hysteresis, oscillatory, noise‑augmented).  
- Enforce safety envelopes.  
- Emit `RtpcFrame` instances.  
- Write frame data to NDJSON telemetry conforming to the frame schema.

**Goal:**  
Give labs a functioning core for experiments, reachable via FFI from Unreal and via CLI for offline tests.

***

### 2.3 BCI Input Adapter Spec

**Target repo:** HorrorPlace‑Neural‑Resonance‑Lab  
**Type:** Design document  

**Scope:**  
Define:

- How canonical BCI fields from the research input schema are sampled and normalized.  
- How they are injected into `InputSample` alongside invariants and metrics.  
- Latency and sampling guarantees for mapping functions.  
- Separation of concerns: devices, raw signals, and consent UX remain elsewhere.

**Goal:**  
Ensure the mapping engine sees consistent, abstracted BCI inputs regardless of device.

***

## 3. Engine/Lua Integration (Horror.Place, Orchestrator)

### 3.1 DeadLantern Audio Mapping Controller Spec

**Target repo:** Horror.Place  
**Type:** Design document  

**Scope:**  
Define engine‑facing behavior for an audio controller that:

- Holds current `family_code` and variant info.  
- Gathers invariants, metrics, and BCI abstractions per frame.  
- Calls into the Rust mapping engine and applies resulting RTPC values.  
- Integrates with Telemetry logging.  
- Supports lab‑only commands/flags for switching families during sessions.

**Goal:**  
Create a reusable pattern for audio mapping controllers that can be applied to dead_lantern and other styles.

***

### 3.2 Lua Audio Mapping Glue Interface

**Target repo:** Horror.Place  
**Type:** API specification  

**Scope:**  
Define a narrow Lua interface:

- Functions such as `AudioMapping.setFamily(family_code, variant_id)`, `AudioMapping.update(dt, playerId)`, `AudioMapping.getRtpcFrame()`, `AudioMapping.logSample(regionId, tileId, playerId)`.  
- Contracts for what the Lua layer may and may not do (no mutation, no safety logic; just routing and logging).

**Goal:**  
Provide a common Lua surface across engines, letting any style opt‑into mapping families without bespoke glue.

***

### 3.3 Engine Integration Guide for Hex Mapping Families

**Target repo:** Horror.Place  
**Type:** Engine‑agnostic guide  

**Scope:**  
Explain:

- How to bind RTPC names from `target_rtpc_set` to engine audio parameters (e.g., Unreal+Wwise, Godot, proprietary mixers).  
- How to implement lab‑only switching (console commands, debug menus).  
- How to route Telemetry events to Orchestrator for collection.

**Goal:**  
Give engine teams a clear, documented path to using mapping families safely.

***

### 3.4 Orchestrator Job Spec – Mapping Telemetry Ingestion

**Target repo:** Horror.Place‑Orchestrator  
**Type:** Design document  

**Scope:**  
Specify a service job that:

- Watches for lab‑generated telemetry streams/files.  
- Validates them against the lab schemas.  
- Produces privacy‑scrubbed summaries for Redacted‑Chronicles.  
- Triggers Dead‑Ledger attestation workflows when threshold conditions are met.

**Goal:**  
Integrate mapping experiments into the existing constellation dataflow.

***

## 4. Analysis and Persona Hooks (Neural‑Resonance‑Lab, Spectral‑Foundry, Dead‑Ledger)

### 4.1 Kotlin Analysis Toolkit Spec – Mapping Family Evaluation

**Target repo:** HorrorPlace‑Neural‑Resonance‑Lab  
**Type:** Design document  

**Scope:**  
Define tools that:

- Load frame and summary telemetry schemas.  
- Group data by `family_code`, `variant_id`, `region_class`, `player_state_phase`, and persona mode.  
- Compute metric deltas (UEC, ARR, CDL, EMD, STCI) and safety stats for each cluster.  
- Classify mutations as beneficial, neutral, or detrimental.  
- Export candidate parameter configurations back to Rust.

**Goal:**  
Operationalize the experimental taxonomy and evolution signals.

***

### 4.2 Experimental Taxonomy Reference

**Target repo:** HorrorPlace‑Neural‑Resonance‑Lab  
**Type:** Reference document  

**Scope:**  
Summarize:

- Experimental categories: Region‑Type, Player‑State, Temporal/Pacing, Cross‑Modal, Persona‑Feedback.  
- Required tags and filters in telemetry.  
- Default hypotheses and success metrics per category.

**Goal:**  
Ensure experiment designers follow a consistent structure, supporting comparable results.

***

### 4.3 Persona Hook Extension Spec (Archivist/Witness/Echo)

**Target repo:** HorrorPlace‑Spectral‑Foundry  
**Type:** Design document  

**Scope:**  
Describe:

- New internal persona state fields such as `aperture_ledger_weight`, `audio_pressure_bias`, `mapping_family_preferences`.  
- How these fields are updated from mapping telemetry summaries.  
- How personas use these fields to bias behavior (e.g., preferring families that historically raised UEC/ARR without overload).

**Goal:**  
Let personas evolve their behavior in response to mapping research outcomes without changing public persona schemas.

***

### 4.4 Dead‑Ledger Attestation Pattern for Mapping Families

**Target repo:** HorrorPlace‑Dead‑Ledger‑Network  
**Type:** Governance spec  

**Scope:**  
Define:

- The shape of attestations for mapping families (family_code, version, tiers allowed, region classes, safety envelope summary, metric band achieved, counts of validating sessions).  
- How proofs are generated from Redacted‑Chronicles summaries.  
- How core repos can query these attestations.

**Goal:**  
Bring mapping families into cryptographic governance while preserving lab privacy.

***

## 5. Visual DeadLantern Alignment and Cross‑Modal Mapping (Horror.Place, Neural‑Resonance‑Lab)

### 5.1 STYLE.VISUAL.DEADLANTERN.V1 Contract Skeleton

**Target repo:** Horror.Place  
**Type:** Style contract draft  

**Scope:**  
Define:

- Visual parameters and bounds (inner/outer radius, opacity, edge softness, desaturation, flicker/motion caps).  
- BCI intensity profiles (UNDERSTIMULATED, OPTIMALSTRESS, OVERWHELMED) and their parameter envelopes.  
- Safety rules (brightness and radius change caps, flicker frequency band, maximum strong mask duration).  
- Metric targets (UEC, ARR, STCI, DET bands).  
- Telemetry requirements (mask parameters, modes, and metrics per frame).

**Goal:**  
Align the visual dead_lantern style with the same safety and metric doctrine used for audio mapping.

***

### 5.2 Cross‑Modal Mapping Spec – Visual/Audio Family Pairing

**Target repo:** HorrorPlace‑Neural‑Resonance‑Lab  
**Type:** Research document  

**Scope:**  
Describe:

- How to log and analyze combinations of `visual_family_code` and `audio_family_code`.  
- How to represent `phase_relation` for oscillatory mappings versus physiology.  
- Recommended experimental designs for aligned, complementary, and counterpoint pairings.

**Goal:**  
Support cross‑modal experiments and prevent adoption of misaligned pairs that hurt entertainment metrics.

***

### 5.3 File‑Generation Tracking and Governance Notes

**Target repo:** HorrorPlace‑Constellation‑Contracts  
**Type:** Tracking document (this file)  

**Scope:**  

- Enumerate all planned files listed above.  
- Track status per item (planned, in‑draft, in‑review, merged).  
- Record cross‑repo dependencies and CI hooks (e.g., mapping manifest schema must exist before Rust engine loads it; telemetry schemas must exist before Orchestrator job is implemented).

**Goal:**  
Serve as the central, discoverable roadmap for audio–BCI hex mapping work across the VM‑constellation.
