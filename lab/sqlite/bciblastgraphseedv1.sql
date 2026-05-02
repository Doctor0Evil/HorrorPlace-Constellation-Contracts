-- ----------------------------------------------------------------------------
-- labsqlitebciblastgraphseedv1.sql
-- Target repo: HorrorPlace-Constellation-Contracts
-- Tier: 1 (Public, canonical SQL)
--
-- Blast Graph Seed Data v1
--
-- Purpose
--   Seed the blast-graph layer with:
--     - One initial blast profile aligned with default policy.
--     - A small test region of file nodes and neighbor edges.
--
-- Dependencies
--   Requires schema from labsqlite_blastgraph_v1.sql to be loaded first.
--   Assumes:
--     - constellationvmnode has at least one VM node for Horror.Place
--       and HorrorPlace-Codebase-of-Death (see labsqlitebciconstellationwiringv1.sql).
--     - constellationfilemanifest exists and can accept new entries.
-- ----------------------------------------------------------------------------

PRAGMA foreign_keys = ON;

-- ----------------------------------------------------------------------------
-- Helper: resolve vmnodeids for key repos
-- (Use temporary variables via WITH clauses or just inline SELECTs where needed.)
-- For SQLite seed files we often inline the lookup via subqueries.
-- ----------------------------------------------------------------------------

-- ----------------------------------------------------------------------------
-- Seed 1: Initial blast profile for a generic Tier-2 region wiring
--
-- Profile: BLAST.DEFAULT.TIER2.ROOM
--   - Scope: room
--   - Tier: tier-2
--   - max_depth: 4
--   - max_distance: 4.0
--   - max_neighbors: 8
--   - det_cap: 5.0
--   - cic_cap: 3.0
--   - aos_cap: 3.0
--   - risk_cap: 6.0
--   - risk weights: equal (1.0 each)
--   - engagement UEC/ARR bands: mid-range, CDL cap moderate.
-- ----------------------------------------------------------------------------
INSERT INTO constellation_blast_profile (
    profilekey,
    vmnodeid,
    objectkind,
    neighbor_scope,
    tier,
    max_depth,
    max_distance,
    max_neighbors,
    det_cap,
    cic_cap,
    aos_cap,
    risk_cap,
    w_det,
    w_cic,
    w_aos,
    uec_band_min,
    uec_band_max,
    arr_band_min,
    arr_band_max,
    cdl_cap,
    policy_id,
    schemakey,
    notes,
    createdat,
    updatedat
)
VALUES (
    'BLAST.DEFAULT.TIER2.ROOM',
    (SELECT vmnodeid FROM constellationvmnode WHERE vmname = 'HorrorPlace-Codebase-of-Death' LIMIT 1),
    'regionContract',
    'room',
    'tier-2',
    4,
    4.0,
    8,
    5.0,
    3.0,
    3.0,
    6.0,
    1.0,
    1.0,
    1.0,
    0.40,   -- UEC band min
    0.80,   -- UEC band max
    0.40,   -- ARR band min
    0.85,   -- ARR band max
    0.75,   -- CDL cap
    'blast-radius-policy-default-tier2-room-v1',
    'blast-radius-policy-v1',
    'Seeded default Tier-2 room profile for early blast tests.',
    datetime('now'),
    datetime('now')
);

-- ----------------------------------------------------------------------------
-- Seed 2: Small test region
--
-- Three file nodes:
--   - REGION.A (regionContract)
--   - REGION.B (regionContract)
--   - REGION.C (regionContract)
--
-- Wiring:
--   A -> B (low det_delta, good hvf_alignment)
--   B -> C (moderate det_delta, still under cap)
--
-- All in HorrorPlace-Codebase-of-Death for simplicity.
-- ----------------------------------------------------------------------------

-- Insert file manifests for test region files (if not already present).
INSERT INTO constellationfilemanifest (
    filepath,
    filename,
    repository,
    tier,
    filetype,
    schemaref,
    dependson,
    status,
    checksumsha256,
    generatedat,
    updatedat
)
VALUES
    ('contracts/regions/test/region_a.json',
     'region_a.json',
     'HorrorPlace-Codebase-of-Death',
     2,
     'config',
     'regioncontractv1',
     NULL,
     'generated',
     NULL,
     datetime('now'),
     datetime('now')),
    ('contracts/regions/test/region_b.json',
     'region_b.json',
     'HorrorPlace-Codebase-of-Death',
     2,
     'config',
     'regioncontractv1',
     NULL,
     'generated',
     NULL,
     datetime('now'),
     datetime('now')),
    ('contracts/regions/test/region_c.json',
     'region_c.json',
     'HorrorPlace-Codebase-of-Death',
     2,
     'config',
     'regioncontractv1',
     NULL,
     'generated',
     NULL,
     datetime('now'),
     datetime('now'));

-- Create file nodes bound to those manifest rows with exemplar metrics.
INSERT INTO constellation_file_node (
    fileid,
    vmnodeid,
    objectkind,
    cic,
    aos,
    det,
    lsg,
    hvf,
    shci,
    uec,
    emd,
    stci,
    cdl,
    arr,
    tags,
    meta,
    createdat,
    updatedat
)
SELECT
    fm.fileid,
    vn.vmnodeid,
    'regionContract',
    -- region A metrics (baseline)
    CASE fm.filename
        WHEN 'region_a.json' THEN 0.4  -- cic
        WHEN 'region_b.json' THEN 0.6
        WHEN 'region_c.json' THEN 0.7
    END,
    CASE fm.filename
        WHEN 'region_a.json' THEN 0.3  -- aos
        WHEN 'region_b.json' THEN 0.5
        WHEN 'region_c.json' THEN 0.6
    END,
    CASE fm.filename
        WHEN 'region_a.json' THEN 2.0  -- det
        WHEN 'region_b.json' THEN 2.8
        WHEN 'region_c.json' THEN 3.5
    END,
    CASE fm.filename
        WHEN 'region_a.json' THEN 0.4  -- lsg
        WHEN 'region_b.json' THEN 0.5
        WHEN 'region_c.json' THEN 0.6
    END,
    CASE fm.filename
        WHEN 'region_a.json' THEN 0.5  -- hvf
        WHEN 'region_b.json' THEN 0.7
        WHEN 'region_c.json' THEN 0.8
    END,
    CASE fm.filename
        WHEN 'region_a.json' THEN 0.5  -- shci
        WHEN 'region_b.json' THEN 0.6
        WHEN 'region_c.json' THEN 0.7
    END,
    CASE fm.filename
        WHEN 'region_a.json' THEN 0.55 -- uec
        WHEN 'region_b.json' THEN 0.65
        WHEN 'region_c.json' THEN 0.70
    END,
    CASE fm.filename
        WHEN 'region_a.json' THEN 0.50 -- emd
        WHEN 'region_b.json' THEN 0.60
        WHEN 'region_c.json' THEN 0.65
    END,
    CASE fm.filename
        WHEN 'region_a.json' THEN 0.60 -- stci
        WHEN 'region_b.json' THEN 0.62
        WHEN 'region_c.json' THEN 0.64
    END,
    CASE fm.filename
        WHEN 'region_a.json' THEN 0.55 -- cdl
        WHEN 'region_b.json' THEN 0.60
        WHEN 'region_c.json' THEN 0.65
    END,
    CASE fm.filename
        WHEN 'region_a.json' THEN 0.60 -- arr
        WHEN 'region_b.json' THEN 0.68
        WHEN 'region_c.json' THEN 0.72
    END,
    json('[ "test-region", "blast-lab" ]'),
    json_object('note', 'seed test region for blast graph'),
    datetime('now'),
    datetime('now')
FROM constellationfilemanifest fm
JOIN constellationvmnode vn
  ON vn.vmname = 'HorrorPlace-Codebase-of-Death'
WHERE fm.filepath IN (
    'contracts/regions/test/region_a.json',
    'contracts/regions/test/region_b.json',
    'contracts/regions/test/region_c.json'
);

-- Seed neighbor edges:
--   A -> B (distance 1.0, modest DET increase, good alignment)
--   B -> C (distance 1.0, moderate DET increase, still under cap)
INSERT INTO constellation_neighbor_edge (
    sourcefilenodeid,
    targetfilenodeid,
    neighbor_scope,
    distance,
    radius_band,
    edge_kind,
    cic_overlap,
    aos_overlap,
    det_delta,
    hvf_alignment,
    lsg_gradient,
    shci_coupling,
    uec_delta,
    arr_delta,
    cdl_delta,
    prism_contract_id,
    policy_id,
    notes,
    createdat,
    updatedat
)
SELECT
    src.filenodeid,
    tgt.filenodeid,
    'room',
    1.0,
    'immediate',
    'test-link',
    -- cic_overlap: higher when CIC similar
    0.9,
    -- aos_overlap:
    0.8,
    -- det_delta:
    (tgt.det - src.det),
    -- hvf_alignment:
    0.9,
    -- lsg_gradient:
    (tgt.lsg - src.lsg),
    -- shci_coupling:
    0.7,
    -- uec_delta:
    (tgt.uec - src.uec),
    -- arr_delta:
    (tgt.arr - src.arr),
    -- cdl_delta:
    (tgt.cdl - src.cdl),
    NULL,
    'blast-radius-policy-default-tier2-room-v1',
    'Seed edge A->B in test region.',
    datetime('now'),
    datetime('now')
FROM constellation_file_node src
JOIN constellation_file_node tgt
  ON 1=1
WHERE src.fileid = (
          SELECT fileid FROM constellationfilemanifest
          WHERE filepath = 'contracts/regions/test/region_a.json'
          LIMIT 1
      )
  AND tgt.fileid = (
          SELECT fileid FROM constellationfilemanifest
          WHERE filepath = 'contracts/regions/test/region_b.json'
          LIMIT 1
      );

INSERT INTO constellation_neighbor_edge (
    sourcefilenodeid,
    targetfilenodeid,
    neighbor_scope,
    distance,
    radius_band,
    edge_kind,
    cic_overlap,
    aos_overlap,
    det_delta,
    hvf_alignment,
    lsg_gradient,
    shci_coupling,
    uec_delta,
    arr_delta,
    cdl_delta,
    prism_contract_id,
    policy_id,
    notes,
    createdat,
    updatedat
)
SELECT
    src.filenodeid,
    tgt.filenodeid,
    'room',
    1.0,
    'ring-1',
    'test-link',
    0.85,
    0.75,
    (tgt.det - src.det),
    0.85,
    (tgt.lsg - src.lsg),
    0.65,
    (tgt.uec - src.uec),
    (tgt.arr - src.arr),
    (tgt.cdl - src.cdl),
    NULL,
    'blast-radius-policy-default-tier2-room-v1',
    'Seed edge B->C in test region.',
    datetime('now'),
    datetime('now')
FROM constellation_file_node src
JOIN constellation_file_node tgt
  ON 1=1
WHERE src.fileid = (
          SELECT fileid FROM constellationfilemanifest
          WHERE filepath = 'contracts/regions/test/region_b.json'
          LIMIT 1
      )
  AND tgt.fileid = (
          SELECT fileid FROM constellationfilemanifest
          WHERE filepath = 'contracts/regions/test/region_c.json'
          LIMIT 1
      );

-- End of labsqlitebciblastgraphseedv1.sql
