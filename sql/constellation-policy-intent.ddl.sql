-- Constellation Policy Intent and Net-Change Corridor DDL
-- This file defines tables for policy intents, diffs, and corridors,
-- intended to live alongside the constellation-wide SQLite index.

PRAGMA foreign_keys = ON;

----------------------------------------------------------------------
-- 1. Policy intents (immutable history with current pointer)
----------------------------------------------------------------------

CREATE TABLE IF NOT EXISTS policyintent (
  intentid        INTEGER PRIMARY KEY,
  id              TEXT NOT NULL,                 -- logical ID (UUID or ledger ID)
  policyname      TEXT NOT NULL,
  version         INTEGER NOT NULL,
  tier            TEXT NOT NULL,                 -- Tier1Public, Tier2Internal, ...
  createdat       TEXT NOT NULL,                 -- ISO-8601
  authorkind      TEXT NOT NULL,                 -- human, agent, system
  authorid        TEXT NOT NULL,
  authordisplay   TEXT,
  sourcekind      TEXT NOT NULL,                 -- manual-edit, ai-suggestion, ...
  previousintentid INTEGER,                      -- FK to policyintent.intentid
  status          TEXT NOT NULL,                 -- draft, proposed, approved, ...
  iscurrent       INTEGER NOT NULL DEFAULT 0,    -- 0/1
  deadledgerref   TEXT,
  blastprofileref TEXT,
  governancecorridorref TEXT,
  intentspecjson  TEXT NOT NULL,                 -- full intentSpec JSON
  compiledjson    TEXT,                          -- compiled block JSON
  netchangejson   TEXT,                          -- netChange JSON block
  metajson        TEXT,                          -- meta JSON block
  UNIQUE (policyname, version),
  UNIQUE (id),
  FOREIGN KEY (previousintentid) REFERENCES policyintent(intentid)
);

CREATE INDEX IF NOT EXISTS idx_policyintent_policyname ON policyintent (policyname);
CREATE INDEX IF NOT EXISTS idx_policyintent_current
  ON policyintent (policyname)
  WHERE iscurrent = 1;

----------------------------------------------------------------------
-- 2. Policy intent diffs
----------------------------------------------------------------------

CREATE TABLE IF NOT EXISTS policyintentdiff (
  diffid          INTEGER PRIMARY KEY,
  id              TEXT NOT NULL,                 -- logical ID
  policyname      TEXT NOT NULL,
  fromintentid    INTEGER NOT NULL,              -- FK to policyintent
  tointentid      INTEGER NOT NULL,              -- FK to policyintent
  createdat       TEXT NOT NULL,
  authorkind      TEXT NOT NULL,
  authorid        TEXT NOT NULL,
  authordisplay   TEXT,
  deadledgerref   TEXT,
  summaryjson     TEXT,                          -- summary block JSON
  changesjson     TEXT NOT NULL,                 -- full changes JSON
  UNIQUE (id),
  FOREIGN KEY (fromintentid) REFERENCES policyintent(intentid),
  FOREIGN KEY (tointentid)   REFERENCES policyintent(intentid)
);

CREATE INDEX IF NOT EXISTS idx_policyintentdiff_policy
  ON policyintentdiff (policyname, createdat);

----------------------------------------------------------------------
-- 3. Net-change corridors
----------------------------------------------------------------------

CREATE TABLE IF NOT EXISTS netchangecorridor (
  corridorid      INTEGER PRIMARY KEY,
  id              TEXT NOT NULL,                 -- logical ID
  name            TEXT NOT NULL,
  tier            TEXT NOT NULL,
  createdat       TEXT NOT NULL,
  description     TEXT,
  scopejson       TEXT,                          -- scope block JSON
  thresholdsjson  TEXT NOT NULL,                 -- thresholds block JSON
  weightsjson     TEXT,                          -- weights block JSON
  metajson        TEXT,
  UNIQUE (id)
);

CREATE INDEX IF NOT EXISTS idx_netchangecorridor_tier
  ON netchangecorridor (tier, name);

----------------------------------------------------------------------
-- 4. Optional quick reference from policyintent to net-change evaluation
--    (denormalized view for tooling)
----------------------------------------------------------------------

CREATE VIEW IF NOT EXISTS v_policyintent_netchange AS
SELECT
  p.intentid,
  p.policyname,
  p.version,
  p.status,
  p.iscurrent,
  p.governancecorridorref,
  p.netchangejson
FROM policyintent AS p;
