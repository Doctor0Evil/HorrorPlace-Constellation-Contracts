---
title: QPU Datashard Vocabulary
doc_type: protocol_doc
version: 1.0.0
status: draft
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
  - verifiers_registry
  - bundle_attestation
  - spectral_seed_attestation
  - agent_attestation
---

# QPU Datashard Vocabulary

This document defines the QPU Datashard envelope vocabulary and explains how it connects Seeds, Dead-Ledger proof envelopes, BCI swing functions, and cross-repo CI inside the HorrorPlace VM-Constellation. It is designed to be safe for Tier-1 publication: datashards carry only invariants, metrics, safety metadata, and opaque references – never raw trauma or BCI payloads.

The canonical JSON Schema for the envelope is `schemas/qpu_datashard_envelope_v1.json`. All QPU datashards must validate against that schema when emitted as NDJSON.

---

## 1. Purpose and role in the constellation

The QPU Datashard layer is a Tier-0 “wire format” that every participating repo can speak. It sits underneath existing contracts and schemas:

- Under Seeds (Atrocity-Seeds, Spectral-Foundry), it provides a uniform NDJSON view of region seeds, event seeds, and spectral seeds.
- Under persona and event contracts (Horror.Place, Codebase-of-Death), it exposes implementations and behaviors as persona-impl shards.
- Under Dead-Ledger, it carries proof metadata (not proofs) as zkp-proof-envelope shards.
- Under BCI research (Neural-Resonance-Lab), it carries bci-swing-function and telemetry-snapshot shards that are safe to ship across tiers.
- Under policy and governance repos, it exposes policy-profile shards that encode safety tiers and entitlement profiles.

The goal is to give orchestrators, CI pipelines, and AI/Copilot generators one minimal, stable envelope shape that can be parsed in Rust, Lua, PowerShell, and Python, while all rich content remains governed by existing schemas and Dead-Ledger proofs.

---

## 2. Envelope shape (qpu_datashard_envelope_v1)

All QPU datashards are single JSON objects written one-per-line as NDJSON. The schema enforces:

- One canonical URI in `schema` so validation is stable.
- A small set of routing and governance fields that must always be present.
- Explicit invariants and metric descriptors.
- Strict bans on additional properties that might carry raw content.

### 2.1 Core routing fields

Every shard must include:

- `schema`: Always `"https://horror.place/schemas/qpu-datashard-v1.json"` to anchor validation.
- `shard_id`: Stable identifier for the shard (e.g., `qpu.persona.archivist.v1`).
- `shard_kind`: Logical kind of shard (see the vocabulary in section 3).
- `schemaref`: Name or URI of the primary contract schema implemented by this shard (e.g., `personacontractv1.json`, `eventcontractv1.json`, `policyprofilev1.json`).
- `repo_tier`: One of `T1-core`, `T2-vault`, `T3-lab`; encodes the VM-Constellation tier.
- `repo_name`: Canonical repository name, such as `Horror.Place`, `HorrorPlace-Atrocity-Seeds`, `HorrorPlace-Dead-Ledger-Network`.

These fields let orchestrators and CI tools route, filter, and audit shards without introspecting repo-specific formats.

### 2.2 Safety and entitlement fields

The envelope encodes safety posture on every line:

- `safetytier`: Human-readable tier label such as `standard`, `mature`, or `research`.
- `intensity_band`: Integer from 0–10 representing normalized horror intensity.
- `entitlement_profile_id`: Identifier for the entitlement profile governing this shard (for example, `ent.profile.default.standard`, `ent.profile.adult.research.bci`).
- `deadledgerref`: Optional opaque handle such as `deadledger://zkp/sessionsafety/abc123` that binds this shard to a Dead-Ledger proof envelope.

These fields ensure that no shard can be used without an explicit safety and entitlement context, and they allow Dead-Ledger to reason about what must be proved or gated.

### 2.3 Functional role

The envelope distinguishes between types of work using:

- `function_kind`: Functional role of the shard, for example `static-contract`, `bci-swing`, `telemetry-slice`, `policy-card`.

This lets generators and tools reuse the same envelope for contracts, runtime behaviors, and telemetry summaries, while still keeping responsibilities clear.

---

## 3. Shard kinds: thin vocabulary surface

The `shard_kind` field defines a small, stable vocabulary that all repos share. Each kind is bound to one primary schema in the wider constellation.

### 3.1 Core shard kinds

The initial vocabulary includes:

- `persona-impl`  
  NDJSON view of persona or agent contracts. Typical `schemaref`: `personacontractv1.json`. These shards encode SHCI ranges, invariant expectations, and metric tendencies for an implementation.

- `invariant-bundle`  
  Thin projection of region or site bundles, usually mirrored from Black-Archivum. Typical `schemaref`: `invariantbundlev1.json`. Invariants such as CIC, MDI, AOS, RRM, FCF, SPR, RWF, DET, HVF, LSG, SHCI are present as numbers or bands; no narrative data appears.

- `spectral-seed`  
  Cross-repo view of Seed-like contracts. Typical `schemaref`: `seedcontractv1.json` or `eventcontractv1.json`. These shards capture stage and metric intent but route to the underlying seed files via `payload_ref`.

- `policy-profile`  
  Encodes safety rules, charter requirements, and ARR/CDL/DET caps for a cohort or configuration. Typical `schemaref`: `policyprofilev1.json`. BCI swing functions and orchestrators must respect these profiles.

- `telemetry-snapshot`  
  Aggregated, anonymized summaries of telemetry or BCI classification results over a region × time × persona slice. Typical `schemaref`: `telemetrysnapshotv1.json`. They use entertainment metrics and invariants only; no raw samples or identifiers.

### 3.2 Extended shard kinds

Additional kinds reuse the same envelope:

- `mechanic-spec`  
  Surprise mechanic contracts (for example, the SURP.* family) expressed as implication-only JSON contracts.

- `style-profile`  
  Art and audio style descriptors expressed in terms of invariants and metrics, not assets. Routes into style contract schemas in Horror.Place.

- `zkp-proof-envelope`  
  Metadata view of a Dead-Ledger proof envelope. Typical `schemaref`: `zkpproofv1.json`. The shard carries proof identifiers, invariant and metric bands, and prooftype, but never inline proof blobs.

- `bci-swing-function`  
  QPU-wrapped description of a BCI swing function and its allowed effects on metrics. Typical `schemaref`: `bciswingfnv1.json`. These are lab-tier artifacts that can still be referenced from vaults and core.

Each `shard_kind` is intended to map to exactly one primary schema in the schema spine, so tools can statically link envelope kinds to underlying schema contracts.

---

## 4. Invariants and metrics: aligning with the spine

The envelope’s `invariants` and `metrics` blocks are deliberately shaped to match the existing invariant and entertainment metric schemas.

### 4.1 Invariants block

The `invariants` object contains the full set of invariant codes:

- `CIC`, `MDI`, `AOS`, `RRM`, `FCF`, `SPR`, `RWF`, `LSG`, `SHCI`  
  All normalized in [0,1], expressing location and history properties.

- `DET`  
  Expressed in [0,10], representing exposure thresholds.

- `HVF`  
  Encoded as a small object `{ "mag": number in [0,1], "dir": string }` to represent both magnitude and qualitative direction of the haunt vector.

For some shard kinds (like persona-impl, spectral-seed, policy-profile), these invariants are the expected or constrained bands for the shard. For others (like telemetry-snapshot), they represent the background invariants in effect when the telemetry was collected.

### 4.2 Metrics block

The `metrics` object uses a reusable `metricDescriptor` type with optional fields:

- `band`: `[min, max]` range in [0,1].
- `target_band`: `[min, max]` range that the shard intends to maintain.
- `delta`: numeric change the shard tends to induce (positive or negative).
- `hard_cap`: scalar maximum allowed value.
- `max_band`: `[min, max]` range that must not be exceeded.

At least one of these must be present for each metric. The supported metrics are:

- `UEC` – Uncertainty Engagement Coefficient.
- `EMD` – Evidential Mystery Density.
- `STCI` – Safe–Threat Contrast Index.
- `CDL` – Cognitive Dissonance Load.
- `ARR` – Ambiguous Resolution Ratio.

This descriptor works for:

- Seeds that target specific metric envelopes.
- Persona implementations that tend to push metrics up or down.
- Policy profiles that enforce caps and target ranges.
- BCI swing functions that propose bounded deltas.
- Telemetry snapshots that report realized bands.

---

## 5. BCI binding and swing functions

The envelope includes optional support for BCI-related work that remains strictly proof- and summary-based.

### 5.1 bci_binding

The `bci_binding` field is either `null` or an object with:

- `input_signature`: Name of the abstract BCI summary type, such as `bcistate.alpha_theta_windowed_v1`.
- `inputs`: List of derived feature names (for example, `alpha_power_norm`, `theta_power_norm`, `hrv_band`, `eda_tonic_band`).
- `classification_band`: Qualitative label such as `calm`, `mild-arousal`, `high-arousal`, `fatigued`.
- `bcistate_schema`: Schema reference for the summarized BCI object.

Only pre-processed, derived features and classification labels are allowed. Raw EEG channels, raw video frames, or unprocessed time series must never appear in QPU datashards.

### 5.2 bci-swing-function shards

A BCI swing function is represented as a `bci-swing-function` shard with:

- `function_kind`: `bci-swing`.
- `invariants`: The invariant context where the function is valid (for example, high CIC and AOS vs low CIC highways).
- `metrics`: Allowed deltas and caps for UEC, EMD, STCI, CDL, ARR.
- `safetytier`, `intensity_band`, `entitlement_profile_id`: Safety envelope for the function.
- `bci_binding`: The input_signature and feature list it uses.
- `zkp`: A declaration of which Dead-Ledger prooftype (for example, `bcistate` or `sessionsafety`) must attest correct use.

The actual transformation logic lives in code referenced by `payload_ref`. The shard is a summary card that tools, CI, and Dead-Ledger can inspect without understanding implementation details.

---

## 6. Dead-Ledger and ZKP alignment

QPU datashards are not proofs, but the vocabulary is designed to integrate cleanly with Dead-Ledger.

### 6.1 deadledgerref and zkp

Two fields coordinate with proof infrastructure:

- `deadledgerref`: Opaque handle that links a shard to a proof envelope stored in HorrorPlace-Dead-Ledger-Network. It is treated as a capability; no fields from the proof are exposed.
- `zkp`: Optional object with:
  - `proof_type`: Name of the prooftype this shard expects (for example, `spectral_seed_attestation`, `bundle_attestation`, `bcistate`, `sessionsafety`).
  - `required`: Boolean indicating whether a valid proof must be present and verified before the shard can be acted upon.

CI and orchestrator code can enforce that:

- Any shard with `zkp.required = true` must have a valid `deadledgerref`.
- The referenced proof envelope must use a `prooftype` compatible with the shard’s `zkp.proof_type`.
- Proof envelopes themselves can be summarized as `zkp-proof-envelope` shard kinds.

### 6.2 Proof envelope shell alignment

The QPU datashard envelope is designed to align with, but not duplicate, the Dead-Ledger proof envelope shell. The proof shell contains:

- `proofid`, `prooftype`, `verifierid`, `proofsystem`, `proof_blob_ref`.
- `public_inputs` that include tier, intensity_band, invariant_bands, metric_bands, entitlement_profile, and deadledger_surface.

QPU datashards reuse the same invariant and metric bands, but express them as local state or intent rather than as public inputs to a proof. This separation allows Dead-Ledger to remain the authority on proofs, while QPU datashards remain generic routing and intent summaries.

---

## 7. Seeds, personas, and policies

The datashard vocabulary is deliberately shaped to sit directly under existing contract schemas.

### 7.1 Seeds (Atrocity-Seeds and Spectral-Foundry)

Seed-related shards:

- `spectral-seed` kinds with `schemaref` pointing at seed contract schemas.
- `invariants` describing the region or sequence bands.
- `metrics` describing intended UEC/EMD/STCI/CDL/ARR envelopes.
- `payload_ref` pointing to the concrete seed JSON in depot repos.

Seeds remain governed by their own schemas and CI. QPU datashards simply provide a streamable, cross-repo index of where seeds live and what invariant/metric envelopes they target.

### 7.2 Personas and events (Codebase-of-Death and core)

Persona and event implementations use:

- `persona-impl` shards with `schemaref` referencing the persona or event contract schema.
- `invariants` describing regions or conditions under which the implementation is valid.
- `metrics` describing typical impact on UEC, EMD, etc.
- `safetytier` and `entitlement_profile_id` to prevent research-grade agents from leaking into standard experiences.

These shards can be produced during CI or build steps and consumed by marketplace-like repos or ranking systems without ever touching implementation code directly.

### 7.3 Policies and entitlement

Policy repos emit:

- `policy-profile` shards, each bound to a `safetytier` and `entitlement_profile_id`.
- `metrics` describing global caps and preferred bands.
- Optional `zkp` metadata for proofs required to use each profile.

This makes it possible to enforce, for example, that all shards with `safetytier = standard` must also satisfy stricter DET caps and ARR bounds.

---

## 8. NDJSON rules and CI integration

QPU datashards are intended for cross-repo streaming via NDJSON. To keep them safe and analyzable, the following rules apply:

### 8.1 Self-contained lines

Every line must be self-contained:

- All required fields in the schema must appear.
- No shard may depend on context from previous or subsequent lines.
- Orchestrators must be able to validate and route a single line in isolation.

This allows simple line-based tooling in CI, ingestion scripts, and GitHub Actions.

### 8.2 No raw trauma or BCI

The envelope explicitly avoids fields that can carry raw content:

- No free-text narrative, dialogue, or historical description fields.
- No raw BCI channels, unaggregated timestamps, or physiological traces.
- Only invariants, aggregated metrics, derived BCI features, and opaque references to proofs or payloads.

Repos that work with raw material must do so internally and emit only derived, schema-validated datashards at their boundaries.

### 8.3 Schema validation in CI

HorrorPlace-Constellation-Contracts and other repos can add CI jobs to:

- Validate that all lines in a `*.ndjson` file conform to `qpu_datashard_envelope_v1.json`.
- Enforce that `shard_kind` and `schemaref` pairs are in a known registry.
- Verify that `deadledgerref` strings, where present, map to known verifiers and proof families.
- Check that no forbidden fields or patterns (such as raw asset URLs or base64 blobs) appear.

This integrates naturally with existing schema-validation and content-leak-scan jobs.

---

## 9. Implementation notes per language

The envelope is designed to be easy to consume across the constellation’s main languages.

### 9.1 Rust

- Define a `QpuDatashard` struct mirroring the schema.
- Use Serde for JSON deserialization.
- Provide per-`shard_kind` enums and helper methods to interpret `metrics` descriptors (for example, computing effective caps).

### 9.2 Lua

- Treat each NDJSON line as one table returned by a JSON decode function.
- Provide a small `QPU` module that:
  - Validates basic presence of required fields.
  - Routes shards by `shard_kind` to the appropriate game or simulation subsystem.
  - Exposes invariants and metrics via the existing `H.` and `Telemetry.` APIs.

### 9.3 PowerShell and CI scripting

- Use `ConvertFrom-Json` on each NDJSON line to work with shards in pipeline scripts.
- Implement simple filters for `shard_kind`, `safetytier`, and `repo_tier`.
- Connect to Dead-Ledger or orchestrator services using only `deadledgerref` and `zkp.proof_type`, never pulling in proof blobs in CI.

---

## 10. Extensibility and governance

The QPU Datashard vocabulary is intentionally small and must evolve slowly:

- Adding new `shard_kind` values should require a schema version bump and documentation updates.
- Adding fields is discouraged; instead, extend payload schemas referenced by `schemaref` and `payload_ref`.
- Any change that might allow raw trauma or BCI data into the envelope should be rejected at review and CI levels.

By keeping QPU datashard envelopes narrow and proof-aligned, the VM-Constellation gains a shared, auditable language that can be used by AI, Copilot, and humans without compromising safety or doctrinal constraints.
