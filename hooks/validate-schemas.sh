#!/usr/bin/env bash
set -euo pipefail

# Horror$Place Constellation Guardrail: Schema Validation Pre-Commit Hook
# Validates staged JSON/NDJSON files against their referenced schemaref using local schemas.
# Dependencies: python3, jsonschema (pip install jsonschema)

echo "🔍 Running pre-commit schema validation..."

STAGED_FILES=$(git diff --cached --name-only --diff-filter=ACM | grep -E '\.(json|ndjson)$' || true)

if [ -z "$STAGED_FILES" ]; then
  exit 0
fi

python3 - "$STAGED_FILES" <<'PYCODE'
import json
import sys
import os
from pathlib import Path
from jsonschema import Draft202012Validator
import subprocess

def load_schemas(schema_dir: Path) -> dict:
    schemas = {}
    for schema_file in schema_dir.rglob("*.json"):
        try:
            with schema_file.open("r", encoding="utf-8") as f:
                schema = json.load(f)
            if "$id" in schema:
                schemas[schema["$id"]] = schema
        except Exception as e:
            print(f"WARNING: Failed to load schema {schema_file}: {e}", file=sys.stderr)
    return schemas

def get_staged_content(file_path: str) -> str:
    try:
        result = subprocess.run(["git", "show", f":{file_path}"], capture_output=True, text=True, check=True)
        return result.stdout
    except subprocess.CalledProcessError as e:
        print(f"ERROR: Could not read staged content for {file_path}: {e}", file=sys.stderr)
        return ""

def main():
    staged_list = sys.argv[1].splitlines()
    schema_dir = Path("schemas")
    if not schema_dir.is_dir():
        print("ERROR: schemas/ directory not found. Run hook from repo root.", file=sys.stderr)
        sys.exit(1)

    schemas = load_schemas(schema_dir)
    if not schemas:
        print("WARNING: No schemas found. Skipping validation.", file=sys.stderr)
        sys.exit(0)

    errors_found = False
    for fpath in staged_list:
        content = get_staged_content(fpath)
        if not content.strip():
            continue

        lines = [content] if not fpath.endswith(".ndjson") else content.splitlines()
        for i, line in enumerate(lines):
            line = line.strip()
            if not line:
                continue
            try:
                data = json.loads(line)
            except json.JSONDecodeError as e:
                print(f"❌ INVALID JSON in {fpath} line {i+1}: {e}")
                errors_found = True
                continue

            schemaref = data.get("schemaref")
            if not schemaref or schemaref not in schemas:
                print(f"❌ MISSING/UNKNOWN SCHEMA in {fpath} line {i+1}: schemaref '{schemaref}' not found in schemas/")
                errors_found = True
                continue

            validator = Draft202012Validator(schemas[schemaref])
            validation_errors = sorted(validator.iter_errors(data), key=lambda e: e.path)
            if validation_errors:
                loc = f"line {i+1}"
                if "id" in data:
                    loc = f"entry {data['id']}"
                for err in validation_errors:
                    path_str = "/".join(str(p) for p in err.path)
                    print(f"❌ VALIDATION ERROR in {fpath} ({loc}) path '{path_str}': {err.message}")
                errors_found = True

    if errors_found:
        print("\n🚫 Pre-commit schema validation FAILED. Please fix the errors above.")
        sys.exit(1)
    else:
        print("✅ Pre-commit schema validation PASSED.")
        sys.exit(0)

if __name__ == "__main__":
    main()
PYCODE
