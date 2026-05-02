-- Core tileset definition: static geometry and invariants.
CREATE TABLE IF NOT EXISTS bci_tileset (
    tileset_id      INTEGER PRIMARY KEY,
    name            TEXT NOT NULL,
    region_class    TEXT NOT NULL,
    cic             REAL NOT NULL,  -- 0..1
    aos             REAL NOT NULL,  -- 0..1
    mdi             REAL NOT NULL,  -- 0..1
    det_baseline    REAL NOT NULL,  -- 0..10
    lsg             REAL NOT NULL,  -- 0..1
    spr             REAL NOT NULL,  -- 0..1
    shci            REAL NOT NULL   -- 0..1
);

-- Geometry profiles: contract-level bindings as seen by the lab.
CREATE TABLE IF NOT EXISTS bci_geometry_profile (
    profile_id          INTEGER PRIMARY KEY,
    profile_key         TEXT NOT NULL UNIQUE,   -- e.g. "corridor-highCIC"
    tileset_id          INTEGER NOT NULL,
    tier                TEXT NOT NULL,          -- lab|standard|mature
    status              TEXT NOT NULL,          -- experimental|candidate|live|deprecated
    safety_profile_id   TEXT NOT NULL,
    notes               TEXT,
    FOREIGN KEY (tileset_id) REFERENCES bci_tileset(tileset_id)
);

-- Metrics windows: sliding BCI metric slices per session.
CREATE TABLE IF NOT EXISTS bci_metrics_window (
    window_id       INTEGER PRIMARY KEY,
    session_id      INTEGER NOT NULL,
    player_id       INTEGER NOT NULL,
    tick_index      INTEGER NOT NULL,
    tick_seconds    REAL NOT NULL,   -- frame duration
    uec             REAL NOT NULL,   -- 0..1
    emd             REAL NOT NULL,   -- 0..1
    stci            REAL NOT NULL,   -- 0..1
    cdl             REAL NOT NULL,   -- 0..1
    arr             REAL NOT NULL,   -- 0..1
    det             REAL NOT NULL,   -- 0..10
    stress_score    REAL NOT NULL,   -- 0..1
    stress_band     TEXT NOT NULL,   -- Low|Medium|High|Extreme
    attention_band  TEXT NOT NULL,   -- Distracted|Neutral|Focused|HyperFocused
    visual_overload REAL NOT NULL,   -- 0..1
    startle_spike   INTEGER NOT NULL, -- 0|1
    signal_quality  TEXT NOT NULL    -- Good|Degraded|Unavailable
);

-- Geometry selection events: kernel mapping decisions.
CREATE TABLE IF NOT EXISTS geometry_selection_event (
    event_id            INTEGER PRIMARY KEY,
    session_id          INTEGER NOT NULL,
    player_id           INTEGER NOT NULL,
    tick_index          INTEGER NOT NULL,
    profile_id          INTEGER NOT NULL,
    binding_id          TEXT NOT NULL,
    det_before          REAL NOT NULL,
    det_after           REAL NOT NULL,
    csi_before          REAL NOT NULL,
    csi_after           REAL NOT NULL,
    overload_flag       INTEGER NOT NULL,  -- 0|1
    selection_score     REAL NOT NULL,
    candidate_count     INTEGER NOT NULL,
    FOREIGN KEY (profile_id) REFERENCES bci_geometry_profile(profile_id)
);

-- Simple hysteresis violation view: DET jumps > 0.25 over 2 ticks.
CREATE VIEW IF NOT EXISTS v_det_hysteresis_violation AS
SELECT
    e1.session_id,
    e1.player_id,
    e1.tick_index AS tick_index_start,
    e2.tick_index AS tick_index_end,
    e1.det_after  AS det_start,
    e2.det_after  AS det_end,
    (e2.det_after - e1.det_after) AS det_delta
FROM geometry_selection_event e1
JOIN geometry_selection_event e2
  ON e1.session_id = e2.session_id
 AND e1.player_id  = e2.player_id
 AND e2.tick_index = e1.tick_index + 2
WHERE (e2.det_after - e1.det_after) > 0.25;

-- Basic session summary view stub (expand later as needed).
CREATE VIEW IF NOT EXISTS v_session_summary AS
SELECT
    session_id,
    player_id,
    COUNT(*)                                    AS ticks,
    AVG(uec)                                    AS uec_mean,
    AVG(CASE WHEN stress_band = 'OPTIMALSTRESS' THEN 1.0 ELSE 0.0 END) AS optimal_stress_fraction,
    AVG(arr)                                    AS arr_mean
FROM bci_metrics_window
GROUP BY session_id, player_id;
