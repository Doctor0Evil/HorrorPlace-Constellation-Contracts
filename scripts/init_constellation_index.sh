#!/bin/sh
# File: scripts/init_constellation_index.sh
# Purpose: Initialize and populate the HorrorPlace constellation index SQLite database.
# Constraints: Uses only sqlite3, sh, find, grep, awk, sed (no Rustup/Cargo).

set -e

DB_PATH="${1:-db/constellation-index.db}"
SCHEMA_FILE="db/constellation_index.sql"
PIPELINE_SCHEMA="db/schema_bci_pipeline.sql"
FIELD_USAGE_SCHEMA="db/schema_field_usage.sql"
NAV_QUERIES="db/queries/constellation-navigation.sql"

echo "Initializing constellation index at ${DB_PATH}..."

# Remove existing DB to start fresh
rm -f "${DB_PATH}"

# Create tables from schema files
echo "Creating tables from ${SCHEMA_FILE}..."
sqlite3 "${DB_PATH}" < "${SCHEMA_FILE}"

echo "Creating pipeline tables from ${PIPELINE_SCHEMA}..."
sqlite3 "${DB_PATH}" < "${PIPELINE_SCHEMA}"

echo "Creating field usage tables from ${FIELD_USAGE_SCHEMA}..."
sqlite3 "${DB_PATH}" < "${FIELD_USAGE_SCHEMA}"

# Populate hp_repo with all known HorrorPlace repos
echo "Populating hp_repo table..."
sqlite3 "${DB_PATH}" << 'SQL_EOF'
-- Insert all known HorrorPlace constellation repos
INSERT INTO hp_repo (name, git_url, local_root, local_checkout, role, is_temporary) VALUES
  -- Core runtime repos
  ('Rotting-Visuals-BCI', 'https://github.com/Doctor0Evil/Rotting-Visuals-BCI', NULL, 0, 'runtime', 0),
  ('HorrorPlace-Dead-Ledger-Network', 'https://github.com/Doctor0Evil/HorrorPlace-Dead-Ledger-Network', NULL, 0, 'ledger', 0),
  ('HorrorPlace-Neural-Resonance-Lab', 'https://github.com/Doctor0Evil/HorrorPlace-Neural-Resonance-Lab', NULL, 0, 'runtime', 0),
  
  -- Contracts and schema repos
  ('HorrorPlace-Constellation-Contracts', 'https://github.com/Doctor0Evil/HorrorPlace-Constellation-Contracts', '.', 1, 'contracts', 0),
  ('HorrorPlace-Spectral-Foundry', 'https://github.com/Doctor0Evil/HorrorPlace-Spectral-Foundry', NULL, 0, 'contracts', 0),
  
  -- Analysis and tooling repos
  ('Codebase-of-Death', 'https://github.com/Doctor0Evil/Codebase-of-Death', NULL, 0, 'analysis', 0),
  ('HorrorPlace-RotCave', 'https://github.com/Doctor0Evil/HorrorPlace-RotCave', NULL, 0, 'tooling', 0),
  
  -- Additional constellation repos from manifests
  ('HorrorPlace-Black-Archivum', 'https://github.com/Doctor0Evil/HorrorPlace-Black-Archivum', NULL, 0, 'analysis', 0),
  ('HorrorPlace-Obscura-Nexus', 'https://github.com/Doctor0Evil/HorrorPlace-Obscura-Nexus', NULL, 0, 'runtime', 0),
  ('HorrorPlace-Liminal-Continuum', 'https://github.com/Doctor0Evil/HorrorPlace-Liminal-Continuum', NULL, 0, 'runtime', 0),
  ('HorrorPlace-Redacted-Chronicles', 'https://github.com/Doctor0Evil/HorrorPlace-Redacted-Chronicles', NULL, 0, 'analysis', 0),
  ('HorrorPlace-Process-Gods-Research', 'https://github.com/Doctor0Evil/HorrorPlace-Process-Gods-Research', NULL, 0, 'analysis', 0),
  ('HorrorPlace-Atrocity-Seeds', 'https://github.com/Doctor0Evil/HorrorPlace-Atrocity-Seeds', NULL, 0, 'runtime', 0);

-- Add ignore list for non-repo directories
INSERT INTO hp_repo_ignore (path, reason) VALUES
  ('crates', 'Rust crates compiled externally, indexed as static text'),
  ('tools', 'Mixed tooling, some Rust sources treated as static text'),
  ('node_modules', 'External dependencies, not part of constellation index'),
  ('.git', 'Git metadata, not content');
SQL_EOF

echo "hp_repo populated successfully."
