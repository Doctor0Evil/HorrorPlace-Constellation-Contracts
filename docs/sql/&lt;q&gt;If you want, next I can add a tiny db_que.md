<img src="https://r2cdn.perplexity.ai/pplx-full-logo-primary-dark%402x.png" style="height:64px;margin-right:32px"/>
```
# <q>If you want, next I can add a tiny db/queries/constellation-navigation.sql snippet set (e.g. -- componentsByRepo, -- schemasForDomain) that assumes this DDL and JSON shape, so agents always start from a fixed, token‑cheap query vocabulary.</q> Yes, and expand the vocabulary to trigger by research-conditions, which will guide, and direct ai-chat to generate the full production-quality code when the research is available, or there is enough-data to produce it.
```

Below is a `db/queries/constellation-navigation.sql` snippet set that:

- Assumes the `hp_repo` / `hp_component` DDL and the broader field/pipeline schemas you already have.[^1]
- Provides a small, named query vocabulary for navigation.
- Adds **“research‑condition”** helpers that let AI‑chat decide when it has enough coverage / context to generate full production‑quality code (e.g. “all schemas present”, “core wiring mapped”, “field usage complete”).[^1]

You can drop this file into `db/queries/constellation-navigation.sql` in HorrorPlace‑Constellation‑Contracts and reference it from `docs/for-ai-agents.md`.[^1]

***

## 1. Core navigation snippets

All queries are short, parameterized, and designed to be copy‑pasted by agents with a minimal substitution (e.g. `:repoName`, `:domain`).[^1]

```sql
-- File db/queries/constellation-navigation.sql
-- Target repo Doctor0Evil/HorrorPlace-Constellation-Contracts
-- Purpose Canonical, low-token query snippets for AI agents.

------------------------------------------------------------
-- 1. Basic listing: repositories and components
------------------------------------------------------------

-- componentsByRepo
-- Input :repoName
-- Usage List all components in a given repo.
SELECT
    r.name        AS repo,
    c.kind        AS kind,
    c.path        AS path,
    c.domain      AS domain,
    c.tags        AS tags,
    c.summary     AS summary
FROM hp_repo AS r
JOIN hp_component AS c ON c.repo_id = r.repo_id
WHERE r.name = :repoName
ORDER BY c.kind, c.path;

-- schemasForDomain
-- Input :domainName
-- Usage Find all schema components in a domain (bci, palette, ledger, wiring, etc.).
SELECT
    r.name    AS repo,
    c.path    AS schema_path,
    c.kind    AS kind,
    c.summary AS summary
FROM hp_repo AS r
JOIN hp_component AS c ON c.repo_id = r.repo_id
WHERE c.domain = :domainName
  AND c.kind IN ('schema', 'sqlschema')
ORDER BY r.name, c.path;

-- reposByRole
-- Input :roleName (runtime, contracts, ledger, analysis, tooling, etc.)
SELECT
    name      AS repo,
    git_url   AS git_url,
    role      AS role,
    local_root,
    local_checkout,
    is_temporary
FROM hp_repo
WHERE role = :roleName
ORDER BY name;

-- componentsByKind
-- Input :kindName (schema, sqlschema, rustmodule, bcipipeline, canregistry, palette, doc, etc.)
SELECT
    r.name AS repo,
    c.path,
    c.domain,
    c.summary
FROM hp_repo AS r
JOIN hp_component AS c ON c.repo_id = r.repo_id
WHERE c.kind = :kindName
ORDER BY r.name, c.path;
```


***

## 2. Wiring and pipeline context snippets

These assume the `bcipipelinestage` / `bcipipelineedge` tables you already drafted, plus `fieldusage`.[^1]

```sql
------------------------------------------------------------
-- 2. Wiring: BCI pipeline and field usage
------------------------------------------------------------

-- pipelineStagesForRepo
-- Input :repoName
-- Usage Show all pipeline stages declared for a repo.
SELECT
    s.stageid,
    s.repo,
    s.stagekey,
    s.name,
    s.layer,
    s.inputtype,
    s.outputtype,
    s.primaryfile
FROM bcipipelinestage AS s
WHERE s.repo = :repoName
ORDER BY s.layer, s.stageid;

-- pipelineEdgesFromRepo
-- Input :repoName
-- Usage Show edges that originate from stages in a repo (including cross-repo hops).
SELECT
    s1.repo AS from_repo,
    s1.stagekey AS from_stage,
    s2.repo AS to_repo,
    s2.stagekey AS to_stage,
    e.protocol,
    e.description
FROM bcipipelineedge AS e
JOIN bcipipelinestage AS s1 ON s1.stageid = e.fromstageid
JOIN bcipipelinestage AS s2 ON s2.stageid = e.tostageid
WHERE s1.repo = :repoName
ORDER BY from_repo, from_stage, to_repo, to_stage;

-- pipelineForInputType
-- Input :inputType (e.g. 'ai-bci-geometry-request-v1')
-- Usage List all stages that accept a given logical input type.
SELECT
    s.repo,
    s.stagekey,
    s.name,
    s.layer,
    s.outputtype,
    s.primaryfile
FROM bcipipelinestage AS s
WHERE s.inputtype = :inputType
ORDER BY s.repo, s.stageid;

-- fieldUsageEverywhere
-- Input :fieldPath (e.g. 'bciSummary.visualOverloadIndex')
-- Usage Show where a logical field appears across the constellation.
SELECT
    fieldpath,
    repo,
    locationtype,
    locationpath,
    containername,
    containerfield,
    role,
    note
FROM fieldusage
WHERE fieldpath = :fieldPath
ORDER BY repo, locationtype, locationpath;
```


***

## 3. Research‑condition helpers (when is there “enough data”?)

These queries let AI‑chat test whether it has the necessary coverage to safely synthesize production code for a given **domain**, **field**, or **pipeline path**.[^1]

### 3.1. Domain completeness for schemas

```sql
------------------------------------------------------------
-- 3. Research conditions: domain coverage
------------------------------------------------------------

-- researchReadyDomainSchemas
-- Input :domainName
-- Condition Domain is "ready" if we have at least:
--   * one request/response JSON schema
--   * one runtime log SQL schema
--   * one wiring / pipeline schema
-- This is a heuristic trigger for generating end-to-end code for that domain.
SELECT
    :domainName          AS domain,
    SUM(CASE WHEN c.kind = 'schema'
               AND c.path LIKE 'schemas/%request%' THEN 1 ELSE 0 END) AS request_schemas,
    SUM(CASE WHEN c.kind = 'schema'
               AND c.path LIKE 'schemas/%response%' THEN 1 ELSE 0 END) AS response_schemas,
    SUM(CASE WHEN c.kind = 'sqlschema'
               AND c.path LIKE '%monstermode%' THEN 1 ELSE 0 END) AS runtime_sql_schemas,
    SUM(CASE WHEN c.kind = 'sqlschema'
               AND c.path LIKE '%bci-pipeline%' THEN 1 ELSE 0 END) AS pipeline_sql_schemas
FROM hp_repo AS r
JOIN hp_component AS c ON c.repo_id = r.repo_id
WHERE c.domain = :domainName;
```

Usage pattern for agents:

- Run `researchReadyDomainSchemas` for `domain = 'bci'`.
- If `request_schemas >= 1`, `response_schemas >= 1`, and `runtime_sql_schemas >= 1`, treat as “enough schema surface” to synthesize full request/response handlers and loggers.[^1]


### 3.2. Field usage completeness

```sql
-- researchReadyField
-- Input :fieldPath
-- Condition Field is "ready" if it appears in:
--   * at least one JSON schema
--   * at least one SQL table
--   * at least one runtime struct/code location
-- This signals that production bindings can be generated without guessing.
SELECT
    :fieldPath AS fieldpath,
    SUM(CASE WHEN locationtype = 'jsonschema' THEN 1 ELSE 0 END) AS jsonschema_count,
    SUM(CASE WHEN locationtype = 'sqltable'   THEN 1 ELSE 0 END) AS sqltable_count,
    SUM(CASE WHEN locationtype IN ('ruststruct','cppstruct','shader') THEN 1 ELSE 0 END) AS code_count
FROM fieldusage
WHERE fieldpath = :fieldPath;
```

Agent behavior:

- For a candidate parameter such as `visual.motionSmear`, call `researchReadyField`.
- Only generate a full implementation if `jsonschema_count >= 1` and `sqltable_count >= 1`; if `code_count = 0`, treat it as “new field, schema‑only” and generate scaffolding code plus TODO markers.[^1]


### 3.3. Pipeline path sufficiency

```sql
-- researchReadyPipelinePath
-- Input :startInputType, :endLayer
-- Example from 'ai-bci-geometry-request-v1' to 'persistence'
-- Condition "ready" if there is at least one path touching a ledger/persistence stage.
SELECT
    COUNT(*) AS candidate_edges
FROM bcipipelineedge AS e
JOIN bcipipelinestage AS s1 ON s1.stageid = e.fromstageid
JOIN bcipipelinestage AS s2 ON s2.stageid = e.tostageid
WHERE s1.inputtype = :startInputType
  AND s2.layer     = :endLayer;
```

- If `candidate_edges > 0`, AI‑chat can safely synthesize **cross‑repo wiring** (e.g. Rotting‑Visuals → Dead‑Ledger), because a logical path has been defined in `bcipipelinestage/edge`.[^1]

***

## 4. Research‑condition presets for typical HorrorPlace tasks

These are “shortcut” queries that map directly to common code‑generation goals.[^1]

```sql
------------------------------------------------------------
-- 4. Task-specific readiness checks
------------------------------------------------------------

-- researchReady_MonsterModeBCI
-- Goal Safe to synthesize full monster-mode BCI handler + logger.
SELECT
    SUM(CASE WHEN c.kind = 'schema'
               AND c.path = 'schemas/ai-bci-geometry-request-v1.json' THEN 1 ELSE 0 END) AS has_request,
    SUM(CASE WHEN c.kind = 'schema'
               AND c.path = 'schemas/ai-bci-geometry-response-v1.json' THEN 1 ELSE 0 END) AS has_response,
    SUM(CASE WHEN c.kind = 'schema'
               AND c.path = 'schemas/bci-geometry-binding-v1.json' THEN 1 ELSE 0 END) AS has_binding,
    SUM(CASE WHEN c.kind = 'sqlschema'
               AND c.path LIKE '%rottingvisuals-monstermode.sql' THEN 1 ELSE 0 END) AS has_runtime_sql,
    SUM(CASE WHEN c.kind = 'sqlschema'
               AND c.path LIKE '%constellation-ontology.sql' THEN 1 ELSE 0 END) AS has_palette_ontology
FROM hp_repo AS r
JOIN hp_component AS c ON c.repo_id = r.repo_id
WHERE r.name = 'Rotting-Visuals-BCI';

-- researchReady_CANRegistryRotViz
-- Goal Safe to synthesize CAN registry loaders and safety checks.
SELECT
    SUM(CASE WHEN c.kind = 'schema'
               AND c.path LIKE 'schemas/can-token-registry-rot-visuals-v1.json' THEN 1 ELSE 0 END) AS has_can_schema,
    SUM(CASE WHEN c.kind = 'sqlschema'
               AND c.path LIKE '%cantokens.sql' THEN 1 ELSE 0 END) AS has_can_sql,
    SUM(CASE WHEN c.domain = 'bci'
               AND c.kind = 'rustmodule'
               AND c.path LIKE '%canregistry%' THEN 1 ELSE 0 END) AS has_can_rust
FROM hp_repo AS r
JOIN hp_component AS c ON c.repo_id = r.repo_id;
```

Agent‑side logic for these “presets”:

- If all `has_*` columns are `>= 1`, treat the lane as **“research‑complete”**, and allow code generation for:
    - Request/response structs and mapping glue.
    - SQLite loggers.
    - CAN registry upsert/validation logic.
- If some are 0, stay in **“research” mode**:
    - Propose schema/SQL patches instead of runtime code.
    - Ask the user which missing components should be created or inferred.

***

## 5. How this guides full‑code generation

You can add a short section in `docs/for-ai-agents.md` explaining how to use this vocabulary:[^1]

- Start with **navigation snippets**: `componentsByRepo`, `schemasForDomain`, `pipelineStagesForRepo`.
- Evaluate **research‑conditions** (`researchReadyDomainSchemas`, `researchReadyField`, `researchReady_MonsterModeBCI`).
- If readiness thresholds pass, it is allowed to:
    - Generate or refactor Rust / C++ / SQLite code for that lane.
    - Assume schemas and logs are stable enough to treat as authoritative.
- If thresholds fail, remain in design‑only mode and limit output to:
    - Schema drafts.
    - SQL DDL sketches.
    - Minimal stub modules with explicit TODO comments.

If you’d like, next I can:

- Draft that `docs/for-ai-agents.md` “decision tree” section that explicitly maps these query names to behaviors (“if this count is zero, do X; otherwise, do Y”), so your agents consistently gate code generation on these research‑conditions.

<div align="center">⁂</div>

[^1]: 55ecd068-6fbd-49be-b3e9-152b69bad05b.md

