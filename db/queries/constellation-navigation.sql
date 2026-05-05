-- File db/queries/constellation-navigation.sql
-- Target repo Doctor0Evil/HorrorPlace-Constellation-Contracts
-- Purpose Canonical, low-token query snippets for AI agents.

PRAGMA foreign_keys = ON;

------------------------------------------------------------
-- 1. Basic listing: repositories and components
------------------------------------------------------------

-- componentsByRepo
-- Input :repoName
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
-- Input :roleName
SELECT
    name           AS repo,
    git_url        AS git_url,
    role           AS role,
    local_root,
    local_checkout,
    is_temporary
FROM hp_repo
WHERE role = :roleName
ORDER BY name;

-- componentsByKind
-- Input :kindName
SELECT
    r.name AS repo,
    c.path,
    c.domain,
    c.summary
FROM hp_repo AS r
JOIN hp_component AS c ON c.repo_id = r.repo_id
WHERE c.kind = :kindName
ORDER BY r.name, c.path;

------------------------------------------------------------
-- 2. Wiring: BCI pipeline and field usage
------------------------------------------------------------

-- pipelineStagesForRepo
-- Input :repoName
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
SELECT
    s1.repo     AS from_repo,
    s1.stagekey AS from_stage,
    s2.repo     AS to_repo,
    s2.stagekey AS to_stage,
    e.protocol,
    e.description
FROM bcipipelineedge AS e
JOIN bcipipelinestage AS s1 ON s1.stageid = e.fromstageid
JOIN bcipipelinestage AS s2 ON s2.stageid = e.tostageid
WHERE s1.repo = :repoName
ORDER BY from_repo, from_stage, to_repo, to_stage;

-- pipelineForInputType
-- Input :inputType
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
-- Input :fieldPath
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

------------------------------------------------------------
-- 3. Research conditions: domain coverage
------------------------------------------------------------

-- researchReadyDomainSchemas
-- Input :domainName
SELECT
    :domainName AS domain,
    SUM(
        CASE
            WHEN c.kind = 'schema'
             AND c.path LIKE 'schemas/%request%' THEN 1
            ELSE 0
        END
    ) AS request_schemas,
    SUM(
        CASE
            WHEN c.kind = 'schema'
             AND c.path LIKE 'schemas/%response%' THEN 1
            ELSE 0
        END
    ) AS response_schemas,
    SUM(
        CASE
            WHEN c.kind = 'sqlschema'
             AND c.path LIKE '%monstermode%' THEN 1
            ELSE 0
        END
    ) AS runtime_sql_schemas,
    SUM(
        CASE
            WHEN c.kind = 'sqlschema'
             AND c.path LIKE '%bci-pipeline%' THEN 1
            ELSE 0
        END
    ) AS pipeline_sql_schemas
FROM hp_repo AS r
JOIN hp_component AS c ON c.repo_id = r.repo_id
WHERE c.domain = :domainName;

-- researchReadyField
-- Input :fieldPath
SELECT
    :fieldPath AS fieldpath,
    SUM(
        CASE
            WHEN locationtype = 'jsonschema' THEN 1
            ELSE 0
        END
    ) AS jsonschema_count,
    SUM(
        CASE
            WHEN locationtype = 'sqltable' THEN 1
            ELSE 0
        END
    ) AS sqltable_count,
    SUM(
        CASE
            WHEN locationtype IN ('ruststruct', 'cppstruct', 'shader') THEN 1
            ELSE 0
        END
    ) AS code_count
FROM fieldusage
WHERE fieldpath = :fieldPath;

-- researchReadyPipelinePath
-- Input :startInputType, :endLayer
SELECT
    COUNT(*) AS candidate_edges
FROM bcipipelineedge AS e
JOIN bcipipelinestage AS s1 ON s1.stageid = e.fromstageid
JOIN bcipipelinestage AS s2 ON s2.stageid = e.tostageid
WHERE s1.inputtype = :startInputType
  AND s2.layer     = :endLayer;

------------------------------------------------------------
-- 4. Task-specific readiness checks
------------------------------------------------------------

-- researchReady_MonsterModeBCI
SELECT
    SUM(
        CASE
            WHEN c.kind = 'schema'
             AND c.path = 'schemas/ai-bci-geometry-request-v1.json' THEN 1
            ELSE 0
        END
    ) AS has_request,
    SUM(
        CASE
            WHEN c.kind = 'schema'
             AND c.path = 'schemas/ai-bci-geometry-response-v1.json' THEN 1
            ELSE 0
        END
    ) AS has_response,
    SUM(
        CASE
            WHEN c.kind = 'schema'
             AND c.path = 'schemas/bci-geometry-binding-v1.json' THEN 1
            ELSE 0
        END
    ) AS has_binding,
    SUM(
        CASE
            WHEN c.kind = 'sqlschema'
             AND c.path LIKE '%rottingvisuals-monstermode.sql' THEN 1
            ELSE 0
        END
    ) AS has_runtime_sql,
    SUM(
        CASE
            WHEN c.kind = 'sqlschema'
             AND c.path LIKE '%constellation-ontology.sql' THEN 1
            ELSE 0
        END
    ) AS has_palette_ontology
FROM hp_repo AS r
JOIN hp_component AS c ON c.repo_id = r.repo_id
WHERE r.name = 'Rotting-Visuals-BCI';

-- researchReady_CANRegistryRotViz
SELECT
    SUM(
        CASE
            WHEN c.kind = 'schema'
             AND c.path LIKE 'schemas/can-token-registry-rot-visuals-v1.json' THEN 1
            ELSE 0
        END
    ) AS has_can_schema,
    SUM(
        CASE
            WHEN c.kind = 'sqlschema'
             AND c.path LIKE '%cantokens.sql' THEN 1
            ELSE 0
        END
    ) AS has_can_sql,
    SUM(
        CASE
            WHEN c.domain = 'bci'
             AND c.kind = 'rustmodule'
             AND c.path LIKE '%canregistry%' THEN 1
            ELSE 0
        END
    ) AS has_can_rust
FROM hp_repo AS r
JOIN hp_component AS c ON c.repo_id = r.repo_id;
