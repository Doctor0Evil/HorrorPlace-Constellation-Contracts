-- Constellation SQLite Schema v1
-- Logical schema for output/constellation.db

PRAGMA foreign_keys = ON;

----------------------------------------------------------------------
-- 1. Repositories and manifests
----------------------------------------------------------------------

CREATE TABLE repos (
    id          INTEGER PRIMARY KEY,
    name        TEXT    NOT NULL UNIQUE,
    visibility  TEXT    NOT NULL,         -- public | private
    tier        INTEGER NOT NULL,         -- 1 | 2 | 3
    git_url     TEXT    NOT NULL,
    role        TEXT    NOT NULL,
    created_at  TEXT    NOT NULL,         -- ISO8601
    updated_at  TEXT    NOT NULL          -- ISO8601
);

CREATE TABLE repo_manifests (
    id                   INTEGER PRIMARY KEY,
    repo_id              INTEGER NOT NULL REFERENCES repos(id),
    manifest_path        TEXT    NOT NULL,
    schema_ref           TEXT    NOT NULL,
    raw_json             TEXT,
    ai_authoring_rules   TEXT,
    one_file_per_request INTEGER NOT NULL,    -- 0 | 1
    require_deadledger_ref INTEGER NOT NULL   -- 0 | 1
);

CREATE TABLE routing_rules (
    id                  INTEGER PRIMARY KEY,
    repo_id             INTEGER NOT NULL REFERENCES repos(id),
    object_kind         TEXT    NOT NULL,
    tier                TEXT    NOT NULL,     -- e.g. T1-core, T2-vault, T3-lab
    schema_ref          TEXT    NOT NULL,
    default_path_pattern TEXT   NOT NULL,
    policy_notes        TEXT
);

CREATE INDEX idx_routing_rules_kind_tier
    ON routing_rules (object_kind, tier);

----------------------------------------------------------------------
-- 2. Schema spine and consumers
----------------------------------------------------------------------

CREATE TABLE schemas (
    id                            INTEGER PRIMARY KEY,
    schema_ref                    TEXT    NOT NULL UNIQUE,
    family                        TEXT    NOT NULL,    -- invariant | metric | contract | tooling | telemetry | registry
    title                         TEXT    NOT NULL,
    version                       TEXT    NOT NULL,
    repo_id                       INTEGER NOT NULL REFERENCES repos(id),
    path                          TEXT    NOT NULL,
    draft                         TEXT    NOT NULL,    -- JSON Schema draft version
    additional_properties_forbidden INTEGER NOT NULL   -- 0 | 1
);

CREATE TABLE schema_fields (
    id          INTEGER PRIMARY KEY,
    schema_id   INTEGER NOT NULL REFERENCES schemas(id),
    field_path  TEXT    NOT NULL,       -- JSON Pointer
    field_name  TEXT    NOT NULL,
    field_type  TEXT    NOT NULL,       -- number | string | object | array | boolean
    min_value   REAL,
    max_value   REAL,
    enum_values TEXT                    -- JSON-encoded array, nullable
);

CREATE INDEX idx_schema_fields_schema
    ON schema_fields (schema_id);

CREATE TABLE schema_consumers (
    id          INTEGER PRIMARY KEY,
    schema_id   INTEGER NOT NULL REFERENCES schemas(id),
    repo_id     INTEGER NOT NULL REFERENCES repos(id),
    file_path   TEXT    NOT NULL,
    object_kind TEXT,
    usage_kind  TEXT    NOT NULL        -- contract | registry | code | test | telemetry
);

CREATE INDEX idx_schema_consumers_schema
    ON schema_consumers (schema_id);

----------------------------------------------------------------------
-- 3. Invariants and metrics
----------------------------------------------------------------------

CREATE TABLE invariants (
    code        TEXT PRIMARY KEY,       -- CIC, AOS, DET, etc.
    name        TEXT NOT NULL,
    description TEXT,
    min_value   REAL NOT NULL,
    max_value   REAL NOT NULL,
    derived     INTEGER NOT NULL        -- 0 | 1
);

CREATE TABLE metrics (
    code        TEXT PRIMARY KEY,       -- UEC, EMD, STCI, CDL, ARR
    name        TEXT NOT NULL,
    description TEXT,
    min_value   REAL NOT NULL,
    max_value   REAL NOT NULL
);

CREATE TABLE invariant_relations (
    id             INTEGER PRIMARY KEY,
    invariant_code TEXT NOT NULL REFERENCES invariants(code),
    depends_on     TEXT,                -- JSON array of invariant codes
    rule_type      TEXT NOT NULL,       -- formula | band_rule | other
    rule_json      TEXT NOT NULL        -- JSON-encoded rule spec
);

CREATE INDEX idx_invariant_relations_code
    ON invariant_relations (invariant_code);

CREATE TABLE metric_recommendations (
    id              INTEGER PRIMARY KEY,
    metric_code     TEXT NOT NULL REFERENCES metrics(code),
    object_kind     TEXT NOT NULL,
    tier            TEXT NOT NULL,
    recommended_min REAL NOT NULL,
    recommended_max REAL NOT NULL
);

CREATE INDEX idx_metric_recommendations_metric
    ON metric_recommendations (metric_code);

----------------------------------------------------------------------
-- 4. Registries and entries
----------------------------------------------------------------------

CREATE TABLE registries (
    id         INTEGER PRIMARY KEY,
    repo_id    INTEGER NOT NULL REFERENCES repos(id),
    name       TEXT    NOT NULL,    -- regions | events | personas | styles | seeds | ...
    schema_ref TEXT    NOT NULL,
    path       TEXT    NOT NULL
);

CREATE TABLE registry_entries (
    id           INTEGER PRIMARY KEY,
    registry_id  INTEGER NOT NULL REFERENCES registries(id),
    entry_id     TEXT    NOT NULL,  -- canonical ID from NDJSON
    schemaref    TEXT    NOT NULL,
    tier         TEXT    NOT NULL,
    deadledger_ref TEXT,
    payload_ref  TEXT,
    raw_line     TEXT
);

CREATE INDEX idx_registry_entries_registry
    ON registry_entries (registry_id);

CREATE INDEX idx_registry_entries_entry_id
    ON registry_entries (entry_id);

CREATE TABLE registry_entry_invariants (
    id             INTEGER PRIMARY KEY,
    entry_id       INTEGER NOT NULL REFERENCES registry_entries(id),
    invariant_code TEXT    NOT NULL REFERENCES invariants(code),
    value          REAL    NOT NULL
);

CREATE INDEX idx_reg_entry_inv_entry
    ON registry_entry_invariants (entry_id);

CREATE TABLE registry_entry_metrics (
    id          INTEGER PRIMARY KEY,
    entry_id    INTEGER NOT NULL REFERENCES registry_entries(id),
    metric_code TEXT    NOT NULL REFERENCES metrics(code),
    min_value   REAL    NOT NULL,
    max_value   REAL    NOT NULL
);

CREATE INDEX idx_reg_entry_metrics_entry
    ON registry_entry_metrics (entry_id);

----------------------------------------------------------------------
-- 5. Agents, profiles, prisms
----------------------------------------------------------------------

CREATE TABLE agents (
    id           INTEGER PRIMARY KEY,
    agent_id     TEXT NOT NULL UNIQUE,   -- logical id, matches prismMeta.agentId
    display_name TEXT NOT NULL,
    kind         TEXT NOT NULL           -- generator | refactor | auditor | orchestrator | other
);

CREATE TABLE agent_profiles (
    id                    INTEGER PRIMARY KEY,
    agent_id              INTEGER NOT NULL REFERENCES agents(id),
    profile_id            TEXT    NOT NULL UNIQUE,
    schema_version        TEXT    NOT NULL,
    allowed_tiers         TEXT    NOT NULL,   -- JSON-encoded array
    allowed_object_kinds  TEXT    NOT NULL,   -- JSON-encoded array
    structural_limits_json TEXT   NOT NULL,   -- JSON
    invariant_caps_json   TEXT    NOT NULL,   -- JSON
    metric_bands_json     TEXT    NOT NULL    -- JSON
);

CREATE INDEX idx_agent_profiles_agent
    ON agent_profiles (agent_id);

CREATE TABLE prisms (
    id                          INTEGER PRIMARY KEY,
    prism_id                    TEXT    NOT NULL UNIQUE,
    schema_version              TEXT    NOT NULL,
    session_id                  TEXT    NOT NULL,
    agent_id                    INTEGER NOT NULL REFERENCES agents(id),
    agent_profile_id            INTEGER NOT NULL REFERENCES agent_profiles(id),
    target_repo_id              INTEGER NOT NULL REFERENCES repos(id),
    path                        TEXT    NOT NULL,
    tier                        TEXT    NOT NULL,
    file_hash                   TEXT    NOT NULL,
    schema_ref                  TEXT    NOT NULL,
    validation_schema_validated INTEGER NOT NULL,   -- 0 | 1
    validation_subset_validated INTEGER NOT NULL,   -- 0 | 1
    validation_ledger_validated INTEGER NOT NULL,   -- 0 | 1
    validation_clamped          INTEGER NOT NULL,   -- 0 | 1
    validation_timestamp        TEXT    NOT NULL    -- ISO8601
);

CREATE TABLE prism_dependencies (
    id            INTEGER PRIMARY KEY,
    prism_id      INTEGER NOT NULL REFERENCES prisms(id),
    dependency_id TEXT    NOT NULL,   -- region/policy/seed/proof id
    kind          TEXT    NOT NULL,   -- policy | region | seed | proof | style | other
    repo_name     TEXT,
    path          TEXT
);

CREATE INDEX idx_prism_deps_prism
    ON prism_dependencies (prism_id);

----------------------------------------------------------------------
-- 6. Files and chunks
----------------------------------------------------------------------

CREATE TABLE files (
    id            INTEGER PRIMARY KEY,
    repo_id       INTEGER NOT NULL REFERENCES repos(id),
    path          TEXT    NOT NULL,
    schema_ref    TEXT,
    object_kind   TEXT,
    language      TEXT,
    line_count    INTEGER NOT NULL,
    byte_size     INTEGER NOT NULL,
    last_modified TEXT    NOT NULL     -- ISO8601
);

CREATE INDEX idx_files_repo_path
    ON files (repo_id, path);

CREATE TABLE chunks (
    id                 INTEGER PRIMARY KEY,
    file_id            INTEGER NOT NULL REFERENCES files(id),
    chunk_id           TEXT    NOT NULL UNIQUE,
    start_line         INTEGER NOT NULL,
    end_line           INTEGER NOT NULL,
    approx_token_cost  INTEGER NOT NULL,
    chunk_kind         TEXT    NOT NULL,   -- schema | contract | runtime-lua | registry | doc | other
    invariants_used_json TEXT,
    metrics_used_json  TEXT,
    prism_refs_json    TEXT
);

CREATE INDEX idx_chunks_file
    ON chunks (file_id);
