---
doc-type: guide-v1
title: AI Authoring – Three-Artifact Changesets
status: draft
schema-ref: schema.horrorplace.constellation.docs.guide-v1.json
last-reviewed: 2026-04-05
---

# AI Authoring – Three-Artifact Changesets

This guide defines how AI agents and tools should structure constructive workloads using 1–3 file changesets, wrapped in prism envelopes and validated by shared CI workflows.

## Core Doctrine

AI-chat systems in the HorrorPlace VM-constellation behave as deterministic compilers, not freeform text generators. Every constructive request is treated as a single logical transaction, expressed as a **three-artifact changeset**:

- One changeset per request.
- Between 1 and 3 primary artifacts per changeset.
- A single, coherent intent and schema family per changeset.

Single-file responses are preferred for simplicity, but small bundles of 2–3 tightly scoped files are allowed when they form one atomic operation.

## When 1–3 Files Are Allowed

A changeset may include up to three primary artifacts when all of the following are true:

- **Shared intent**: all artifacts contribute to one clear goal (for example, “introduce a new persona” or “add a new nature seed”).  
- **Shared schema family**: all artifacts validate against a single canonical schema family (for example, persona-contract-v1 and its test/registry companions).  
- **Tight coupling**: removing any one artifact would leave the changeset incomplete or incoherent.

Typical safe patterns:

- 1 file: a single contract or registry entry.  
- 2 files: a contract plus its registry line, or a contract plus a minimal test.  
- 3 files: contract JSON, associated test/spec file, and one registry line.

Any operation that would require more than three primary artifacts must be decomposed into multiple requests or handled by a higher-level orchestrator.

## Prism Envelopes and Changesets

Every constructive request must produce an AI authoring envelope (prism envelope) that declares the changeset:

- `targetRepo`: canonical repository name.  
- `targetPaths`: array of 1–3 relative file paths.  
- `schemaRef`: canonical schema URI (or small set within one family).  
- `tier`: `public`, `vault`, or `lab`.  
- `referencedIds`: stable IDs (regions, events, personas, styles, policies).  
- `invariants` / `metrics`: invariants and metrics touched by this changeset.  
- Optional `deadledgerRef` for attested or restricted content.

The envelope is the single source of truth for CI and orchestration. Engines and tools must not infer intent from filenames alone.

## Agent Profiles and Per-Agent Caps

The constellation distinguishes between the global 1–3 cap and per-agent policies:

- The global contract allows up to three primary artifacts per changeset.  
- Each `agentProfile` document declares `maxFilesPerRequest` for that agent.  
- Conservative agents may be limited to 1 file; more capable compilers may use up to 3.

CI must validate that:

- The number of artifacts declared in the envelope is between 1 and 3.  
- The number does not exceed `agentProfile.maxFilesPerRequest`.  
- All declared artifacts share at least one schema family and intent tag.

## CI and Pre-Commit Enforcement

Shared CI workflows imported from HorrorPlace-Constellation-Contracts enforce this doctrine:

- Schema validation for all artifacts against the canonical schema spine.  
- Envelope validation to ensure 1–3 `targetPaths` and consistent metadata.  
- Invariant and metric range checks (for example, DET within 0–10).  
- Registry linting (IDs, `schemaRef` prefixes, `deadledgerRef` structure).  
- AI-authoring linting to confirm the presence of `targetRepo`, `targetPaths`, `schemaRef`, `tier`, and declared invariants/metrics.

Any changeset that declares zero files, more than three files, or a mixture of unrelated schema families must be rejected by CI.

## Authoring Guidance for AI Agents

When generating a changeset:

- Prefer a single, well-formed artifact unless additional files are necessary to keep the transaction coherent.  
- Do not split a logically atomic operation across multiple requests to bypass the 1–3 cap.  
- Query the schema spine index and NDJSON registries to select valid IDs, paths, and schema URIs before emitting files.  
- Ensure all artifacts in the set can be validated independently and together, with no hidden dependencies outside the declared references.

By following this pattern, AI agents can safely increase throughput while preserving deterministic validation and cross-repository coherence.
