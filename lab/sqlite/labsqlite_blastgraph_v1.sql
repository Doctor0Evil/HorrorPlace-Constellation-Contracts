-- ----------------------------------------------------------------------------
-- labsqlite_blastgraph_v1.sql
-- Target repo: HorrorPlace-Constellation-Contracts
-- Tier: 1 (Public, canonical SQL)
--
-- Blast Graph Tables v1
--
-- Purpose
--   Extend the VM-Constellation wiring DB with a graph layer that treats files
--   as first-class nodes and models neighbor relationships with invariant and
--   metric deltas. This layer feeds:
--     - Blast-radius computation (H.Blast / H.SQLiteBlast).
--     - Prism blast summaries (prism_blast_summary).
--     - Offline Rust/BCI analysis tools (blast-policy tuner).
--
-- Dependencies
--   Assumes the following tables already exist (from labsqlitebciconstellationwiringv1.sql):
--     - constellationvmnode(vmnodeid, vmname, ...)
--     - constellationfilemanifest(fileid, filepath, filename, repository, tier, ...)
--     - constellationschemaregistry(schemaid, schemakey, ...)
--
-- All IDs are INTEGER PRIMARY KEY AUTOINCREMENT unless otherwise noted.
-- ----------------------------------------------------------------------------

PRAGMA foreign_keys = ON;

-- ----------------------------------------------------------------------------
-- Table: constellation_file_node
--
-- Purpose:
--   Elevate file manifest entries to first-class graph nodes with invariant
--   and metric snapshots. Each row corresponds to exactly one fileid in
--   constellationfilemanifest.
--
-- Notes:
--   - objectkind is a normalized, engine-facing label (e.g., "eventContract",
--     "regionContract", "prismMonumentContract").
--   - invariant/metric columns are nullable; missing values mean "not yet
--     measured" or "not applicable" rather than zero.
-- ----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS constellation_file_node (
    filenodeid       INTEGER PRIMARY KEY,
    fileid           INTEGER NOT NULL UNIQUE,
    vmnodeid         INTEGER NOT NULL,
    objectkind       TEXT NOT NULL,           -- e.g., eventContract, regionContract, prismMonumentContract

    -- Invariant snapshot (0–1 or 0–10 depending on metric family, documented in contracts)
    cic              REAL,                    -- Cognitive Intensity Coefficient
    aos              REAL,                    -- Affective Opacity Score
    det              REAL,                    -- Distress / Explicitness Tension
    lsg              REAL,                    -- Liminal Spatial Gradient
    hvf              REAL,                    -- Horror Valence Flux
    shci             REAL,                    -- Spectral Horror Coupling Index

    -- Engagement / telemetry metrics (normalized)
    uec              REAL,                    -- Uncanny Engagement Coefficient
    emd              REAL,                    -- Emotional Modulation Depth
    stci             REAL,                    -- Story Thread Coherence Index
    cdl              REAL,                    -- Cognitive Demand Load
    arr              REAL,                    -- Ambiguous Resolution Reward

    -- Optional tags / JSON enrichment for future extensions
    tags             TEXT,                    -- JSON array of tags (e.g., ["field-A","tier-2"])
    meta             TEXT,                    -- JSON object with additional annotations

    createdat        TEXT NOT NULL,           -- ISO 8601
    updatedat        TEXT NOT NULL,           -- ISO 8601

    FOREIGN KEY (fileid)  REFERENCES constellationfilemanifest(fileid),
    FOREIGN KEY (vmnodeid) REFERENCES constellationvmnode(vmnodeid)
);

CREATE INDEX IF NOT EXISTS idx_file_node_fileid
    ON constellation_file_node(fileid);

CREATE INDEX IF NOT EXISTS idx_file_node_vmnodeid
    ON constellation_file_node(vmnodeid);

CREATE INDEX IF NOT EXISTS idx_file_node_objectkind
    ON constellation_file_node(objectkind);

-- ----------------------------------------------------------------------------
-- Table: constellation_neighbor_edge
--
-- Purpose:
--   Adjacency list for the blast graph. Each row encodes a directed neighbor
--   relationship between two file nodes plus metric deltas that describe how
--   invariants and engagement change across the edge.
--
-- Notes:
--   - Directed edges let you model asymmetric relationships if desired.
--   - neighbor_scope and edge_kind are controlled vocabularies enforced at
--     the contract/schema level (e.g., "tile","room","region","global";
--     "prism-link","pcg-link","author-link").
-- ----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS constellation_neighbor_edge (
    edgeid                 INTEGER PRIMARY KEY,
    sourcefilenodeid       INTEGER NOT NULL,
    targetfilenodeid       INTEGER NOT NULL,

    -- Scope & structural metadata
    neighbor_scope         TEXT NOT NULL,     -- e.g., tile, room, region, global
    distance               REAL NOT NULL,     -- numeric distance (e.g., graph distance or spatial metric)
    radius_band            TEXT,              -- e.g., immediate, ring-1, ring-2
    edge_kind              TEXT NOT NULL,     -- e.g., prism-link, dependency, style-link

    -- Differential invariant metrics (delta = target - source)
    cic_overlap            REAL,              -- similarity / overlap of CIC between nodes
    aos_overlap            REAL,              -- similarity / overlap of AOS
    det_delta              REAL,              -- DET_target - DET_source
    hvf_alignment          REAL,              -- alignment of horror valence (e.g., cosine similarity)
    lsg_gradient           REAL,              -- LSG_target - LSG_source
    shci_coupling          REAL,              -- SHCI coupling strength across edge

    -- Optional: deltas on engagement metrics
    uec_delta              REAL,              -- UEC_target - UEC_source
    arr_delta              REAL,              -- ARR_target - ARR_source
    cdl_delta              REAL,              -- CDL_target - CDL_source

    -- Contracts / provenance
    prism_contract_id      TEXT,              -- optional: ID of prism-monument-contract-v1 that created this edge
    policy_id              TEXT,              -- blast-radius-policy id used when wiring
    notes                  TEXT,              -- freeform or JSON for debug annotations

    createdat              TEXT NOT NULL,     -- ISO 8601
    updatedat              TEXT NOT NULL,     -- ISO 8601

    FOREIGN KEY (sourcefilenodeid) REFERENCES constellation_file_node(filenodeid),
    FOREIGN KEY (targetfilenodeid) REFERENCES constellation_file_node(filenodeid)
);

CREATE INDEX IF NOT EXISTS idx_neighbor_edge_source
    ON constellation_neighbor_edge(sourcefilenodeid);

CREATE INDEX IF NOT EXISTS idx_neighbor_edge_target
    ON constellation_neighbor_edge(targetfilenodeid);

CREATE INDEX IF NOT EXISTS idx_neighbor_edge_scope
    ON constellation_neighbor_edge(neighbor_scope);

CREATE INDEX IF NOT EXISTS idx_neighbor_edge_edgekind
    ON constellation_neighbor_edge(edge_kind);

-- ----------------------------------------------------------------------------
-- Table: constellation_blast_profile
--
-- Purpose:
--   Policy-driven limits and targets for blast-radius queries. Each row
--   describes a profile that can be selected based on VM node, object kind,
--   neighbor_scope, and content tier.
--
--   This table is the SQLite embodiment of blast-radius-policy-v1 instances:
--     - Hard caps: det_cap, cic_cap, aos_cap, risk_cap.
--     - Risk weights: w_det, w_cic, w_aos.
--     - Engagement targets: uec_band, arr_band, cdl_cap.
--
-- Notes:
--   - The JSON contract file (blast-radius-policy-v1 NDJSON) should be the
--     canonical configuration; this table can be populated from it as a lab
--     or runtime cache.
-- ----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS constellation_blast_profile (
    blastprofileid      INTEGER PRIMARY KEY,
    profilekey          TEXT NOT NULL UNIQUE,  -- e.g., "BLAST.FIELD-A.TIER2.DEFAULT"
    vmnodeid            INTEGER,               -- optional: constrain to a specific VM node
    objectkind          TEXT,                  -- optional: constrain to object kind (e.g., "regionContract")
    neighbor_scope      TEXT NOT NULL,         -- e.g., tile, room, region, global
    tier                TEXT NOT NULL,         -- e.g., tier-1, tier-2, tier-3 or "FieldA-Tier2"

    -- Structural limits
    max_depth           INTEGER NOT NULL,      -- maximum hop count for blast queries
    max_distance        REAL NOT NULL,        -- maximum distance for edges considered
    max_neighbors       INTEGER NOT NULL,     -- cap on fan-out per node under this profile

    -- Hard caps on cumulative invariants / risk
    det_cap             REAL NOT NULL,        -- maximum allowed cumulative DET delta
    cic_cap             REAL NOT NULL,        -- maximum |ΔCIC|
    aos_cap             REAL NOT NULL,        -- maximum |ΔAOS|
    risk_cap            REAL NOT NULL,        -- maximum scalar risk R(p)

    -- Risk weights (for scalar R(p) computation)
    w_det               REAL NOT NULL,
    w_cic               REAL NOT NULL,
    w_aos               REAL NOT NULL,

    -- Engagement targets (soft constraints)
    uec_band_min        REAL,                 -- lower bound of desired UEC band
    uec_band_max        REAL,
    arr_band_min        REAL,
    arr_band_max        REAL,
    cdl_cap             REAL,                 -- upper bound on acceptable mean CDL

    -- Provenance / linkage
    policy_id           TEXT NOT NULL,        -- blast-radius-policy-v1 identifier
    schemakey           TEXT,                 -- optional link to constellationschemaregistry
    notes               TEXT,                 -- freeform or JSON

    createdat           TEXT NOT NULL,
    updatedat           TEXT NOT NULL,

    FOREIGN KEY (vmnodeid)  REFERENCES constellationvmnode(vmnodeid)
);

CREATE INDEX IF NOT EXISTS idx_blast_profile_scope_tier
    ON constellation_blast_profile(neighbor_scope, tier);

CREATE INDEX IF NOT EXISTS idx_blast_profile_vmnode
    ON constellation_blast_profile(vmnodeid);

CREATE INDEX IF NOT EXISTS idx_blast_profile_objectkind
    ON constellation_blast_profile(objectkind);

-- ----------------------------------------------------------------------------
-- View: vblastgraph_overview
--
-- Purpose:
--   High-level overview of the blast graph for AI-agent and lab queries.
--   Summarizes node counts and edge counts per VM node and object kind.
-- ----------------------------------------------------------------------------
CREATE VIEW IF NOT EXISTS vblastgraph_overview AS
SELECT
    vn.vmname                  AS vmname,
    vn.tier                    AS vmtier,
    fn.objectkind              AS objectkind,
    COUNT(DISTINCT fn.filenodeid) AS nodecount,
    COUNT(DISTINCT ne.edgeid)  AS edgecount
FROM constellationvmnode vn
LEFT JOIN constellationfilemanifest fm
    ON fm.repository = vn.vmname
LEFT JOIN constellation_file_node fn
    ON fn.fileid = fm.fileid
LEFT JOIN constellation_neighbor_edge ne
    ON ne.sourcefilenodeid = fn.filenodeid
WHERE vn.status = 'active'
GROUP BY vn.vmname, vn.tier, fn.objectkind;

-- ----------------------------------------------------------------------------
-- View: vblast_profile_selector
--
-- Purpose:
--   Helper view for selecting the most specific blast profile for a given
--   vmnodeid, objectkind, neighbor_scope, and tier. Intended for use by
--   H.Blast / H.SQLiteBlast as a lookup surface.
-- ----------------------------------------------------------------------------
CREATE VIEW IF NOT EXISTS vblast_profile_selector AS
SELECT
    bp.blastprofileid,
    bp.profilekey,
    bp.vmnodeid,
    bp.objectkind,
    bp.neighbor_scope,
    bp.tier,
    bp.max_depth,
    bp.max_distance,
    bp.max_neighbors,
    bp.det_cap,
    bp.cic_cap,
    bp.aos_cap,
    bp.risk_cap,
    bp.w_det,
    bp.w_cic,
    bp.w_aos,
    bp.uec_band_min,
    bp.uec_band_max,
    bp.arr_band_min,
    bp.arr_band_max,
    bp.cdl_cap,
    bp.policy_id
FROM constellation_blast_profile bp;

-- End of labsqlite_blastgraph_v1.sql
