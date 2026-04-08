# BCI-Dev-Guide

Repository: `HorrorPlace-Constellation-Contracts`  
Version: 1.0  
Scope: Device → feature server → EEGFeatureContract → HorrorDirector → engine

***

## 1. Overview & constraints

This guide defines the **schema‑first** contract for all BCI work in Horror$Place: device integration, feature extraction, policy, and engine hooks. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/842c2d27-18c3-4246-8040-11c76bb58157/a-new-addition-to-the-rivers-o-kBKpoeZeQ9mj8PyJlgtxSg.md)

High‑level architecture:

- **Device / backend:** BrainFlow or LSL driver reads EEG from a physical board or simulator.  
- **Feature server:** Python/Rust process runs `EEGCanonicalV1` and emits `EEGFeatureContract` NDJSON over TCP or file. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/842c2d27-18c3-4246-8040-11c76bb58157/a-new-addition-to-the-rivers-o-kBKpoeZeQ9mj8PyJlgtxSg.md)
- **HorrorDirector:** Ingests `EEGFeatureContract`, applies BCI intensity policy, and outputs normalized `EEGFeatures` and tension states.  
- **Engine layer:** Unity or Unreal subsystems read `EEGFeatures` (live or replay) and drive HorrorDirector‑controlled mappings into game systems. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

Non‑negotiable constraints:

- Schema‑first: all BCI JSON must validate against schemas in `HorrorPlace-Constellation-Contracts` (feature contract, policy, traces, experiments). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- No PII, no raw EEG: only derived features and intensity metrics are allowed in constellation repos and logs. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- All runtime EEG passes through HorrorDirector and BCI policies before affecting gameplay; device specifics are never visible to game scripts. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/842c2d27-18c3-4246-8040-11c76bb58157/a-new-addition-to-the-rivers-o-kBKpoeZeQ9mj8PyJlgtxSg.md)

***

## 2. Invariants & metrics

Horror$Place already defines invariants and entertainment metrics that BCI must align with. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/842c2d27-18c3-4246-8040-11c76bb58157/a-new-addition-to-the-rivers-o-kBKpoeZeQ9mj8PyJlgtxSg.md)

Core invariants:

- CIC, MDI, AOS, RRM, FCF, SPR, RWF, DET, HVF, LSG, SHCI: describe historical trauma, mythic density, archival opacity, ritual residue, spectral plausibility, dread threshold, haunt vectors, and liminal stress. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/842c2d27-18c3-4246-8040-11c76bb58157/a-new-addition-to-the-rivers-o-kBKpoeZeQ9mj8PyJlgtxSg.md)

Entertainment metrics:

- UEC, EMD, STCI, CDL, ARR: 0–1 bands describing uncertainty, emerging malevolence, spectral threat concealment, cognitive dissonance, and ambiguous resolution. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/1a78de7d-721b-4a76-b3b4-9fde57441d18/this-research-focuses-on-creat-IQAMBfJDSAit.SxDaXO6Gg.md)

In `EEGFeatureContract`:

- `horror_context` includes a minimal invariant snapshot (e.g., CIC, AOS, DET, HVF, LSG, SHCI) and current entertainment metrics (UEC, ARR, CDL, EMD, STCI) for the region or scene. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/1a78de7d-721b-4a76-b3b4-9fde57441d18/this-research-focuses-on-creat-IQAMBfJDSAit.SxDaXO6Gg.md)
- These fields are **inputs** to mapping logic and BCI policy—not outputs that mapping can arbitrarily override.

***

## 3. Schemas and contracts

This section enumerates BCI‑side schemas and where they live.

### 3.1 EEGFeatureContract

Canonical ID: `EEGFeatureContractv1`  
Suggested path: `Horror.Place/schemas/EEGFeatureContractv1.json` (mirrored from spine). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/842c2d27-18c3-4246-8040-11c76bb58157/a-new-addition-to-the-rivers-o-kBKpoeZeQ9mj8PyJlgtxSg.md)

Field inventory (conceptual):

- `schema_id`, `schema_version`.  
- `session_id`, `window_id`, `timestamp`.  
- `device_id` (logical), `backend` (`"BrainFlow"` or `"LSL"`), no hardware serials.  
- `horror_context` – CIC, AOS, DET, HVF, LSG, SHCI, UEC, ARR, etc., within invariant ranges. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/1a78de7d-721b-4a76-b3b4-9fde57441d18/this-research-focuses-on-creat-IQAMBfJDSAit.SxDaXO6Gg.md)
- `eeg_features` – canonical feature set (see §5), including band powers and composite scores.  
- `quality` – signal quality flags, dropped channels, etc.

### 3.2 BCI‑intensity policy schema

Canonical ID: `bci-intensity-policy-v1`  
Path: `HorrorPlace-Constellation-Contracts/schemas/bci-intensity-policy-v1.json`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/842c2d27-18c3-4246-8040-11c76bb58157/a-new-addition-to-the-rivers-o-kBKpoeZeQ9mj8PyJlgtxSg.md)

Role:

- Defines per‑tier caps and guardrails: maximum tension, rate of change, cooldowns, overload behavior.  
- Includes bounds on HorrorDirector outputs (e.g., max tension, max spawn multiplier, max post‑process weight).

### 3.3 Synthetic trace & expectations schemas

Canonical IDs:

- `synthetic-eeg-feature-trace-v1` – NDJSON trace of `EEGFeatureContract` frames.  
- `synthetic-eeg-expectations-v1` – expectations for HorrorDirector outputs along the trace (tension, events, rate caps). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/842c2d27-18c3-4246-8040-11c76bb58157/a-new-addition-to-the-rivers-o-kBKpoeZeQ9mj8PyJlgtxSg.md)

Paths (examples):

- `HorrorPlace-Spectral-Foundry/schemas/synthetic-eeg-feature-trace-v1.json`  
- `HorrorPlace-Spectral-Foundry/schemas/synthetic-eeg-expectations-v1.json`

### 3.4 ExperimentConfig schema

Canonical ID: `experiment-config-v1`  
Path: `HorrorPlace-Atrocity-Seeds/experiments/ExperimentConfigExamples/ExperimentConfig.schema.json`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/842c2d27-18c3-4246-8040-11c76bb58157/a-new-addition-to-the-rivers-o-kBKpoeZeQ9mj8PyJlgtxSg.md)

Fields:

- `conditions`, `blocks`, `scenes`, BCI source (live/replay), mapping profiles, policy reference.

***

## 4. Device integration patterns

Only two backend families are supported: **BrainFlow** and **LSL**. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/842c2d27-18c3-4246-8040-11c76bb58157/a-new-addition-to-the-rivers-o-kBKpoeZeQ9mj8PyJlgtxSg.md)

### 4.1 Logical device registry and IEEGDevice

Schema: `EEGDeviceRegistry.json` (in `HorrorPlace-Constellation-Contracts` or `Horror.Place`).

Pattern:

- Each device entry contains: logical ID, backend type (`"BrainFlow"` / `"LSL"`), channel layout, sampling rate, feature server endpoint.  
- Drivers implement an `IEEGDevice` interface in the engine (C#/C++):

  - `Connect()`, `Disconnect()`, `ReadSamples()`, `GetFeatures()` returning `EEGFeatures`.

### 4.2 Python feature server pattern

Feature server (backend repo):

- Uses BrainFlow `BoardShim` or LSL inlet to read samples.  
- Runs `EEGCanonicalV1` (see §5) in a loop over windows.  
- Emits `EEGFeatureContractv1` JSON frames as NDJSON over TCP or writes NDJSON files.

Game code never talks to BrainFlow or LSL directly; it only consumes `EEGFeatureContract` or `EEGFeatures` from the feature server. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/842c2d27-18c3-4246-8040-11c76bb58157/a-new-addition-to-the-rivers-o-kBKpoeZeQ9mj8PyJlgtxSg.md)

***

## 5. Feature extraction (EEGCanonicalV1)

The canonical pipeline from raw EEG to `eeg_features` inside `EEGFeatureContract` is `EEGCanonicalV1`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

Stages:

1. **Windowing:** fixed length (e.g., 1–2 seconds), with overlap.  
2. **Preprocessing:** band‑pass filter, notch, rereferencing as required.  
3. **PSD and bands:** compute PSD per channel, integrate into bands (delta, theta, alpha, beta, gamma).  
4. **Regions & composites:** aggregate per region (e.g., frontal/central/parietal) and compute composite scores (e.g., arousal, valence, attention). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
5. **BCI metrics:** derived indices (e.g., fear index, overload score) mapped into normalized  features, but not directly into UEC/ARR; those stay in HorrorDirector. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/1a78de7d-721b-4a76-b3b4-9fde57441d18/this-research-focuses-on-creat-IQAMBfJDSAit.SxDaXO6Gg.md)

All features are stored in `EEGFeatureContract.eeg_features` using schema‑defined names and units.

***

## 6. Engine integration

Each engine mirrors the same conceptual pattern: a feature subsystem plus HorrorDirector.

### 6.1 Unity

Repository: `Horror.Place` (Unity side)

Core types:

- `EEGFeatures` – struct mirroring a subset of `EEGFeatureContract.eeg_features`.  
- `EEGFeatureService` – singleton that connects to feature server, maintains latest `EEGFeatures`, and optionally replays traces. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/842c2d27-18c3-4246-8040-11c76bb58157/a-new-addition-to-the-rivers-o-kBKpoeZeQ9mj8PyJlgtxSg.md)
- `HorrorDirector` – reads `EEGFeatures`, invariants, and policy; produces tension, mode, and a small set of knobs (e.g., post‑process weight, spawn multiplier).

Pattern:

- Gameplay scripts read from HorrorDirector, not from EEG or BrainFlow directly.  
- Config switches between **live** (`EEGFeatureService` TCP) and **replay** (trace NDJSON).

### 6.2 Unreal

Repository: `Death-Engine`

Core types:

- `FEEGFeatures` – struct equivalent of `EEGFeatures`.  
- `UEEGFeatureSubsystem` – subsystem that ingests features (live/replay) and exposes them to gameplay code. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- HorrorDirector subsystem – maps features + invariants + policy into engine‑level parameters.

Pattern:

- Actor components/Blueprints depend on HorrorDirector outputs (e.g., `GetTension()`, `GetEnemySpawnScale()`), not on EEG devices.

### 6.3 Replay sources

Both engines must support:

- Live source (feature server).  
- File replay (synthetic or recorded traces).  
- “Null” source for debugging (no BCI input).

Source is selected via config, not code changes.

***

## 7. Adaptive logic & safety

HorrorDirector encapsulates all BCI–to–game adaptation. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/1a78de7d-721b-4a76-b3b4-9fde57441d18/this-research-focuses-on-creat-IQAMBfJDSAit.SxDaXO6Gg.md)

### 7.1 Mapping strategies

Allowed strategies (per mapping config):

- **Linear:** direct scaling from index to output, with caps.  
- **Zones:** discrete bands (low/medium/high) with stepwise outputs.  
- **Curves:** sigmoid, piecewise, hysteresis, or oscillatory profiles (defined in config, executed in engine/lab code). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/1a78de7d-721b-4a76-b3b4-9fde57441d18/this-research-focuses-on-creat-IQAMBfJDSAit.SxDaXO6Gg.md)
- **PID‑like:** simple feedback with proportional/integral terms, bounded by policy.

Each mapping config references:

- Input feature(s): e.g., `fear_index`, `overload_score`.  
- Target output: `tension`, `volume_weight`, `spawn_multiplier`, etc.  
- Strategy type + parameters (slopes, thresholds, time constants).

### 7.2 BCI intensity policies and overrides

BCI intensity policy:

- Defines caps on tension, rate of change, and cooldown behavior per safety tier.  
- Overrides mapping outputs when overload flags are set (e.g., clamp tension, reduce stimuli, widen deadlantern mask). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/1a78de7d-721b-4a76-b3b4-9fde57441d18/this-research-focuses-on-creat-IQAMBfJDSAit.SxDaXO6Gg.md)

Direct vs indirect control:

- **Direct:** BCI directly influences a visible knob (e.g., tunnel radius). Use sparingly, with strong caps.  
- **Indirect:** BCI only nudges probabilities or weights (e.g., slight increase in chance of subtle events). Preferred default.

***

## 8. Experiment & analysis

### 8.1 Experiment orchestration patterns

Experiments use `experiment-config-v1`:

- Define `conditions` (e.g., mapping profile A/B, different policies).  
- Define `blocks` and `scenes` with region types, expected UEC/ARR bands, and BCI source (live vs replay). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/1a78de7d-721b-4a76-b3b4-9fde57441d18/this-research-focuses-on-creat-IQAMBfJDSAit.SxDaXO6Gg.md)

Orchestrator responsibilities:

- Run blocks in sequence; set HorrorDirector mapping and policy per condition.  
- Log full traces of `EEGFeatureContract`, HorrorDirector outputs, and key game events as NDJSON.

### 8.2 NDJSON logging rules

Logging must:

- Use NDJSON with a clear schema (`synthetic-eeg-feature-trace-v1`, director output schemas, etc.). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- Never record raw EEG samples or PII; only features and derived metrics.  
- Include references to experiment config ID, policy ID, mapping profile ID.

### 8.3 Standard analysis notebooks

Golden notebooks (backend / lab repo):

- `EEGCanonicalBandPowerNotebook` – demonstrates loading NDJSON, computing statistics per band, and cross‑checking against `EEGCanonicalV1` expectations. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/1a78de7d-721b-4a76-b3b4-9fde57441d18/this-research-focuses-on-creat-IQAMBfJDSAit.SxDaXO6Gg.md)
- Mapping evaluation notebook – compares UEC/ARR/CDL/STCI under different mapping configs and policies.

The dev guide references these notebooks as standard entry points for analysis.

***

## 9. AI‑chat playbook summary

AI tools must treat this guide as their primary contract for BCI work. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

Must:

- Read `BCI-Dev-Guide.md` and relevant JSON Schemas (EEGFeatureContract, policies, traces, experiment configs) before generating BCI code or config.  
- Use existing types and patterns: `EEGFeatureContract`, `EEGFeatures`, `IEEGDevice`, `EEGFeatureService`, `FEEGFeatures`, `UEEGFeatureSubsystem`, HorrorDirector.  
- Respect BCI intensity policies and caps in any mapping or director code.  
- Always provide tests or synthetic trace configs when introducing new mappings.

Must not:

- Access EEG hardware directly from game scripts or engine code.  
- Add fields to `EEGFeatureContract` or other schemas without explicit schema updates.  
- Log raw EEG waveforms or any PII into HorrorPlace repos. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

Golden examples (to copy patterns from):

- **Unity:** `UnityBCIExampleScene` – minimal scene with `EEGFeatureService` + HorrorDirector driving one post‑process effect and an enemy spawn rate.  
- **Unreal:** `UnrealBCIExampleMap` – map using `UEEGFeatureSubsystem` and HorrorDirector to drive a post‑process volume and simple AI intensity.  
- **Backend:** `EEGCanonicalFeatureServer.py` – BrainFlow/LSL → `EEGFeatureContract` feature server.  
- **Analysis:** `EEGCanonicalBandPowerNotebook.ipynb` – NDJSON analysis notebook.

These examples are the normative recipes; AI‑generated code should follow their shapes and reference the same schema IDs and field names.
