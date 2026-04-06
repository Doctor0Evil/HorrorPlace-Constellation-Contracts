# CHAT_DIRECTOR Research Helper — Module‑by‑Module Outline

This document lists the research tasks required to gather the data and decisions needed to implement all CHAT_DIRECTOR files in HorrorPlace‑Constellation‑Contracts. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/51ebf54f-ab08-48b6-8cf2-ce929db3a87a/the-algorithmic-architecture-o-A7N0Cgi2QFqkAHFdhxjM2g.md)

***

## 1. Manifests & Tiers (`model/manifest_types.rs`, `validate/manifests.rs`)

**1.1 Tier taxonomy and charter text**  
- Finalize the full list of constellation tiers (T1, T2, T3, vault, lab, etc.) and their doctrinal roles (public contract surface, vault, lab, etc.). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- Write one canonical “charter rationale” sentence per tier explaining why certain content is allowed/forbidden; these will populate `authoringHints.tierRationale` in `repo-manifest.hpc.*.json`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- Decide which tiers require Dead‑Ledger proofs, which only reference them, and which never touch horror content. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

**1.2 Repo catalog and allowed objectKinds**  
- Enumerate all constellation repos (Horror.Place, Atrocity-Seeds, Spectral-Foundry, Codebase-of-Death, Black-Archivum, Orchestrator, etc.). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/51ebf54f-ab08-48b6-8cf2-ce929db3a87a/the-algorithmic-architecture-o-A7N0Cgi2QFqkAHFdhxjM2g.md)
- For each repo, list allowed `objectKind` values (moodContract, regionContractCard, eventContract, seedContractCard, personaContract, policyEnvelope, Lua policy, adapter, etc.). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/51ebf54f-ab08-48b6-8cf2-ce929db3a87a/the-algorithmic-architecture-o-A7N0Cgi2QFqkAHFdhxjM2g.md)
- Decide, per repo, where each objectKind is stored on disk (directories and filename patterns) and whether the repo is allowed to contain raw narrative or only contracts. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

**1.3 Repo policies and authoring hints**  
- For each repo, decide `rules.oneFilePerRequest`, `rules.requireDeadledgerRef`, and `rules.minRwfForTier`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/51ebf54f-ab08-48b6-8cf2-ce929db3a87a/the-algorithmic-architecture-o-A7N0Cgi2QFqkAHFdhxjM2g.md)
- Author short `authoringHints` values per repo:  
  - Why one‑file‑per‑request exists for this repo.  
  - Why Dead‑Ledger is required or not.  
  - How cross‑repo references are supposed to behave (who may reference whom, at which tiers). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- Identify “staging” or “vault” repos to recommend as `authoringHints.defaultStagingRepo` when RWF is too low for Tier 1. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

**1.4 Cross‑repo reference policy**  
- Define, per repo and tier, what cross‑refs are allowed (e.g., T1 may reference T2 vault IDs but not T3 lab IDs, or vice versa). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- Decide whether any repos must remain “reference‑only” toward others (e.g., public repos may not be referenced by experimental labs, or vice versa). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- Document ID patterns and which registry owns which ID families, enabling `lookup_repo_for_id` logic. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/51ebf54f-ab08-48b6-8cf2-ce929db3a87a/the-algorithmic-architecture-o-A7N0Cgi2QFqkAHFdhxjM2g.md)

_Data produced: finalized `repo-manifest.hpc.*.json` templates and the Rust mirrors in `model/manifest_types.rs`, plus manifest‑driven rules for `validate/manifests.rs`._

***

## 2. Schema Spine & Invariants (`spine.rs`, `validate/invariants.rs`, `generate/*`)

**2.1 Canonical invariant and metric definitions**  
- Confirm full list of invariants (CIC, MDI, AOS, RRM, FCF, SPR, RWF, DET, HVF, LSG, SHCI) and their numeric ranges, including DET’s extended range if used. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/51ebf54f-ab08-48b6-8cf2-ce929db3a87a/the-algorithmic-architecture-o-A7N0Cgi2QFqkAHFdhxjM2g.md)
- Confirm full list of entertainment metrics (UEC, EMD, STCI, CDL, ARR) and their canonical ranges and typical “target bands.” [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/51ebf54f-ab08-48b6-8cf2-ce929db3a87a/the-algorithmic-architecture-o-A7N0Cgi2QFqkAHFdhxjM2g.md)
- For each invariant/metric, document: required vs. optional, typical default band, and whether ranges differ by tier or phase. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/51ebf54f-ab08-48b6-8cf2-ce929db3a87a/the-algorithmic-architecture-o-A7N0Cgi2QFqkAHFdhxjM2g.md)

**2.2 Cross‑metric and cross‑invariant rules**  
- List all known cross‑metric interactions (e.g., “if DET > 8.0, CDL floor becomes ≥ 0.4”; “high CIC widens SHCI bands by 15%”). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- Define which interactions are hard errors vs. soft warnings (e.g., ARR too low vs. DET too high for a tier). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- Decide how tileClass (spawn, battlefront, liminal, etc.) modifies invariant bands and metric targets. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/51ebf54f-ab08-48b6-8cf2-ce929db3a87a/the-algorithmic-architecture-o-A7N0Cgi2QFqkAHFdhxjM2g.md)

**2.3 Skeleton bands by archetype and tileClass**  
- For region archetypes (marsh, war‑front, backrooms, sanctuary, etc.), define canonical CIC/AOS/LSG/HVF profiles by tileClass and tier. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- For mood archetypes (combat, liminal, sanctuary, exploration), define typical invariant bands and metric targets per tileClass. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/51ebf54f-ab08-48b6-8cf2-ce929db3a87a/the-algorithmic-architecture-o-A7N0Cgi2QFqkAHFdhxjM2g.md)
- For seed and event archetypes, define DET, SHCI, and UEC/CDL/ARR trajectories over space and time for a small set of named patterns (“slow burn,” “sudden rupture,” etc.). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

_Data produced: fully populated `invariants-spine.v1.json` and `entertainment-metrics-spine.v1.json`, plus archetype tables used by `generate/region.rs`, `seed.rs`, `mood.rs`, `event.rs` and enforcement logic in `validate/invariants.rs`._

***

## 3. Authoring Tooling Schemas (`model/request_types.rs`, `model/response_types.rs`, `validate/schema.rs`, `validate/envelopes.rs`)

**3.1 AiAuthoringRequest fields and defaults**  
- Finalize `ai-authoring-request-v1.json` with fields: `intent`, `objectKind`, `targetRepo`, `phase`, `tier`, `schemaref`, `referencedIds`, SHCI bands, optional `candidateKinds`, and `intendedMetrics/intendedInvariants`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/51ebf54f-ab08-48b6-8cf2-ce929db3a87a/the-algorithmic-architecture-o-A7N0Cgi2QFqkAHFdhxjM2g.md)
- Decide when `candidateKinds` should be used and how many candidates are acceptable before requiring user/AI disambiguation. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- Define `RequestDefaults` behavior: which fields are inferred from the spine, manifests, and tier policy, and how to explain each default (`RequestDefaults::explain`). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

**3.2 AiAuthoringResponse and ValidatedFile**  
- Finalize `ai-authoring-response-v1.json` with: exactly one primary artifact, envelope metadata, `generatedBy`, optional registry diffs, and `targetRepo/targetPath` hints. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/51ebf54f-ab08-48b6-8cf2-ce929db3a87a/the-algorithmic-architecture-o-A7N0Cgi2QFqkAHFdhxjM2g.md)
- Decide the structure of `ValidatedFile` including `softDiagnostics` for warnings and how those map to invariant, manifest, and envelope layers. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- Define minimal provenance fields AI must always populate: `generatedBy`, `schemaRef`, `timestamp`, and `requestId`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

**3.3 Prism envelopes and Dead‑Ledger/ZKP placeholders**  
- Finalize `prism-envelope-v1.json`: envelope fields, `envelopeVersion`, `prismMeta`, cryptographic placeholders, and optional ZKP fields. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/51ebf54f-ab08-48b6-8cf2-ce929db3a87a/the-algorithmic-architecture-o-A7N0Cgi2QFqkAHFdhxjM2g.md)
- Decide which cryptographic/ZKP fields are required now, which are optional placeholders, and which will be filled by CI or Dead‑Ledger services. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- Establish envelope versioning policy (how v1 vs v2 are recognized, accepted, or rejected) and how `config.rs` will negotiate active envelope versions. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

**3.4 JSON Schema validation behavior**  
- Decide whether to support a `quick_check` mode that validates only required fields and top‑level structure vs. full strict validation. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- Confirm custom JSON Schema formats (`hpc-id`, `schemaref-uri`, etc.) and how they are validated and surfaced as diagnostics. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/51ebf54f-ab08-48b6-8cf2-ce929db3a87a/the-algorithmic-architecture-o-A7N0Cgi2QFqkAHFdhxjM2g.md)
- Standardize error translation: every schema error must have a JSON Pointer and an “advice” string describing the fix. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/51ebf54f-ab08-48b6-8cf2-ce929db3a87a/the-algorithmic-architecture-o-A7N0Cgi2QFqkAHFdhxjM2g.md)

_Data produced: final `ai-authoring-request-v1.json`, `ai-authoring-response-v1.json`, `prism-envelope-v1.json`, plus matching Rust types and schema wrappers._

***

## 4. Phases & Roles (`phases.rs`, `validate/mod.rs`)

**4.1 Phase model and allowed operations**  
- Confirm the five phases: Schema0, Registry1, Bundles2, LuaPolicy3, Adapters4, and their responsibilities. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/51ebf54f-ab08-48b6-8cf2-ce929db3a87a/the-algorithmic-architecture-o-A7N0Cgi2QFqkAHFdhxjM2g.md)
- For each `(phase, objectKind, repo)` triple, decide allowed vs. forbidden status; encode this as data (from spine/manifest) rather than hardcoded in code where possible. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/51ebf54f-ab08-48b6-8cf2-ce929db3a87a/the-algorithmic-architecture-o-A7N0Cgi2QFqkAHFdhxjM2g.md)
- Define example artifacts per phase for docs and `Phase::describe` output (used by CLI `describe`). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

**4.2 Phase‑specific invariant strictness**  
- Decide whether lower phases (0–1) have looser invariant/metric ranges (experimental) than higher phases (2–4). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- For any relaxed ranges, document exact overrides and how they are computed from the spine. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- Define how SHCI and RWF constraints tighten as artifacts move through phases. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

**4.3 Promotion criteria and rollback**  
- Define machine‑verifiable promotion predicates per phase (e.g., all `referencedIds` resolved; invariant coverage ≥ 80%; no experimental flags). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- Decide when demotion is allowed (e.g., failed Lua policy checks causing fallback from Phase 3 to Phase 2) and what happens to dependent artifacts. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/51ebf54f-ab08-48b6-8cf2-ce929db3a87a/the-algorithmic-architecture-o-A7N0Cgi2QFqkAHFdhxjM2g.md)
- Map promotion/rollback rules into `Phase::promotion_checklist(objectKind)` and `PhaseForbidden.phaseUpgradeSuggestion` semantics. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

**4.4 Validation orchestration order**  
- Confirm the desired order of validation layers: schema → invariants/metrics → manifest/tier → envelope/provenance. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/51ebf54f-ab08-48b6-8cf2-ce929db3a87a/the-algorithmic-architecture-o-A7N0Cgi2QFqkAHFdhxjM2g.md)
- Decide whether failures should short‑circuit later layers or whether all layers should run to give a comprehensive diagnostic set. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- Define ranking heuristic for `ValidationResult::ranked_diagnostics` (severity, layer, and fix ordering) and standard “advice” strings. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

_Data produced: explicit phase tables, promotion criteria, and validation ordering rules feeding into `phases.rs` and `validate/mod.rs`._

***

## 5. Skeleton Generation (`generate/mod.rs`, `generate/region.rs`, `seed.rs`, `mood.rs`, `event.rs`)

**5.1 Skeleton field classification**  
- For each contract schema (regionContractCard, seedContractCard, moodContract, eventContract), classify fields as: structural (DO_NOT_MODIFY), design parameters (AI_FILL), and optional/advanced (NEW_IN_VERSION). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/51ebf54f-ab08-48b6-8cf2-ce929db3a87a/the-algorithmic-architecture-o-A7N0Cgi2QFqkAHFdhxjM2g.md)
- Decide which IDs and references should be auto‑generated vs. filled by AI (e.g., `id` from a pattern vs. `displayName` from AI). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- Establish how much pre‑filling is acceptable before creativity is constrained (target number of AI_FILL slots per contract type). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

**5.2 Archetype libraries per family**  
- For regions, define a small set of archetypes (e.g., `marsh`, `war_front`, `backrooms`) with invariant/metric profiles and tileClass mapping. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- For moods, define archetypes like `combat`, `liminal`, `sanctuary` with per‑tileClass bands. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- For seeds and events, define named pacing templates and stage modulation grids. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

**5.3 Cross‑family bundles**  
- Decide on a minimal “bundle” spec tying region, seed, mood, and event contracts that share a lore context (e.g., a shared `bundleId` and target region). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- Define what “coherent” means across bundle members (matching CIC bands, compatible SHCI bands, consistent DET/ARR trade‑offs). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- Establish how `generate_coherent_bundle` shares invariant envelopes and metric targets across skeletons. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

**5.4 Skeleton telemetry and improvement**  
- Decide what `GenerationMetrics` to track: objectKind, skeletonUsed, validation success/failure, failure codes. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- Define how these metrics are aggregated and exposed (CLI `stats` or log file) so that skeleton patterns can be iteratively refined. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

_Data produced: archetype catalogs, field classification tables per schema, and a telemetry spec for generation success rates._

***

## 6. CLI & Binary Interface (`cli/*`, `src/bin/hpc-chat-director.rs`)

**6.1 CLI behavior and discoverability**  
- Specify subcommands, options, and expected JSON outputs for: `init`, `plan`, `validate-response`, `apply`, `describe`, and optionally `batch`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/51ebf54f-ab08-48b6-8cf2-ce929db3a87a/the-algorithmic-architecture-o-A7N0Cgi2QFqkAHFdhxjM2g.md)
- Define the JSON schema for CLI errors (`CliErrorSchema`) including `version`, `code`, `layer`, `message`, `jsonPointer`, and `remediation`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- Decide what `describe` should return for combinations of `--object-kind`, `--phase`, and `--tier` (ObjectKindProfile contents). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

**6.2 Plan enrichment and generation guides**  
- Decide which spine/manifest data is included in a `plan`’s `"generationGuide"`: allowed ranges, archetypes, common validation pitfalls, example IDs. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/51ebf54f-ab08-48b6-8cf2-ce929db3a87a/the-algorithmic-architecture-o-A7N0Cgi2QFqkAHFdhxjM2g.md)
- Define size limits and summarization rules to keep guides prompt‑friendly for AI models. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- Determine whether skeletons are attached to plan output directly or only referenced via hints. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

**6.3 Apply behavior and hooks**  
- Specify the exact JSON format of `apply --dry-run` results (fields, action types, hash formats). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- Decide allowed `postApplyHooks` types (schema validation, registry lints, Lua/QC scripts) and how they are declared in manifests. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- Clarify behavior when hooks fail (fail apply vs. warn only; how to surface this to AI callers). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

**6.4 Exit codes and version handshake**  
- Finalize exit code mapping: `0` success, `1` validation failure, `2` configuration error, `3` phase violation, `4` manifest routing error, etc. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- Decide which capabilities `--version --json` should report: binary version, spine version, envelope versions, known objectKinds, phase support. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/51ebf54f-ab08-48b6-8cf2-ce929db3a87a/the-algorithmic-architecture-o-A7N0Cgi2QFqkAHFdhxjM2g.md)
- Define how AI orchestrators are expected to use this handshake to choose compatible modes or decline operations. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

_Data produced: CLI spec for docs, structured error schema, and consistent behavior contracts for AI and CI integrations._

***

## 7. Tests & Telemetry (`tests/*`, cross‑module)

**7.1 Rule coverage and test mapping**  
- List all major rules/invariants you care about (one‑file‑per‑request, tier rules, DET caps by tier, ARR floors by style, SHCI coupling rules, etc.). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- Design tests for each rule across the full path: “invalid artifact” → `validate_response` error → CLI failure. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- Build `tests/test_rule_map.json` mapping test names to rule IDs and descriptions to keep coverage transparent. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

**7.2 AI failure‑mode regression corpus**  
- Collect real AI errors from current workflows: schema failures, manifest routing mistakes, invariant violations, missing proofs, etc. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- For each common failure, design a minimal failing input and a test that asserts the current, correct diagnostic response. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- Establish a process for adding new regressions when AI behavior evolves or new patterns appear. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

**7.3 Golden files for skeletons and CLI flows**  
- Capture golden skeleton outputs for each archetype per contract family (region, seed, mood, event). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- Define `.expected.json` formats and `--update-golden` procedures for intentional changes. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- Add CLI smoke tests (init/plan/validate/apply) with golden JSON outputs to detect behavioral drift. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

**7.4 Telemetry schemas for authoring and runtime**  
- Define telemetry schemas for authoring validation logs (GenerationMetrics) and runtime session metrics (UEC/EMD/STCI/CDL/ARR vs. contract IDs). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- Decide how telemetry informs future changes to invariant/metric bands in the spine and skeleton archetypes. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)
- Set up simple aggregation procedures or scripts to periodically feed telemetry summaries back into design discussions. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/2c681b6f-1845-4a79-9464-ddf8cfa3208d/this-research-focuses-on-desig-DemATE1ZRtOBxLRQlhB93g.md)

_Data produced: test plan and telemetry specs that close the loop between rules, validator behavior, AI behavior, and runtime outcomes._
