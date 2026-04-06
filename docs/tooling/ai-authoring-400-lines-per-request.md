---
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
  - agent_attestation
  - spectral_seed_attestation
  - bci_state_proof
---

# AI Authoring 400-Lines-Per-Request

This document defines the **400-lines-per-request** contract for AI authoring inside the HorrorPlace VM-Constellation. The previous “one-file-per-request” rule is upgraded to require that each accepted request produces a single artifact with a minimum structural and informational depth of roughly 400 lines per JSON object, markdown document, or planning file. The goal is to ensure that every AI-generated artifact is not just syntactically valid, but also architecturally dense enough to be useful for long-lived, cross-repo work.

The contract is designed for AI-chat agents, IDE integrations, and external platforms that emit structured outputs for HorrorPlace repositories. It remains content-safe: all generated artifacts MUST respect the no-raw-horror, no-BCI-raw-data doctrine and adhere to invariants and entertainment metrics defined in the core schemas.

---

## 1. Rationale for 400-lines-per-request

The VM-Constellation’s repos are not simple code snippets; they are schema-first, contract-driven systems that coordinate invariants, entertainment metrics, registries, proofs, and runtime behaviors. Lightweight single-page stubs often fail to capture the necessary structure, commentary, and test intent. The 400-lines-per-request rule addresses this by making each AI-authored artifact large enough to be:

- Structurally rich: enough sections, properties, and examples to guide future extensions.
- Self-documenting: clear explanations for fields, invariants, metrics, and cross-repo bindings.
- CI-ready: minimal but concrete test hints, usage notes, and future-proofing details.

A “line” here is a single line in the stored file, as rendered in version control. For JSON and NDJSON, this is the physical line break; for markdown, it is the line-level formatting in the document. The rule is approximate: CI and tooling may configure a tolerance band, but the intent is that each accepted artifact is roughly 400 lines or more.

---

## 2. Scope of the 400-line requirement

The 400-lines-per-request contract applies to AI-authored artifacts that are:

- Stored in public or shared HorrorPlace repositories.
- Intended to define contracts, schemas, envelope vocabularies, or core docs.
- Consumed by other repos as sources of truth, e.g., schemas, spine indices, vocabulary docs, and policy specs.

The requirement does not apply to:

- Tiny helper files such as `.gitignore`, `.editorconfig`, or single-line NDJSON examples.
- Auto-generated lock files or machine-produced logs.
- Pure test fixtures that intentionally stay small.

The default expectation is that **all AI-authored planning and contract files in `docs/` and `schemas/`** satisfy the 400-line rule, unless explicitly exempted by local repository policy.

---

## 3. Request and response objects

The underlying request/response schemas stay conceptually similar (`ai-authoring-request.v1` and `ai-authoring-response.v1`). The main change is an additional expectation about the size and richness of the `payload` or file being produced.

### 3.1 Authoring request

An AI agent receives an `ai-authoring-request.v1` object that includes at least:

- `targetRepo` — which repository should receive the artifact.
- `targetPath` — where in the repo the file should live.
- `schemaRef` — which canonical schema the payload must validate against, if applicable.
- `tier` — `T1-core`, `T2-vault`, or `T3-lab`.
- `purpose` — what the artifact is meant to define, document, or configure.

The request MAY also include:

- `prismMetaRef` — pointer to a prism profile describing constraints on style, safety, and cross-repo hooks.
- `sizeHint` — optional override if a local policy allows smaller files for a specific operation.

Even with a `sizeHint`, CI can enforce a minimum line count based on repository-specific rules.

### 3.2 Authoring response

The AI agent responds with an `ai-authoring-response.v1` object containing:

- `targetRepo`, `targetPath`, `schemaRef` — copied or derived from the request.
- `payload` — the actual file content, structured per `schemaRef` or as markdown/text.
- `prismMeta` — optional inline metadata describing the generator and constraints applied.

The `payload` MUST:

1. Validate against the referenced schema if `schemaRef` is non-empty.
2. Fit the doctrine (no raw horror, no raw BCI data, no PII).
3. Satisfy the configured 400-line policy for the target repo and path, unless explicitly exempted.

---

## 4. Defining “400 lines” for different file types

Different file types count lines in different ways. Tooling MUST use simple line counting to avoid ambiguity.

### 4.1 JSON and JSON Schema

For JSON files (including JSON Schema):

- Each physical line in the stored file counts as one line.
- Pretty-printing with indentation is allowed and encouraged.
- Comments are not permitted in strict JSON, so every line is part of the payload structure.

An AI-generated JSON Schema intended for cross-repo contracts should contain:

- A clear `$schema` and `$id`.
- `title`, `description`, and comments-like explanations embedded as `description` fields.
- Detailed `properties` with `description` for each field.
- Optional `examples` sections or documentation fields that add explanatory weight.

These elements naturally increase the line count while also adding real architectural value.

### 4.2 Markdown documents

For markdown files in `docs/`:

- Every visible line in the document counts—headings, lists, paragraphs, code blocks, and tables.
- Multi-line paragraphs still increment the line count per line-break in the file.
- Inline code and fenced code blocks are encouraged for clarity and examples.

A 400-line markdown document can blend:

- High-level overview.
- Context and rationale.
- Field-by-field or section-by-section explanations.
- Small example snippets for JSON objects, NDJSON lines, or CLI invocations.
- Short “how to integrate” sections for external tools.

### 4.3 NDJSON registries

For NDJSON:

- Each object line is one line.
- Header comments are not permitted as NDJSON does not support comments.
- A 400-line NDJSON file might be used for dense test fixtures or large example sets and is generally not required for boilerplate examples.

The 400-line policy usually applies to documentation and schemas, not to large NDJSON data files. NDJSON examples in `registry/formats/*.ndjson.example` remain intentionally small.

---

## 5. Minimum structure expectations

The 400-line rule is not only about size; it encodes a minimum structural richness for AI-authored files.

### 5.1 For JSON Schema contracts

A JSON Schema generated under this contract SHOULD include:

- A top-level `description` that explains the contract’s role in the constellation.
- `properties` with `description` fields for each key.
- A clear `required` section.
- `examples` arrays where helpful.
- Optional `definitions` or `$defs` for reusable substructures.
- Explicit constraints for invariants (e.g., `minimum`, `maximum`) and metrics.

By elaborating descriptions and adding small example payloads, the schema becomes both machine-checkable and human-readable, and naturally reaches the target line count.

### 5.2 For planning markdown docs

A planning document in `docs/` under this rule SHOULD include:

- Problem statement and architectural context.
- Clear sections for “Contract role”, “Schema references”, “Repo interactions”.
- Example request/response snippets or QPU datashard lines.
- Lists of open questions or future work, scoped to avoid implementation details that belong in vault/lab repos.

The extra lines are used for explanation, not filler prose.

---

## 6. Authoring flow with 400-line requirement

The typical authoring flow with this contract is:

1. **Prepare the request**  
   A human or orchestrator builds an `ai-authoring-request.v1` object for a specific repo and path, referencing the canonical schema (if any) and stating the purpose.

2. **Call the AI agent**  
   The request is sent to an AI-chat system, Copilot Task, or similar tool that understands this contract and the HorrorPlace schema surface.

3. **Generate a dense artifact**  
   The agent generates a candidate `payload` that:
   - Is structurally valid.
   - Is roughly 400 lines or more.
   - Uses the appropriate schemas, invariants, and metrics.
   - Stays within safety and doctrine constraints.

4. **Run local validation**  
   The authoring flow performs:
   - JSON Schema validation or markdown linting.
   - Line-count checking against repository policy.
   - Optional additional semantic checks (e.g., references to known `schemaRef` IDs).

5. **Submit to CI**  
   If validation passes, the authoring response is submitted to version control (e.g., as a PR). CI re-runs validations, including the 400-line rule.

6. **Review and merge**  
   Human reviewers can focus on semantics and alignment with architecture, trusting that structure, size, and basic validation are already enforced.

---

## 7. CI and pre-commit enforcement

The 400-lines-per-request rule is enforced by tools and CI pipelines in `HorrorPlace-Constellation-Contracts` and other repos that opt in.

### 7.1 Pre-commit hooks

A typical pre-commit configuration might:

- Detect newly added or modified files under `docs/` and `schemas/` that match AI-authored patterns (e.g., contain a header indicating AI assistance).
- Count lines in each candidate file.
- Reject commits where AI-authored files do not meet the configured minimum count.

### 7.2 CI workflows

CI workflows can:

- Validate JSON and YAML files against their schemas.
- Count lines in files that match naming patterns, such as `ai-authoring-*.md` or `*-contract.v1.json`.
- Emit descriptive errors when a file is too small or lacks structural richness.

These checks keep the line-count rule enforceable and visible in logs, without requiring manual counting.

---

## 8. Integrating external platforms

External platforms such as Copilot, Claude, or custom IDE tooling can integrate with this contract by:

- Treating the 400-line rule as a target, not a hard-coded number, configurable via repo metadata.
- Using the schemas in `HorrorPlace-Constellation-Contracts` as ground truth for field names and structures.
- Using energy on structure and documentation instead of raw boilerplate: more `description`, more examples, more cross-references to `schemaRef`, invariants, and metrics.

When an external platform generates a candidate artifact, it should:

- Explicitly check its current line count.
- Expand descriptions, field notes, and examples if necessary to exceed the minimum.
- Avoid adding meaningless filler or random text; every line should support understanding, validation, or future maintenance.

---

## 9. Exceptions and overrides

There are legitimate cases where a smaller file is appropriate. To handle these without weakening the contract:

- Repos MAY declare a small set of exceptions by path, e.g., `docs/tooling/README.md`.
- Requests MAY include a `sizeHint` flag, but CI and maintainers retain the final say.
- Exemptions SHOULD be documented in repository configuration files or README sections.

This keeps the rule strong by default while allowing focused, justified exceptions.

---

## 10. Migration from one-file-per-request

For existing workflows that were designed for “one-file-per-request” without a line-count expectation:

1. Keep the semantic contract: one request still produces exactly one artifact per response.
2. Add the structural requirement: that artifact should be expanded to a 400-line scale when it is a planning, schema, or contract file.
3. Update documentation and templates to reference `ai-authoring-400-lines-per-request` instead of the old “one-file-per-request” document.

Over time, new repos and tools should treat the 400-line requirement as the default for planning and contract work in the constellation.

---

## 11. Summary

The AI Authoring 400-Lines-Per-Request contract preserves the simplicity of “one artifact per request” while acknowledging the complexity and ambition of the HorrorPlace VM-Constellation. It ensures that AI-authored artifacts are dense, well-documented, and structurally useful across repos, rather than being minimal stubs that require extensive manual follow-up.

External platforms and AI agents that adopt this contract can produce higher-quality outputs that plug directly into HorrorPlace repositories, reduce rework, and align with the invariants and metric frameworks that govern the entire constellation.
