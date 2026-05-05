-- File: db/schema_bci_pipeline.sql
-- Target: Horror$Place constellation (multi-repo)
-- Purpose: model BCI processing pipeline as a directed graph of stages and edges.

PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS bci_pipeline_stage (
    stage_id        INTEGER PRIMARY KEY AUTOINCREMENT,

    repo            TEXT NOT NULL,   -- 'Rotting-Visuals-BCI', 'HorrorPlace-Dead-Ledger-Network', etc.
    stage_key       TEXT NOT NULL,   -- short stable key, e.g. 'rvbci_request_in', 'rvviz_geometry_compute'.
    name            TEXT NOT NULL,   -- human label, e.g. 'Rotting-Visuals Geometry Compute'.
    layer           TEXT NOT NULL,   -- 'ingest', 'compute', 'render', 'log', 'persistence', 'network', etc.

    -- Data contracts at this stage.
    input_type      TEXT NOT NULL,   -- logical input type, e.g. 'ai-bci-geometry-request-v1', 'BciSummary+Invariants'.
    output_type     TEXT NOT NULL,   -- e.g. 'AiBciGeometryResponseV1', 'BindingRows', 'LedgerEvent'.

    -- File hints for static analysis.
    primary_file    TEXT NOT NULL,   -- main source file or schema, e.g. 'src/monstermode/mod.rs'.
    aux_files       TEXT,            -- optional, comma-separated or small JSON string of helpers.

    -- Short note to keep token cost low.
    note            TEXT
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_bci_stage_key
    ON bci_pipeline_stage (repo, stage_key);

CREATE TABLE IF NOT EXISTS bci_pipeline_edge (
    edge_id         INTEGER PRIMARY KEY AUTOINCREMENT,

    from_stage_id   INTEGER NOT NULL REFERENCES bci_pipeline_stage(stage_id) ON DELETE CASCADE,
    to_stage_id     INTEGER NOT NULL REFERENCES bci_pipeline_stage(stage_id) ON DELETE CASCADE,

    protocol        TEXT NOT NULL,   -- 'in-process', 'sqlite', 'http', 'ipc', 'file'.
    description     TEXT             -- short note, ≤ 160 chars.
);

CREATE INDEX IF NOT EXISTS idx_bci_edge_from_to
    ON bci_pipeline_edge (from_stage_id, to_stage_id);
