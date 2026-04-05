## 1. Cleaned “condensed rulebook” text

Use this as the authoritative doc section; you can drop it into a repo doc or README without modification. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)

***

### 1. Global doctrine: Horror.Place is the spine

Horror.Place is the **Schema Authority**. It defines the canonical invariants, entertainment metrics, and core contracts (event, region, persona, style), and these are treated as the system’s physics layer. Every other repository only consumes or mirrors these schemas; no repository is allowed to silently fork or drift them. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

All repositories must use the canonical invariants schema (CIC, MDI, AOS, RRM, FCF, SPR, RWF, DET, HVF, LSG, SHCI, preconditions) and the entertainment metrics schema (UEC, EMD, STCI, CDL, ARR) exactly as defined, including ranges and additionalProperties: false. AI‑chat generated code or data must respect these field names and ranges, with special attention to DET being a numeric 0–10 field. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/b86f5942-50b7-460e-a54b-1044e30630e5/create-a-task-for-microsoft-s-3ukEpSkxTiieobAHUuxocQ.md)

HorrorPlace-Constellation-Contracts sits above the entire constellation as a **contract‑only governance layer**. It defines cross‑repo JSON Schemas, NDJSON registry formats, AI‑authoring envelopes, and reusable CI/workflow templates but ships no raw horror content and no runtime engine code. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)

***

### 2. Repo roles and where AI may place files

Each repository has a narrow, enforced role; AI placement must align with that role. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/b86f5942-50b7-460e-a54b-1044e30630e5/create-a-task-for-microsoft-s-3ukEpSkxTiieobAHUuxocQ.md)

| Repo                              | Tier | Visibility | Role for AI‑generated artifacts                                                   | Allowed AI content                                                    |
|-----------------------------------|------|------------|-----------------------------------------------------------------------------------|------------------------------------------------------------------------|
| Horror.Place                      | T1   | Public     | Schema authority, public registries, doctrinal docs                               | JSON Schemas, NDJSON registry entries, docs, public API stubs; no raw horror content |
| Horror.Place-Orchestrator         | T1   | Public     | Service that ingests signed descriptors and updates registries atomically         | Python/PowerShell code, configs, tests, docs; no seeds or raw horror  |
| HorrorPlace-Constellation-Contracts | T1 | Public     | Cross‑repo contract and CI layer                                                  | Schemas, registry formats, AI‑authoring contracts, reusable workflows; no horror content |
| HorrorPlace-Black-Archivum        | T2   | Private    | Historical invariant bundles                                                      | Structured invariant bundles, contracts; no seeds or encounter logic  |
| HorrorPlace-Atrocity-Seeds        | T2   | Private    | PCG seed vault: event/region/style seeds bound to invariants                      | JSON seeds, seed schemas, registries; no raw lore or explicit horror  |
| HorrorPlace-Codebase-of-Death     | T2   | Private    | Engine code vault                                                                 | Rust/Lua engine implementations, APIs bound to invariants/metrics     |
| HorrorPlace-Spectral-Foundry      | T2   | Private    | Spectral entity and style forge                                                   | Spectral/style seed schemas, style contracts, metrics bindings        |
| HorrorPlace-Obscura-Nexus         | T2   | Private    | Routing and DSL nexus                                                             | ALN configs, routing graphs, experimental DSL schemas                 |
| HorrorPlace-Liminal-Continuum     | T2   | Private    | Liminal space generation                                                          | Invariant‑driven environment contracts and generators                 |
| HorrorPlace-Process-Gods-Research | T3   | Private    | Experimental metrics and process theology                                         | Experimental schemas, analysis tools                                  |
| HorrorPlace-Redacted-Chronicles   | T3   | Private    | Narrative/chronicle vault                                                         | Redacted lore contracts only, never explicit narrative content        |
| HorrorPlace-Neural-Resonance-Lab  | T3   | Private    | BCI/neural metrics                                                                | BCI/telemetry schemas and analysis                                    |
| HorrorPlace-Dead-Ledger-Network   | T3   | Private    | ZKP and entitlement authority                                                     | ZKP proof schemas, verifier registries, protocol/docs                 |

An AI‑chat agent must never create horror seeds, gore specifications, or explicit horror content in Horror.Place, HorrorPlace-Constellation-Contracts, Horror.Place-Orchestrator, or HorrorPlace-Dead-Ledger-Network. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)

***

### 3. Canonical file‑placement rules (all repos)

Across the constellation, AI‑generated artifacts follow a single placement pattern defined by HorrorPlace‑Constellation‑Contracts and the schema spine workplan. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/b86f5942-50b7-460e-a54b-1044e30630e5/create-a-task-for-microsoft-s-3ukEpSkxTiieobAHUuxocQ.md)

One‑file‑per‑request  
Every AI request that “creates something” must emit exactly one primary file (JSON, NDJSON, Markdown, or code) plus at most the necessary registry line(s). There are no scatter‑shot multi‑file dumps. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

Explicit target repo and path  
Each generated artifact must declare and respect: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)

- targetRepo – one of the canonical constellation repositories.
- targetPath – an allowed directory path for that repo (e.g., schemas/…, registry/…, events/…, regions/…, docs/…).

AI must not invent arbitrary directory trees; it must select from the directory plans documented per repository. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/b86f5942-50b7-460e-a54b-1044e30630e5/create-a-task-for-microsoft-s-3ukEpSkxTiieobAHUuxocQ.md)

Schemas drive structure  
Every JSON or NDJSON artifact must validate against a canonical schema: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/b86f5942-50b7-460e-a54b-1044e30630e5/create-a-task-for-microsoft-s-3ukEpSkxTiieobAHUuxocQ.md)

- Invariants and metrics: schemas/invariants_v1.json, schemas/entertainment_metrics_v1.json from Horror.Place.
- Contracts: eventcontract_v1.json, regioncontract_v1.json, persona_contract_v1.json, stylecontract_v1.json, etc., as defined in Horror.Place and mirrored where appropriate.
- AI authoring envelopes: ai-authoring-request-v1 and ai-authoring-response-v1 from HorrorPlace‑Constellation‑Contracts.

AI must not add undeclared fields, change types, or violate prescribed ranges. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)

Registries as the only discovery surface  
Entities are discovered exclusively through registries: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/b86f5942-50b7-460e-a54b-1044e30630e5/create-a-task-for-microsoft-s-3ukEpSkxTiieobAHUuxocQ.md)

- Horror.Place: registry/events.json, registry/regions.json, registry/styles.json, registry/personas.json.
- Atrocity-Seeds: registry/events.json, registry/regions.json, and any style registries.

AI may only append registry lines that conform to registry schemas (JSON or NDJSON) with required fields such as id, schemaref, path (git@ URI), invariant_bundle, hash, tier, and deadledgerref. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)

Dead‑Ledger references at the spine  
Any registry entry that refers to restricted content (events, regions, personas, styles) must include a deadledgerref object that matches Dead‑Ledger’s proof envelope and verifier conventions. AI must not omit deadledgerref for new restricted entries. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/b86f5942-50b7-460e-a54b-1044e30630e5/create-a-task-for-microsoft-s-3ukEpSkxTiieobAHUuxocQ.md)

CI as gatekeeper  
All repositories import CI workflows from HorrorPlace‑Constellation‑Contracts to enforce: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)

- Schema and registry validation.
- Invariant and metric range checks (including DET 0–10).
- ZKP/deadledgerref conformity where required.
- Content‑leak scans (no raw horror content in contract‑only repos).

AI code‑generation must assume CI will fail and block merges on any violation; artifacts must be emitted in a CI‑clean state. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)

***

### 4. Atrocity‑Seeds rules (structure + seeds)

HorrorPlace‑Atrocity‑Seeds is a Tier‑2 PCG seed vault. It turns invariant bundles from Black‑Archivum into event/region/style seeds and must be treated as an implication‑only vault. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c39d09a-81b2-4d31-9b84-4ce1577f86b7/HorrorPlace-Atrocity-Seeds-Vault-Repository-Files.docx)

Allowed directories for AI output in Atrocity‑Seeds: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/953caf50-2b43-4572-9629-f7ccaaa6cf32/this-research-focuses-on-a-hyb-_gqufYEsTuG79lpVfO045g.md)

- schemas/  
  - eventcontract_v1.json (mirror of Horror.Place event schema; may add local annotations but no structural drift).  
  - regioncontract_v1.json (same rule).  
  - Style or seed schemas (e.g., gore_assembly_style_v1.json), still implication‑only and bound to upstream style contracts.
- events/  
  - Individual event seed JSON files that conform to eventcontract_v1.json, containing only IDs, invariant bindings, metric intent hints, and references/hashes.
- regions/  
  - Region seed JSON files that conform to regioncontract_v1.json, bound to invariants and content‑free.
- registry/  
  - events.json and regions.json as the discovery surfaces for seeds, referencing events/*.json and regions/*.json via git@ URIs and hashes.
- scripts/  
  - Validation, hash verification, and PCG helper scripts (e.g., validate_event.py, verify_atrocity_seeds_hashes.py, pcg_generate_region.py).
- .github/workflows/ or .circleci/  
  - CI configs enforcing schema validation, hash verification, content‑leak scanning, and descriptor signing.
- docs/, constellation/  
  - Architectural and workflow documentation, implication‑only.

AI must not create new top‑level directories in Atrocity‑Seeds and must keep seeds, schemas, registries, scripts, and docs within these lanes. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/609689e0-7a5b-4fa6-8941-a8dac9b92a38/verifying-the-history-to-encou-SBAy_UkvSxKN_dcR1HCoZg.md)

Seed content rules: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c39d09a-81b2-4d31-9b84-4ce1577f86b7/HorrorPlace-Atrocity-Seeds-Vault-Repository-Files.docx)

- No raw lore or history: Seeds reference upstream data only via IDs, hashes, or URIs; they contain no descriptive text, no narrative prose, and no graphic content.
- Required fields for every seed (event, region, style):
  - id – stable seed ID.
  - bundleref.id – ID of the invariant bundle from Black‑Archivum or equivalent upstream anchor.
  - invariants – local bindings for CIC, AOS, RRM, SHCI, etc., within canonical ranges.
  - metrics_intent – target bands for UEC, EMD, STCI, CDL, ARR.
  - safetytier – maturity classification.
  - intensityband – numeric 0–10 intensity; high intensity is ≥ 8.
  - schemaref – canonical schema ID (schema://Horror.Place/...).
  - deadledgerref – mandatory for high‑intensity seeds (intensityband ≥ 8).
- Engines and Orchestrator discover seeds only via Atrocity‑Seeds and Horror.Place registries; Atrocity‑Seeds is never queried directly at runtime.

CI in Atrocity‑Seeds must: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/953caf50-2b43-4572-9629-f7ccaaa6cf32/this-research-focuses-on-a-hyb-_gqufYEsTuG79lpVfO045g.md)

- Validate each seed against its schema.
- Compute and verify canonical SHA‑256 hashes against registry hash fields.
- Run content‑leak scans to ensure no raw content or URLs appear in seeds.
- Sign artifact descriptors and dispatch them to the Orchestrator.

AI‑generated scripts and workflows must integrate with this pipeline rather than bypass it. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c39d09a-81b2-4d31-9b84-4ce1577f86b7/HorrorPlace-Atrocity-Seeds-Vault-Repository-Files.docx)

***

### 5. AI‑chat “prism contract” behavior

HorrorPlace‑Constellation‑Contracts defines AI‑chat behavior as a prism‑contract compilation step. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

For every generated file, an AI agent must conceptually populate: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

- targetRepo – which repository owns this artifact, consistent with repo roles.
- targetPath – exact path inside that repository.
- schemaref – canonical schema ID (for JSON/NDJSON artifacts).
- tier – T1/T2/T3 classification, inferred from repo role.
- referencedIds – list of IDs (regions, events, invariant bundles, styles, proof envelopes) that this artifact links to.
- invariants_used and metrics_used – which invariant and metric fields this artifact reads or sets, aligned with the canonical spine.

The agent must then: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

- Generate a single file that validates against schemaref.
- Optionally generate the corresponding registry entry (one NDJSON line or JSON object) if the artifact needs to become discoverable.
- Assume the shared CI pack will run schema validation, registry linting, invariant/metric range checks, deadledgerref checks, and leak scans.

For Atrocity‑Seeds, AI must additionally ensure that any new seed: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/609689e0-7a5b-4fa6-8941-a8dac9b92a38/verifying-the-history-to-encou-SBAy_UkvSxKN_dcR1HCoZg.md)

- Connects to an existing or newly defined invariant bundle in Black‑Archivum (by ID).
- Is referenced from Horror.Place registries via git@ URIs and hashes (never direct file URLs).
- Does not modify or invent schema fields.

When asked to “create X”, an AI‑chat agent follows this decision tree: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

- X is a schema, registry format, AI‑authoring contract, or reusable CI/workflow → HorrorPlace‑Constellation‑Contracts (schemas/, registry‑schemas/, workflows/, tooling/).
- X is a domain contract or public registry entry (event/region/persona/style) → Horror.Place (schemas/ or registry/).
- X is a historical invariant bundle → HorrorPlace‑Black‑Archivum (contracts/invariant_bundles/).
- X is a PCG seed (event, region, style) bound to invariants → HorrorPlace‑Atrocity‑Seeds (events/, regions/, schemas/, registry/).
- X is runtime engine logic (Lua/Rust) or AI behavior tree → HorrorPlace‑Codebase-of-Death (engine source and Lua bindings).
- X is a spectral entity or style contract → HorrorPlace‑Spectral‑Foundry (style/seeding schemas, spectral contracts).
- X is a routing rule or inter‑repo config → HorrorPlace‑Obscura-Nexus (ALN configs, routing graphs).
- X is a ZKP proof schema, verifier registry, or protocol doc → HorrorPlace‑Dead-Ledger-Network (schemas/, registry/, docs/).

For Atrocity‑Seeds documentation specifically (README, constellation notes, workflow docs), AI must: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/953caf50-2b43-4572-9629-f7ccaaa6cf32/this-research-focuses-on-a-hyb-_gqufYEsTuG79lpVfO045g.md)

- Keep docs purely architectural and contractual.
- Explicitly reiterate that:
  - Seeds are implication‑only and schema‑bound.
  - Engines never query the vault at runtime in production.
  - CI enforces schema + hash + no‑raw‑content guarantees.
  - High‑intensity seeds require deadledgerref for attested review.

***

## 2. Where the chart belongs (repo, filename, path)

The repo‑roles chart you included is a governance artifact that describes the constellation as a whole and encodes AI‑placement rules. By doctrine, that belongs in the contract/gov layer, not in a seed or engine repo. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

**Target repository**  
- HorrorPlace-Constellation-Contracts (public, contract‑only governance layer). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

**Recommended file + path**

- `targetRepo`: `HorrorPlace-Constellation-Contracts`  
- `targetPath`: `docs/constellation/repo-roles-and-ai-placement.md`

Rationale: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/b86f5942-50b7-460e-a54b-1044e30630e5/create-a-task-for-microsoft-s-3ukEpSkxTiieobAHUuxocQ.md)

- It is cross‑repo governance, not horror content or runtime logic.
- It is part of the “AI‑authoring rules / placement rules” surface that other tools and agents should read before generating files.
- Under docs/constellation/, it naturally sits beside any future diagrams or NDJSON registry examples that describe the constellation topology.

Inside `repo-roles-and-ai-placement.md`, you can:

- Use the cleaned text and table from sections 2–3 above.
- Optionally link to:
  - The schema spine index (once present, e.g., schemas/schema-spine-index-v1.json).
  - The NDJSON registry formats in `schemas/registry/` or `docs/registry/`.

If you want a shorter, inline version of the chart for the repo’s root README, you can also add a “Repo Roles at a Glance” subsection to `README.md` in HorrorPlace‑Constellation‑Contracts and reference this deeper doc, but the canonical, detailed chart should live at `docs/constellation/repo-roles-and-ai-placement.md` as above. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/5e6af2d6-c19b-4f35-be2f-94b2c0818ac4/from-chaos-to-code-engineering-bAyCRooEQbm7o.KNSJl3VA.md)
