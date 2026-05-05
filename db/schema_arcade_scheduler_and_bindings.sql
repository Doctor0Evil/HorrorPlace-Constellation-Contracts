-- File: db/schema_arcade_scheduler_and_bindings.sql
-- Target repo: HorrorPlace-Constellation-Contracts
-- Purpose: Scheduler weights, BCI binding logs, and analysis views for arcade pods.

PRAGMA foreign_keys = ON;

------------------------------------------------------------
-- 1. Arcade scheduler weights
------------------------------------------------------------

CREATE TABLE IF NOT EXISTS arcade_scheduler_weights (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    scope_type  TEXT NOT NULL CHECK (scope_type IN ('default', 'room', 'pod')),
    scope_id    TEXT,               -- NULL for default, room_id for 'room', pod_id for 'pod'
    floor_id    TEXT,               -- optional: "B1", "L2", etc.
    region_tag  TEXT,               -- optional: "necrotic-vignette-hall", "corpsebloom-altars"
    w_energy    REAL NOT NULL,
    w_quality   REAL NOT NULL,
    w_safety    REAL NOT NULL,
    created_ts  INTEGER NOT NULL DEFAULT (strftime('%s','now')),
    updated_ts  INTEGER NOT NULL DEFAULT (strftime('%s','now')),
    UNIQUE (scope_type, scope_id)
);

CREATE INDEX IF NOT EXISTS idx_arcade_weights_scope
    ON arcade_scheduler_weights (scope_type, scope_id);

CREATE INDEX IF NOT EXISTS idx_arcade_weights_floor_region
    ON arcade_scheduler_weights (floor_id, region_tag);

CREATE TRIGGER IF NOT EXISTS trg_arcade_weights_updated_ts
AFTER UPDATE ON arcade_scheduler_weights
FOR EACH ROW
BEGIN
    UPDATE arcade_scheduler_weights
    SET updated_ts = strftime('%s','now')
    WHERE id = OLD.id;
END;

------------------------------------------------------------
-- 1.1 Seed default weights
------------------------------------------------------------

INSERT OR IGNORE INTO arcade_scheduler_weights
    (scope_type, scope_id, w_energy, w_quality, w_safety)
VALUES
    ('default', NULL, 0.40, 0.50, 0.10);

------------------------------------------------------------
-- 1.2 Seed room-level weights
------------------------------------------------------------

INSERT OR IGNORE INTO arcade_scheduler_weights
    (scope_type, scope_id, floor_id, region_tag, w_energy, w_quality, w_safety)
VALUES
    ('room', 'phoenix_room_01', 'B1', 'necrotic-vignette-hall', 0.55, 0.35, 0.10),
    ('room', 'lab_room_99',     'L2', 'corpsebloom-lounge',     0.30, 0.60, 0.10);

------------------------------------------------------------
-- 1.3 Seed pod-level weights
------------------------------------------------------------

INSERT OR IGNORE INTO arcade_scheduler_weights
    (scope_type, scope_id, floor_id, region_tag, w_energy, w_quality, w_safety)
VALUES
    ('pod', 'pod_arcade_low_01',  'B1', 'necrotic-vignette-hall', 0.70, 0.25, 0.05),
    ('pod', 'pod_arcade_high_01', 'L2', 'corpsebloom-altars',     0.30, 0.65, 0.05);

------------------------------------------------------------
-- 1.4 Example admin nudge for a single pod
------------------------------------------------------------

-- Adjusts one pod's weights during a test session.
INSERT INTO arcade_scheduler_weights (scope_type, scope_id, floor_id, region_tag, w_energy, w_quality, w_safety)
VALUES ('pod', 'pod_arcade_low_01', 'B1', 'necrotic-vignette-hall', 0.35, 0.60, 0.05)
ON CONFLICT(scope_type, scope_id) DO UPDATE SET
    w_energy  = excluded.w_energy,
    w_quality = excluded.w_quality,
    w_safety  = excluded.w_safety;

------------------------------------------------------------
-- 2. BCI binding geometry log
------------------------------------------------------------

-- Assumes bcirequestframe(frameid, pod_id, floor_id, region_tag, timestamp_ms, ...) exists.

CREATE TABLE IF NOT EXISTS bcibindinggeometry (
    bindingid      INTEGER PRIMARY KEY AUTOINCREMENT,
    frameid        INTEGER NOT NULL REFERENCES bcirequestframe(frameid) ON DELETE CASCADE,
    bindingkey     TEXT NOT NULL,   -- e.g. "monster-zombie-vomit"
    region         TEXT NOT NULL,   -- e.g. "foveal-lower", "peripheral-ring"
    patternname    TEXT NOT NULL,   -- e.g. "necrotic-vignette", "corpsebloom"
    patternid      INTEGER,         -- normalized PatternId if available
    maskradius     REAL NOT NULL,
    maskfeather    REAL NOT NULL,
    decaygrain     REAL NOT NULL,
    colordesat     REAL NOT NULL,
    veinoverlay    REAL NOT NULL,
    motionsmear    REAL NOT NULL,
    infectedgain   REAL NOT NULL,
    squadmuffle    REAL NOT NULL,
    heartbeatgain  REAL NOT NULL,
    breathgain     REAL NOT NULL,
    ringinglevel   REAL NOT NULL,
    directlevel    REAL NOT NULL,
    created_ts     INTEGER NOT NULL DEFAULT (strftime('%s','now'))
);

CREATE INDEX IF NOT EXISTS idx_bcibindinggeometry_frame
    ON bcibindinggeometry (frameid);

CREATE INDEX IF NOT EXISTS idx_bcibindinggeometry_pattern
    ON bcibindinggeometry (patternname);

CREATE INDEX IF NOT EXISTS idx_bcibindinggeometry_patternid
    ON bcibindinggeometry (patternid);

CREATE INDEX IF NOT EXISTS idx_bcibindinggeometry_region
    ON bcibindinggeometry (region);

------------------------------------------------------------
-- 3. Analysis views
------------------------------------------------------------

-- View: pattern usage counts per pod in a time window.
CREATE VIEW IF NOT EXISTS v_pattern_usage_per_pod AS
SELECT
    rf.pod_id            AS pod_id,
    bg.patternname       AS patternname,
    COUNT(*)             AS uses,
    MIN(rf.timestamp_ms) AS first_ts,
    MAX(rf.timestamp_ms) AS last_ts
FROM bcirequestframe AS rf
JOIN bcibindinggeometry AS bg
  ON bg.frameid = rf.frameid
GROUP BY rf.pod_id, bg.patternname;

-- Example query with filters:
-- SELECT * FROM v_pattern_usage_per_pod
-- WHERE pod_id = :pod_id
--   AND first_ts >= :start_ms
--   AND last_ts  <= :end_ms
-- ORDER BY uses DESC;

-- View: pattern mix per region_tag and floor.
CREATE VIEW IF NOT EXISTS v_pattern_mix_per_region AS
SELECT
    rf.floor_id     AS floor_id,
    rf.region_tag   AS region_tag,
    bg.patternname  AS patternname,
    COUNT(*)        AS uses
FROM bcirequestframe AS rf
JOIN bcibindinggeometry AS bg
  ON bg.frameid = rf.frameid
GROUP BY rf.floor_id, rf.region_tag, bg.patternname;

-- Example filtered usage:
-- SELECT * FROM v_pattern_mix_per_region
-- WHERE floor_id = 'B1'
--   AND region_tag = 'corpsebloom-altars'
-- ORDER BY uses DESC;
