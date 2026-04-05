---
invariants_used: [CIC, MDI, AOS, RRM, FCF, SPR, RWF, DET, HVF, LSG, SHCI]
metrics_used: [UEC, EMD, STCI, CDL, ARR]
tiers: [T1_public, T2_private, T3_private]
deadledger_surface: [zkpproofv1_schema, verifiers_registry, bundle_attestation, agent_attestation]
---

# AI Authoring Guidelines v1

This document defines how AI/chat tools must behave when generating files for the Horror$Place VM-constellation. It explains the use of `ai_file_envelope` and `ai_research_plan` as standard contracts for planning, generating, and validating artifacts across all repositories.

The goal is to treat AI-chat as a deterministic compiler: every authoring action has a clear target repository and path, adheres to canonical schemas, and never bypasses the constellation’s safety and governance rules.

---

## 1. Scope and goals

These guidelines apply to any AI/chat tool that:

- Proposes new files or edits in any Horror$Place repository.  
- Generates seeds, schemas, registries, docs, or code for the VM-constellation.  
- Assists publishers, developers, or researchers in modifying constellation repositories.

The primary goals are:

- **Deterministic placement**: every file declares exactly which repo and path it belongs to.  
- **Schema-first structure**: all JSON/NDJSON content validates against canonical schemas.  
- **Research-first safety**: when information is missing, AI emits a research plan instead of an unsafe file.  
- **One-file-per-request discipline**: each authoring interaction focuses on a single primary artifact plus necessary registry edits.

---

## 2. Repositories and roles (AI view)

AI tools must be aware of the full VM-constellation and choose a target repository based on artifact type:

- **Tier 1 (public)**  
  - `Horror.Place`: schema authority, public registries, doctrinal docs.  
  - `Horror.Place-Orchestrator`: orchestrator service code, registry synchronization.  
  - `HorrorPlace-Constellation-Contracts`: cross-repo schemas, registry formats, AI authoring contracts, reusable CI.

- **Tier 2 (private)**  
  - `HorrorPlace-Codebase-of-Death`: engine implementations.  
  - `HorrorPlace-Black-Archivum`: historical invariant bundles.  
  - `HorrorPlace-Spectral-Foundry`: spectral entities and style contracts.  
  - `HorrorPlace-Atrocity-Seeds`: PCG seeds (events/regions/styles), implication-only.  
  - `HorrorPlace-Obscura-Nexus`: routing/ALN configs and experimental DSLs.  
  - `HorrorPlace-Liminal-Continuum`: liminal/environmental contracts.

- **Tier 3 (private)**  
  - `HorrorPlace-Process-Gods-Research`: experimental metrics and research.  
  - `HorrorPlace-Redacted-Chronicles`: redacted chronicles and response data.  
  - `HorrorPlace-Neural-Resonance-Lab`: BCI/neural schemas and analysis.  
  - `HorrorPlace-Dead-Ledger-Network`: ZKP envelopes, verifiers, entitlement protocols.

AI must never place horror seeds or explicit content in public-contract or core governance repositories (HorrorPlace-Constellation-Contracts, Horror.Place-Orchestrator, HorrorPlace-Dead-Ledger-Network).

---

## 3. ai_file_envelope: generation contract

The `ai_file_envelope` is a standard wrapper that describes an AI-generated file: which repo it belongs to, where it should live, which schema it targets, and what its content is.

### 3.1. Schema reference

The envelope is governed by:

- `schemas/ai_file_envelope_v1.json` in HorrorPlace-Constellation-Contracts.

AI-generated envelopes must conform to that schema to be considered valid.

### 3.2. Required fields

Every envelope must include:

- `target_repo`  
  The logical repository name (one of the 12 constellation repos).

- `target_path`  
  The repository-relative file path (for example, `events/aral_dissolution_v1.json`, `registry/events.json`, `docs/ai_authoring_guidelines_v1.md`).

- `schemaref`  
  A canonical schema URI for JSON/NDJSON content (for example, `schema://Horror.Place/eventcontract_v1.json`), or `null` for free-form text/code files.

- `tier`  
  One of `T1_public`, `T2_private`, `T3_private`, consistent with the chosen repository.

- `description`  
  A concise summary (10–512 characters) of what the file does and why it exists.

- `content_language`  
  Describes the body format: `json`, `ndjson`, `markdown`, `python`, `lua`, `rust`, `yaml`, or `text`.

- `body`  
  The actual file content. For JSON/NDJSON, this is a structured object. For text/code/Markdown, it is an object containing a `text` field.

Optional `cross_repo_links` entries may be added to list referenced bundles, descriptors, schemas, or configs in other repos.

### 3.3. Usage pattern

AI tools should:

1. Resolve the user’s request to an artifact type (e.g., “Atrocity-Seeds event seed”, “Horror.Place registry entry”, “Orchestrator workflow”).  
2. Choose `target_repo` and `target_path` according to the constellation’s directory conventions.  
3. Select an appropriate `schemaref` if the content is JSON/NDJSON.  
4. Construct `body` so it is ready to be committed as-is at `target_path`.  
5. Run local validation (where possible) against the referenced schema before presenting the envelope.

Downstream CI in each repo can then:

- Extract `body` and write it to `target_path`.  
- Validate the file against the schema indicated by `schemaref`.  
- Enforce one-file-per-envelope discipline and cross-repo consistency.

---

## 4. ai_research_plan: research-first contract

When information is insufficient or when the requested operation is high-risk, AI must avoid guessing and instead emit an `ai_research_plan`.

### 4.1. Schema reference

The research plan envelope is governed by:

- `schemas/ai_research_plan_v1.json` in HorrorPlace-Constellation-Contracts.

Any research plan must validate against this schema.

### 4.2. Required fields

Each research plan must include:

- `plan_id`  
  A stable ID, such as `research.atrocity_seeds.new_event_seed.v1`.

- `target_repo`  
  Where the eventual file will live if the plan is executed.

- `intended_artifact_type`  
  High-level target (e.g., `seed_event`, `seed_region`, `registry_entry`, `doc`, `orchestrator_workflow`).

- `status`  
  Typically `"research_required"` when first issued.

- `reason`  
  A clear explanation of why research is needed (for example, missing invariant bundle ID, missing intensity constraints, or ambiguous tier).

- `required_actions`  
  A list of steps, each with an `id`, `description`, optional `tools_or_sources`, and `expected_output`. These steps define exactly what the AI (or human) should do or look up before safe generation.

- `next_action_for_user`  
  A natural-language prompt guiding the user to confirm details or provide missing inputs.

### 4.3. Usage pattern

When the AI cannot safely generate a file:

1. Emit an `ai_research_plan` explaining what is missing and what must be done.  
2. Ask the user to confirm parameters (e.g., theme, maximum intensity band, desired tier).  
3. Once the user provides the necessary information or approves the plan, proceed in a subsequent step to generate the actual `ai_file_envelope`.

This approach replaces refusals with structured redirection, while still respecting safety and governance.

---

## 5. Authoring rules across the constellation

### 5.1. One-file-per-request discipline

For every authoring interaction, AI should:

- Focus on a single primary file per envelope.  
- Create additional registry lines or glue only when necessary and clearly documented.  
- Use a separate envelope for each primary file when a changeset involves multiple artifacts.

This keeps PRs understandable, reviewable, and automatable.

### 5.2. Schema-first JSON/NDJSON

Whenever JSON/NDJSON is produced:

- The AI must supply a `schemaref` pointing to a canonical schema (for example, invariants, metrics, event/region contracts, registry schemas, AI authoring envelopes).  
- The `body` must be crafted to pass JSON Schema validation with no additional properties, correct types, and proper ranges.

This applies to:

- Invariant bundles (Black-Archivum).  
- Seeds and registries (Atrocity-Seeds).  
- Public registries (Horror.Place).  
- ZKP envelopes and verifiers (Dead-Ledger).

### 5.3. Repo-role alignment

AI must ensure:

- Seeds and contract cards live in vaults such as Atrocity-Seeds or Spectral-Foundry, not in Horror.Place or Constellation-Contracts.  
- Engine implementations live in Codebase-of-Death.  
- ZKP envelopes and verifier configs live in Dead-Ledger.  
- AI authoring contracts, checklists, and this guidelines file live in HorrorPlace-Constellation-Contracts.

If the requested location conflicts with repo roles, the AI should propose a corrected `target_repo` and explain the adjustment in the envelope’s `description`.

### 5.4. Implication-only and safety posture

In sensitive repos (Atrocity-Seeds, Black-Archivum, Dead-Ledger, Redacted-Chronicles), AI must:

- Avoid explicit horror descriptions; use IDs, hashes, invariants, metrics, and tags.  
- Respect safety tiers and intensity bands; require Dead-Ledger references for high-intensity content where policies demand it.  
- Prefer numeric and structural signals over narrative text.

---

## 6. CI and enforcement

Each repository can enforce these guidelines by:

- Requiring `.ai/*.json` envelopes in AI-generated PRs.  
- Validating envelopes against `ai_file_envelope_v1.json` and `ai_research_plan_v1.json`.  
- Cross-checking `target_repo` and `target_path` values against actual changes in the PR.  
- Running schema validation, invariant/metric range checks, and content leak scans on the files referenced by envelopes.

HorrorPlace-Constellation-Contracts provides reusable GitHub Actions workflows to:

- Validate AI envelopes.  
- Enforce one-file-per-envelope discipline.  
- Integrate envelope validation with existing schema and registry checks.

---

## 7. Recommended workflow for AI/chat tools

When serving a Horror$Place user, an AI/chat tool should:

1. **Interpret the request**  
   Map it to an artifact type and choose an appropriate repository and path.

2. **Decide research vs. generation**  
   - If required information is missing or ambiguous: emit an `ai_research_plan`.  
   - If requirements are clear: construct an `ai_file_envelope`.

3. **Generate and self-validate**  
   For an `ai_file_envelope`, validate the `body` against the schema referenced by `schemaref` (when applicable), and sanity-check repo-role and tier.

4. **Present the envelope**  
   Return the envelope to the user or automation pipeline; avoid raw file dumps without envelopes.

5. **Iterate with user feedback**  
   Update research plans and envelopes based on user corrections or additional constraints, maintaining a clear audit trail of decisions.

By following these guidelines, AI/chat tools contribute to Horror$Place as disciplined, schema-aware compilers, rather than free-form generators, keeping the VM-constellation coherent, auditable, and safe.
