# Constellation SQLite Index

This document describes the constellation-wide SQLite index (`constellation.db`) and how it serves as the canonical discovery and routing layer for AI-chat systems, CHATDIRECTOR, hp-chunker, and engine tooling.

## 1. Purpose

The SQLite index compiles all contract-first knowledge about a VM-constellation into a single, queryable database:

- Repositories and their manifests (tier, role, AI authoring rules).
- Schema spine (canonical JSON Schemas, invariants, metrics, and consumers).
- NDJSON registries and IDs for regions, events, personas, styles, seeds.
- Prism metadata for AI-generated artifacts and their dependencies.
- Token-aware chunk metadata over files, used by AI-chat and coding agents.

Instead of guessing file paths or scanning Git trees, tools query this index to decide where artifacts live, which schemas apply, and how they are wired into history and DeadLedger surfaces.

## 2. File location and lifecycle

By convention, the indexer writes a single SQLite file:

- Default path: `output/constellation.db`
- Owner: Orchestrator and tooling (not committed to source control).
- Producer: A dedicated Rust CLI (e.g. `hpc-constellation-indexer`) that ingests all constellation repos and rebuilds the index on demand or in CI.

Downstream tools treat `constellation.db` as read-only.

## 3. Schema contract

The logical shape of the database is defined by:

- `schemas/tooling/constellation-sqlite-schema.v1.json`

This JSON Schema describes:

- Required tables and their columns.
- Expected types and constraints (e.g. primary keys, foreign-key relationships).
- Normalized representation of schema spines, manifests, registries, prisms, and chunks.

Any indexer implementation must produce a database whose tables and columns conform to this contract. Any consumer (CHATDIRECTOR, hp-chunker, Lua helpers) may rely on this contract for queries.

## 4. Table families

At a high level, the index organizes data into the following families:

### 4.1 Repositories and manifests

Tables:

- `repos`
- `repo_manifests`
- `routing_rules`

These tables encode which repositories exist, which tiers and roles they occupy, and how object kinds and tiers route deterministically to a `targetRepo` and path pattern.

### 4.2 Schema spine and consumers

Tables:

- `schemas`
- `schema_fields`
- `schema_consumers`

These tables encode the canonical schema spine: which JSON Schemas are available, the numeric bounds for key fields (such as invariants and metrics), and where those schemas are consumed across repos.

### 4.3 Invariants and metrics

Tables:

- `invariants`
- `metrics`
- `invariant_relations`
- `metric_recommendations`

These tables capture the horror invariants and entertainment metrics as a first-class numeric catalog, along with derived relationships and recommended bands by object kind and tier.

### 4.4 Registries

Tables:

- `registries`
- `registry_entries`
- `registry_entry_invariants`
- `registry_entry_metrics`

These tables are the structured equivalent of NDJSON registries. Every region, event, persona, style, or seed entry becomes a row with a stable ID, schemaref, tier, DeadLedger reference, and its invariant/metric bindings.

### 4.5 Prism and authoring metadata

Tables:

- `agents`
- `agent_profiles`
- `prisms`
- `prism_dependencies`

These tables record which agents and profiles generate artifacts, which prism metadata was attached, and which upstream contracts and proofs each artifact depends on. They form the backbone for bidirectional prism contracts and provenance analysis.

### 4.6 Files and chunks

Tables:

- `files`
- `chunks`

These tables map physical files in each repo and expose token-aware chunks for AI-chat and coding agents. Chunks carry approximate token cost, chunk kind, and lists of invariants/metrics they touch.

## 5. Indexer responsibilities

The constellation indexer is responsible for:

1. Discovering all repos and reading their `repo-manifest.hpc.json`.
2. Registering canonical schemas, invariants, and metrics from the spine.
3. Scanning registries and registering every entry with its IDs and bands.
4. Walking file trees, inferring schemas and object kinds where possible.
5. Ingesting chunk manifests (if present) and writing `chunks` rows.
6. Parsing prism metadata and agent profiles and registering them in the appropriate tables.

If any of these steps fail to conform to the JSON Schema contract, the indexer must refuse to emit or update `constellation.db`.

## 6. How CHATDIRECTOR uses the index

CHATDIRECTOR should treat `constellation.db` as the primary discovery surface:

- `plan` uses `routing_rules`, `schemas`, `invariants`, `metrics`, and agent profiles to resolve `objectKind + tier` into a single `targetRepo`, path pattern, schemaref, and safe invariant/metric bands.
- `validate-response` cross-checks all generated artifacts against the database, verifying that routing, schema ranges, and registry references match the canonical index, not the prompt text.

No authoring request should be accepted without a valid, fresh SQLite index.

## 7. How hp-chunker and agents use the index

hp-chunker and AI agents can:

- Query `files` and `schemas` to select candidate files for chunking.
- Query `chunks` to select token-sized slices appropriate for a given session budget.
- Query `schema_consumers`, `registry_entries`, and `prisms` to understand where a change will propagate before generating any code or JSON.

This turns the constellation into a machine-navigable graph instead of a set of opaque Git repositories.

## 8. Versioning and evolution

The `version` field in `constellation-sqlite-schema.v1.json` is used to evolve the index safely:

- Minor changes that only add optional columns may bump the version within `v1` and keep older tools compatible.
- Breaking changes (removing or renaming tables or columns) require a new major version (e.g. `v2`) and a migration strategy.

CHATDIRECTOR, hp-chunker, and other tools must check the reported schema version before performing queries and fail fast if the database is incompatible with their expectations.
