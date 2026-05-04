PRAGMA foreign_keys = ON;

----------------------------------------------------------------------
-- Core tables
----------------------------------------------------------------------

CREATE TABLE IF NOT EXISTS graphnode (
  nodeid     INTEGER PRIMARY KEY,
  snapshotid INTEGER NOT NULL,
  tenantid   TEXT    NOT NULL,
  kind       TEXT    NOT NULL,
  labels     TEXT,
  det        REAL,
  cic        REAL,
  aos        REAL,
  createdat  TEXT    NOT NULL,
  updatedat  TEXT    NOT NULL
);

CREATE TABLE IF NOT EXISTS graphedge (
  edgeid            INTEGER PRIMARY KEY,
  snapshotid        INTEGER NOT NULL,
  tenantid          TEXT    NOT NULL,
  sourcenodeid      INTEGER NOT NULL,
  targetnodeid      INTEGER NOT NULL,
  distance          REAL    NOT NULL,
  successprob       REAL    NOT NULL DEFAULT 1.0,
  requiresconntrack INTEGER NOT NULL DEFAULT 0,
  conntracktuple    TEXT,
  enabled           INTEGER NOT NULL DEFAULT 1,
  detdelta          REAL    NOT NULL DEFAULT 0.0,
  cicdelta          REAL    NOT NULL DEFAULT 0.0,
  aosdelta          REAL    NOT NULL DEFAULT 0.0,
  createdat         TEXT    NOT NULL,
  updatedat         TEXT    NOT NULL,
  FOREIGN KEY (sourcenodeid) REFERENCES graphnode(nodeid),
  FOREIGN KEY (targetnodeid) REFERENCES graphnode(nodeid)
);

----------------------------------------------------------------------
-- Indexes
----------------------------------------------------------------------

CREATE INDEX IF NOT EXISTS idx_graphnode_snapshot_tenant
  ON graphnode(snapshotid, tenantid, nodeid);

CREATE INDEX IF NOT EXISTS idx_graphnode_kind
  ON graphnode(snapshotid, tenantid, kind);

CREATE INDEX IF NOT EXISTS idx_graphnode_labels
  ON graphnode(snapshotid, tenantid, labels);

CREATE INDEX IF NOT EXISTS idx_graphedge_forward
  ON graphedge(snapshotid, tenantid, sourcenodeid, enabled);

CREATE INDEX IF NOT EXISTS idx_graphedge_forward_conn
  ON graphedge(snapshotid, tenantid, sourcenodeid, requiresconntrack, enabled);

CREATE INDEX IF NOT EXISTS idx_graphedge_target
  ON graphedge(snapshotid, tenantid, targetnodeid);

CREATE INDEX IF NOT EXISTS idx_graphedge_src_prob
  ON graphedge(snapshotid, tenantid, sourcenodeid, successprob DESC)
  WHERE enabled = 1;

CREATE INDEX IF NOT EXISTS idx_graphedge_conntrack_tuple
  ON graphedge(snapshotid, tenantid, conntracktuple)
  WHERE requiresconntrack = 1;

----------------------------------------------------------------------
-- Helper view: HS-zone selector (by kind and optional label)
----------------------------------------------------------------------

CREATE VIEW IF NOT EXISTS v_hs_nodes AS
SELECT
  n.snapshotid,
  n.tenantid,
  n.nodeid,
  n.kind,
  n.labels,
  n.det,
  n.cic,
  n.aos
FROM graphnode AS n
WHERE n.kind = 'hs-zone';

----------------------------------------------------------------------
-- Parameterized probabilistic reachability query (template)
--
-- Parameters expected:
--   :snapshot_id     INTEGER
--   :tenant_id       TEXT
--   :origin_node_id  INTEGER
--   :hs_zone_kind    TEXT     (e.g. 'hs-zone')
--   :max_depth       INTEGER
--   :max_distance    REAL
--   :max_det_sum     REAL or NULL
--   :max_paths       INTEGER
--
-- Usage pattern:
--   Prepare this statement as-is and bind parameters from Rust/Lua.
----------------------------------------------------------------------

-- The following WITH RECURSIVE is meant to be used as a standalone
-- prepared statement (not a view) because it requires runtime params.

-- Example name when prepared from host code:
--   hpc_reachability_prob

WITH RECURSIVE paths AS (
  SELECT
    e.edgeid                                AS edgeid,
    e.sourcenodeid                          AS originid,
    e.targetnodeid                          AS currentid,
    1                                       AS depth,
    e.distance                              AS distance_sum,
    CASE
      WHEN e.successprob IS NULL OR e.successprob <= 0.0
        THEN 1e6
      ELSE -LOG(e.successprob)
    END                                     AS log_cost,
    e.detdelta                              AS det_sum,
    e.cicdelta                              AS cic_sum,
    e.aosdelta                              AS aos_sum,
    printf('%d', e.edgeid)                  AS edge_ids,
    printf('%d', e.sourcenodeid) || ',' ||
      printf('%d', e.targetnodeid)          AS node_ids
  FROM graphedge AS e
  WHERE
    e.snapshotid   = :snapshot_id
    AND e.tenantid = :tenant_id
    AND e.sourcenodeid = :origin_node_id
    AND e.enabled  = 1

  UNION ALL

  SELECT
    e.edgeid                                AS edgeid,
    p.originid                              AS originid,
    e.targetnodeid                          AS currentid,
    p.depth + 1                             AS depth,
    p.distance_sum + e.distance             AS distance_sum,
    p.log_cost + CASE
      WHEN e.successprob IS NULL OR e.successprob <= 0.0
        THEN 1e6
      ELSE -LOG(e.successprob)
    END                                     AS log_cost,
    p.det_sum + e.detdelta                  AS det_sum,
    p.cic_sum + e.cicdelta                  AS cic_sum,
    p.aos_sum + e.aosdelta                  AS aos_sum,
    p.edge_ids || ',' || printf('%d', e.edgeid)
                                            AS edge_ids,
    p.node_ids || ',' || printf('%d', e.targetnodeid)
                                            AS node_ids
  FROM paths AS p
  JOIN graphedge AS e
    ON e.snapshotid   = :snapshot_id
   AND e.tenantid     = :tenant_id
   AND e.sourcenodeid = p.currentid
   AND e.enabled      = 1
  WHERE
    p.depth < :max_depth
    AND p.distance_sum + e.distance <= :max_distance
    AND INSTR(p.node_ids, ',' || printf('%d', e.targetnodeid)) = 0
    AND (:max_det_sum IS NULL OR p.det_sum + e.detdelta <= :max_det_sum)
),

hs_paths AS (
  SELECT
    p.originid,
    p.currentid           AS hs_node_id,
    p.depth,
    p.distance_sum,
    p.log_cost,
    EXP(-p.log_cost)      AS path_prob,
    p.det_sum,
    p.cic_sum,
    p.aos_sum,
    p.edge_ids,
    p.node_ids
  FROM paths AS p
  JOIN graphnode AS n
    ON n.snapshotid = :snapshot_id
   AND n.tenantid   = :tenant_id
   AND n.nodeid     = p.currentid
  WHERE n.kind = :hs_zone_kind
)
SELECT *
FROM hs_paths
ORDER BY log_cost ASC
LIMIT :max_paths;

----------------------------------------------------------------------
-- Conntrack BEFORE and AFTER variants (as views)
--
-- These normalize the WHERE clauses so host code can reuse the
-- same path logic with different edge filters.
----------------------------------------------------------------------

CREATE VIEW IF NOT EXISTS v_graphedge_before_conntrack AS
SELECT *
FROM graphedge
WHERE
  enabled           = 1
  AND requiresconntrack = 0;

CREATE VIEW IF NOT EXISTS v_graphedge_after_conntrack AS
SELECT *
FROM graphedge
WHERE
  enabled = 1
  AND (requiresconntrack = 0 OR conntracktuple IS NOT NULL);
