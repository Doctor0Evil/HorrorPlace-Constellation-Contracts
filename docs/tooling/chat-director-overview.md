# CHAT_DIRECTOR Tooling Overview

CHAT_DIRECTOR is the contract-aware orchestration layer for Horror.Place constellations. It sits on top of JSON Schemas, NDJSON registries, and a spine of invariants and entertainment metrics, providing a single, schema-first interface for AI chats and humans to author, validate, and apply contract cards.

This document explains how the Rust crate, Python tooling, Lua helpers, and CI workflows fit together.

---

## Core concepts

CHAT_DIRECTOR operates over a constellation: a set of schemas, invariants/metrics spines, and registries that describe horror-relevant entities (regions, moods, events, seeds, personas, policies).

Key concepts:

- **Object kinds** – contract families such as `regionContractCard`, `seedContractCard`, `moodContract`, and related manifest types.
- **Invariants** – persistent, history-linked metrics (CIC, MDI, AOS, RRM, FCF, SPR, RWF, DET, HVF, LSG, SHCI).
- **Entertainment metrics** – player-facing or AI-facing measurements (UEC, EMD, STCI, CDL, ARR).
- **Spine index** – a normalized JSON file (`schema-spine-index-v1.json`) that connects schemas, invariant keys, and metric keys to tiers and contract families.
- **Registries** – NDJSON files (regions, moods, events, seeds) that store concrete contract instances.

CHAT_DIRECTOR uses these inputs to make machine-checked decisions about where and how new contracts may be added or modified.

---

## Architecture layers

CHAT_DIRECTOR is intentionally layered:

1. **Rust crate and CLI** – authoritative implementation of routing, validation, and manifest logic.
2. **Python tooling** – thin wrappers over JSON Schema libraries and the spine, used by CI and AI agents for fast checks:
   - `hpc-validate-schema.py` – validates all `schemas/**/*.json`.
   - `hpc-lint-registry.py` – lints NDJSON registries, enforcing ID and reference rules.
   - `hpc-generate-spine-index.py` – builds `schema-spine-index-v1.json` from schemas and spines.
   - `aiauthoringvalidator.py` – cheap envelope checks for AI authoring flows.
3. **Lua helpers** – lightweight, embeddable clients for runtimes:
   - `hpccontractcards.lua` – band-checks invariant bindings and metric targets for a contract card.
   - `hpcregistryclient.lua` – loads NDJSON registries, resolves IDs, and validates references.
4. **CI workflows** – GitHub Actions front-ends that invoke the above in a consistent order.

All layers read the same JSON artifacts; there is no second source of truth for schemas or invariants.

---

## What CHAT_DIRECTOR does (behaviorally)

At a high level, CHAT_DIRECTOR:

- Accepts `ai-authoring-request` envelopes from AI chats and humans.
- Uses manifests, spine bands, and registry state to plan safe contract edits.
- Exposes CLIs like `plan`, `validate-response`, and `apply` to structure AI-driven changes.
- Emits machine-readable diagnostics that describe schema failures, band violations, and routing problems.

The goal is to allow AI agents to treat the repository as a contract space: a place where each path, ID, and numeric band is checked before data lands in registries.

---

## Tooling responsibilities vs. Rust crate responsibilities

- The **Rust crate**:
  - Enforces invariant and manifest rules in depth.
  - Implements routing, trust tiers, and phase boundaries.
  - Owns the canonical CLI (`hpc-chat-director`).

- The **Python and Lua tooling**:
  - Provide cheap, early feedback.
  - Surface schema, envelope, and band-level issues quickly.
  - Never reimplement full manifest or invariant logic.

This separation keeps CI, editors, and AI agents responsive while reserving final authority for the Rust crate.
