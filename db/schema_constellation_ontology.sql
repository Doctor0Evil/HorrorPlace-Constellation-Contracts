-- File: db/schema_constellation_ontology.sql
-- Target: Horror$Place Constellation (palette + swatch ontology)
-- Mirrors: swatch-index ontology, palette registry, palette meta (valence/arousal).

PRAGMA foreign_keys = ON;

----------------------------------------------------------------------
-- 1. Swatch index semantic role ontology
--    Global: applies Horror$Place-wide, independent of any style pack.
----------------------------------------------------------------------

CREATE TABLE IF NOT EXISTS swatch_index_role (
    role_id         TEXT PRIMARY KEY,     -- 'deepbackground', 'midaccent', 'overloadflash', etc.
    role_index      INTEGER NOT NULL,     -- e.g. 0..7 for 8-slot palettes.
    title           TEXT NOT NULL,        -- Short label, e.g. 'Deep Background'.
    description     TEXT NOT NULL         -- ≤ 256 chars, semantic usage description.
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_swatch_index_role_index
    ON swatch_index_role (role_index);

----------------------------------------------------------------------
-- 2. Palette groups registry
--    Style-agnostic groups like 'colddecay', 'blooddry', 'gangrenegreen'.
----------------------------------------------------------------------

CREATE TABLE IF NOT EXISTS palette_group (
    group_id        TEXT PRIMARY KEY,     -- 'colddecay', 'blooddry', 'gangrenegreen', 'uiwarning', etc.
    version         TEXT NOT NULL,        -- 'v1', 'v1-rotviz', etc.
    style_pack      TEXT NOT NULL,        -- 'Rotting-Visuals', 'Fungal-Bloom', etc.
    description     TEXT NOT NULL,        -- ≤ 256 chars.
    tags            TEXT,                 -- comma-separated tags, e.g. 'decay,lowvalence,mediumarousal'.

    -- Affective metadata as in palettemeta.json.
    valence         REAL,                 -- 0..1, higher = more pleasant/safe.
    arousal         REAL,                 -- 0..1, higher = more intense/exciting.

    -- Optional notes for AI-agents; keep concise to limit token cost.
    note            TEXT
);

CREATE INDEX IF NOT EXISTS idx_palette_group_style
    ON palette_group (style_pack);

CREATE INDEX IF NOT EXISTS idx_palette_group_affect
    ON palette_group (valence, arousal);

----------------------------------------------------------------------
-- 3. Palette swatches (hex-coded colors per group + semantic role).
--    This is the SQL mirror of your palette registry JSON. [file:3]
----------------------------------------------------------------------

CREATE TABLE IF NOT EXISTS palette_swatch (
    group_id        TEXT NOT NULL REFERENCES palette_group(group_id) ON DELETE CASCADE,
    swatch_index    INTEGER NOT NULL,     -- 0..N-1 within the group.
    hex_value       TEXT NOT NULL,        -- '1A0000', '3B0C0C', etc.
    role_id         TEXT NOT NULL REFERENCES swatch_index_role(role_id),
                                            -- semantic role e.g. 'midaccent', 'overloadflash'.
    description     TEXT,                 -- short usage note; keep ≤ 160 chars.

    PRIMARY KEY (group_id, swatch_index)
);

CREATE INDEX IF NOT EXISTS idx_palette_swatch_role
    ON palette_swatch (role_id);

CREATE INDEX IF NOT EXISTS idx_palette_swatch_hex
    ON palette_swatch (hex_value);

----------------------------------------------------------------------
-- 4. Optional: ontology ↔ BCI mapping hints
--    This ties swatch roles and palettes to typical BCI states, matching your text. [file:3]
----------------------------------------------------------------------

CREATE TABLE IF NOT EXISTS swatch_bci_hint (
    hint_id         INTEGER PRIMARY KEY AUTOINCREMENT,

    group_id        TEXT NOT NULL REFERENCES palette_group(group_id) ON DELETE CASCADE,
    role_id         TEXT NOT NULL REFERENCES swatch_index_role(role_id),

    -- Recommended BCI ranges that favor this group+role.
    stress_min      REAL,   -- 0..1, recommended lower bound of stressScore.
    stress_max      REAL,   -- 0..1, recommended upper bound of stressScore.
    overload_min    REAL,   -- 0..1, for visualOverloadIndex.
    overload_max    REAL,   -- 0..1.
    attention_band  TEXT,   -- 'drifting','focused','locked', or NULL for 'any'.

    -- Optional entertainment hints.
    UEC_min         REAL,   -- 0..1, engagement lower bound.
    UEC_max         REAL,   -- 0..1.

    note            TEXT    -- short explanation, e.g. 'Peak horror flash at high stress, high overload.'
);

CREATE INDEX IF NOT EXISTS idx_swatch_bci_hint_group_role
    ON swatch_bci_hint (group_id, role_id);

----------------------------------------------------------------------
-- 5. Optional: analysis/logging helper table for offline studies
--    Minimal hook; your main color usage log can live elsewhere. [file:3]
----------------------------------------------------------------------

CREATE TABLE IF NOT EXISTS palette_usage_summary (
    summary_id      INTEGER PRIMARY KEY AUTOINCREMENT,

    group_id        TEXT NOT NULL REFERENCES palette_group(group_id) ON DELETE CASCADE,
    role_id         TEXT NOT NULL REFERENCES swatch_index_role(role_id),

    -- Aggregate statistics from runtime logs.
    uses_count      INTEGER NOT NULL,     -- total uses across frames.
    avg_stress      REAL,                 -- mean stressScore when this role was used.
    avg_overload    REAL,                 -- mean visualOverloadIndex.
    avg_valence     REAL,                 -- derived from palettemeta or user ratings if available.
    avg_arousal     REAL                  -- ditto.
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_palette_usage_group_role
    ON palette_usage_summary (group_id, role_id);
