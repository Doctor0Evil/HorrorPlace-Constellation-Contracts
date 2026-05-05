-- File db/queries/bci_pipeline_inspection.sql
-- Target repo Doctor0Evil/HorrorPlace-Constellation-Contracts
-- Purpose Ready-made, low-token inspection queries for BCI pipeline stages and edges.

PRAGMA foreign_keys = ON;

------------------------------------------------------------
-- 1. Stage-level inspection
------------------------------------------------------------

-- listStagesByRepo
-- Input :repoName
-- Usage: List all stages in a repo with safety and trust metadata.
SELECT
    s.stageid,
    s.repo,
    s.stagekey,
    s.name,
    s.layer,
    s.inputtype,
    s.outputtype,
    s.primaryfile,
    s.latency_budget_ms,
    s.jitter_budget_ms,
    s.max_qps,
    s.critical_safety,
    s.failure_mode,
    s.trust_zone,
    s.handles_can_tokens
FROM bci_pipeline_stage AS s
WHERE s.repo = :repoName
ORDER BY s.layer, s.stagekey;

-- listCriticalSafetyStages
-- Input: none
-- Usage: Show all stages marked as safety-critical.
SELECT
    s.stageid,
    s.repo,
    s.stagekey,
    s.name,
    s.layer,
    s.inputtype,
    s.outputtype,
    s.primaryfile,
    s.trust_zone,
    s.failure_mode
FROM bci_pipeline_stage AS s
WHERE s.critical_safety = 1
ORDER BY s.repo, s.layer, s.stagekey;

-- listCanHandlingStages
-- Input: none
-- Usage: Show stages that directly handle CAN tokens.
SELECT
    s.stageid,
    s.repo,
    s.stagekey,
    s.name,
    s.layer,
    s.trust_zone,
    s.critical_safety,
    s.primaryfile
FROM bci_pipeline_stage AS s
WHERE s.handles_can_tokens = 1
ORDER BY s.repo, s.layer, s.stagekey;

-- listStagesWithLatencyBudgets
-- Input: none
-- Usage: Show stages that declare latency budgets for audits.
SELECT
    s.stageid,
    s.repo,
    s.stagekey,
    s.name,
    s.layer,
    s.latency_budget_ms,
    s.jitter_budget_ms,
    s.max_qps
FROM bci_pipeline_stage AS s
WHERE s.latency_budget_ms IS NOT NULL
ORDER BY s.repo, s.layer, s.stagekey;

------------------------------------------------------------
-- 2. Edge-level inspection
------------------------------------------------------------

-- listEdgesForRepo
-- Input :repoName
-- Usage: Show edges where the from-stage belongs to a given repo.
SELECT
    e.edgeid,
    s1.repo      AS from_repo,
    s1.stagekey  AS from_stage,
    s2.repo      AS to_repo,
    s2.stagekey  AS to_stage,
    e.protocol_type,
    e.protocol_detail,
    e.crosses_trust_boundary,
    e.description
FROM bci_pipeline_edge AS e
JOIN bci_pipeline_stage AS s1 ON s1.stageid = e.fromstageid
JOIN bci_pipeline_stage AS s2 ON s2.stageid = e.tostageid
WHERE s1.repo = :repoName
ORDER BY from_repo, from_stage, to_repo, to_stage;

-- listTrustBoundaryEdges
-- Input: none
-- Usage: Show edges that cross an explicit trust boundary.
SELECT
    e.edgeid,
    s1.repo      AS from_repo,
    s1.stagekey  AS from_stage,
    s1.trust_zone AS from_trust_zone,
    s2.repo      AS to_repo,
    s2.stagekey  AS to_stage,
    s2.trust_zone AS to_trust_zone,
    e.protocol_type,
    e.protocol_detail,
    e.crosses_trust_boundary,
    e.description
FROM bci_pipeline_edge AS e
JOIN bci_pipeline_stage AS s1 ON s1.stageid = e.fromstageid
JOIN bci_pipeline_stage AS s2 ON s2.stageid = e.tostageid
WHERE e.crosses_trust_boundary = 1
ORDER BY from_repo, from_stage, to_repo, to_stage;

-- listCanHandlingBoundaryEdges
-- Input: none
-- Usage: Show edges where at least one side handles CAN tokens and the edge crosses a trust boundary.
SELECT
    e.edgeid,
    s1.repo      AS from_repo,
    s1.stagekey  AS from_stage,
    s1.trust_zone AS from_trust_zone,
    s1.handles_can_tokens AS from_handles_can,
    s2.repo      AS to_repo,
    s2.stagekey  AS to_stage,
    s2.trust_zone AS to_trust_zone,
    s2.handles_can_tokens AS to_handles_can,
    e.protocol_type,
    e.protocol_detail,
    e.crosses_trust_boundary,
    e.description
FROM bci_pipeline_edge AS e
JOIN bci_pipeline_stage AS s1 ON s1.stageid = e.fromstageid
JOIN bci_pipeline_stage AS s2 ON s2.stageid = e.tostageid
WHERE e.crosses_trust_boundary = 1
  AND (s1.handles_can_tokens = 1 OR s2.handles_can_tokens = 1)
ORDER BY from_repo, from_stage, to_repo, to_stage;

-- listNetworkEdges
-- Input: none
-- Usage: Inspect all network-based edges.
SELECT
    e.edgeid,
    s1.repo      AS from_repo,
    s1.stagekey  AS from_stage,
    s2.repo      AS to_repo,
    s2.stagekey  AS to_stage,
    e.protocol_type,
    e.protocol_detail,
    e.crosses_trust_boundary,
    e.description
FROM bci_pipeline_edge AS e
JOIN bci_pipeline_stage AS s1 ON s1.stageid = e.fromstageid
JOIN bci_pipeline_stage AS s2 ON s2.stageid = e.tostageid
WHERE e.protocol_type = 'network'
ORDER BY from_repo, from_stage, to_repo, to_stage;

------------------------------------------------------------
-- 3. Path inspection (ingest → ledger / persistence)
------------------------------------------------------------

-- simplePathsIngestToLayer
-- Input :startLayer, :endLayer
-- Usage: List edges that participate in any path from stages in startLayer to stages in endLayer.
-- Note: This is a one-hop helper; multi-hop path walking can be done in shell or app code.

SELECT
    s1.stageid    AS from_stageid,
    s1.repo       AS from_repo,
    s1.stagekey   AS from_stagekey,
    s1.layer      AS from_layer,
    s2.stageid    AS to_stageid,
    s2.repo       AS to_repo,
    s2.stagekey   AS to_stagekey,
    s2.layer      AS to_layer,
    e.edgeid,
    e.protocol_type,
    e.crosses_trust_boundary,
    e.description
FROM bci_pipeline_edge AS e
JOIN bci_pipeline_stage AS s1 ON s1.stageid = e.fromstageid
JOIN bci_pipeline_stage AS s2 ON s2.stageid = e.tostageid
WHERE s1.layer = :startLayer
  AND s2.layer = :endLayer
ORDER BY from_repo, from_stagekey, to_repo, to_stagekey;

-- ingestToLedgerOneHop
-- Input: none
-- Usage: Quick sanity check for direct ingest → ledger edges (if any).
SELECT
    s1.repo     AS from_repo,
    s1.stagekey AS from_stage,
    s1.layer    AS from_layer,
    s2.repo     AS to_repo,
    s2.stagekey AS to_stage,
    s2.layer    AS to_layer,
    e.protocol_type,
    e.crosses_trust_boundary,
    e.description
FROM bci_pipeline_edge AS e
JOIN bci_pipeline_stage AS s1 ON s1.stageid = e.fromstageid
JOIN bci_pipeline_stage AS s2 ON s2.stageid = e.tostageid
WHERE s1.layer = 'ingest'
  AND s2.layer = 'ledger'
ORDER BY from_repo, from_stage, to_repo, to_stage;

------------------------------------------------------------
-- 4. Safety / latency audit helpers
------------------------------------------------------------

-- stagesWithTightLatencyBudget
-- Input :maxBudget
-- Usage: Find stages with latency budgets at or below a threshold (e.g. 5 ms).
SELECT
    s.stageid,
    s.repo,
    s.stagekey,
    s.layer,
    s.latency_budget_ms,
    s.jitter_budget_ms,
    s.critical_safety
FROM bci_pipeline_stage AS s
WHERE s.latency_budget_ms IS NOT NULL
  AND s.latency_budget_ms <= :maxBudget
ORDER BY s.latency_budget_ms ASC, s.repo, s.stagekey;

-- safetyCriticalWithLooseFailureMode
-- Input: none
-- Usage: Flag safety-critical stages that are configured as fail_open or missing a failure_mode.
SELECT
    s.stageid,
    s.repo,
    s.stagekey,
    s.layer,
    s.failure_mode,
    s.trust_zone,
    s.handles_can_tokens
FROM bci_pipeline_stage AS s
WHERE s.critical_safety = 1
  AND (s.failure_mode IS NULL OR s.failure_mode = 'fail_open')
ORDER BY s.repo, s.stagekey;

-- canHandlingInExternalZones
-- Input: none
-- Usage: Highlight stages that handle CAN tokens outside the core trust zone.
SELECT
    s.stageid,
    s.repo,
    s.stagekey,
    s.layer,
    s.trust_zone,
    s.critical_safety,
    s.primaryfile
FROM bci_pipeline_stage AS s
WHERE s.handles_can_tokens = 1
  AND (s.trust_zone IS NULL OR s.trust_zone <> 'core')
ORDER BY s.repo, s.stagekey;
