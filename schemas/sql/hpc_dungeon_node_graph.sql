-- schemas/sql/hpc_dungeon_node_graph.sql

-- Core node table: each row corresponds to a dungeon-node-contract instance.
CREATE TABLE IF NOT EXISTS dungeon_nodes (
    node_id         INTEGER PRIMARY KEY,
    node_uid        TEXT NOT NULL UNIQUE,   -- dungeon-node-contract nodeId
    contract_uid    TEXT NOT NULL,          -- link to contracts table / registry id
    region_ref      TEXT NOT NULL,          -- regionContractCard id or ref
    role            TEXT NOT NULL,          -- "spawn","hub","liminal","deadend",...
    repo_slug       TEXT NOT NULL,          -- owning repo, e.g. "Horror.Place"
    tier            TEXT NOT NULL           -- "T1","T2","T3"
);

CREATE INDEX IF NOT EXISTS idx_dungeon_nodes_uid
    ON dungeon_nodes(node_uid);

CREATE INDEX IF NOT EXISTS idx_dungeon_nodes_region
    ON dungeon_nodes(region_ref);


-- Edge table: adjacency and liminal markers between nodes.
CREATE TABLE IF NOT EXISTS dungeon_edges (
    edge_id         INTEGER PRIMARY KEY,
    src_node_id     INTEGER NOT NULL,
    dst_node_id     INTEGER NOT NULL,
    edge_kind       TEXT NOT NULL,          -- "twoway","locked","rituallocked","hidden"
    liminal_tag     TEXT,                   -- "doorway","stairdown","bridge",...
    directed        INTEGER NOT NULL DEFAULT 0,  -- 0=undirected,1=directed

    FOREIGN KEY (src_node_id) REFERENCES dungeon_nodes(node_id),
    FOREIGN KEY (dst_node_id) REFERENCES dungeon_nodes(node_id)
);

CREATE INDEX IF NOT EXISTS idx_dungeon_edges_src
    ON dungeon_edges(src_node_id);

CREATE INDEX IF NOT EXISTS idx_dungeon_edges_dst
    ON dungeon_edges(dst_node_id);


-- Blast-profile table: precomputed deltas and risk on each edge.
CREATE TABLE IF NOT EXISTS dungeon_edge_blast_profile (
    edge_id         INTEGER PRIMARY KEY,
    cic_delta       REAL NOT NULL DEFAULT 0.0,
    aos_delta       REAL NOT NULL DEFAULT 0.0,
    det_delta       REAL NOT NULL DEFAULT 0.0,
    hvf_delta       REAL NOT NULL DEFAULT 0.0,
    lsg_delta       REAL NOT NULL DEFAULT 0.0,

    uec_delta       REAL NOT NULL DEFAULT 0.0,
    emd_delta       REAL NOT NULL DEFAULT 0.0,
    stci_delta      REAL NOT NULL DEFAULT 0.0,
    cdl_delta       REAL NOT NULL DEFAULT 0.0,
    arr_delta       REAL NOT NULL DEFAULT 0.0,

    shci_delta      REAL NOT NULL DEFAULT 0.0,

    det_cap_violate INTEGER NOT NULL DEFAULT 0,   -- 1 if this edge alone would exceed DET cap
    cic_aos_risk    REAL NOT NULL DEFAULT 0.0     -- combined CIC/AOS risk contribution
);

CREATE INDEX IF NOT EXISTS idx_blast_risk
    ON dungeon_edge_blast_profile(cic_aos_risk);
