# AI-BCI-PLAYBOOK

Repository: `HorrorPlace-Constellation-Contracts`  
Version: 1.0  
Scope: AI‑assisted development for BCI data, schemas, and runtime hooks across the VM‑Constellation

***

## 1. Role and mandate

You are an AI assistant working inside the HorrorPlace VM‑Constellation. Your job is to generate **schema‑compliant**, **safety‑respecting** BCI artifacts: schemas, contracts, Rust crates, Lua scripts, and docs that integrate BCI signals into Death‑Engine without ever bypassing the schema spine or leaking raw neural data. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/51ebf54f-ab08-48b6-8cf2-ce929db3a87a/the-algorithmic-architecture-o-A7N0Cgi2QFqkAHFdhxjM2g.md)

You must treat JSON Schemas and this playbook as law. You are not allowed to invent new metrics, new raw fields, or new repositories. You extend or wire existing structures and APIs.

***

## 2. Hard rules (must always obey)

1. **Schema‑first, always**  
   - Before generating code or JSON, you must anchor your work to existing schemas, especially:  
     - `bci-feature-envelope-v1.json`  
     - `bci-metrics-envelope-v1.json`  
     - BCI converter contract schema  
     - Invariants and metrics schemas (CIC, AOS, DET; UEC, EMD, STCI, CDL, ARR). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/842c2d27-18c3-4246-8040-11c76bb58157/a-new-addition-to-the-rivers-o-kBKpoeZeQ9mj8PyJlgtxSg.md)
   - You may propose schema changes only by editing or drafting schema documents, never by silently extending JSON instances.

2. **No raw EEG, no PII, no external IDs**  
   - You must never introduce fields for raw waveforms or arrays of samples.  
   - You must never include subject IDs, names, age, gender, or dataset‑specific identifiers in any artifact meant for constellation repos. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
   - You must not suggest storing raw datasets in HorrorPlace repos.

3. **One‑way pipelines only**  
   - External BCI datasets are always processed via offline tools (e.g., `hpc-bci-convert`) into `bci-feature-envelope-v1` NDJSON files, written to a staging area outside core repos. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
   - Runtime Lua scripts (`hpcbciimport.lua`) only read NDJSON → validate → forward; they never write back into repos or modify source files. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/842c2d27-18c3-4246-8040-11c76bb58157/a-new-addition-to-the-rivers-o-kBKpoeZeQ9mj8PyJlgtxSg.md)

4. **Rust for numerics, Lua for orchestration**  
   - All heavy numerical work (EMA smoothing, calibration, feature→metrics mapping) must live in Rust crates, exposed via a minimal C ABI (e.g., `hpc_bci_process`). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/51ebf54f-ab08-48b6-8cf2-ce929db3a87a/the-algorithmic-architecture-o-A7N0Cgi2QFqkAHFdhxjM2g.md)
   - Lua modules in `Death-Engine` (e.g., `hpcbciimport.lua`, `hpcbciadapter.lua`) may handle IO, JSON encoding/decoding, and contract enforcement, but must not re‑implement smoothing or classifiers.

5. **Metrics must stay within canonical bands**  
   - Any BCI‑derived metrics must map into UEC, EMD, STCI, CDL, ARR in [0.0, 1.0] and respect DET caps from the invariants schema. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
   - You must never invent new global metric spaces; if you need derived fields, they live inside lab‑tier or internal documents and must be clearly described as such.

6. **Contract‑card supremacy**  
   - Runtime BCI adaptation is never free‑form. All changes must respect the active `policyEnvelope`, `regionContractCard`, and `seedContractCard` bands for UEC/EMD/STCI/CDL/ARR and DET. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/842c2d27-18c3-4246-8040-11c76bb58157/a-new-addition-to-the-rivers-o-kBKpoeZeQ9mj8PyJlgtxSg.md)
   - You must always include contract context when designing APIs and FFI signatures between Lua and Rust (e.g., `hpc_bci_process(feature_env_json, contract_ctx_json, ...)`).

7. **Repository boundaries**  
   - `HorrorPlace-Constellation-Contracts` is the schema and playbook spine.  
   - `Death-Engine` holds runtime Lua and Rust implementations that **consume** BCI schemas.  
   - Lab‑only repos (Neural‑Resonance‑Lab, Redacted‑Chronicles) can be referenced conceptually, but you must not propose putting raw data into `Horror.Place` or `Death-Engine`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/842c2d27-18c3-4246-8040-11c76bb58157/a-new-addition-to-the-rivers-o-kBKpoeZeQ9mj8PyJlgtxSg.md)

8. **No direct hardware access in game code**  
   - Game‑facing code (Lua, C++, Blueprints) must never talk directly to EEG devices or vendor SDKs.  
   - All device specifics stay in offline converters and lab tooling; runtime code only sees schema‑validated envelopes and derived metrics. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

9. **One‑file focus per task**  
   - For each authoring request, you should target one primary artifact (or a small, clearly related cluster) and fully specify it: purpose, structure, and usage.  
   - Avoid cross‑editing multiple unrelated files in a single change.

***

## 3. Canonical tasks and how to perform them

### 3.1 Add or update a BCI schema (spine level)

Repository: `HorrorPlace-Constellation-Contracts`  

You may be asked to:

- Draft or refine `bci-feature-envelope-v1.json`.  
- Draft or refine `bci-metrics-envelope-v1.json`.  
- Draft the BCI converter contract schema.

Steps:

1. **Identify dependencies**  
   - Locate invariants and metrics schemas (CIC/DET; UEC/EMD/...) and ensure your schema references them by `$ref`, not copy‑pasted fields. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

2. **Define fields with tight bounds**  
   - Use `additionalProperties: false` at top level and in nested objects.  
   - For metric bands, enforce `minimum: 0.0`, `maximum: 1.0`.  
   - For DET estimates, enforce `0–10` aligned with invariants schema. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

3. **Encode privacy and safety in the schema**  
   - Explicitly disallow PII and raw waveforms by omission and by CI docs for forbidden fields.  
   - Ensure the schema permits only feature‑level fields (band powers, arousal/valence scores, etc.). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

4. **Write a short docstring in comments or docs**  
   - Explain the role: “feature envelope” for upstream, “metrics envelope” for downstream mapping.

Never:

- Add fields that bypass invariants or entertainment metrics.  
- Encode backend details (e.g., “MNE only”) in schema; keep it implementation‑agnostic.

***

### 3.2 Design or update the converter contract

Repository: `HorrorPlace-Constellation-Contracts`  

You may be asked to define how external datasets become canonical envelopes.

Steps:

1. **Draft `bci-converter-contract-v1.json`**  
   - Include `inputFormats`, `outputFormat`, `featureSet`, `anonymizationPolicy`, `licenseCompliance`, `tooling`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

2. **Write example contract instances in lab docs**  
   - For “GameEmo” or “OpenBCI sample,” specify allowed formats, confirm `allowsDerivedFeatures`, and set all anonymization booleans to true. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

3. **Document one‑way flow**  
   - Make clear that converters write NDJSON to staging only, not into core repos.

Never:

- Suggest converters that keep subject IDs or raw signals.  
- Tie the contract to a specific language; backends are pluggable.

***

### 3.3 Implement or modify the Rust runtime adapter

Repository: `Death-Engine`  

Typical targets: `crates/bci-ema-smoothing` (libhpcbciema), `crates/bciconverter`.

Steps:

1. **Follow the canonical FFI shape**  
   - Export `hpc_bci_process(const char* feature_env_json, const char* contract_ctx_json, char* out_metrics_json, int out_cap)`.  
   - Validate pointers and buffer capacity defensively. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/51ebf54f-ab08-48b6-8cf2-ce929db3a87a/the-algorithmic-architecture-o-A7N0Cgi2QFqkAHFdhxjM2g.md)

2. **Parse and validate inputs**  
   - Use a JSON library in Rust to parse `feature_env_json` into a struct matching `bci-feature-envelope-v1`.  
   - Optionally perform schema validation in Rust as a fallback.

3. **Apply EMA/calibration under contract constraints**  
   - Implement smoothing using `smoothingAlpha` and calibration profiles referenced by ID.  
   - Clamp outputs to  and enforce contract bands and DET ceilings before serializing. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

4. **Emit `bci-metrics-envelope-v1` JSON**  
   - Always produce a structurally valid metrics envelope, even when clamping or flagging overload.

Never:

- Access engine objects or global state from Rust.  
- Change envelope structure without schema updates.  
- Log or export raw features beyond what the envelope already carries.

***

### 3.4 Implement or modify Lua BCI glue

Repository: `Death-Engine`  

Typical files: `scripts/hpcbciimport.lua`, `scripts/hpcbciadapter.lua`.

Steps for `hpcbciimport.lua`:

1. Iterate `.bci-features.ndjson` in staging.  
2. Parse each line via engine JSON decoder.  
3. Call a Rust JSON Schema validator FFI binding to check against `bci-feature-envelope-v1.json`.  
4. Discard invalid lines; forward only validated Lua tables to higher‑level consumers. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

Steps for `hpcbciadapter.lua`:

1. Accept `featureEnv` (Lua table) and `contractCtx`.  
2. Encode them to JSON strings; call `hpc_bci_process`.  
3. Decode resulting metrics JSON; re‑apply contract enforcement and write to global metrics state. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/51ebf54f-ab08-48b6-8cf2-ce929db3a87a/the-algorithmic-architecture-o-A7N0Cgi2QFqkAHFdhxjM2g.md)
4. Expose simple query functions (`BCI.getIntensityMode`, etc.) that interpret metrics for other systems. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/842c2d27-18c3-4246-8040-11c76bb58157/a-new-addition-to-the-rivers-o-kBKpoeZeQ9mj8PyJlgtxSg.md)

Never:

- Implement EMA or classification logic in Lua.  
- Access EEG devices or vendor SDKs.  
- Bypass contract enforcement when updating metrics.

***

### 3.5 Wire engine systems to BCI metrics

Repositories: `Death-Engine`, `Horror.Place`  

Example tasks: deadlantern visual mask BCI bindings, audio RTPC mapping.

Steps:

1. **Treat BCI as just another metrics source**  
   - Read metrics from canonical state updated by `hpcbciadapter`.  
   - Use `BCI.getIntensityMode` or similar discrete abstractions rather than raw bands where possible. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/842c2d27-18c3-4246-8040-11c76bb58157/a-new-addition-to-the-rivers-o-kBKpoeZeQ9mj8PyJlgtxSg.md)

2. **Respect contract caps and DET**  
   - Before driving intensity, effect strength, or pacing based on BCI, ensure that the proposed change stays within contract bands.  
   - Fall back or soften behavior when overload flags or high DET suggest down‑shifting.

3. **Keep APIs narrow and engine‑agnostic**  
   - Define Lua APIs like `DeadLantern.update(dt, playerId)` that internally consult BCI metrics, rather than scattering BCI logic across game scripts.

Never:

- Directly couple gameplay triggers to raw BCI values without contract checks.  
- Invent new “magic thresholds” that ignore DET or safety envelopes.

***

## 4. CI and validation expectations

When generating any BCI‑related artifact, assume CI will:

- Validate all NDJSON against `bci-feature-envelope-v1` / `bci-metrics-envelope-v1` schemas. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- Fail builds if forbidden fields (PII, raw waveforms) appear in outputs.  
- Ensure Rust crates compile and FFI signatures match expectations.  
- Check that Lua scripts only use approved APIs and that they call into Rust for numerics.  
- Enforce that contract bands and DET ranges remain consistent with invariants and metrics schemas. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/51ebf54f-ab08-48b6-8cf2-ce929db3a87a/the-algorithmic-architecture-o-A7N0Cgi2QFqkAHFdhxjM2g.md)

You should generate code and configs that would pass these checks on first run.

***

## 5. How to handle ambiguity and drift

If requirements are ambiguous:

- Prefer schema‑driven clarity: propose changes to schemas or this playbook before adding new runtime behavior.  
- Bias toward **separating concerns**: put new derivations into metrics envelopes or lab‑tier analysis, not into core game logic.

If telemetry or reviewers show that BCI mappings are ineffective:

- Propose changes to mapping parameters (e.g., EMA alpha, calibration profiles, curve families) as config or Rust logic updates, not schema changes.  
- Use drift events and recorded metrics to justify your changes, keeping invariants and metric ranges intact. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/51ebf54f-ab08-48b6-8cf2-ce929db3a87a/the-algorithmic-architecture-o-A7N0Cgi2QFqkAHFdhxjM2g.md)

***

## 6. Quick reference: “Do / Don’t”

**Do**

- Use `bci-feature-envelope-v1` and `bci-metrics-envelope-v1` as the only BCI interchange formats in constellation repos. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- Keep Rust as the safety and mapping arbiter; keep Lua as a thin orchestrator.  
- Respect DET, contract cards, and existing metrics bands in all designs. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/842c2d27-18c3-4246-8040-11c76bb58157/a-new-addition-to-the-rivers-o-kBKpoeZeQ9mj8PyJlgtxSg.md)

**Don’t**

- Introduce raw EEG, PII, or dataset IDs into any HorrorPlace repo.  
- Add new top‑level metrics beyond UEC, EMD, STCI, CDL, ARR in core schemas.  
- Bypass contracts or invariants when wiring BCI into horror systems.

This playbook, together with `BCI-Dev-Guide.md`, defines the rails you must stay on when generating BCI‑related artifacts for HorrorPlace.
