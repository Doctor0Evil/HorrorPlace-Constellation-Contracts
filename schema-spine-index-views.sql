-- schema-spine-index-views.sql
-- Views to resolve objectKind + tier into repo, schema, and metric bands.

PRAGMA foreign_keys = ON;

----------------------------------------------------------------------
-- 1. Routing: objectKind + tier → repo + schema
----------------------------------------------------------------------

CREATE VIEW IF NOT EXISTS v_object_kind_routing AS
SELECT
    f.objectkind        AS objectKind,
    v.tier              AS tier,
    v.vmnodeid          AS vmNodeId,
    v.vmname            AS repoSlug,
    v.gitremote         AS gitRemote,
    f.schemauri         AS schemaUri,
    s.schemakind        AS schemaKind,
    s.repopath          AS schemaPath
FROM constellationfilemanifest AS f
JOIN constellationvmnode AS v
    ON v.vmnodeid = f.vmnodeid
LEFT JOIN constellationschemaregistry AS s
    ON s.schemauri = f.schemauri;

CREATE INDEX IF NOT EXISTS idx_file_manifest_objectkind_tier
    ON constellationfilemanifest (objectkind);

----------------------------------------------------------------------
-- 2. Bands: observed invariant/metric bands by objectKind + tier
----------------------------------------------------------------------

CREATE VIEW IF NOT EXISTS v_object_kind_bands AS
SELECT
    fn.objectkind           AS objectKind,
    v.tier                  AS tier,
    AVG(fn.cic)             AS cic_mean,
    MIN(fn.cic)             AS cic_min,
    MAX(fn.cic)             AS cic_max,
    AVG(fn.aos)             AS aos_mean,
    MIN(fn.aos)             AS aos_min,
    MAX(fn.aos)             AS aos_max,
    AVG(fn.det)             AS det_mean,
    MIN(fn.det)             AS det_min,
    MAX(fn.det)             AS det_max,
    AVG(fn.uec)             AS uec_mean,
    AVG(fn.arr)             AS arr_mean,
    AVG(fn.cdl)             AS cdl_mean
FROM constellationfilenode AS fn
JOIN constellationvmnode AS v
    ON v.vmnodeid = fn.vmnodeid
GROUP BY fn.objectkind, v.tier;

CREATE INDEX IF NOT EXISTS idx_file_node_objectkind_tier
    ON constellationfilenode (objectkind, vmnodeid);

----------------------------------------------------------------------
-- 3. Combined view: routing + bands for CHATDIRECTOR
----------------------------------------------------------------------

CREATE VIEW IF NOT EXISTS v_object_kind_schema_index AS
SELECT
    r.objectKind           AS objectKind,
    r.tier                 AS tier,
    r.vmNodeId             AS vmNodeId,
    r.repoSlug             AS repoSlug,
    r.gitRemote            AS gitRemote,
    r.schemaUri            AS schemaUri,
    r.schemaKind           AS schemaKind,
    r.schemaPath           AS schemaPath,
    b.cic_min              AS cic_min,
    b.cic_max              AS cic_max,
    b.det_min              AS det_min,
    b.det_max              AS det_max,
    b.uec_mean             AS uec_mean,
    b.arr_mean             AS arr_mean,
    b.cdl_mean             AS cdl_mean
FROM v_object_kind_routing AS r
LEFT JOIN v_object_kind_bands AS b
  ON b.objectKind = r.objectKind
 AND b.tier = r.tier;
