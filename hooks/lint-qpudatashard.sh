#!/usr/bin/env bash
set -euo pipefail

# Horror$Place Constellation Guardrail: QPU Data Shard Lint Pre-Commit Hook
# Validates staged .qpudatashard.ndjson files for schema compliance, ranges, and origin integrity.

echo "📊 Running pre-commit QPU shard lint..."

STAGED_FILES=$(git diff --cached --name-only --diff-filter=ACM | grep -E '\.qpudatashard\.ndjson$' || true)

if [ -z "$STAGED_FILES" ]; then
  exit 0
fi

python3 - "$STAGED_FILES" <<'PYCODE'
import json
import sys
import re
import subprocess
from pathlib import Path

def get_staged_content(file_path: str) -> str:
    try:
        result = subprocess.run(["git", "show", f":{file_path}"], capture_output=True, text=True, check=True)
        return result.stdout
    except subprocess.CalledProcessError as e:
        print(f"ERROR: Could not read staged content for {file_path}", file=sys.stderr)
        return ""

INVARIANTS_0_1 = ['CIC', 'MDI', 'AOS', 'RRM', 'FCF', 'SPR', 'RWF', 'HVF', 'LSG', 'SHCI']
INVARIANTS_0_10 = ['DET']
METRICS_0_1 = ['UEC', 'EMD', 'STCI', 'CDL', 'ARR']

def main():
    staged_list = sys.argv[1].splitlines()
    errors_found = False
    total_shards = 0

    for fpath in staged_list:
        content = get_staged_content(fpath)
        if not content.strip():
            continue

        lines = content.splitlines()
        for i, line in enumerate(lines):
            line = line.strip()
            if not line:
                continue
            
            try:
                shard = json.loads(line)
            except json.JSONDecodeError as e:
                print(f"❌ INVALID JSON in {fpath} line {i+1}: {e}")
                errors_found = True
                continue

            total_shards += 1
            shard_id = shard.get('shardId', f'Line {i+1}')

            # 1. Schemaref
            schemaref = shard.get('schemaref')
            if schemaref != 'schema:Horror.Place/constellation/qpudatashard-v1':
                print(f"❌ INVALID SCHEMA REF in {fpath} line {i+1}: '{schemaref}'")
                errors_found = True

            # 2. Origin
            origin = shard.get('origin')
            if not isinstance(origin, dict):
                print(f"❌ MISSING ORIGIN in {fpath} line {i+1}")
                errors_found = True
            elif not origin.get('agentId') or not origin.get('sourceRepo'):
                print(f"❌ ORIGIN MISSING REQUIRED FIELDS in {fpath} line {i+1}")
                errors_found = True

            # 3. Payload
            payload = shard.get('payload')
            if not isinstance(payload, dict):
                print(f"❌ MISSING PAYLOAD in {fpath} line {i+1}")
                errors_found = True
            else:
                p_type = payload.get('type')
                valid_types = ['region-metrics', 'persona-metrics', 'schema-impact', 'drift-alert', 'experiment-result']
                if p_type not in valid_types:
                    print(f"❌ INVALID PAYLOAD.TYPE in {fpath} line {i+1}: '{p_type}'")
                    errors_found = True

                # Invariant Ranges
                invs = payload.get('invariants', {})
                if isinstance(invs, dict):
                    for k, v in invs.items():
                        if k in INVARIANTS_0_1 and (v < 0 or v > 1):
                            print(f"❌ OUT OF RANGE INVARIANT '{k}={v}' in {fpath} line {i+1} [0-1]")
                            errors_found = True
                        elif k in INVARIANTS_0_10 and (v < 0 or v > 10):
                            print(f"❌ OUT OF RANGE INVARIANT '{k}={v}' in {fpath} line {i+1} [0-10]")
                            errors_found = True

                # Metric Ranges
                mets = payload.get('metrics', {})
                if isinstance(mets, dict):
                    for k, v in mets.items():
                        if k in METRICS_0_1 and (v < 0 or v > 1):
                            print(f"❌ OUT OF RANGE METRIC '{k}={v}' in {fpath} line {i+1} [0-1]")
                            errors_found = True

    if errors_found:
        print(f"\n🚫 Pre-commit QPU shard lint FAILED. Checked {total_shards} shards.")
        sys.exit(1)
    else:
        print(f"✅ Pre-commit QPU shard lint PASSED. Checked {total_shards} shards.")
        sys.exit(0)

if __name__ == "__main__":
    main()
PYCODE
