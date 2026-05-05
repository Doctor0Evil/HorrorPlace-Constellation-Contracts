#!/bin/sh
# File: scripts/check_tooling_usage.sh
# Purpose: Scan for prohibited tooling commands (rustup, cargo, etc.) in scripts and CI config.
# Constraints: Uses only grep, sh (no Rustup/Cargo).
# Exit codes: 0 = no prohibited tools found, 1 = prohibited tools detected

set -e

echo "Checking for prohibited tooling usage..."
echo ""

PROHIBITED_PATTERNS="
rustup
cargo build
cargo check
cargo test
cargo run
cargo install
npm install
yarn install
pnpm install
pip install
cmake --build
make all
make build
"

FOUND_ISSUES=0

# Files to scan
SCAN_DIRS="scripts .github ci .ci workflows"
SCAN_EXTENSIONS="*.sh *.yml *.yaml *.toml"

echo "Scanning for prohibited commands..."
echo ""

for PATTERN in $PROHIBITED_PATTERNS; do
    # Skip empty patterns
    [ -z "$PATTERN" ] && continue
    
    # Search in scripts directory
    MATCHES=$(find scripts -type f \( -name "*.sh" -o -name "*.yml" -o -name "*.yaml" \) -exec grep -l "$PATTERN" {} \; 2>/dev/null || true)
    
    if [ -n "$MATCHES" ]; then
        echo "FOUND PROHIBITED: '${PATTERN}' in:"
        echo "$MATCHES" | while read -r FILE; do
            LINE_NUM=$(grep -n "$PATTERN" "$FILE" | head -1 | cut -d: -f1)
            CONTEXT=$(grep -n "$PATTERN" "$FILE" | head -1)
            echo "  - ${FILE}:${LINE_NUM}: ${CONTEXT}"
        done
        FOUND_ISSUES=$((FOUND_ISSUES + 1))
    fi
done

# Also check for Cargo.toml references that imply building
if [ -f "Cargo.toml" ]; then
    # This is OK - we allow Cargo.toml to exist for documentation purposes
    # But we should not run cargo commands
    echo "Note: Cargo.toml found (allowed for static indexing, but cargo commands are prohibited)"
fi

# Check CI workflow files if they exist
if [ -d ".github/workflows" ]; then
    for WF in .github/workflows/*.yml .github/workflows/*.yaml; do
        [ -f "$WF" ] || continue
        
        if grep -q "rustup\|cargo build\|cargo check\|cargo test" "$WF" 2>/dev/null; then
            echo "FOUND PROHIBITED in CI workflow ${WF}:"
            grep -n "rustup\|cargo build\|cargo check\|cargo test" "$WF" | head -5
            FOUND_ISSUES=$((FOUND_ISSUES + 1))
        fi
    done
fi

# Summary
echo ""
echo "=== Summary ==="

if [ "$FOUND_ISSUES" -gt 0 ]; then
    echo "FAILED: Found ${FOUND_ISSUES} prohibited tooling reference(s)."
    echo ""
    echo "This repository enforces a native-tools-only policy."
    echo "Allowed tools: sqlite3, sh, find, grep, awk, sed"
    echo "Prohibited tools: rustup, cargo, npm, yarn, pip (for builds)"
    echo ""
    echo "See docs/agent-tooling-policy.md for details."
    exit 1
else
    echo "PASSED: No prohibited tooling usage detected."
    echo ""
    echo "Allowed tools verified: sqlite3, sh, find, grep, awk, sed"
    exit 0
fi
