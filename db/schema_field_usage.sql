-- File: db/schema_field_usage.sql
-- Target: Horror$Place constellation (multi-repo index)
-- Purpose: map logical fields (BciSummary, invariants, geometry params, palette roles, etc.)
--          to their usage locations in schemas, SQL, and code.

PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS field_usage (
    usage_id        INTEGER PRIMARY KEY AUTOINCREMENT,

    -- Logical field identifier, stable across repos and languages.
    field_path      TEXT NOT NULL, 
    -- Examples: 'bciSummary.stressScore',
    --           'invariants.CIC',
    --           'visual.maskRadius',
    --           'audio.ringingLevel',
    --           'palette.swatchIndex',
    --           'can.token.max_gain'.

    -- Where this usage lives.
    repo            TEXT NOT NULL,   -- 'Rotting-Visuals-BCI', 'HorrorPlace-Dead-Ledger-Network', etc.
    location_type   TEXT NOT NULL,   -- 'json_schema', 'sql_table', 'rust_struct', 'cpp_struct', 'shader', 'doc'.
    location_path   TEXT NOT NULL,   -- Relative path, e.g. 'schemas/ai-bci-geometry-request-v1.json'.

    -- Extra structural context, kept small for low token cost.
    container_name  TEXT,            -- e.g. 'AiBciGeometryRequestV1', 'bcirequestframe', 'VisualParams'.
    container_field TEXT,            -- e.g. 'stressscore', 'maskradius', 'CIC'.
    role            TEXT,            -- 'input', 'output', 'invariant_gate', 'derived_param', 'log_column', etc.

    -- Brief machine-readable note, ≤ 160 chars.
    note            TEXT
);

CREATE INDEX IF NOT EXISTS idx_field_usage_field
    ON field_usage (field_path);

CREATE INDEX IF NOT EXISTS idx_field_usage_repo
    ON field_usage (repo, location_type);

CREATE INDEX IF NOT EXISTS idx_field_usage_location
    ON field_usage (location_path);
