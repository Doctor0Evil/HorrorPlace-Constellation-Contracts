#!/usr/bin/env bash
set -euo pipefail

# Horror$Place Constellation Guardrail: Registry Lint Pre-Commit Hook
# Enforces NDJSON registry rules on staged files: ID uniqueness, required fields, schemaref prefix, and reference presence.

echo "📋 Running pre-commit registry lint..."

STAGED_FILES=$(git diff --cached --name-only --diff-filter=ACM | grep -E '\.ndjson$' || true)

if [ -z "$STAGED_FILES" ]; then
  exit 0
fi

python3 - "$STAGED_FILES" <<'PYCODE'
import json
import sys
import re
from pathlib import Path
import subprocess

def get_staged_content(file_path: str) -> str:
    try:
        result = subprocess.run(["git", "show", f":{file_path}"], capture_output=True, text=True, check=True)
        return result.stdout
    except subprocess.CalledProcessError as e:
        print(f"ERROR: Could not read staged content for {file_path}", file=sys.stderr)
        return ""

def main():
    staged_list = sys.argv[1].splitlines()
    seen_ids = set()
    errors_found = False
    total_entries = 0

    for fpath in staged_list:
        content = get_staged_content(fpath)
        if not content.strip():
            continue

        for i, line in enumerate(content.splitlines()):
            line = line.strip()
            if not line:
                continue
            try:
                entry = json.loads(line)
            except json.JSONDecodeError as e:
                print(f"❌ INVALID JSON in {fpath} line {i+1}: {e}")
                errors_found = True
                continue

            total_entries += 1

            # 1. Required fields
            required = ['id', 'schemaref', 'tier', 'metadata']
            for field in required:
                if field not in entry:
                    print(f"❌ MISSING FIELD '{field}' in {fpath} line {i+1}")
                    errors_found = True

            # 2. ID Uniqueness
            entry_id = entry.get('id')
            if entry_id:
                if entry_id in seen_ids:
                    print(f"❌ DUPLICATE ID '{entry_id}' in {fpath} line {i+1}")
                    errors_found = True
                else:
                    seen_ids.add(entry_id)

            # 3. Schemaref Prefix
            schemaref = entry.get('schemaref', '')
            if not re.match(r'^schema:Horror\.Place/.+', schemaref):
                print(f"❌ INVALID SCHEMA PREFIX '{schemaref}' in {fpath} line {i+1}. Must start with 'schema:Horror.Place/'")
                errors_found = True

            # 4. Tier Range
            tier = entry.get('tier')
            if not isinstance(tier, int) or tier not in [1, 2, 3]:
                print(f"❌ INVALID TIER '{tier}' in {fpath} line {i+1}. Must be 1, 2, or 3.")
                errors_found = True

            # 5. Reference Field Presence
            if not any(k in entry for k in ['artifactid', 'cid', 'deadledgerref']):
                print(f"❌ MISSING REFERENCE (artifactid/cid/deadledgerref) in {fpath} line {i+1}")
                errors_found = True

            # 6. Metadata Status
            meta = entry.get('metadata', {})
            if isinstance(meta, dict):
                status = meta.get('status')
                if status not in ['draft', 'active', 'archived']:
                    print(f"❌ INVALID METADATA.STATUS '{status}' in {fpath} line {i+1}")
                    errors_found = True

    if errors_found:
        print(f"\n🚫 Pre-commit registry lint FAILED. Checked {total_entries} entries.")
        sys.exit(1)
    else:
        print(f"✅ Pre-commit registry lint PASSED. Checked {total_entries} entries.")
        sys.exit(0)

if __name__ == "__main__":
    main()
PYCODE
