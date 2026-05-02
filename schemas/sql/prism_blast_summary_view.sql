-- schemas/sql/prism_blast_summary_view.sql

-- View: v_prism_blast_summary
-- Summarizes blast neighborhoods for each prism_id.

CREATE VIEW IF NOT EXISTS v_prism_blast_summary AS
WITH
    -- 1. Per-prism aggregates across its blast neighborhood.
    per_prism AS (
        SELECT
            bn.prism_id                      AS prism_id,
            COUNT(*)                         AS blast_size,
            MAX(bn.risk_score)              AS max_risk_score,
            MAX(bn.det_sum)                 AS max_det_delta,
            AVG(bn.uec)                     AS avg_uec,
            AVG(bn.arr)                     AS avg_arr,
            AVG(bn.cdl)                     AS avg_cdl
        FROM blast_neighborhood AS bn
        GROUP BY bn.prism_id
    ),

    -- 2. Per-prism, per-repo exposure counts.
    per_prism_repo AS (
        SELECT
            bn.prism_id                      AS prism_id,
            bn.repo_slug                     AS repo_slug,
            COUNT(*)                         AS repo_count
        FROM blast_neighborhood AS bn
        GROUP BY bn.prism_id, bn.repo_slug
    ),

    -- 3. Join counts to get exposure fractions.
    repo_exposure AS (
        SELECT
            r.prism_id,
            r.repo_slug,
            CAST(r.repo_count AS REAL) / CAST(p.blast_size AS REAL) AS fraction
        FROM per_prism_repo AS r
        JOIN per_prism AS p
          ON p.prism_id = r.prism_id
    ),

    -- 4. Aggregate repo exposure into JSON arrays per prism.
    repo_exposure_json AS (
        SELECT
            re.prism_id AS prism_id,
            json_group_array(
                json_object(
                    'repoSlug', re.repo_slug,
                    'fraction', re.fraction
                )
            ) AS repo_exposure_json
        FROM repo_exposure AS re
        GROUP BY re.prism_id
    )

-- 5. Final view: join prism contracts, per-prism metrics, and exposure.
SELECT
    pc.prism_id                               AS prismId,
    pc.policy_id                              AS policyId,
    pc.origin_node_id                         AS originNodeId,
    pc.origin_repo_slug                       AS originRepoSlug,

    p.blast_size                              AS blastSize,

    json_object(
        'maxRiskScore', p.max_risk_score,
        'maxDetDelta', p.max_det_delta
    )                                         AS riskExtremes,

    json_object(
        'avgUEC', p.avg_uec,
        'avgARR', p.avg_arr,
        'avgCDL', p.avg_cdl
    )                                         AS engagementAverages,

    rej.repo_exposure_json                    AS repoExposure,

    datetime('now')                           AS computedAt

FROM prism_contracts AS pc
JOIN per_prism AS p
  ON p.prism_id = pc.prism_id
LEFT JOIN repo_exposure_json AS rej
  ON rej.prism_id = pc.prism_id;
