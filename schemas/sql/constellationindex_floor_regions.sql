-- Target DB: constellationindex.db

PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS floor_region_anchors (
    anchor_id     INTEGER PRIMARY KEY AUTOINCREMENT,
    floor_id      TEXT NOT NULL,      -- e.g. "B1"
    region_id     TEXT NOT NULL,      -- e.g. "hallway", "altar", "lounge"
    region_tag    TEXT NOT NULL,      -- e.g. "necrotic-hall", "corpsebloom-altar"
    t             REAL NOT NULL,      -- normalized position [0, 1] along floor path

    -- Scheduler weight seeds (expected bias at anchor)
    w_energy      REAL NOT NULL,
    w_quality     REAL NOT NULL,
    w_safety      REAL NOT NULL,

    -- Optional descriptive metrics for analytics / AI-chat
    horror_bias   TEXT,               -- e.g. "energy-heavy-necrotic", "quality-heavy-corpsebloom"
    notes         TEXT,               -- free-form description

    UNIQUE(floor_id, region_id)
);

-- Example anchors for floor B1 (hallway -> altar -> lounge)
INSERT OR IGNORE INTO floor_region_anchors
    (floor_id, region_id, region_tag, t, w_energy, w_quality, w_safety, horror_bias, notes)
VALUES
    ('B1', 'hallway', 'necrotic-hall',       0.0, 0.65, 0.25, 0.10, 'energy-heavy-necrotic',
     'Long corridor with Necrotic Vignette bias; conserve energy, sustain dread'),
    ('B1', 'altar',   'corpsebloom-altar',   0.5, 0.45, 0.45, 0.10, 'balanced-transition',
     'Central altar; mix Necrotic and CorpseBloom visuals evenly'),
    ('B1', 'lounge',  'corpsebloom-lounge',  1.0, 0.30, 0.60, 0.10, 'quality-heavy-corpsebloom',
     'Lounge zone; favor high-intensity CorpseBloom patterns');
