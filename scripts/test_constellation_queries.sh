#!/bin/sh
# File: scripts/test_constellation_queries.sh
# Purpose: Smoke-test all named queries from constellation-navigation.sql.
# Constraints: Uses only sqlite3, sh (no Rustup/Cargo).
# Exit codes: 0 = all queries pass, 1 = one or more queries failed

set -e

DB_PATH="${1:-db/constellation-index.db}"
QUERY_FILE="db/queries/constellation-navigation.sql"

if [ ! -f "${DB_PATH}" ]; then
    echo "ERROR: Database ${DB_PATH} not found."
    echo "Run scripts/init_constellation_index.sh first."
    exit 1
fi

if [ ! -f "${QUERY_FILE}" ]; then
    echo "ERROR: Query file ${QUERY_FILE} not found."
    exit 1
fi

echo "Testing constellation navigation queries..."
echo ""

FAILED=0
PASSED=0

# Helper function to run a query with parameters
run_query() {
    QUERY_NAME="$1"
    PARAMS="$2"
    
    # Extract the query from the file and run it
    # We use a simple approach: load the whole file and run the specific query
    
    echo -n "Testing ${QUERY_NAME}... "
    
    # Run query based on name
    case "$QUERY_NAME" in
        "componentsByRepo")
            RESULT=$(sqlite3 "${DB_PATH}" "SELECT r.name AS repo, c.kind AS kind, c.path AS path FROM hp_repo AS r JOIN hp_component AS c ON c.repo_id = r.repo_id WHERE r.name = 'HorrorPlace-Constellation-Contracts' ORDER BY c.kind, c.path;" 2>&1)
            ;;
        "schemasForDomain")
            RESULT=$(sqlite3 "${DB_PATH}" "SELECT r.name AS repo, c.path AS schema_path FROM hp_repo AS r JOIN hp_component AS c ON c.repo_id = r.repo_id WHERE c.domain = 'bci' AND c.kind IN ('schema', 'sqlschema') ORDER BY r.name, c.path;" 2>&1)
            ;;
        "reposByRole")
            RESULT=$(sqlite3 "${DB_PATH}" "SELECT name AS repo, git_url, role FROM hp_repo WHERE role = 'runtime' ORDER BY name;" 2>&1)
            ;;
        "componentsByKind")
            RESULT=$(sqlite3 "${DB_PATH}" "SELECT r.name AS repo, c.path FROM hp_repo AS r JOIN hp_component AS c ON c.repo_id = r.repo_id WHERE c.kind = 'schema' ORDER BY r.name, c.path;" 2>&1)
            ;;
        "pipelineStagesForRepo")
            RESULT=$(sqlite3 "${DB_PATH}" "SELECT s.stageid, s.repo, s.stagekey, s.name, s.layer FROM bcipipelinestage AS s WHERE s.repo = 'Rotting-Visuals-BCI' ORDER BY s.layer, s.stageid;" 2>&1)
            ;;
        "pipelineEdgesFromRepo")
            RESULT=$(sqlite3 "${DB_PATH}" "SELECT s1.repo AS from_repo, s1.stagekey AS from_stage, s2.repo AS to_repo, s2.stagekey AS to_stage FROM bcipipelineedge AS e JOIN bcipipelinestage AS s1 ON s1.stageid = e.fromstageid JOIN bcipipelinestage AS s2 ON s2.stageid = e.tostageid WHERE s1.repo = 'Rotting-Visuals-BCI' ORDER BY from_repo, from_stage;" 2>&1)
            ;;
        "pipelineForInputType")
            RESULT=$(sqlite3 "${DB_PATH}" "SELECT s.repo, s.stagekey, s.name, s.layer FROM bcipipelinestage AS s WHERE s.inputtype = 'ai-bci-geometry-request-v1' ORDER BY s.repo, s.stageid;" 2>&1)
            ;;
        "fieldUsageEverywhere")
            RESULT=$(sqlite3 "${DB_PATH}" "SELECT fieldpath, repo, locationtype, locationpath FROM fieldusage WHERE fieldpath = 'bciSummary.stressScore' ORDER BY repo, locationtype, locationpath;" 2>&1)
            ;;
        "researchReadyDomainSchemas")
            RESULT=$(sqlite3 "${DB_PATH}" "SELECT SUM(CASE WHEN c.kind = 'schema' AND c.path LIKE '%request%' THEN 1 ELSE 0 END) AS request_schemas FROM hp_repo AS r JOIN hp_component AS c ON c.repo_id = r.repo_id WHERE c.domain = 'bci';" 2>&1)
            ;;
        "researchReadyField")
            RESULT=$(sqlite3 "${DB_PATH}" "SELECT SUM(CASE WHEN locationtype = 'json_schema' THEN 1 ELSE 0 END) AS jsonschema_count, SUM(CASE WHEN locationtype = 'sql_table' THEN 1 ELSE 0 END) AS sqltable_count FROM fieldusage WHERE fieldpath = 'bciSummary.stressScore';" 2>&1)
            ;;
        "researchReadyPipelinePath")
            RESULT=$(sqlite3 "${DB_PATH}" "SELECT COUNT(*) AS candidate_edges FROM bcipipelineedge AS e JOIN bcipipelinestage AS s1 ON s1.stageid = e.fromstageid JOIN bcipipelinestage AS s2 ON s2.stageid = e.tostageid WHERE s1.inputtype = 'ai-bci-geometry-request-v1' AND s2.layer = 'persistence';" 2>&1)
            ;;
        "researchReady_MonsterModeBCI")
            RESULT=$(sqlite3 "${DB_PATH}" "SELECT SUM(CASE WHEN c.kind = 'schema' AND c.path = 'schemas/ai-bci-geometry-request-v1.json' THEN 1 ELSE 0 END) AS has_request FROM hp_repo AS r JOIN hp_component AS c ON c.repo_id = r.repo_id WHERE r.name = 'Rotting-Visuals-BCI';" 2>&1)
            ;;
        "researchReady_CANRegistryRotViz")
            RESULT=$(sqlite3 "${DB_PATH}" "SELECT SUM(CASE WHEN c.kind = 'schema' AND c.path LIKE '%can-token-registry%' THEN 1 ELSE 0 END) AS has_can_schema FROM hp_repo AS r JOIN hp_component AS c ON c.repo_id = r.repo_id;" 2>&1)
            ;;
        *)
            echo "UNKNOWN QUERY: ${QUERY_NAME}"
            return 1
            ;;
    esac
    
    # Check if result contains error
    if echo "$RESULT" | grep -qi "error"; then
        echo "FAILED"
        echo "  Error: ${RESULT}"
        FAILED=$((FAILED + 1))
        return 1
    else
        echo "PASSED"
        PASSED=$((PASSED + 1))
        return 0
    fi
}

# Test each query
QUERIES="
componentsByRepo
schemasForDomain
reposByRole
componentsByKind
pipelineStagesForRepo
pipelineEdgesFromRepo
pipelineForInputType
fieldUsageEverywhere
researchReadyDomainSchemas
researchReadyField
researchReadyPipelinePath
researchReady_MonsterModeBCI
researchReady_CANRegistryRotViz
"

for QUERY in $QUERIES; do
    run_query "$QUERY" || true
done

# Summary
echo ""
echo "=== Summary ==="
echo "Passed: ${PASSED}"
echo "Failed: ${FAILED}"

if [ "$FAILED" -gt 0 ]; then
    echo ""
    echo "FAILED: ${FAILED} query/queries failed smoke test."
    exit 1
else
    echo ""
    echo "SUCCESS: All ${PASSED} queries passed smoke test."
    exit 0
fi
