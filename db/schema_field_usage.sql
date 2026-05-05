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

INSERT INTO field_usage (
    field_path, repo, location_type, location_path,
    container_name, container_field, role, note
) VALUES

  -- BciSummary fields, JSON schema + Rust struct + SQL log column.
('bciSummary.stressScore', 'Rotting-Visuals-BCI', 'json_schema',
 'schemas/ai-bci-geometry-request-v1.json',
 'ai-bci-geometry-request-v1', 'stressScore', 'input',
 'Normalized 0-1 scalar stress, primary horror driver'),

('bciSummary.stressScore', 'Rotting-Visuals-BCI', 'rust_struct',
 'src/monstermode/mod.rs',
 'BciSummary', 'stressScore', 'input',
 'Runtime struct field, mirrors JSON schema'),

('bciSummary.stressScore', 'Rotting-Visuals-BCI', 'sql_table',
 'db/schema_rottingvisuals_monstermode.sql',
 'bcirequestframe', 'stressscore', 'log_column',
 'Logged per frame for offline analysis'),

-- Visual geometry parameter.
('visual.maskRadius', 'Rotting-Visuals-BCI', 'json_schema',
 'schemas/bci-geometry-binding-v1.json',
 'bci-geometry-binding-v1', 'maskRadius', 'output',
 'Shrinking mask radius, convex combination of stress and overload'),

('visual.maskRadius', 'Rotting-Visuals-BCI', 'sql_table',
 'db/schema_rottingvisuals_monstermode.sql',
 'bcibindinggeometry', 'maskradius', 'log_column',
 'Logged per binding for reconstruction of Rotting-Visuals geometry'),

('visual.maskRadius', 'Rotting-Visuals-BCI', 'rust_struct',
 'src/monstermode/mod.rs',
 'VisualParams', 'maskRadius', 'derived_param',
 'Computed from BciSummary and Invariants, used for shaders'),

-- Invariant gate mapping.
('invariants.CIC', 'Rotting-Visuals-BCI', 'json_schema',
 'schemas/ai-bci-geometry-request-v1.json',
 'ai-bci-geometry-request-v1', 'CIC', 'invariant_gate',
 'Constellation Integrity Coefficient, 0-1 safety factor'),

('invariants.CIC', 'Rotting-Visuals-BCI', 'rust_struct',
 'src/monstermode/mod.rs',
 'Invariants', 'CIC', 'invariant_gate',
 'Used to dampen extreme visual and audio effects'),

-- CAN registry-specific field.
('can.token.max_gain', 'Rotting-Visuals-BCI', 'json_schema',
 'schemas/can-token-registry-rot-visuals-v1.json',
 'ai-can-token-registry-v1', 'max_effective_gain', 'safety_limit',
 'Upper bound for CAN-driven parameter modulation'),

('can.token.max_gain', 'Rotting-Visuals-BCI', 'sql_table',
 'db/schema_can_tokens.sql',
 'can_token', 'max_gain', 'safety_limit',
 'Mirrors JSON max_effective_gain for queryable audits');

SELECT repo, location_type, location_path, container_name, container_field, role
FROM field_usage
WHERE field_path = 'bciSummary.visualOverloadIndex';
