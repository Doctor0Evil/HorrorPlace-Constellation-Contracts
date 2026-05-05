#!/bin/sh
# File: scripts/check_fieldusage_complete.sh
# Purpose: Verify that must-track fields have both jsonschema and sqltable entries.
# Constraints: Uses only sqlite3, sh, grep, awk, sed (no Rustup/Cargo).
# Exit codes: 0 = success, 1 = missing field coverage

set -e

DB_PATH="${1:-db/constellation-index.db}"

if [ ! -f "${DB_PATH}" ]; then
    echo "ERROR: Database ${DB_PATH} not found."
    echo "Run scripts/init_constellation_index.sh first."
    exit 1
fi

echo "Checking field usage completeness for must-track fields..."
echo ""

# Define must-track fields (critical BCI metrics and invariants)
MUST_TRACK_FIELDS="
bciSummary.stressScore
bciSummary.visualOverloadIndex
bciSummary.startleSpike
invariants.CIC
invariants.DET
invariants.CDL
invariants.ARR
visual.maskRadius
visual.motionSmear
audio.heartbeatGain
can.token.max_gain
palette.swatchIndex
"

MISSING_COUNT=0

for FIELD in $MUST_TRACK_FIELDS; do
    # Check for jsonschema entry
    JSON_COUNT=$(sqlite3 "${DB_PATH}" "SELECT COUNT(*) FROM field_usage WHERE field_path = '${FIELD}' AND location_type = 'json_schema';")
    
    # Check for sqltable entry
    SQL_COUNT=$(sqlite3 "${DB_PATH}" "SELECT COUNT(*) FROM field_usage WHERE field_path = '${FIELD}' AND location_type = 'sql_table';")
    
    if [ "$JSON_COUNT" -eq 0 ]; then
        echo "MISSING json_schema: ${FIELD}"
        MISSING_COUNT=$((MISSING_COUNT + 1))
    fi
    
    if [ "$SQL_COUNT" -eq 0 ]; then
        echo "MISSING sql_table: ${FIELD}"
        MISSING_COUNT=$((MISSING_COUNT + 1))
    fi
done

# Report results
if [ "$MISSING_COUNT" -gt 0 ]; then
    echo ""
    echo "FAILED: ${MISSING_COUNT} field coverage gap(s) found."
    echo "Run scripts/populate_field_usage.sh to add missing entries."
    exit 1
else
    echo "SUCCESS: All must-track fields have both json_schema and sql_table entries."
    
    # Show coverage summary
    echo ""
    echo "Field coverage summary:"
    sqlite3 -column -header "${DB_PATH}" "
    SELECT 
        field_path,
        SUM(CASE WHEN location_type = 'json_schema' THEN 1 ELSE 0 END) as json_schemas,
        SUM(CASE WHEN location_type = 'sql_table' THEN 1 ELSE 0 END) as sql_tables,
        SUM(CASE WHEN location_type = 'rust_struct' THEN 1 ELSE 0 END) as rust_structs
    FROM field_usage
    WHERE field_path IN (
        'bciSummary.stressScore',
        'bciSummary.visualOverloadIndex',
        'bciSummary.startleSpike',
        'invariants.CIC',
        'invariants.DET',
        'invariants.CDL',
        'invariants.ARR',
        'visual.maskRadius',
        'visual.motionSmear',
        'audio.heartbeatGain',
        'can.token.max_gain',
        'palette.swatchIndex'
    )
    GROUP BY field_path;
    "
    
    exit 0
fi
