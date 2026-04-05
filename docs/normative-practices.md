***

# Normative Practices for VM‑Constellations

This document defines the normative rules that all AI agents, tools, CI workflows, and engines must follow when generating, wiring, or consuming files that depend on HorrorPlace‑Constellation‑Contracts. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)

It is written for: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

- AI‑chat systems acting as deterministic compilers.  
- CI / pre‑commit authors.  
- Engine and tooling integrators (Lua, Rust, Unreal, Godot, etc.).  

***

## 1. Schema Spine as Physics Layer

The schema spine is the immutable “physics layer” of the constellation. All data and behavior must conform to it. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)

### 1.1 Canonical schema families

The spine is composed of: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

- Invariants  
  CIC, MDI, AOS, RRM, FCF, SPR, RWF, DET, HVF, LSG, SHCI.  
  These describe history, trauma, and spectral plausibility.  

- Entertainment metrics  
  UEC, EMD, STCI, CDL, ARR.  
  These describe the intended and observed player experience.  

- Generic contracts  
  Style, event, persona, routing, entitlement, telemetry envelopes.  

- AI authoring contracts  
  Authoring request / response envelopes, prism envelopes, and contract cards.  

### 1.2 Non‑negotiable rules

- All constellation data that references history, fear, or style must validate against one of the canonical schemas in `schemas/`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)
- No repository may extend core invariants or metrics in‑place. Changes require: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)
  - A new schema version.  
  - A logged migration plan.  
- Local extensions (e.g., engine‑specific fields) must live under clearly namespaced, schema‑allowed extension objects, never as silent top‑level fields. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)

***

## 2. NDJSON Registries as the Global Index

NDJSON registries are the single source of truth for “what exists” in a constellation. Engines and AI must discover entities via registries, not guessed paths. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

### 2.1 Registry files

Typical registries: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)

- `registry/registry-regions.ndjson`  
- `registry/registry-events.ndjson`  
- `registry/registry-personas.ndjson`  
- `registry/registry-styles.ndjson`  
- Additional registries for VM roles or other entities as needed.  

Each file contains one logical entity per line (newline‑delimited JSON). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

### 2.2 Core registry entry shape

All registry lines must validate against a canonical `registry-entry-v1` schema. At minimum: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)

- `id`  
  Stable, globally unique identifier (string).  

- `schemaref`  
  Canonical schema URI this entry conforms to.  

- `tier`  
  One of: `public`, `vault`, `lab` (or a documented superset, if extended).  

- Implementation reference (one or more of):  
  - `artifact_id` – repo‑local or cross‑repo identifier.  
  - `cid` – content address (e.g., IPFS‑like).  
  - `deadledgerref` – object containing proof envelope metadata.  

- Optional metadata:  
  - Human‑readable name, tags, descriptions.  
  - Engine integration hints (e.g., which module consumes this entry).  

### 2.3 Normative constraints

- Registries may not embed raw horror content or large assets; they store only IDs, hashes, and proof references. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)
- `schemaref` must point to a canonical schema in the spine. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)
- `deadledgerref` (when present) must conform to Dead‑Ledger proof schemas and reference a valid verifier. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)
- Duplicate IDs within the same registry are forbidden. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)
- Registries must be the first stop for AI and tools when resolving where to place or find artifacts. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

***

## 3. Prism / AI‑Authoring Contracts

AI‑chat and tools are treated as deterministic compilers that emit structured artifacts, not freeform text. All AI‑generated artifacts must be wrapped in a prism envelope. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)

### 3.1 Up to three files per request

The relaxed but still disciplined rule: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

- Each authoring request that “creates” something may produce between one and three primary artifact files plus optional registry deltas.  
- All artifacts from a single request must be described in one prism envelope or a nested changeset structure and validated as a single logical transaction.  

Typical safe patterns include: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)

- Single contract file.  
- Region + persona pair.  
- Contract card + one or two registry lines.  

Anything beyond three primary files must be split into multiple requests or handled by a higher‑level orchestrator. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

### 3.2 Prism envelope normative fields

Every AI‑generated artifact set MUST include the following in its prism envelope: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)

- `targetRepo`  
  The repository the artifact(s) should be committed to.  

- `targetPath` (or an array of paths when multiple artifacts are present)  
  Exact path(s) within `targetRepo` (relative to repo root).  

- `schemaref` and/or `schemaVersion`  
  Canonical schema ID / URI and version each file must validate against.  

- `tier`  
  Environment or visibility tier (`public`, `vault`, `lab`, etc.).  

- `referencedIds`  
  Array of IDs for related entities (regions, events, personas, styles, seeds).  

- Optional `deadledgerref`  
  Required for artifacts that must be cryptographically attested (age‑gated, chartered, or otherwise constrained).  

The prism envelope may wrap: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

- One or more generic contracts (event, persona, style, routing, entitlement) up to the three‑artifact cap.  
- One or more contract cards (policy, region, seed).  
- A changeset (e.g., two contract files and one or more registry lines) that CI treats as a single transaction.  

### 3.3 Behavior expectations for AI

Before generating a prism: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)

- Query the schema spine index to determine the required `schemaref` and allowed fields.  
- Query NDJSON registries to select valid IDs for cross‑references.  
- Select `tier` based on the project’s governance rules (e.g., production content cannot target `lab` tiers).  

The AI must never: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

- Invent unregistered schema IDs.  
- Emit artifacts that cannot be validated by the shared CI workflows.  
- Bypass prism envelopes when targeting constellation repositories.  

***

## 4. Contract Card Hierarchy

Contract cards define hierarchical constraints on invariants and metrics. They are the canonical way to express “what is allowed” at different scopes. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

### 4.1 Three core card types

Normatively, a constellation uses: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

1. `policyEnvelope`  
   Global bounds for a slice of work (project, campaign, or content band).  
   - Declares allowed styles, invariant ranges, and metric bands.  
   - Anchored to a specific `targetRepo`, `path`, `tier`, and `deadledgerref`.  

2. `regionContractCard`  
   Bounds for a specific region / area under a `policyEnvelope`.  
   - Invariant and metric caps must be subsets of the policy’s bounds.  
   - Declares region‑level styles and allowed seeds.  

3. `seedContractCard`  
   A single seed or micro‑experience within a region.  
   - Invariants and metrics must be subsets of the region card’s bounds.  
   - Typically bound to PCG rules, AI behaviors, and audio/VFX hooks downstream.  

### 4.2 Narrowing rule (non‑negotiable)

For all contract card hierarchies: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

- Policy → Region → Seed must be strictly narrowing.  
- No contract at a lower level may widen invariant or metric bands versus its parent.  
- Widening requires editing the parent contract, with normal CI and review.  

### 4.3 AI behavior and cards

AI‑chat systems that propose new regions or seeds: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

- MUST start by drafting or editing the appropriate contract card.  
- MUST ensure numeric invariants and metrics respect parent bounds.  
- MAY then propose downstream implementation hints, but those hints must reference the contract card, not replace it.  

***

## 5. Request Granularity and Changesets

To avoid chaotic file sprawl while allowing small bundles, the constellation enforces a “three‑files‑maximum per request” discipline for AI‑authored content. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)

### 5.1 Definition

For a single AI authoring turn: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)

- Between one and three primary artifact files may be emitted (contracts, schema‑conforming JSON, or similar).  
- The prism envelope may include zero or more registry deltas, but those are considered a logical extension of the same change.  

### 5.2 Changesets

- Batched operations (e.g., a region + persona + registry entries) are allowed via explicit changeset envelopes that: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)
  - Declare all artifacts and registry changes.  
  - Are validated atomically by CI.  
  - Are applied as a single logical transaction.  

### 5.3 Benefits

- Deterministic CI outcomes (accept / reject a small, bounded set of artifacts). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)
- Easier audit and provenance tracking. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)
- Simplified integration for editors and engines. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

***

## 6. Cross‑Repo CI / Pre‑Commit Pack

All constellation‑aware repositories are expected to import and use a shared CI and pre‑commit pack shipped by this repository. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/b86f5942-50b7-460e-a54b-1044e30630e5/create-a-task-for-microsoft-s-3ukEpSkxTiieobAHUuxocQ.md)

### 6.1 Standard workflows

The pack includes reusable GitHub Actions and local hooks that perform: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/b86f5942-50b7-460e-a54b-1044e30630e5/create-a-task-for-microsoft-s-3ukEpSkxTiieobAHUuxocQ.md)

- Schema validation  
  Validate all JSON files (and relevant YAML, if used) against canonical schemas in this repo.  

- Registry linting  
  Validate all NDJSON registries for:  
  - `id` uniqueness.  
  - Correct `schemaref` prefixes.  
  - Well‑formed `deadledgerref` objects.  
  - Required fields (`id`, `schemaref`, `tier`, implementation references).  

- Invariant and metric range checks  
  Enforce canonical ranges:  
  - DET in `[0, 10]`.  
  - Invariants in `[0, 1]` (unless otherwise documented).  
  - Entertainment metrics in `[0, 1]`.  

- ZKP and entitlement conformity  
  Validate proof envelopes against Dead‑Ledger schemas:  
  - Correct structure of `deadledgerref`.  
  - Valid verifier references and circuit types.  

- AI authoring lint  
  Ensure all AI‑generated files:  
  - Include `targetRepo`, `targetPath`, `schemaref`, `tier`.  
  - Declare which invariants/metrics they touch or bind.  
  - Do not embed raw content in contract‑only repositories.  

### 6.2 Required adoption

Normative practice: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/b86f5942-50b7-460e-a54b-1044e30630e5/create-a-task-for-microsoft-s-3ukEpSkxTiieobAHUuxocQ.md)

- All Tier‑1 and Tier‑2 repositories (core, vaults, orchestrators) MUST adopt the shared CI pack.  
- Tier‑3 labs SHOULD adopt the pack or an explicitly documented superset.  
- Repositories that do not import these workflows must be treated as experimental and clearly marked as such.  

***

## 7. Schema Spine Index and Repo Manifests

To enable machine navigation and impact analysis, the constellation relies on a spine index and per‑repo manifests. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/b86f5942-50b7-460e-a54b-1044e30630e5/create-a-task-for-microsoft-s-3ukEpSkxTiieobAHUuxocQ.md)

### 7.1 Schema spine index

A canonical `schema-spine-index` document: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)

- Maps each invariant, metric, and contract type to its canonical schema file.  
- Lists all known consumers:  
  - Repositories.  
  - Files (or directory patterns).  
  - Engine modules and APIs.  

Normative use: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

- AI‑chat agents query the spine index before generating or changing files.  
- CI uses the index to compute impact graphs when a schema changes.  

### 7.2 Per‑repo manifests

Each constellation‑aware repository SHOULD provide a machine‑readable manifest that declares: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/b86f5942-50b7-460e-a54b-1044e30630e5/create-a-task-for-microsoft-s-3ukEpSkxTiieobAHUuxocQ.md)

- Which schemas it consumes (by `schemaref`).  
- Which registries it reads/writes.  
- Which modules (Lua, Rust, etc.) implement those contracts.  
- Which tiers and environments it participates in.  

AI and tools use these manifests to: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/b86f5942-50b7-460e-a54b-1044e30630e5/create-a-task-for-microsoft-s-3ukEpSkxTiieobAHUuxocQ.md)

- Discover valid `targetRepo` and `targetPath` values.  
- Choose appropriate tiers for artifacts.  
- Route prism envelopes to the correct CI workflows.  

***

## 8. Engine and Lua Integration Norms

Engines must not access raw invariant or metric tables directly. They must go through a narrow, stable API. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/b86f5942-50b7-460e-a54b-1044e30630e5/create-a-task-for-microsoft-s-3ukEpSkxTiieobAHUuxocQ.md)

### 8.1 Canonical access patterns

Exposed via language‑appropriate bindings (often Lua), all engines should rely on functions such as: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/b86f5942-50b7-460e-a54b-1044e30630e5/create-a-task-for-microsoft-s-3ukEpSkxTiieobAHUuxocQ.md)

- `H.CIC(region_id, tile_id)`  
- `H.MDI(region_id, tile_id)`  
- `H.AOS(region_id, tile_id)`  
- `H.RRM(region_id, tile_id)`  
- `H.FCF(region_id, tile_id)`  
- `H.SPR(region_id, tile_id)`  
- `H.RWF(region_id, tile_id)`  
- `H.DET(region_id, tile_id)`  
- `H.HVF(region_id, tile_id)`  
- `H.LSG(region_id, tile_id)`  
- `H.SHCI(region_id, tile_id)`  

And for metrics: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/b86f5942-50b7-460e-a54b-1044e30630e5/create-a-task-for-microsoft-s-3ukEpSkxTiieobAHUuxocQ.md)

- `H.metrics(session_id, region_id)` returning UEC, EMD, STCI, CDL, ARR bands or snapshots.  

And for contracts: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

- `H.contract(id)` returning the resolved contract card (policy, region, or seed) for that ID.  

### 8.2 Behavioral requirement

All runtime systems (PCG, spectral AI, audio, VFX, UI) must: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/b86f5942-50b7-460e-a54b-1044e30630e5/create-a-task-for-microsoft-s-3ukEpSkxTiieobAHUuxocQ.md)

- Query invariants and metrics via this API before deciding behavior.  
- Avoid hard‑coding invariant values or metric ranges in engine logic.  
- Treat contract cards as the authoritative source of allowed behaviors and ranges.  

***

## 9. Content and Safety Constraints

HorrorPlace‑Constellation‑Contracts is a contract‑only repository. It must remain compatible with public hosting and platform policies. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)

### 9.1 Prohibited content in this repo

This repository must not contain: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)

- Explicit horror scenes or graphic descriptions.  
- Raw trauma payloads (logs, transcripts, unredacted archives).  
- Game engine code that directly implements horror content.  
- Large binary assets (images, audio, video) embedding horror content.  

### 9.2 Allowed content

This repository may contain: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

- Schemas and contracts (JSON, YAML, Markdown specs).  
- Registry formats and example NDJSON lines (opaque references only).  
- CI and pre‑commit workflows.  
- Engine‑agnostic integration examples (pseudo‑Lua, Rust, GDScript, etc.), as long as they reference contracts and invariants rather than explicit scenes.  

### 9.3 Downstream responsibility

Downstream repositories: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/b86f5942-50b7-460e-a54b-1044e30630e5/create-a-task-for-microsoft-s-3ukEpSkxTiieobAHUuxocQ.md)

- MUST respect the contracts and safety expectations defined here.  
- MAY contain concrete implementations and assets, subject to their own policies.  
- SHOULD keep public‑facing repos implication‑driven (references, metrics, invariants), with sensitive content gated through appropriate proof and entitlement layers.  

***

## 10. AI‑Agent Etiquette and Guarantees

AI systems that claim to be “constellation‑aware” are expected to follow these normative practices. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)

### 10.1 Before generating

An AI agent must: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

- Load or query:  
  - The schema spine.  
  - The schema spine index.  
  - Relevant NDJSON registries.  
  - Target repository manifests.  

- Determine:  
  - The correct `schemaref` and schema version.  
  - A valid `targetRepo`, `targetPath`, and `tier`.  
  - Valid `referencedIds` for all links.  

### 10.2 While generating

- Always emit a prism envelope. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)
- Always target between one and three primary artifacts per request (unless explicitly using a changeset schema that enforces the same cap). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)
- Always conform to contract card narrowing rules when touching invariants and metrics. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

### 10.3 After generating

- Expect CI to: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/b86f5942-50b7-460e-a54b-1044e30630e5/create-a-task-for-microsoft-s-3ukEpSkxTiieobAHUuxocQ.md)
  - Validate the artifact(s) against the canonical schema(s).  
  - Lint registry entries.  
  - Enforce invariant and metric ranges.  
  - Validate any `deadledgerref`.  

- Accept that CI decisions are authoritative: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)
  - If CI fails, the artifact set is considered invalid.  
  - Subsequent generations must correct or supersede the failing artifacts.  

***

## 11. Versioning and Evolution

Because this repository defines cross‑repo law, changes must be deliberate and documented. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)

### 11.1 Semantic versioning

- Core schemas use semantic versioning in their IDs or metadata. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)
- Breaking changes REQUIRE:  
  - A new major schema version.  
  - Migration notes.  
  - Updates to the schema spine index.  

### 11.2 Deprecation

- Deprecated schemas and fields must be: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)
  - Marked as such in documentation.  
  - Supported for a defined deprecation window.  
  - Gradually phased out via automated CI warnings and migration tools.  

### 11.3 Impact analysis

When a schema changes: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/b86f5942-50b7-460e-a54b-1044e30630e5/create-a-task-for-microsoft-s-3ukEpSkxTiieobAHUuxocQ.md)

- Use the schema spine index to compute all dependent repos and files.  
- Open automated issues or prepare PRs to update or re‑validate those dependents.  
- Treat prism contracts as bidirectional:  
  - They guide generation.  
  - They also propagate validation and refactor signals across the constellation.  

***
