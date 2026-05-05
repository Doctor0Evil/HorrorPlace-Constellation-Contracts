#!/bin/sh
# File: scripts/check_bci_pipeline_wiring.sh
# Purpose: Lint BCI pipeline wiring for orphan stages and missing paths.
# Constraints: Uses only sqlite3, sh, grep, awk, sed (no Rustup/Cargo).
# Exit codes: 0 = success (warnings only), 1 = critical wiring issues found

set -e

DB_PATH="${1:-db/constellation-index.db}"

if [ ! -f "${DB_PATH}" ]; then
    echo "ERROR: Database ${DB_PATH} not found."
    echo "Run scripts/init_constellation_index.sh first."
    exit 1
fi

echo "Checking BCI pipeline wiring integrity..."
echo ""

WARNINGS=0
CRITICAL=0

# Check 1: Find orphan stages (stages with no incoming or outgoing edges)
echo "=== Check 1: Orphan Stages ==="
ORPHANS=$(sqlite3 "${DB_PATH}" "
SELECT s.repo, s.stage_key, s.layer
FROM bci_pipeline_stage s
WHERE s.stage_id NOT IN (SELECT from_stage_id FROM bci_pipeline_edge)
  AND s.stage_id NOT IN (SELECT to_stage_id FROM bci_pipeline_edge);
")

if [ -n "$ORPHANS" ]; then
    echo "WARNING: Found orphan stages (no edges connected):"
    echo "$ORPHANS" | while IFS='|' read -r repo stage layer; do
        echo "  - ${repo}/${stage} (layer: ${layer})"
    done
    WARNINGS=$(echo "$ORPHANS" | wc -l)
else
    echo "OK: No orphan stages found."
fi
echo ""

# Check 2: Verify path exists from ingestion to persistence for main experience types
echo "=== Check 2: Ingestion-to-Persistence Paths ==="

# Check for ai-bci-geometry-request-v1 -> persistence path
PATH_EXISTS=$(sqlite3 "${DB_PATH}" "
SELECT COUNT(*) 
FROM bci_pipeline_edge e
JOIN bci_pipeline_stage s1 ON s1.stage_id = e.from_stage_id
JOIN bci_pipeline_stage s2 ON s2.stage_id = e.to_stage_id
WHERE s1.input_type = 'ai-bci-geometry-request-v1'
  AND s2.layer = 'persistence';
")

if [ "$PATH_EXISTS" -gt 0 ]; then
    echo "OK: Path exists from 'ai-bci-geometry-request-v1' to 'persistence' layer (${PATH_EXISTS} edges)."
else
    echo "CRITICAL: No path from 'ai-bci-geometry-request-v1' to 'persistence' layer!"
    CRITICAL=$((CRITICAL + 1))
fi
echo ""

# Check 3: Verify each layer has at least one stage
echo "=== Check 3: Layer Coverage ==="
LAYERS_WITHOUT_STAGES=$(sqlite3 "${DB_PATH}" "
SELECT 'ingest' as layer
WHERE NOT EXISTS (SELECT 1 FROM bci_pipeline_stage WHERE layer = 'ingest')
UNION ALL
SELECT 'compute'
WHERE NOT EXISTS (SELECT 1 FROM bci_pipeline_stage WHERE layer = 'compute')
UNION ALL
SELECT 'render'
WHERE NOT EXISTS (SELECT 1 FROM bci_pipeline_stage WHERE layer = 'render')
UNION ALL
SELECT 'log'
WHERE NOT EXISTS (SELECT 1 FROM bci_pipeline_stage WHERE layer = 'log')
UNION ALL
SELECT 'persistence'
WHERE NOT EXISTS (SELECT 1 FROM bci_pipeline_stage WHERE layer = 'persistence');
")

if [ -n "$LAYERS_WITHOUT_STAGES" ]; then
    echo "CRITICAL: Missing stages for layers:"
    echo "$LAYERS_WITHOUT_STAGES" | while read -r layer; do
        echo "  - ${layer}"
    done
    CRITICAL=$((CRITICAL + $(echo "$LAYERS_WITHOUT_STAGES" | wc -l)))
else
    echo "OK: All required layers have at least one stage."
fi
echo ""

# Check 4: Cross-repo edge verification
echo "=== Check 4: Cross-Repo Wiring ==="
CROSS_REPO_EDGES=$(sqlite3 "${DB_PATH}" "
SELECT COUNT(*)
FROM bci_pipeline_edge e
JOIN bci_pipeline_stage s1 ON s1.stage_id = e.from_stage_id
JOIN bci_pipeline_stage s2 ON s2.stage_id = e.to_stage_id
WHERE s1.repo != s2.repo;
")

echo "Cross-repo edges: ${CROSS_REPO_EDGES}"
if [ "$CROSS_REPO_EDGES" -eq 0 ]; then
    echo "WARNING: No cross-repo edges defined. Pipeline may be incomplete."
    WARNINGS=$((WARNINGS + 1))
else
    echo "OK: Cross-repo wiring is present."
    
    # Show cross-repo connections
    echo ""
    echo "Cross-repo connections:"
    sqlite3 -column -header "${DB_PATH}" "
    SELECT s1.repo as from_repo, s2.repo as to_repo, e.protocol
    FROM bci_pipeline_edge e
    JOIN bci_pipeline_stage s1 ON s1.stage_id = e.from_stage_id
    JOIN bci_pipeline_stage s2 ON s2.stage_id = e.to_stage_id
    WHERE s1.repo != s2.repo
    LIMIT 10;
    "
fi
echo ""

# Check 5: Protocol diversity
echo "=== Check 5: Protocol Usage ==="
sqlite3 -column -header "${DB_PATH}" "
SELECT protocol, COUNT(*) as count 
FROM bci_pipeline_edge 
GROUP BY protocol 
ORDER BY count DESC;
"
echo ""

# Summary
echo "=== Summary ==="
echo "Warnings: ${WARNINGS}"
echo "Critical issues: ${CRITICAL}"

if [ "$CRITICAL" -gt 0 ]; then
    echo ""
    echo "FAILED: ${CRITICAL} critical wiring issue(s) found."
    echo "Pipeline wiring is incomplete and may break AI-chat navigation."
    exit 1
elif [ "$WARNINGS" -gt 0 ]; then
    echo ""
    echo "PASSED with warnings: ${WARNINGS} warning(s) found."
    echo "Pipeline wiring is functional but could be improved."
    exit 0
else
    echo ""
    echo "PASSED: Pipeline wiring is complete and healthy."
    exit 0
fi
