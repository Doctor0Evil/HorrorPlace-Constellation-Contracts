-- Invariants rule matching and effective action views, plus
-- PRAGMA profiles and invariants_snapshot definition.

------------------------------------------------------------
-- 1. Table: invariants_snapshot
------------------------------------------------------------

CREATE TABLE IF NOT EXISTS invariants_snapshot (
    snapshot_id   TEXT PRIMARY KEY,
    timestamp     TEXT NOT NULL,
    region_id     TEXT NOT NULL,
    tile_id       TEXT NOT NULL,
    cic           REAL,
    mdi           REAL,
    aos           REAL,
    rrm           REAL,
    fcf           REAL,
    spr           REAL,
    rwf           REAL,
    det           REAL,
    hvf           REAL,
    lsg           REAL,
    shci          REAL,
    meta_json     TEXT CHECK (meta_json IS NULL OR length(meta_json) <= 8192)
);

CREATE INDEX IF NOT EXISTS idx_invariants_snapshot_region_tile
    ON invariants_snapshot (region_id, tile_id, timestamp);

CREATE INDEX IF NOT EXISTS idx_invariants_snapshot_timestamp
    ON invariants_snapshot (timestamp);

------------------------------------------------------------
-- 2. Table: invariant_rules
------------------------------------------------------------

CREATE TABLE IF NOT EXISTS invariant_rules (
    rule_id      INTEGER PRIMARY KEY AUTOINCREMENT,
    chain_id     TEXT NOT NULL,
    priority     INTEGER NOT NULL,
    active       INTEGER NOT NULL DEFAULT 1,
    match_region TEXT,
    match_tile   TEXT,
    min_cic      REAL,
    max_cic      REAL,
    min_det      REAL,
    max_det      REAL,
    min_lsg      REAL,
    max_lsg      REAL,
    action       TEXT NOT NULL,
    action_param TEXT
);

CREATE INDEX IF NOT EXISTS idx_invariant_rules_chain_priority
    ON invariant_rules (chain_id, priority);

CREATE INDEX IF NOT EXISTS idx_invariant_rules_match_region_tile
    ON invariant_rules (match_region, match_tile);

------------------------------------------------------------
-- 3. View: v_invariant_rule_matches
--    All rules that match each snapshot.
------------------------------------------------------------

CREATE VIEW IF NOT EXISTS v_invariant_rule_matches AS
SELECT
    s.snapshot_id,
    s.timestamp,
    s.region_id,
    s.tile_id,
    s.cic,
    s.det,
    s.lsg,
    r.rule_id,
    r.chain_id,
    r.priority,
    r.action,
    r.action_param
FROM invariants_snapshot AS s
JOIN invariant_rules AS r
    ON r.active = 1
   AND (r.match_region IS NULL OR r.match_region = s.region_id)
   AND (r.match_tile   IS NULL OR r.match_tile   = s.tile_id)
   AND (r.min_cic IS NULL OR s.cic >= r.min_cic)
   AND (r.max_cic IS NULL OR s.cic <= r.max_cic)
   AND (r.min_det IS NULL OR s.det >= r.min_det)
   AND (r.max_det IS NULL OR s.det <= r.max_det)
   AND (r.min_lsg IS NULL OR s.lsg >= r.min_lsg)
   AND (r.max_lsg IS NULL OR s.lsg <= r.max_lsg);

------------------------------------------------------------
-- 4. View: v_invariant_effective_action
--    Deterministic winner per (snapshot_id, chain_id).
--    Tie-breaker: lowest rule_id among rules with minimum priority.
------------------------------------------------------------

CREATE VIEW IF NOT EXISTS v_invariant_effective_action AS
SELECT
    m.snapshot_id,
    m.chain_id,
    m.action,
    m.action_param,
    m.priority,
    m.rule_id
FROM v_invariant_rule_matches AS m
JOIN (
    SELECT
        snapshot_id,
        chain_id,
        MIN(priority) AS min_priority
    FROM v_invariant_rule_matches
    GROUP BY snapshot_id, chain_id
) AS best
  ON best.snapshot_id  = m.snapshot_id
 AND best.chain_id     = m.chain_id
 AND best.min_priority = m.priority
JOIN (
    SELECT
        snapshot_id,
        chain_id,
        priority,
        MIN(rule_id) AS min_rule_id
    FROM v_invariant_rule_matches
    GROUP BY snapshot_id, chain_id, priority
) AS tie
  ON tie.snapshot_id = m.snapshot_id
 AND tie.chain_id    = m.chain_id
 AND tie.priority    = m.priority
 AND tie.min_rule_id = m.rule_id;

------------------------------------------------------------
-- 5. Insert pattern for invariants_snapshot
------------------------------------------------------------

-- Example prepared-statement shape (parameters to be bound by the engine):
-- INSERT INTO invariants_snapshot (
--     snapshot_id, timestamp, region_id, tile_id,
--     cic, mdi, aos, rrm, fcf, spr, rwf, det, hvf, lsg, shci,
--     meta_json
-- ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);

------------------------------------------------------------
-- 6. PRAGMA profiles
------------------------------------------------------------

-- 6.1 Runtime profile (persistent storage)
-- Use these at DB open in shipping builds and local playtests.

-- PRAGMA journal_mode = WAL;
-- PRAGMA synchronous = NORMAL;
-- PRAGMA cache_size = -8000;
-- PRAGMA busy_timeout = 5000;
-- PRAGMA temp_store = MEMORY;
-- PRAGMA foreign_keys = ON;

-- 6.2 CI / tmpfs profile (RAM-disk, ephemeral DB)
-- Use these at DB open when running in CI or test harnesses with tmpfs.

-- PRAGMA journal_mode = MEMORY;
-- PRAGMA synchronous = OFF;
-- PRAGMA temp_store = MEMORY;
-- PRAGMA cache_size = -16000;
-- PRAGMA locking_mode = NORMAL;
-- PRAGMA foreign_keys = ON;
