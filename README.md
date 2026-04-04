# HorrorPlace‑Constellation‑Contracts

HorrorPlace‑Constellation‑Contracts defines cross‑platform contracts, schemas, and workflow patterns for building and governing VM‑constellations. It is designed to let AI agents, tools, and humans collaborate safely on multi‑repo, multi‑VM systems without chaotic file placement or raw horror content.

This repository is **contract‑only**: it contains schemas, registries, examples, and CI/workflow templates, but no executable horror logic or explicit horror assets.

---

## Goals

HorrorPlace‑Constellation‑Contracts aims to:

- Provide a stable schema spine for invariants, entertainment metrics, style contracts, and registry entries.
- Define registry formats and ID conventions that make VM‑constellations discoverable and machine‑navigable.
- Offer reusable CI and pre‑commit patterns that enforce schema conformity and cross‑repo wiring rules.
- Document AI‑chat authoring rules so agents can generate correctly wired files on demand.
- Stay platform‑compliant by excluding raw horror content and treating horror as contracts and references only.

---

## Repository Structure

Planned high‑level layout (subject to evolution):

- `schemas/` – JSON Schemas and related contract definitions.
  - `schemas/invariants/` – Safety and history invariants for regions, events, and entities.
  - `schemas/metrics/` – Entertainment and telemetry metrics for sessions and encounters.
  - `schemas/contracts/` – Style, event, persona, entitlement, and routing contracts.

- `registry/` – Registry formats and example NDJSON registries.
  - `registry/regions/` – Region IDs and opaque references.
  - `registry/events/` – Event IDs and opaque references.
  - `registry/styles/` – Style IDs and opaque references.
  - `registry/personas/` – Persona IDs and opaque references.

- `workflows/` – Reusable CI and automation patterns.
  - GitHub Actions workflows for schema validation, registry linting, and drift detection.
  - Templates for cross‑repo signaling and VM‑constellation orchestration.

- `docs/` – Human‑readable specifications and guidelines.
  - Contract descriptions, field semantics, and versioning rules.
  - AI‑chat authoring rules and integration notes.
  - Safety and compliance guidelines.

Names and subdirectories are intentionally generic so other domains can adapt this repository as a constellation contract layer, even outside horror.

---

## Core Concepts

### Schema Spine

The schema spine is the set of canonical schemas that define:

- Invariants for regions, events, entities, and history.
- Entertainment and telemetry metrics for evaluating experiences.
- Style and presentation contracts for visual, audio, and narrative layers.
- Entitlement and proof envelopes for gated content and trust.

All downstream repos and VMs are expected to treat these schemas as the single source of truth and to validate their own data and behavior against them.

### Registries

Registries are newline‑delimited JSON (NDJSON) or equivalent structures that:

- Assign stable IDs to regions, events, styles, personas, and other entities.
- Store only opaque references (artifact IDs, content IDs, proof IDs), never raw assets.
- Provide a single index that AI agents and tools can query instead of guessing file paths.

By centralizing IDs and references, registries keep large constellations navigable and reduce duplication and drift.

### Pre‑Commit and CI Contracts

This repository will define:

- Schema validation jobs to ensure all JSON and NDJSON conform to the canonical schemas.
- Registry linting rules that require required fields, reference types, and prefixes.
- Drift detection tools that compare local copies and consumers against canonical definitions.
- Optional local pre‑commit hooks that run fast checks before a commit lands.

The goal is to make the “correct” way to add or change constellation data also the easiest way.

---

## AI‑Chat and Agent Integration

HorrorPlace‑Constellation‑Contracts assumes AI‑chat agents and tools act as deterministic file compilers:

- One file per request or per generation step.
- Each generated file declares:
  - Target repository and path.
  - Target schema(s) and version(s).
  - Tier or environment (public, vault, lab, etc.).
  - Referenced IDs (regions, events, personas, styles).
- Agents are expected to:
  - Query registries and canonical schemas before generating files.
  - Obey “no raw horror content” rules for contract‑only repos.
  - Lean on CI and lints defined here for validation.

This repository provides the contracts and templates, not the chat logic itself.

---

## Safety and Compliance

To remain GitHub‑safe and broadly compatible:

- No explicit horror scenes, gore, or graphic content are stored here.
- All references to horror are indirect: IDs, metadata, invariants, metrics, or style descriptors.
- Raw assets (images, audio, large binaries) must be referenced via opaque IDs or content addresses, not embedded directly.
- Telemetry and identity‑related data must be represented in a privacy‑aware, contract‑only way.

Downstream repos that use these contracts are responsible for enforcing their own content and safety policies, but this repository is designed to encourage safe defaults.

---

## Versioning and Stability

Contracts in this repository are versioned and treated as public interfaces:

- Schemas use semantic versioning.
- Breaking changes require new schema versions and clear migration guidance.
- Registry formats and workflow templates evolve carefully, with deprecation windows.

Consumers should pin to specific schema and contract versions and upgrade intentionally.

---

## Contributing

Contributions are welcome if they respect the following:

- Changes must preserve the contract‑only nature of this repository.
- New schemas and registries must be fully documented and validated.
- Workflow and CI changes should be minimal, composable, and well‑commented.
- Proposals should include a short rationale and, where possible, an example of use in a VM‑constellation.

Contribution guidelines, code of conduct, and detailed workflows will be added as the repository stabilizes.

---

## Status

This repository is in early design and scaffolding. Initial work focuses on:

- Establishing the first version of the schema spine and registry formats.
- Publishing baseline CI and workflow templates.
- Documenting AI‑chat authoring and integration expectations.

Expect changes, additions, and refinements as the contracts are tested against real VM‑constellations and AI‑driven workflows.
