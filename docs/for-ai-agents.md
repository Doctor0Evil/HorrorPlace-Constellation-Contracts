Below is a `docs/for-ai-agents.md` “decision tree” section that maps the `constellation-navigation.sql` query names to concrete behaviors and research‑gating rules, ready to drop into the repo. It assumes the existing schemas and navigation queries you already have. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_e938e597-611a-4329-b729-0ee85647f6fb/3b290d8b-d698-4e8a-b4fd-8e63b9a22a6e/55ecd068-6fbd-49be-b3e9-152b69bad05b.md)

***

## AI agent decision tree for constellation navigation

This section defines how coding‑agents must use the `db/queries/constellation-navigation.sql` snippets to navigate the HorrorPlace constellation, decide when to stay in research mode, and when it is safe to synthesize full production‑quality code. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_e938e597-611a-4329-b729-0ee85647f6fb/3b290d8b-d698-4e8a-b4fd-8e63b9a22a6e/55ecd068-6fbd-49be-b3e9-152b69bad05b.md)

### 1. Entry: always start from metadata and navigation SQL

Agents must:

- Load `docs/ai-metadata.json` for the active repo to discover:
  - Core schemas and tables (e.g. BCI, palettes, CAN registry).
  - Code modules and logging files.
  - Tooling policy and allowed tools.
- Load `db/queries/constellation-navigation.sql` and treat the named queries there as the canonical navigation vocabulary.

Do not begin by dumping large directories or files. Use the navigation queries first to keep token usage low and wiring clear. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_e938e597-611a-4329-b729-0ee85647f6fb/3b290d8b-d698-4e8a-b4fd-8e63b9a22a6e/55ecd068-6fbd-49be-b3e9-152b69bad05b.md)

***

### 2. Repository and component discovery

When you need to understand which files and components exist:

1. Identify the repo:
   - If you know the repository name, run `componentsByRepo` with `:repoName`.
   - If you only know the role (e.g. runtime, contracts, ledger), run `reposByRole` with `:roleName` to list candidate repos and then call `componentsByRepo` on the one you select. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_e938e597-611a-4329-b729-0ee85647f6fb/3b290d8b-d698-4e8a-b4fd-8e63b9a22a6e/55ecd068-6fbd-49be-b3e9-152b69bad05b.md)

2. Discover schemas:
   - To find all schemas for a domain (e.g. `bci`, `palette`, `wiring`), run `schemasForDomain` with `:domainName`.
   - Use this result to decide which schemas to inspect in detail. Prefer to open only the specific schema files that appear in this query output. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_e938e597-611a-4329-b729-0ee85647f6fb/3b290d8b-d698-4e8a-b4fd-8e63b9a22a6e/55ecd068-6fbd-49be-b3e9-152b69bad05b.md)

3. Filter by component kind:
   - Use `componentsByKind` with `:kindName` to locate all components of a particular type (e.g. `schema`, `sqlschema`, `rustmodule`, `bcipipeline`, `canregistry`).
   - Use these results to restrict further exploration to a small set of files.

Behavior rule:

- If the required component (schema, SQL, or module) does not appear in these results, stay in **research mode**:
  - Propose adding the missing component as a new file or schema.
  - Do not assume its existence or generate runtime code that depends on it.

***

### 3. Wiring and field usage exploration

When you need to understand data flow and field usage across the constellation:

1. Pipeline stages and edges:
   - For a given repo, run `pipelineStagesForRepo` with `:repoName` to list BCI pipeline stages and their layers, input/output types, and primary files.
   - Run `pipelineEdgesFromRepo` with `:repoName` to see how stages in this repo connect to stages in this or other repos. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_e938e597-611a-4329-b729-0ee85647f6fb/3b290d8b-d698-4e8a-b4fd-8e63b9a22a6e/55ecd068-6fbd-49be-b3e9-152b69bad05b.md)

   Behavior:

   - Use these results to understand ingestion → compute → log → persistence chains without reading code first.
   - Only open the `primaryfile` or `auxfiles` for the specific stages you care about.

2. Field usage across repos:
   - To see everywhere a field is used (schema, SQL, code), run `fieldUsageEverywhere` with `:fieldPath` (e.g. `bciSummary.visualOverloadIndex`).
   - Use this to:
     - Confirm the field exists in at least one JSON schema.
     - Confirm the field is logged in at least one SQL table.
     - Confirm there are code locations that reference it.

Behavior:

- If `fieldUsageEverywhere` returns no rows:
  - Treat the field as **undefined** in the constellation.
  - Do not generate bindings or logging code for it.
  - You may propose adding it to schemas and SQL as a design suggestion only.

***

### 4. Research‑condition queries: gating code generation

Agents must use the research queries to decide whether to generate **full production code** or **stay in design / scaffolding mode**. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_e938e597-611a-4329-b729-0ee85647f6fb/3b290d8b-d698-4e8a-b4fd-8e63b9a22a6e/55ecd068-6fbd-49be-b3e9-152b69bad05b.md)

#### 4.1 `researchReadyDomainSchemas`

Query:

- Run `researchReadyDomainSchemas` with `:domainName` (e.g. `bci`, `palette`, `wiring`). The query returns counts:
  - `request_schemas`
  - `response_schemas`
  - `runtime_sql_schemas`
  - `pipeline_sql_schemas` [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_e938e597-611a-4329-b729-0ee85647f6fb/3b290d8b-d698-4e8a-b4fd-8e63b9a22a6e/55ecd068-6fbd-49be-b3e9-152b69bad05b.md)

Decision rule:

- If all of the following are true:
  - `request_schemas >= 1`
  - `response_schemas >= 1`
  - `runtime_sql_schemas >= 1`
- Then:
  - It is safe to generate end‑to‑end code for this domain:
    - Request/response structs and mapping glue.
    - Runtime loggers (e.g. SQL logging for frames, bindings, palettes).
    - Simple pipeline integration using existing schemas.
- Otherwise:
  - Remain in research mode:
    - Propose schema additions or SQL DDL to fill the missing pieces.
    - Avoid implementing full handlers; confine output to stubs and design sketches.

#### 4.2 `researchReadyField`

Query:

- Run `researchReadyField` with `:fieldPath` (e.g. `visual.motionSmear`, `audio.heartbeatGain`).
- It returns:
  - `jsonschema_count`
  - `sqltable_count`
  - `code_count` (Rust/CPP/shader). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_e938e597-611a-4329-b729-0ee85647f6fb/3b290d8b-d698-4e8a-b4fd-8e63b9a22a6e/55ecd068-6fbd-49be-b3e9-152b69bad05b.md)

Decision rule:

- If:
  - `jsonschema_count >= 1`
  - `sqltable_count >= 1`
- Then:
  - You may:
    - Generate production‑quality mapping code between JSON, structs, and SQL.
    - Extend logging and bindings for this field.
- If:
  - `jsonschema_count >= 1`
  - `sqltable_count = 0`
- Then:
  - Treat the field as **schema‑only**:
    - You may propose SQL DDL changes to add the field.
    - Do not assume it is already persisted; avoid reading it from non‑existent columns.
- If:
  - `jsonschema_count = 0`
- Then:
  - Do not generate runtime code that uses this field as part of public contracts.
  - You may:
    - Suggest adding it to schemas.
    - Use it for internal, experimental fields clearly labeled as such.

#### 4.3 `researchReadyPipelinePath`

Query:

- Run `researchReadyPipelinePath` with `:startInputType` and `:endLayer` (e.g. `'ai-bci-geometry-request-v1'` → `'persistence'`).
- It returns `candidate_edges` (number of edges that connect from the given input type to the target layer). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_e938e597-611a-4329-b729-0ee85647f6fb/3b290d8b-d698-4e8a-b4fd-8e63b9a22a6e/55ecd068-6fbd-49be-b3e9-152b69bad05b.md)

Decision rule:

- If `candidate_edges > 0`:
  - Cross‑repo wiring for this path is considered defined.
  - It is safe to:
    - Generate or refactor glue code between stages (e.g. BCI spine → theatre logs → ledger).
    - Assume the high‑level pipeline shape is stable.
- If `candidate_edges = 0`:
  - Do not invent cross‑repo wiring.
  - Stay in research mode and:
    - Propose additions to `bcipipelinestage` / `bcipipelineedge`.
    - Avoid generating end‑to‑end glue that assumes specific network or persistence paths.

***

### 5. Task‑specific readiness checks

These queries are hard‑coded shortcuts for common HorrorPlace tasks. Agents must obey their results when deciding how far to go.

#### 5.1 `researchReady_MonsterModeBCI`

Query returns:

- `has_request`
- `has_response`
- `has_binding`
- `has_runtime_sql`
- `has_palette_ontology` [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_e938e597-611a-4329-b729-0ee85647f6fb/3b290d8b-d698-4e8a-b4fd-8e63b9a22a6e/55ecd068-6fbd-49be-b3e9-152b69bad05b.md)

Decision rule:

- If all of these are `>= 1`:
  - You may generate:
    - Full monster‑mode BCI handlers (request → bindings → response).
    - SQL logging for frames and bindings.
    - Palette / swatch index lookups wired into bindings.
- If any field is `0`:
  - Limit yourself to:
    - Schema and SQL patches to make the set complete.
    - Stub handlers clearly marked as requiring schema/SQL completion.
  - Do not generate “final” production code for the monster‑mode runtime.

#### 5.2 `researchReady_CANRegistryRotViz`

Query returns:

- `has_can_schema`
- `has_can_sql`
- `has_can_rust` [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_e938e597-611a-4329-b729-0ee85647f6fb/3b290d8b-d698-4e8a-b4fd-8e63b9a22a6e/55ecd068-6fbd-49be-b3e9-152b69bad05b.md)

Decision rule:

- If all `>= 1`:
  - You may:
    - Generate or update CAN registry loader code.
    - Wire CAN tables (`cantoken`, `cantokenregistry`) into migrations and startup code.
    - Propose query patterns and helper APIs for reading CAN tokens from SQL.
- If `has_can_schema = 1` but `has_can_sql = 0`:
  - Generate only:
    - SQL DDL and migration steps for registry tables.
  - Do not treat CAN tokens as queryable via SQL yet.
- If `has_can_schema = 0`:
  - Do not generate runtime integration.
  - Only propose schema work or design sketches.

***

### 6. Behavior matrix: zero vs non‑zero counts

For quick reference, agents can use this matrix:

- Any **researchReady\*** query:
  - If all “critical” counts are non‑zero:
    - It is allowed to:
      - Generate production‑quality code and SQL.
      - Assume schemas and core tables exist.
  - If one or more critical counts are zero:
    - Stay in research / scaffolding mode:
      - Propose changes to schemas or SQL.
      - Generate stubs, tests, and design‑only examples.
      - Do not assume runtime availability of missing components.

Agents must never skip these checks when working on:

- New or modified request/response contracts.
- New BCI fields or invariants.
- Cross‑repo wiring (BCI → theatre → ledger → analysis).
- CAN registry exposure and control surfaces.

Using this decision tree keeps code generation aligned with actual repository state, minimizes token usage, and prevents agents from inventing wiring or contracts that are not yet defined in schemas and SQL.
