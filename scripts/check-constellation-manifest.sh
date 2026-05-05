#!/usr/bin/env sh
set -eu

SCHEMA_PATH="${SCHEMA_PATH:-schemas/metadata/hpc-constellation-manifest-v1.json}"
MANIFEST_PATH="${MANIFEST_PATH:-horror_place_constellation_manifest.json}"

if [ ! -f "$SCHEMA_PATH" ]; then
  echo "ERROR: schema not found at $SCHEMA_PATH" >&2
  exit 1
fi

if [ ! -f "$MANIFEST_PATH" ]; then
  echo "ERROR: manifest not found at $MANIFEST_PATH" >&2
  exit 1
fi

# Basic JSON well-formedness check.
if ! jq . "$MANIFEST_PATH" >/dev/null 2>&1; then
  echo "ERROR: manifest is not valid JSON: $MANIFEST_PATH" >&2
  exit 1
fi

# Validate against the JSON Schema.
jsonschema \
  -i "$MANIFEST_PATH" \
  "$SCHEMA_PATH"
