# BCI-Integrated Horror Environment – 12-Month Investigation Overview

This document codifies the 12‑month BCI‑integrated horror environment investigation as a constellation‑wide contract, aligning research phases, safety doctrine, schemas, and lab tooling with the VM‑constellation’s Rust‑truth and schema‑first doctrine.

## 1. Purpose and Scope

This investigation validates and tunes Horror.Place’s BCI‑geometry binding spine across three phases:

- Phase 1 – Framework Validation (Months 1–3; ~35% effort)
- Phase 2 – Environment-Specific Optimization (Months 4–7; ~40% effort)
- Phase 3 – Intervention Testing & Tuning Playbook (Months 8–12; ~25% effort)

The scope covers:

- Validation of the BCI pipeline (feature → metrics → summary → geometry binding).
- Safety‑first enforcement (DET, CSI, hysteresis, bci-safety-profile-v1).
- Environment‑specific preset analysis (Penum‑Cube, Lab‑Plague, Dead‑City‑Ruins, puzzle/loot tuning).
- Controlled intervention experiments and a prescriptive, production‑ready tuning playbook.

This overview is the umbrella document referenced by schemas and lab configs in this repo.

## 2. Phase 1 – Framework Validation (Months 1–3)

### 2.1 Goals

Phase 1 answers: *Does the BCI‑geometry contract framework behave as designed, with stable, auditable mappings from BCI context to horror geometry outputs?*

Quantitative Phase 1 success targets:

- ≥ 70% profile target accuracy when comparing predicted vs. observed metric responses per geometry profile across lab sessions.
- SHCI–neural resonance coupling \(R^2 ≥ 0.65\) between planned spectral‑haunt intensity and recorded BCI responses.
- Mode oscillation rates below 5% between UNDERSTIMULATED ↔ OPTIMALSTRESS ↔ OVERWHELMED per session, given the configured hysteresis bands.

### 2.2 Core Validation Activities

Phase 1 builds the minimal lab toolchain and validates the spine:

- **Contract to SQLite pipeline**
  - `hpc-schema-to-sqlite`: normalize Constellation‑Contracts JSON schemas and NDJSON exports into a single lab SQLite database.
  - `hpc-validate-sqlite`: enforce integrity, foreign‑key consistency, and canonical ranges for invariants, metrics, and BCI fields (all DET‑like values within 0–10, weights in 0–1, etc.).
  - Adapter test harnesses to ensure runtime FFI writes compatible NDJSON envelopes to the same schema set.

- **Full-session clustering**
  - Cluster full BCI game sessions in the lab DB to estimate boundaries for:
    - UNDERSTIMULATED
    - OPTIMALSTRESS
    - OVERWHELMED
  - Enforce minimum Hamming/entropy distance between mode UEC/CDL thresholds: threshold differences ≥ \(H = 0.10\), to prevent rapid oscillation between modes.

- **Tileset-metric response mapping**
  - For each geometry profile (e.g., high‑CIC corridor under OPTIMALSTRESS), test whether observed metric deltas match contract‑encoded intent:
    - Example: UEC uplift of \(0.10 \pm 0.05\) with stable ARR.
  - Tag profiles in the lab DB with “calibrated” vs. “drift‑suspect” flags based on outcome distributions.

### 2.3 Phase 1 Deliverables

- Validated SQLite schema hosting:
  - `bci_tileset`, `bci_geometry_profile`, `bci_metrics_window`,
    `geometry_selection_event`, and preset‑specific tables.
- Baseline mode thresholds (UEC/CDL bands per stress mode) with a committed JSON reference file in `schemas/runtime/`.
- A Phase‑1 status row per artifact in `bci-constellation-progress.csv` (separate file), marking core BCI schemas and lab tools as `Validated` / `Draft` / `Missing`.

## 3. Phase 2 – Environment-Specific Optimization (Months 4–7)

### 3.1 Goals

Phase 2 applies the validated measurement framework to concrete presets, answering: *How well do specific environments respect OPTIMALSTRESS and narrative targets?*

Phase 2 quantitative success targets:

- ≥ 60% of each session’s duration in OPTIMALSTRESS band.
- For ≥ 70% of sessions, ARR endpoint within ±0.10 of the target specified in the preset contract.

### 3.2 Key Environment Analyses

Phase 2 standardizes analysis templates for several “reference environments”:

- **Penum‑Cube Analysis**
  - Investigate rotation mechanics’ influence on Liminal State Gradient (LSG) spikes.
  - Validate whether “rotation phase” geometry profiles regulate disorientation‑driven stress while preserving cognitive engagement (UEC, STCI).

- **Lab‑Plague Ethical Dilemmas**
  - Track the effect of purge/survival decisions on:
    - CIC (Contextual Immersion Coherence),
    - SHCI (Spectral‑Haunt Coupling Index),
    - CDL (Cognitive Dissonance Load).
  - Assert that moral choice branches produce intended narrative weight (ARR, EMD) without safety violations (DET overshoot, CSI overload).

- **Dead‑City‑Ruins Vignette Replay**
  - Study replay and loop degradation:
    - How repeated vignettes affect CDL, ARR, and UEC over multiple exposures.
  - Calibrate “loop degradation” parameters so that repeated exposures increase CDL only to pre‑agreed caps while preserving ARR and avoiding DET overload.

### 3.3 Supporting Tuning Analyses

Additional cross‑environment optimization tasks:

- **Puzzle difficulty vs. CDL**
  - Correlate puzzle attempt traces with CDL spikes.
  - Normalize difficulty bands so CDL stays within target windows and is recoverable via hint interventions.

- **Keycard chain efficiency**
  - Detect chains with 5+ obscure steps and low link_success_rate.
  - Simplify or enrich such chains to reduce dead‑end frustration while keeping ARR in desired suspense ranges.

- **Loot entropy using Shannon metrics**
  - Use per‑session loot event distributions to calculate entropy.
  - Tune loot tables to avoid both over‑predictability and unstructured chaos.

### 3.4 Phase 2 Deliverables

- Environment‑specific “lab specs” (one Markdown per reference environment under `docs/bci/environments/`) encoding:
  - Invariant bands,
  - Target metrics,
  - Observed deviations,
  - Recommended contract changes.
- SQL views or materialized tables like:
  - `v_profile_performance`,
  - `v_region_stress`,
  - `v_session_summary`.
- Updated preset contracts under `schemas/runtime/` with calibrated thresholds for OPTIMALSTRESS and ARR per environment.

## 4. Phase 3 – Intervention Testing & Tuning Playbook (Months 8–12)

### 4.1 Goals

Phase 3 performs controlled A/B experiments on design interventions, answering: *Which interventions measurably improve engagement and reduce overload under strict safety?*

Quantitative Phase 3 targets:

- ≥ 10% improvement in intervention effectiveness (e.g., UEC uplift, satisfaction scores) relative to control.
- ≥ 20% reduction in overload episodes via DET gating, measured by hysteresis violation frequency and overloadFlag incidence.
- Zero safety degradation incidents (no increase in DET/CSI rule violations compared to Phase 2 baselines).

### 4.2 Intervention Types

Standard intervention categories:

- **Hint systems**
  - BCI‑driven evidence highlighting when:
    - CDL > 0.7,
    - link_success_rate < 30%.
  - Evaluate A/B conditions with and without adaptive hinting.

- **Spawn density adjustments**
  - Dynamic enemy/encounter density as a function of stress band and UEC.
  - Example rules:
    - Underengaged for ≥ 60 seconds → trigger booster profiles with `spawn_density ≈ 0.8` within safety caps.
    - Near overload → clamp or reduce spawn density.

- **Haptic feedback synchronization**
  - Align haptic patterns with delta/theta power trajectories, targeting smoother EMD curves.
  - Evaluate whether synchronized haptics amplifies or stabilizes desired metric trajectories without violating DET and CSI bounds.

### 4.3 Tuning Playbook

The final deliverable of Phase 3 is a prescriptive tuning playbook:

- Expressed as a set of “if state, then intervention” contracts, for example:
  - “If `underengaged_duration ≥ 60s`, then select booster profile X with spawn_density in [0.7, 0.9], provided CSI < threshold and DET < limit.”
  - “Gate Tier‑2 content behind `DET ≤ 7.5` AND `SHCI ≥ 0.70` with expected overload reduction ≥ 20%.”
- Encoded as schema‑validated JSON files in `schemas/runtime/` for:
  - `bci-intervention-rule-v1.json`,
  - `bci-tuning-playbook-v1.json`.

### 4.4 Phase 3 Deliverables

- T3‑lab experimental reports per intervention type under `docs/bci/interventions/`.
- A versioned `bci-tuning-playbook-v1.md` narrative summary describing:
  - Intended interventions,
  - Proven effective parameter ranges,
  - Explicit links back to the corresponding JSON contracts.

## 5. Safety-First Hierarchical Constraints

### 5.1 Safety Hierarchy

All phases adhere to an explicit hierarchy:

1. **Hard Safety**
   - DET caps on a 0–10 scale.
   - Overload prevention via bci‑safety‑profile runtime enforcement.
   - CSI (Cooldown Stress Index) compliance with max CSI and enforced cooldown windows.

2. **Hysteresis & Stability**
   - Minimum 30 seconds between geometry mode changes.
   - ARR rise rate caps (e.g., ≤ 0.05 per minute) to avoid premature mystery collapse.
   - DET jump limits (e.g., ≤ 0.25 over 2 seconds), enforced by the `bci-hysteresis-guard`.

3. **Engagement Optimization**
   - Maximize OPTIMALSTRESS duration.
   - Calibrate ARR “sweet spots” only inside safety envelopes.

Any change that improves engagement but degrades safety is categorically rejected at the contract level.

### 5.2 bci-safety-profile-v1 Integration

- `bci-safety-profile-v1.json` captures:
  - Channel‑specific intensity caps.
  - Per‑channel rate‑of‑change limits.
  - CSI/DET‑aware limiter composition.
  - Recovery policies for exits and safe zones.
- Rust kernels enforce safety caps before values reach the engine via:
  - `apply_safety_caps()` functions,
  - BCI stress limiters,
  - CSI‑driven decay.

Phases 1–3 only adjust bindings and presets that reference existing safety profiles, never the safety envelope itself without explicit governance.

## 6. Schema-First Data Architecture

### 6.1 BCI Pipeline Envelopes

The investigation assumes the canonical BCI pipeline:

1. `bci-feature-envelope-v1`
   - Raw, anonymized features (EEG band powers, arousal/valence).
   - No PII, no raw waveforms.

2. `bci-metrics-envelope-v1`
   - Normalized entertainment metrics (UEC, EMD, STCI, CDL, ARR).
   - DET and related dread metrics on 0–10 scales.

3. `bci-summary-v1`
   - Surface state per tick:
     - `stressScore`, `stressBand`, `attentionBand`,
     - `visualOverloadIndex`, `startleSpike`,
     - `signalQuality`.

4. `bci-geometry-binding-v1`
   - Filters current BCI state + invariants + metrics.
   - Selects binding families (e.g., PKLIN, PKSIG) and generates visual/audio/haptic outputs.

All transforms are governed by JSON schemas with `additionalProperties: false`, prohibiting ad‑hoc fields and ensuring determinism and auditability.

### 6.2 Mapping Interface

The unified mapping interface is treated as the canonical math object:

- `bci-mapping-request-v1.json`
  - Aggregates:
    - Session/player IDs,
    - Region/tile IDs,
    - Invariants slice (CIC, LSG, etc.),
    - Metrics slice (UEC, ARR, etc.),
    - `BciSummary`,
    - CSI value,
    - Contract context.

- `bci-mapping-response-v1.json`
  - Returns:
    - Updated metrics,
    - Updated `BciSummary`,
    - BciMappingOutputs (visual, audio, haptic),
    - BciMappingTelemetry (binding selection, scores, caps applied, perf data).

All phases assume this interface as the primary vehicle for lab replay and analysis.

## 7. Tooling & Validation Infrastructure

### 7.1 Lab SQLite Schema

The lab schema must support the full investigation:

- Core tables:
  - `bci_tileset` – tileset definitions and geometry tags.
  - `bci_geometry_profile` – per‑profile contract metadata and calibration results.
  - `bci_metrics_window` – sliding window metrics per session.
  - `geometry_selection_event` – mapping decisions and outputs.
- Preset‑specific tables:
  - Puzzle instances, evidence items, specimen vats, keycard chains.

Standard validation workflow:

1. Export preset contracts → NDJSON via schema‑compliant serialization.
2. Validate NDJSON with `hpc-validate-contracts` against Constellation‑Contracts schemas.
3. Import NDJSON into SQLite via `hpc-import-ndjson`.
4. Run `hpc-validate-sqlite` to check integrity, referential consistency, and canonical ranges.
5. Query utility views (e.g., `v_session_summary`) for metrics and violation counts.

### 7.2 Hysteresis Violation Detection

- Implement SQL self‑joins or window functions to detect 2‑second windows where:
  - `DET_t+Δ - DET_t > 0.25` (normalized scale).
- Tag offending geometry profiles as candidates for:
  - Tighter detCaps in safety profiles.
  - Lower `overwhelmedDwellSecondsMax`.

### 7.3 Telemetry & Promotion Hooks

- NDJSON schemas under `schemas/telemetry/` for:
  - `bci-mapping-activation-v1.json`,
  - `bci-mapping-clamp-v1.json`.
- CLI tools:
  - `bci-eval-binding` – offline replay harness.
  - `bci-eval-promotion` – compute promotion scores for bindings based on lab telemetry and promotion criteria.

These tools are required to close the loop between Phase 2 environment results and Phase 3 intervention tuning.

## 8. VM-Constellation Doctrine Alignment

The investigation is constrained by the constellation’s core doctrine:

- **Rust as Truth**
  - All safety‑critical math, curve evaluation, and caps enforcement live in Rust crates.
  - Lua is forbidden from performing numerical safety logic.

- **Lua as Orchestrator**
  - Lua assembles requests and routes responses.
  - Lua modules construct `BciMappingRequest` tables and interpret `BciMappingResponse` outputs into engine‑agnostic descriptor tables.

- **Schemas Define Reality**
  - JSON schemas are the primary source of truth for:
    - BCI envelopes,
    - Geometry bindings,
    - Safety profiles,
    - Telemetry events,
    - Intervention rules.
  - Code is auto‑derived or manually aligned with these schemas.

- **One-Way Pipeline**
  - No reverse mapping from outputs to raw data.
  - BCI pipelines are strictly feed‑forward; raw signals remain anonymized and inaccessible from game code.

- **Telemetry as View**
  - NDJSON event streams provide a read‑only view:
    - Mapping activations,
    - Clamp events,
    - Kernel perf.
  - Telemetry never defines new semantics; it only observes.

All research and lab artifacts must remain within this framework to be eligible for promotion into production.
