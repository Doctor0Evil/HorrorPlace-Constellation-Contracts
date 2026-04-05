#!/usr/bin/env bash
set -euo pipefail

# Horror$Place Constellation Guardrail: Prism Envelope Lint Pre-Commit Hook
# Validates staged envelope files for max artifacts, role enums, and constraint compliance.

echo "📦 Running pre-commit prism envelope lint..."

# Target .ai/*.json or envelope*.json files in staging
STAGED_FILES=$(git diff --cached --name-only --diff-filter=ACM | grep -E '(\.ai/.*\.json$|envelope.*\.json$)' || true)

if [ -z "$STAGED_FILES" ]; then
  exit 0
fi

python3 - "$STAGED_FILES" <<'PYCODE'
import json
import sys
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
    errors_found = False
    total_files = 0

    for fpath in staged_list:
        content = get_staged_content(fpath)
        if not content.strip():
            continue

        try:
            envelope = json.loads(content)
        except json.JSONDecodeError as e:
            print(f"❌ INVALID JSON in {fpath}: {e}")
            errors_found = True
            continue

        total_files += 1

        # 1. Schemaref Check
        schemaref = envelope.get('schemaref')
        valid_refs = [
            'schema:Horror.Place/constellation/prism-envelope.v1',
            'schema:Horror.Place/constellation/ai-authoring-response-v1'
        ]
        if schemaref not in valid_refs:
            print(f"❌ INVALID SCHEMA REF in {fpath}: '{schemaref}'. Expected prism-envelope.v1 or ai-authoring-response-v1")
            errors_found = True

        # 2. Artifacts Array & Length
        artifacts = envelope.get('artifacts', [])
        if not isinstance(artifacts, list):
            print(f"❌ 'artifacts' is not an array in {fpath}")
            errors_found = True
        elif len(artifacts) > 3:
            print(f"❌ EXCEEDS ARTIFACT CAP in {fpath}: found {len(artifacts)}, max 3")
            errors_found = True

        # 3. Artifact Content Checks
        for i, art in enumerate(artifacts):
            if not isinstance(art, dict):
                print(f"❌ ARTIFACT {i} is not an object in {fpath}")
                errors_found = True
                continue
            
            role = art.get('role')
            if role not in ['primary', 'registry', 'secondary']:
                print(f"❌ INVALID ROLE '{role}' for artifact {i} in {fpath}")
                errors_found = True
            if not art.get('schemaref'):
                print(f"❌ MISSING SCHEMA REF in artifact {i} of {fpath}")
                errors_found = True
            if not art.get('path'):
                print(f"❌ MISSING PATH in artifact {i} of {fpath}")
                errors_found = True

        # 4. Required Blocks
        for block in ['author', 'target', 'intent', 'constraints']:
            if block not in envelope:
                print(f"❌ MISSING REQUIRED BLOCK '{block}' in {fpath}")
                errors_found = True

        # 5. Constraint Check
        constraints = envelope.get('constraints', {})
        if isinstance(constraints, dict) and constraints.get('maxArtifacts') is not None and constraints['maxArtifacts'] != 3:
            print(f"⚠️  WARNING in {fpath}: constraints.maxArtifacts is {constraints['maxArtifacts']}, expected 3 for strict enforcement")

    if errors_found:
        print(f"\n🚫 Pre-commit prism envelope lint FAILED. Checked {total_files} files.")
        sys.exit(1)
    else:
        print(f"✅ Pre-commit prism envelope lint PASSED. Checked {total_files} files.")
        sys.exit(0)

if __name__ == "__main__":
    main()
PYCODE
