# Constellation Contracts: Introduction

This document provides a high-level overview of the `HorrorPlace-Constellation-Contracts` repository and its role within the broader VM-constellation ecosystem.

## Purpose

`HorrorPlace-Constellation-Contracts` is the **contract-first governance layer** for multi-repo, multi-VM systems. It defines:

- Canonical JSON Schemas for invariants, metrics, contracts, and registries.
- NDJSON registry formats for machine-navigable discovery of regions, events, styles, and personas.
- Reusable CI workflows and pre-commit hooks for validation at commit time.
- AI-authoring contracts that enable deterministic, one-file-per-request generation.

The repository contains **no raw horror content**, no executable game logic, and no engine-specific code. It is purely a specification and tooling hub.

## Core Architecture

```
Policy → Region → Seed → Runtime
```

1. **Policy Layer** (`policyEnvelope.v1.json`): Declares global constraints, tier gating, and entitlement hints for a constellation segment.
2. **Region Layer** (`regionContractCard.v1.json`): Binds a geographic or conceptual tile to invariants (CIC, AOS), metrics (UEC, ARR), and style contracts.
3. **Seed Layer** (`seedContractCard.v1.json`): Provides PCG seeds, invariant bundles, and event hooks that drive procedural generation.
4. **Runtime Layer**: Engine-specific implementations that consume the above contracts and emit telemetry back to the spine.

Each layer is validated against its canonical schema before being accepted into a registry or merged into a repository.

## Key Concepts

### Schema Spine
The set of canonical JSON Schemas under `schemas/` that define the structure, types, and constraints for all constellation artifacts. Tools query the spine to validate, generate, or refactor content.

### Contract Cards
Typed JSON documents (`policyEnvelope`, `regionContractCard`, `seedContractCard`) that declare:
- `targetRepo` and `path` for file placement.
- `schemaVersion` for validation.
- `prismMeta` for bidirectional linkage and validation propagation.
- Invariant/metric bindings for runtime behavior.

### NDJSON Registries
Newline-delimited JSON indexes under `registry/` that assign stable IDs to entities and store only opaque references (`artifactid`, `deadledgerref`). AI agents and tools query registries instead of guessing file paths.

### One-File-Per-Request
Baseline rule for AI-chat: each generation step emits exactly one validated contract card or registry line, plus any required registry updates. Batch transactions are built atop this primitive.

### prismMeta / agentProfile
Machine-readable metadata that guides AI generation and enables cross-repo validation. `prismMeta` describes linkage and dependency graphs; `agentProfile` declares agent capabilities and constraints.

## Who Uses This Repository

| Role | Use Case |
|------|----------|
| Engine Authors | Integrate contract validation into Unity, Unreal, Godot, or custom runtimes. |
| AI-Chat Tool Builders | Consume schemas and prismMeta to generate structured, validated outputs. |
| CI Integrators | Add pre-commit guards and registry linting to multi-repo workflows. |
| Constellation Architects | Design new tiers, regions, or personas using canonical contracts. |

## Getting Started

1. Read `docs/overview/design-principles.md` for foundational rules.
2. Explore `schemas/core/` to understand invariant and metric definitions.
3. Run `tooling/python/cli/hpc-validate-schema.py` on an example contract card.
4. Review `examples/minimal-constellation/` for a fully wired scaffold.

## Related Documents

- `docs/schema-spine/schema-spine-index-spec.md`: Detailed spine index specification.
- `docs/tooling/prismMeta-and-agentProfiles.md`: Metadata contracts for AI integration.
- `docs/integration/with-horror-place.md`: How this repo aligns with the `Horror.Place` core.
