-- ============================================================================
-- BCI Blast-Radius Graph Schema v1
-- Purpose: Enable neighbor queries, blast-radius limits, and file-mapping
--          for prism-contracted monuments across the VM-constellation.
-- Tier: 1 (Public, Canonical)
-- Repository: HorrorPlace-Constellation-Contracts
-- ============================================================================

-- ----------------------------------------------------------------------------
-- Table: constellation_file_node
-- Purpose: Graph node representing one file artifact with invariant/metric footprint.
-- ----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS constellation_file_node (
    file_node_id       INTEGER PRIMARY KEY,
    file_id            INTEGER NOT NULL,
    vm_node_id         INTEGER NOT NULL,
    object_kind        TEXT NOT NULL,
    region_id          TEXT,
    tile_id            TEXT,
    
    -- Invariant snapshot (0-10 scale, SHCI 0-1)
    cic                REAL CHECK(cic >= 0 AND cic <= 10),
    aos                REAL CHECK(aos >= 0 AND aos <= 10),
    mdi                REAL CHECK(mdi >= 0 AND mdi <= 10),
    rrm                REAL CHECK(rrm >= 0 AND rrm <= 10),
    fcf                REAL CHECK(fcf >= 0 AND fcf <= 10),
    spr                REAL CHECK(spr >= 0 AND spr <= 10),
    rwf                REAL CHECK(rwf >= 0 AND rwf <= 10),
    det                REAL CHECK(det >= 0 AND det <= 10),
    hvf                REAL CHECK(hvf >= 0 AND hvf <= 10),
    lsg                REAL CHECK(lsg >= 0 AND lsg <= 10),
    shci               REAL CHECK(shci >= 0 AND shci <= 1),
    
    -- Entertainment metrics snapshot (0-10 scale)
    uec                REAL CHECK(uec >= 0 AND uec <= 10),
    emd                REAL CHECK(emd >= 0 AND emd <= 10),
    stci               REAL CHECK(stci >= 0 AND stci <= 10),
    cdl                REAL CHECK(cdl >= 0 AND cdl <= 10),
    arr                REAL CHECK(arr >= 0 AND arr <= 10),
    
    created_at         TEXT NOT NULL,
    updated_at         TEXT NOT NULL,
    
    FOREIGN KEY(file_id) REFERENCES constellation_file_manifest(file_id),
    FOREIGN KEY(vm_node_id) REFERENCES constellation_vm_node(vm_node_id)
);

-- ----------------------------------------------------------------------------
-- Table: constellation_neighbor_edge
-- Purpose: Blast-radius edges between file nodes with horror-specific metrics.
-- ----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS constellation_neighbor_edge (
    edge_id                INTEGER PRIMARY KEY,
    source_file_node_id    INTEGER NOT NULL,
    target_file_node_id    INTEGER NOT NULL,
    
    neighbor_scope         TEXT NOT NULL CHECK(neighbor_scope IN ('tile', 'room', 'region', 'vm', 'constellation')),
    distance               REAL NOT NULL CHECK(distance >= 0),
    radius_band            TEXT CHECK(radius_band IN ('immediate', 'ring-1', 'ring-2', 'ring-3', 'far')),
    
    -- Overlap/differential measures
    cic_overlap            REAL CHECK(cic_overlap >= 0 AND cic_overlap <= 1),
    aos_overlap            REAL CHECK(aos_overlap >= 0 AND aos_overlap <= 1),
    det_delta              REAL CHECK(det_delta >= -10 AND det_delta <= 10),
    hvf_alignment          REAL CHECK(hvf_alignment >= -1 AND hvf_alignment <= 1),
    lsg_gradient           REAL CHECK(lsg_gradient >= -10 AND lsg_gradient <= 10),
    shci_coupling          REAL CHECK(shci_coupling >= 0 AND shci_coupling <= 1),
    
    edge_kind              TEXT NOT NULL CHECK(edge_kind IN ('spatial', 'logical', 'contract', 'telemetry')),
    enabled                INTEGER NOT NULL DEFAULT 1 CHECK(enabled IN (0, 1)),
    
    created_at             TEXT NOT NULL,
    updated_at             TEXT NOT NULL,
    
    FOREIGN KEY(source_file_node_id) REFERENCES constellation_file_node(file_node_id),
    FOREIGN KEY(target_file_node_id) REFERENCES constellation_file_node(file_node_id)
);

-- ----------------------------------------------------------------------------
-- Table: constellation_blast_profile
-- Purpose: Per-VM/scope limits for blast-radius wiring.
-- ----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS constellation_blast_profile (
    blast_profile_id   INTEGER PRIMARY KEY,
    vm_node_id         INTEGER,  -- NULL = global default
    neighbor_scope     TEXT NOT NULL CHECK(neighbor_scope IN ('tile', 'room', 'region', 'vm', 'constellation')),
    max_distance       REAL NOT NULL CHECK(max_distance >= 0),
    max_neighbors      INTEGER NOT NULL CHECK(max_neighbors >= 0),
    det_cap            REAL CHECK(det_cap IS NULL OR (det_cap >= 0 AND det_cap <= 10)),
    uec_target         REAL CHECK(uec_target IS NULL OR (uec_target >= 0 AND uec_target <= 10)),
    emd_target         REAL CHECK(emd_target IS NULL OR (emd_target >= 0 AND emd_target <= 10)),
    arr_target         REAL CHECK(arr_target IS NULL OR (arr_target >= 0 AND arr_target <= 10)),
    created_at         TEXT NOT NULL,
    updated_at         TEXT NOT NULL,
    UNIQUE(vm_node_id, neighbor_scope),
    FOREIGN KEY(vm_node_id) REFERENCES constellation_vm_node(vm_node_id)
);

-- ----------------------------------------------------------------------------
-- Table: region_tile (optional spatial substrate)
-- Purpose: Grid-based tile coordinates per region for spatial blast queries.
-- ----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS region_tile (
    region_id   TEXT NOT NULL,
    tile_id     TEXT NOT NULL,
    x           INTEGER NOT NULL,
    y           INTEGER NOT NULL,
    cic         REAL CHECK(cic >= 0 AND cic <= 10),
    aos         REAL CHECK(aos >= 0 AND aos <= 10),
    det         REAL CHECK(det >= 0 AND det <= 10),
    hvf         REAL CHECK(hvf >= 0 AND hvf <= 10),
    lsg         REAL CHECK(lsg >= 0 AND lsg <= 10),
    shci        REAL CHECK(shci >= 0 AND shci <= 1),
    PRIMARY KEY(region_id, tile_id)
);

-- ----------------------------------------------------------------------------
-- Table: region_tile_neighbor
-- Purpose: Precomputed spatial adjacency with blast-radius bands.
-- ----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS region_tile_neighbor (
    region_id          TEXT NOT NULL,
    source_tile_id     TEXT NOT NULL,
    target_tile_id     TEXT NOT NULL,
    radius_band        TEXT NOT NULL CHECK(radius_band IN ('immediate', 'ring-1', 'ring-2', 'ring-3', 'far')),
    manhattan_distance INTEGER NOT NULL CHECK(manhattan_distance >= 0),
    euclidean_distance REAL NOT NULL CHECK(euclidean_distance >= 0),
    hvf_alignment      REAL CHECK(hvf_alignment >= -1 AND hvf_alignment <= 1),
    lsg_gradient       REAL CHECK(lsg_gradient >= -10 AND lsg_gradient <= 10),
    enabled            INTEGER NOT NULL DEFAULT 1 CHECK(enabled IN (0, 1)),
    PRIMARY KEY(region_id, source_tile_id, target_tile_id),
    FOREIGN KEY(region_id, source_tile_id) REFERENCES region_tile(region_id, tile_id),
    FOREIGN KEY(region_id, target_tile_id) REFERENCES region_tile(region_id, tile_id)
);

-- ----------------------------------------------------------------------------
-- View: v_file_blast_neighbors
-- Purpose: AI-Chat friendly query for approved neighbors within blast profile.
-- ----------------------------------------------------------------------------
CREATE VIEW IF NOT EXISTS v_file_blast_neighbors AS
SELECT
    e.edge_id,
    e.source_file_node_id,
    e.target_file_node_id,
    e.neighbor_scope,
    e.distance,
    e.radius_band,
    e.det_delta,
    e.hvf_alignment,
    e.lsg_gradient,
    e.shci_coupling,
    e.edge_kind,
    fn_src.file_id AS source_file_id,
    fn_src.file_path AS source_file_path,
    fn_src.repository AS source_repository,
    fn_tgt.file_id AS target_file_id,
    fn_tgt.file_path AS target_file_path,
    fn_tgt.repository AS target_repository
FROM constellation_neighbor_edge AS e
JOIN constellation_file_node AS fn_src ON fn_src.file_node_id = e.source_file_node_id
JOIN constellation_file_node AS fn_tgt ON fn_tgt.file_node_id = e.target_file_node_id
JOIN constellation_file_manifest AS f_src ON f_src.file_id = fn_src.file_id
JOIN constellation_file_manifest AS f_tgt ON f_tgt.file_id = fn_tgt.file_id
WHERE e.enabled = 1;

-- ----------------------------------------------------------------------------
-- View: v_tile_blast_neighbors
-- Purpose: Spatial blast queries for dungeon/region tile graphs.
-- ----------------------------------------------------------------------------
CREATE VIEW IF NOT EXISTS v_tile_blast_neighbors AS
SELECT
    n.region_id,
    n.source_tile_id,
    n.target_tile_id,
    n.radius_band,
    n.manhattan_distance,
    n.euclidean_distance,
    n.hvf_alignment,
    n.lsg_gradient,
    t_src.cic AS src_cic,
    t_src.aos AS src_aos,
    t_src.det AS src_det,
    t_tgt.cic AS tgt_cic,
    t_tgt.aos AS tgt_aos,
    t_tgt.det AS tgt_det
FROM region_tile_neighbor AS n
JOIN region_tile AS t_src ON t_src.region_id = n.region_id AND t_src.tile_id = n.source_tile_id
JOIN region_tile AS t_tgt ON t_tgt.region_id = n.region_id AND t_tgt.tile_id = n.target_tile_id
WHERE n.enabled = 1;

-- ----------------------------------------------------------------------------
-- Indexes for blast-radius query performance
-- ----------------------------------------------------------------------------
CREATE INDEX IF NOT EXISTS idx_file_node_vm ON constellation_file_node(vm_node_id);
CREATE INDEX IF NOT EXISTS idx_file_node_region ON constellation_file_node(region_id);
CREATE INDEX IF NOT EXISTS idx_file_node_tile ON constellation_file_node(tile_id);
CREATE INDEX IF NOT EXISTS idx_neighbor_edge_source ON constellation_neighbor_edge(source_file_node_id);
CREATE INDEX IF NOT EXISTS idx_neighbor_edge_target ON constellation_neighbor_edge(target_file_node_id);
CREATE INDEX IF NOT EXISTS idx_neighbor_edge_scope ON constellation_neighbor_edge(neighbor_scope);
CREATE INDEX IF NOT EXISTS idx_blast_profile_vm_scope ON constellation_blast_profile(vm_node_id, neighbor_scope);
CREATE INDEX IF NOT EXISTS idx_tile_neighbor_region ON region_tile_neighbor(region_id);
CREATE INDEX IF NOT EXISTS idx_tile_neighbor_source ON region_tile_neighbor(source_tile_id);
