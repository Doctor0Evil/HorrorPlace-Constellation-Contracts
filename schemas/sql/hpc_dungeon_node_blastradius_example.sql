-- schemas/sql/hpc_dungeon_node_blastradius_example.sql

-- Parameters expected:
--   :start_node_id   INTEGER  -- dungeon_nodes.node_id
--   :max_depth       INTEGER  -- absolute traversal depth cap
--   :det_cap         REAL     -- maximum allowed DET delta along path
--   :risk_cap        REAL     -- maximum allowed combined CIC/AOS risk

WITH RECURSIVE
    frontier AS (
        -- Seed: start from the node itself, depth 0, no cumulative deltas.
        SELECT
            dn.node_id              AS node_id,
            dn.node_uid             AS node_uid,
            0                       AS depth,
            0.0                     AS det_sum,
            0.0                     AS cic_aos_risk_sum,
            CAST('/' || dn.node_id || '/' AS TEXT) AS visited
        FROM dungeon_nodes AS dn
        WHERE dn.node_id = :start_node_id

        UNION ALL

        -- Expand frontier: follow edges outwards, respecting depth and caps.
        SELECT
            next_node.node_id       AS node_id,
            next_node.node_uid      AS node_uid,
            f.depth + 1             AS depth,
            f.det_sum + bp.det_delta AS det_sum,
            f.cic_aos_risk_sum + bp.cic_aos_risk AS cic_aos_risk_sum,
            f.visited || next_node.node_id || '/' AS visited
        FROM frontier AS f
        JOIN dungeon_edges AS e
          ON e.src_node_id = f.node_id
        JOIN dungeon_edge_blast_profile AS bp
          ON bp.edge_id = e.edge_id
        JOIN dungeon_nodes AS next_node
          ON next_node.node_id = e.dst_node_id
        WHERE f.depth < :max_depth
          -- Avoid simple cycles by checking visited path.
          AND INSTR(f.visited, '/' || next_node.node_id || '/') = 0
          -- Hard safety caps along the path.
          AND (f.det_sum + bp.det_delta) <= :det_cap
          AND (f.cic_aos_risk_sum + bp.cic_aos_risk) <= :risk_cap
    ),
    aggregated AS (
        -- One row per reachable node, taking the minimal depth and
        -- associated cumulative metrics.
        SELECT
            node_id,
            node_uid,
            MIN(depth)            AS min_depth,
            MAX(depth)            AS max_depth,
            MIN(det_sum)          AS min_det_sum,
            MAX(det_sum)          AS max_det_sum,
            MIN(cic_aos_risk_sum) AS min_risk_sum,
            MAX(cic_aos_risk_sum) AS max_risk_sum
        FROM frontier
        GROUP BY node_id, node_uid
    )
SELECT
    -- Overall radius: the maximum depth over all reachable nodes.
    (SELECT MAX(min_depth) FROM aggregated) AS safe_radius,

    -- Optional: serialize neighborhood summary as JSON if desired.
    -- For clarity, this example just returns the per-node aggregates.
    node_id,
    node_uid,
    min_depth,
    max_depth,
    min_det_sum,
    max_det_sum,
    min_risk_sum,
    max_risk_sum
FROM aggregated
ORDER BY min_depth ASC, node_id ASC;
