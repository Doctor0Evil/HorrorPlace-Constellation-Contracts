-- File db/schema/bci_pipeline.sql
-- Target repo Doctor0Evil/HorrorPlace-Constellation-Contracts
-- Purpose SQL-based description of BCI pipeline stages and edges across HorrorPlace repos.

PRAGMA foreign_keys = ON;

------------------------------------------------------------
-- 1. BCI pipeline stages
------------------------------------------------------------

CREATE TABLE IF NOT EXISTS bci_pipeline_stage (
    stageid            INTEGER PRIMARY KEY AUTOINCREMENT,

    -- Which repo and logical stage this row describes.
    repo               TEXT NOT NULL,      -- e.g. 'Rotting-Visuals-BCI'
    stagekey           TEXT NOT NULL,      -- e.g. 'bci-validate', 'geometry-compute'
    name               TEXT NOT NULL,      -- human-readable name
    layer              TEXT NOT NULL,      -- e.g. 'ingest','compute','theatre','ledger','analysis'

    -- Logical input/output types flowing through this stage.
    inputtype          TEXT NOT NULL,      -- e.g. 'ai-bci-geometry-request-v1', 'BciSummary'
    outputtype         TEXT NOT NULL,      -- e.g. 'AiBciGeometryResponseV1', 'BciGeometryBindingV1'

    -- Primary file for humans/agents to inspect.
    primaryfile        TEXT NOT NULL,      -- repo-relative path to main implementation file
    auxfiles           TEXT,              -- optional comma-separated list of additional files

    -- Latency and performance expectations.
    latency_budget_ms  REAL,              -- target max per-call latency
    jitter_budget_ms   REAL,              -- acceptable jitter window
    max_qps            REAL,              -- expected or tested maximum QPS

    -- Safety flags and failure behavior.
    critical_safety    INTEGER NOT NULL DEFAULT 0 CHECK (critical_safety IN (0,1)),
    failure_mode       TEXT,              -- 'fail_open','fail_closed','degrade','buffer_and_retry', etc.

    -- Security / trust-zone metadata.
    trust_zone         TEXT,              -- 'core','edge','external','lab-only', etc.
    handles_can_tokens INTEGER NOT NULL DEFAULT 0 CHECK (handles_can_tokens IN (0,1)),

    -- Optional references to code-level functions and signatures.
    primary_fn         TEXT,              -- e.g. 'crate::bci::pipeline::process_geometry'
    input_signature    TEXT,              -- e.g. 'BciSummary, Invariants, PipelineConfig'
    output_signature   TEXT               -- e.g. 'PipelineOutput'
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_bci_stage_repo_key
    ON bci_pipeline_stage (repo, stagekey);

CREATE INDEX IF NOT EXISTS idx_bci_stage_layer
    ON bci_pipeline_stage (layer);

CREATE INDEX IF NOT EXISTS idx_bci_stage_io
    ON bci_pipeline_stage (inputtype, outputtype);

CREATE INDEX IF NOT EXISTS idx_bci_stage_safety
    ON bci_pipeline_stage (critical_safety, handles_can_tokens);

CREATE INDEX IF NOT EXISTS idx_bci_stage_trust_zone
    ON bci_pipeline_stage (trust_zone);

------------------------------------------------------------
-- 2. BCI pipeline edges (wiring between stages)
------------------------------------------------------------

CREATE TABLE IF NOT EXISTS bci_pipeline_edge (
    edgeid              INTEGER PRIMARY KEY AUTOINCREMENT,

    fromstageid         INTEGER NOT NULL REFERENCES bci_pipeline_stage(stageid) ON DELETE CASCADE,
    tostageid           INTEGER NOT NULL REFERENCES bci_pipeline_stage(stageid) ON DELETE CASCADE,

    -- High-level transport classification. Use a controlled vocabulary.
    protocol_type       TEXT NOT NULL,    -- 'in-process','ipc-local','network','sql','file','embedded'
    protocol_detail     TEXT,             -- optional detail, e.g. 'unix_socket','http-json','sqlite-local-file'

    -- Short description of what this edge represents.
    description         TEXT NOT NULL,

    -- Security / trust-boundary metadata.
    crosses_trust_boundary INTEGER NOT NULL DEFAULT 0 CHECK (crosses_trust_boundary IN (0,1))
);

CREATE INDEX IF NOT EXISTS idx_bci_edge_from
    ON bci_pipeline_edge (fromstageid);

CREATE INDEX IF NOT EXISTS idx_bci_edge_to
    ON bci_pipeline_edge (tostageid);

CREATE INDEX IF NOT EXISTS idx_bci_edge_protocol
    ON bci_pipeline_edge (protocol_type);

CREATE INDEX IF NOT EXISTS idx_bci_edge_trust
    ON bci_pipeline_edge (crosses_trust_boundary);

------------------------------------------------------------
-- 3. Optional: protocol_type constraint (for new deployments)
------------------------------------------------------------

-- For fresh databases you can enforce the protocol_type enum directly.
-- For existing deployments, consider applying this as part of a migration
-- once all rows conform.

-- ALTER TABLE bci_pipeline_edge
--     ADD CONSTRAINT chk_bci_edge_protocol_type
--     CHECK (protocol_type IN ('in-process','ipc-local','network','sql','file','embedded'));
