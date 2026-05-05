-- Target DB: constellationindex.db

PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS vein_seeds (
    vein_seed_id   INTEGER PRIMARY KEY AUTOINCREMENT,

    -- Logical identity
    palette_group  TEXT NOT NULL,   -- e.g. "Spectral-Pus"
    vein_style     TEXT NOT NULL,   -- e.g. "spectral-pus-veins-v1"

    -- Deterministic seed + parameters
    seed_value     INTEGER NOT NULL, -- 64-bit logical seed; stored as INTEGER
    params_json    TEXT,             -- optional: scale, thickness, branching, etc.

    -- Where this seed was used
    floor_id       TEXT,             -- e.g. "B1"
    region_id      TEXT,             -- e.g. "altar"
    pod_id         TEXT,             -- optional: specific arcade pod
    patternname    TEXT,             -- e.g. "corpsebloom", "hanging-skin"

    -- Telemetry / provenance
    created_ms     INTEGER NOT NULL, -- unix epoch ms when first used
    created_by     TEXT,             -- e.g. "ai-agent", "designer", "script"
    notes          TEXT
);

CREATE INDEX IF NOT EXISTS idx_vein_seeds_style_floor
    ON vein_seeds (vein_style, floor_id);

CREATE INDEX IF NOT EXISTS idx_vein_seeds_palette_floor
    ON vein_seeds (palette_group, floor_id);

CREATE INDEX IF NOT EXISTS idx_vein_seeds_pod
    ON vein_seeds (pod_id);
