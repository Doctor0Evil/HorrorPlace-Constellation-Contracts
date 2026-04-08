# Horror$Place BCI Dev Guide

This guide is the canonical, schema‑first contract for all BCI work in Horror$Place. It is the primary entry point for human and AI developers and must be kept in sync with schemas, templates, and golden examples.

---

## 1. Overview & Constraints

The Horror$Place BCI stack follows a strict device → feature server → EEGFeatureContract → HorrorDirector → engine flow. Physical EEG devices stream raw signals into a feature server, which computes canonical features and emits JSON that conforms to EEGFeatureContract. Engine layers (Unity and Unreal) consume only these features, never raw signals.

All BCI development is schema‑first. Schemas live in Horror.Place and HorrorPlace‑Constellation‑Contracts, and engine/BCI repos must treat those schemas as the single source of truth. You may not introduce new fields or shapes without updating the relevant schema definitions first.

No personally identifiable information (PII) may appear in any BCI configuration, log, or telemetry payload. BCI pipelines must use pseudonymous session identifiers and device IDs that cannot be traced back to individuals without offline, out‑of‑band mapping.

Raw EEG is never logged. Only derived features and summary statistics may be persisted. Any logging pipeline must be audited to confirm that no raw waveform segments, raw channels, or device‑native binary streams are written to disk or transmitted over telemetry channels.

All EEG signals must flow through HorrorDirector and its policies before reaching engine subsystems or gameplay logic. Direct connections from feature servers or devices to gameplay systems are forbidden. HorrorDirector is the gatekeeper that enforces BCI intensity policies, guardrails, and safety invariants.

---

## 2. Invariants & Metrics

The BCI system uses a small set of canonical metrics and invariants to describe player state and pipeline behavior. These scalars appear in schemas, runtime signals, and analysis tooling and must stay consistent across repos.

Key metrics and invariants include:

- CIC (Cognitive Immersion Coefficient): A normalized score representing how deeply engaged a player appears based on EEG features and in‑game context.
- MDI (Mental Disturbance Index): A bounded measure of cognitive/emotional strain derived from stress and arousal bands.
- AOS (Affective Oscillation Score): A measure of how quickly affective state is changing over time.
- DET (Deterministic Exposure Time): The cumulative duration for which the player has been held above a configured intensity threshold.
- HVF (Horror Variability Factor): A scalar capturing variability in horror stimulus patterns over a sliding window.
- LSG (Liminal State Gradient): A measure of how fast the system is transitioning between calm and horror‑intense states.
- SHCI (Subjective Horror Coherence Index): A composite score that approximates how coherent and intentional the horror experience feels based on scene and feature context.
- UEC (Uncanny Exposure Coefficient): A scalar measuring exposure to uncanny or dissonant stimuli.
- ARR (Adaptive Recovery Rate): A normalized rate at which the system attempts to de‑intensify or recover after high stress periods.

The EEGFeatureContract’s `horror_context` section includes selected members of this metric set. HorrorDirector reads these values to decide when to ramp up or down intensity, trigger cooldowns, or adjust pacing systems.

All metrics are normalized to well‑defined domains (for example, 0–1 or 0–100). The same domains must be used consistently in schemas, mappings, and analysis notebooks. When you introduce a new metric or adjust its domain, update both schemas and reference notebooks.

Invariants such as “MDI must remain below the configured BCI intensity cap for at least N seconds after a cooldown” or “DET must not exceed a safety ceiling without triggering a forced recovery” must be encoded in policy and test recipes, not just in prose.

---

## 3. Schemas and Contracts

The central BCI contracts are defined as JSON Schemas and referenced by ID and path. All code and configuration must conform to these contracts.

### EEGFeatureContract

- Schema ID: `EEGFeatureContractv1`
- Canonical path example: `Horror.Place/schemas/EEGFeatureContractv1.json`

EEGFeatureContract describes the payload sent from feature servers to HorrorDirector and engine layers. Typical sections:

- `meta`: Session ID, timestamp, device ID, sampling rate, version tags.
- `bands`: Canonical band powers (delta, theta, alpha, beta, gamma), optionally per‑region.
- `composite`: Derived features such as stress index, focus index, fatigue, and arousal.
- `horror_context`: Metrics like CIC, MDI, AOS, DET, HVF, LSG, SHCI, UEC, ARR, and any other horror‑specific scalars.
- `debug`: Optional development‑only diagnostics (to be disabled in production).

No gameplay code may depend on debug‑only fields. They should be considered optional and removed in production builds.

### BCI Intensity Policy Schema

- Schema ID: `bci-intensity-policy-v1`
- Canonical path example: `HorrorPlace-Constellation-Contracts/schemas/bci-intensity-policy-v1.json`

This schema defines caps, guardrails, and safety policies for mapping EEG features to in‑game horror intensity. Typical fields:

- Global intensity ceilings for tension, exposure, and rate of change.
- Per‑metric caps for MDI, CIC, and UEC.
- Cooldown thresholds and minimum recovery durations.
- Optional per‑scene overrides and age‑gating constraints.

All mappings must query and respect this schema. You must not hard‑code intensity caps in engine code.

### Synthetic Trace and Expectations Schemas

- Schema ID: `synthetic-eeg-feature-trace-v1`
- Canonical path example: `HorrorPlace-Spectral-Foundry/schemas/synthetic-eeg-feature-trace-v1.json`

Synthetic EEG feature traces are NDJSON or JSONL files containing a sequence of EEGFeatureContract‑compatible snapshots, plus expectations about system behavior. Typical sections:

- `trace_meta`: Scenario description, schema version, intended engine or experiment.
- `frames`: Ordered list of EEG feature snapshots.
- `expectations`: Constraints and assertions about HorrorDirector outputs, including tension curves, cooldown triggers, and intensity ceilings.

Test harnesses and CI jobs must use this schema to validate BCI logic in a deterministic, headless way.

### ExperimentConfig Schema

- Schema ID: `experiment-config-v1`
- Canonical path example: `HorrorPlace-Atrocity-Seeds/experiments/ExperimentConfigExamples/`

ExperimentConfig describes EEG‑driven experiments, including scenes, blocks, and conditions. Typical fields:

- `experiment_id`, `version`, and description.
- `participants`: Non‑identifying references or pseudonyms only (no PII).
- `blocks`: Sequences of conditions, each referencing scenes, durations, and BCI mappings.
- `telemetry`: Logging configuration for NDJSON output and metrics of interest.

Any new experiment must be defined through ExperimentConfig rather than custom ad‑hoc scripts.

---

## 4. Device Integration Patterns

Only BrainFlow and Lab Streaming Layer (LSL) are supported as backends for EEG device integration. Other backends may be added only after updating schemas, playbooks, and tests.

A logical device registry defines the known devices and how they map to BrainFlow board IDs or LSL stream parameters. The registry is typically stored as JSON and used by both feature servers and engine drivers. It must contain:

- Logical device ID.
- Friendly name.
- Backend type (`brainflow` or `lsl`).
- Backend parameters (board ID, serial settings, LSL stream name, etc.).
- Sampling rate and channel layout hints.

The `IEEGDevice` interface is the abstract boundary between hardware and higher‑level systems in engines or orchestration layers. An `IEEGDevice` implementation:

- Reads configuration from the device registry.
- Connects to BrainFlow or LSL.
- Exposes a minimal surface: connect, disconnect, and fetch or subscribe to `EEGFeatures`.

The Python feature server pattern is the canonical way to connect BrainFlow/LSL to EEGFeatureContract JSON. The server:

- Loads logical device configuration.
- Connects to the device via BrainFlow BoardShim or LSL stream.
- Runs the EEGCanonicalV1 pipeline.
- Emits EEGFeatureContract‑compatible NDJSON over TCP or a local socket.

Engine or orchestration code connects to this server and never touches raw EEG buffers.

---

## 5. Feature Extraction

EEGCanonicalV1 is the standard feature extraction pipeline. Deviations must be documented, versioned, and tested.

A typical EEGCanonicalV1 pipeline includes:

- Windowing: Segmenting continuous EEG into fixed‑length windows with overlap, with parameters defined in a config file.
- Pre‑processing: Bandpass filtering, notch filters, artifact rejection, and channel normalization.
- Spectral analysis: Computing power spectral density (PSD) per channel and per band.
- Feature aggregation: Aggregating PSD values into canonical bands (delta, theta, alpha, beta, gamma), optionally per region or hemisphere.
- Composite scores: Deriving higher‑level scalars such as stress, focus, fatigue, and arousal from band ratios and stability metrics.
- Horror context derivation: Computing CIC, MDI, AOS, DET, HVF, LSG, SHCI, UEC, ARR, and related metrics from composite features and temporal buffers.

The mapping from raw EEG to the fields in EEGFeatureContract must be documented next to the schema. Every new or modified feature must specify:

- Input channels and bands.
- Mathematical transform or heuristic used.
- Value range and normalization scheme.
- Dependencies on historical state (for example, rolling averages or exponential smoothing).

Feature extraction code must never embed scene‑specific logic. It should remain generic and scene‑agnostic, with HorrorDirector and engine layers responsible for interpreting features in context.

---

## 6. Engine Integration

The engine integration layer consumes EEGFeatureContract features and maps them into gameplay systems via HorrorDirector and mapping configurations.

### Unity Integration

Unity uses `EEGFeatures` and `EEGFeatureService` as the primary abstractions:

- `EEGFeatures` is a struct or class representing a deserialized EEGFeatureContract snapshot, with fields that match the schema.
- `EEGFeatureService` is a singleton responsible for connecting to the feature server, handling replay sources, and providing the latest `EEGFeatures` instance.

HorrorDirector runs either as a MonoBehaviour singleton or a service that reads `EEGFeatures` and a mapping configuration, then drives outputs such as:

- Post‑processing volume weights.
- Enemy spawn multipliers.
- Music or ambience intensity.
- Encounter pacing parameters.

Unity scenes must not reference BrainFlow, LSL, or hardware details. They only talk to `EEGFeatureService` and HorrorDirector.

### Unreal Integration

Unreal uses `FEEGFeatures` and `UEEGFeatureSubsystem` as the primary abstractions:

- `FEEGFeatures` is a UStruct that mirrors EEGFeatureContract fields.
- `UEEGFeatureSubsystem` is a subsystem responsible for networking, replay, and exposing the latest `FEEGFeatures` to game systems.

A HorrorDirector subsystem or component consumes `FEEGFeatures` and mapping configurations, then writes to material parameters, post‑process volumes, AI controllers, and encounter systems.

Unreal actors or Blueprints interact only with the HorrorDirector subsystem and `FEEGFeatures`, not with hardware or feature server internals.

### Replay Sources

Both Unity and Unreal integrations must support multiple data sources:

- Live: Feature server output over TCP or sockets.
- Replay: NDJSON traces recorded from previous sessions.
- Synthetic: SyntheticEEGFeatureTrace inputs used for testing and QA.

Switching between these sources must be controlled by configuration rather than code changes, typically via a small config file or project setting.

---

## 7. Adaptive Logic & Safety

Adaptive BCI logic maps EEG features and horror context metrics to game‑level parameters. All mappings must be explicit, versioned, and constrained by BCI intensity policies.

Supported mapping strategies include:

- Linear mappings: Direct proportional relationships between a feature and an output.
- Zone mappings: Piecewise behavior with discrete zones and thresholds.
- Curved mappings: Non‑linear curves such as sigmoid, exponential, or custom splines.
- PID‑like controllers: Controllers with proportional, integral, and derivative components for smooth state regulation.

Mapping configurations, such as `HorrorMappingConfig`, encode which strategy is used, hyperparameters, and any hysteresis or smoothing parameters. Mappings must read policy caps from `bci-intensity-policy-v1` and clamp outputs accordingly.

BCI intensity policies define:

- Maximum allowed tension and exposure levels.
- Maximum rate of change for key outputs.
- Cooldown conditions and minimum durations.
- Constraints on how quickly intensity can re‑ramp after a cooldown.

Direct control modes allow EEG features to directly influence a few highly constrained outputs (for example, post‑process intensity) within strict bounds. Indirect control modes treat EEG features as hints that adjust probabilities, pacing, or encounter parameters rather than direct state.

HorrorDirector must encode these guardrails and log any violations to help refine policies and mappings.

---

## 8. Experiment & Analysis

BCI experiments in Horror$Place are driven by schemas and templates to ensure reproducible, auditable behavior.

### Experiment Orchestration

Experiment orchestration patterns are defined by ExperimentConfig. A typical experiment includes:

- A set of scenes or levels, each tagged with experiment metadata.
- Blocks describing sequences of conditions (for example, baseline, low intensity, high intensity).
- Per‑block or per‑condition mapping configurations and policies.
- Pre‑defined timelines and durations, with guardrails against overruns.

Orchestrator scripts or services read ExperimentConfig and drive scene transitions, mapping configuration changes, and data collection.

### NDJSON Logging

EEG and horror telemetry must be logged in NDJSON format with one JSON object per line. Required fields include:

- Timestamp.
- Session ID.
- Experiment and block identifiers.
- EEGFeatureContract snapshot or a compact representation.
- HorrorDirector outputs and key intensity metrics.

Raw EEG is never logged. Compression and retention policies must be configured to comply with privacy and governance requirements.

### Analysis Notebooks

Standard analysis notebooks are provided for:

- Loading NDJSON logs using helper functions such as `load_eeg_ndjson`.
- Computing summary statistics for metrics like MDI, CIC, and DET.
- Visualizing intensity curves across scenes and experiments.
- Evaluating guardrail adherence and policy effectiveness.

When proposing new mappings or policy changes, you must:

- Base them on analysis notebooks or equivalent analyses.
- Include clear references to the datasets and scripts used.
- Update configuration files and test recipes accordingly.

---

## 9. AI‑Chat Playbook Summary

AI tools are treated as BCI programmers that must obey this document and all referenced schemas. A short summary of must‑do and must‑not‑do rules:

Must do:

- Always read `BCI-Dev-Guide.md` and relevant JSON Schemas before changing or generating BCI code and configs.
- Use existing types and patterns (`EEGFeatures`, `FEEGFeatures`, `IEEGDevice`, `EEGFeatureService`, `UEEGFeatureSubsystem`, HorrorDirector).
- Reference schema IDs and canonical paths in comments or docstrings where relevant.
- Respect BCI intensity policies and caps and ensure mappings are always expressed as configuration.

Must not do:

- Access EEG hardware directly from gameplay scripts or engine subsystems.
- Introduce new fields into EEGFeatureContract or other schemas without explicit instructions and schema updates.
- Log raw EEG waveforms or any PII.
- Hard‑code caps or safety thresholds directly in code when policies exist.

AI‑generated outputs must include or reference test recipes, synthetic traces, or replay setups that validate mappings and guardrails. When test or schema validation fails, AI‑driven workflows must preserve schema interfaces and IDs while correcting implementation details.

This guide is intentionally dense and should be cross‑linked from schemas, templates, and example projects. Treat it as the OpenAPI‑style contract for Horror$Place BCI work: schemas define structures, this guide defines patterns and invariants, and examples encode idioms.
