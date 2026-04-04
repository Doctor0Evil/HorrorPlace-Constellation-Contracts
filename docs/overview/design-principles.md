# Design Principles

This document outlines the foundational rules that guide the development and evolution of `HorrorPlace-Constellation-Contracts`.

## 1. Contract-First, Content-Agnostic

- All artifacts must conform to a canonical JSON Schema before being accepted.
- Schemas define structure, types, ranges, and required fields—not content.
- Raw horror assets, narrative text, or engine-specific logic never live here.

## 2. Small, Strict, Boring

- Files are minimal: one concern per file, no embedded logic.
- Validation is deterministic: same input → same output, no randomness.
- CI failures are explicit: non-zero exit codes, clear error messages.

## 3. Machine-Navigable by Design

- Every entity has a stable ID and appears in an NDJSON registry.
- Registries store opaque references only; implementation details are abstracted.
- Tools query the schema spine index to discover consumers and dependencies.

## 4. One-File-Per-Request Baseline

- AI-chat agents emit exactly one validated file per generation step.
- Each file declares `targetRepo`, `path`, `schemaVersion`, and referenced IDs.
- Batch transactions are composable primitives built atop this rule.

## 5. Bidirectional Validation Propagation

- Contract cards include `prismMeta` that describes linkage and dependency graphs.
- When a schema changes, the spine index identifies affected consumers.
- CI triggers validation across dependent repos to prevent silent drift.

## 6. Tier-Aware Governance

- Contracts declare a `tier` field (`public`, `vault`, `lab`) that gates access and validation rigor.
- Public tiers enforce GitHub-compliant content; vault/lab tiers allow cryptographic gating via `deadledgerref`.
- Orchestrator services (e.g., `Horror.Place-Orchestrator`) mediate cross-tier updates.

## 7. Telemetry-Driven Refinement

- Runtime metrics (UEC, EMD, STCI, CDL, ARR) are captured in strictly typed envelopes.
- Telemetry feeds back into contract evolution: schemas adapt based on empirical data.
- Drift events (`runtime-drift-event.v1.json`) flag invariant violations for review.

## 8. Platform and Engine Agnostic

- Schemas use standard JSON Schema (Draft 2020-12) with no engine-specific extensions.
- Tooling provides Python CLI and Lua helpers, but contracts are language-neutral.
- Integration examples (`docs/integration/`) show adoption patterns for multiple engines.

## 9. Versioned with Clear Migration Paths

- Schemas use semantic versioning (`v1`, `v1.1`, `v2`).
- Breaking changes require deprecation windows and migration guides.
- The spine index tracks schema versions and consumer compatibility.

## 10. Research-Separated from Normative Specs

- `research/` contains exploratory notes, open questions, and non-binding proposals.
- Only files under `schemas/`, `registry/`, and `tooling/` are normative and CI-enforced.
- Design decisions are logged in `research/logs/` for traceability.

## Anti-Patterns to Avoid

- ❌ Embedding raw content, URLs, or engine paths in contracts or registries.
- ❌ Using `$ref` loops or undefined definitions in JSON Schemas.
- ❌ Allowing AI agents to guess file paths or infer relationships without registry lookup.
- ❌ Modifying canonical schemas without updating the spine index and consumer mappings.

## Enforcement Mechanisms

- `schema-validate.yml`: Validates all `.json` files against their declared `$schema`.
- `registry-lint.yml`: Checks NDJSON entries for required fields and reference formats.
- `ai-authoring-validate.yml`: Ensures AI-generated contract cards meet prismMeta constraints.
- Local pre-commit hooks (`tooling/python/cli/`) provide immediate feedback during development.

## Related Documents

- `docs/overview/constellation-contracts-intro.md`: High-level architecture overview.
- `docs/schema-spine/invariants-and-metrics-spine.md`: Canonical invariant and metric definitions.
- `docs/tooling/constellation-precommit-pack.md`: How to adopt validation hooks in downstream repos.
