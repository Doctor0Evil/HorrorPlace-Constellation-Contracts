#!/bin/sh
# File: scripts/check_constellation_index.sh
# Purpose: Validate that all top-level HorrorPlace repo directories have hp_repo entries.
# Constraints: Uses only sqlite3, sh, find, grep, awk, sed (no Rustup/Cargo).
# Exit codes: 0 = success, 1 = missing repos in hp_repo

set -e

DB_PATH="${1:-db/constellation-index.db}"
MANIFEST_DIR="manifests"

if [ ! -f "${DB_PATH}" ]; then
    echo "ERROR: Database ${DB_PATH} not found."
    echo "Run scripts/init_constellation_index.sh first to initialize the constellation index."
    exit 1
fi

echo "Checking constellation index consistency..."

# Get list of repos from manifests (extract repoName from manifest files)
MISSING_COUNT=0
MISSING_REPOS=""

for manifest in ${MANIFEST_DIR}/repo-manifest.*.json; do
    if [ -f "$manifest" ]; then
        # Extract repoName using grep and sed (no jq dependency)
        REPO_NAME=$(grep '"repoName"' "$manifest" | head -1 | sed 's/.*"repoName"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/')
        
        if [ -n "$REPO_NAME" ]; then
            # Check if this repo exists in hp_repo table
            EXISTS=$(sqlite3 "${DB_PATH}" "SELECT COUNT(*) FROM hp_repo WHERE name = '${REPO_NAME}';")
            
            if [ "$EXISTS" -eq 0 ]; then
                echo "MISSING: ${REPO_NAME} (from ${manifest})"
                MISSING_COUNT=$((MISSING_COUNT + 1))
                MISSING_REPOS="${MISSING_REPOS}${REPO_NAME}\n"
            fi
        fi
    fi
done

# Also check for expected core repos that should always be present
CORE_REPOS="Rotting-Visuals-BCI HorrorPlace-Dead-Ledger-Network HorrorPlace-Constellation-Contracts HorrorPlace-Neural-Resonance-Lab HorrorPlace-Spectral-Foundry Codebase-of-Death HorrorPlace-RotCave"

for CORE_REPO in ${CORE_REPOS}; do
    EXISTS=$(sqlite3 "${DB_PATH}" "SELECT COUNT(*) FROM hp_repo WHERE name = '${CORE_REPO}';")
    if [ "$EXISTS" -eq 0 ]; then
        echo "MISSING CORE REPO: ${CORE_REPO}"
        MISSING_COUNT=$((MISSING_COUNT + 1))
        MISSING_REPOS="${MISSING_REPOS}${CORE_REPO}\n"
    fi
done

# Report results
if [ "$MISSING_COUNT" -gt 0 ]; then
    echo ""
    echo "FAILED: ${MISSING_COUNT} repo(s) missing from hp_repo table."
    echo "Run scripts/init_constellation_index.sh to populate missing entries."
    exit 1
else
    echo "SUCCESS: All known repos are present in hp_repo table."
    
    # Show summary
    TOTAL_REPOS=$(sqlite3 "${DB_PATH}" "SELECT COUNT(*) FROM hp_repo;")
    echo "Total repos indexed: ${TOTAL_REPOS}"
    
    # Show role distribution
    echo ""
    echo "Repos by role:"
    sqlite3 -column -header "${DB_PATH}" "SELECT role, COUNT(*) as count FROM hp_repo GROUP BY role ORDER BY count DESC;"
    
    exit 0
fi
