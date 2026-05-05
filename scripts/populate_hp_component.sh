#!/bin/sh
# File: scripts/populate_hp_component.sh
# Purpose: Populate hp_component table with key schemas, SQL files, and docs.
# Constraints: Uses only sqlite3, sh, find, grep, awk, sed (no Rustup/Cargo).

set -e

DB_PATH="${1:-db/constellation-index.db}"

if [ ! -f "${DB_PATH}" ]; then
    echo "Error: Database ${DB_PATH} not found. Run init_constellation_index.sh first."
    exit 1
fi

echo "Populating hp_component table..."

# Helper function to get repo_id
get_repo_id() {
    sqlite3 "${DB_PATH}" "SELECT repo_id FROM hp_repo WHERE name = '$1';"
}

# Get repo IDs
ROT_VIS_ID=$(get_repo_id "Rotting-Visuals-BCI")
DEAD_LEDGER_ID=$(get_repo_id "HorrorPlace-Dead-Ledger-Network")
CONST_CONTRACTS_ID=$(get_repo_id "HorrorPlace-Constellation-Contracts")
NEURAL_LAB_ID=$(get_repo_id "HorrorPlace-Neural-Resonance-Lab")
SPECTRAL_FOUND_ID=$(get_repo_id "HorrorPlace-Spectral-Foundry")
CODEBASE_DEATH_ID=$(get_repo_id "Codebase-of-Death")
ROTCVE_ID=$(get_repo_id "HorrorPlace-RotCave")

# Insert components for HorrorPlace-Constellation-Contracts (local repo)
sqlite3 "${DB_PATH}" << SQL_EOF
-- Constellation-Contracts: Core schemas
INSERT INTO hp_component (repo_id, kind, path, summary, domain, tags) VALUES
  (${CONST_CONTRACTS_ID}, 'schema', 'schemas/bci/bci-feature-envelope-v2.json', 'BCI feature envelope schema for BCI data ingestion and validation', 'bci', 'request,bci,envelope'),
  (${CONST_CONTRACTS_ID}, 'schema', 'schemas/ai-bci-geometry-request-v1.json', 'AI-driven BCI geometry request schema for theatre/arcade mode rendering', 'bci', 'request,geometry,theatre'),
  (${CONST_CONTRACTS_ID}, 'schema', 'schemas/bci-geometry-binding-v1.json', 'BCI-to-geometry binding schema mapping BCI metrics to visual parameters', 'bci', 'binding,geometry,output'),
  (${CONST_CONTRACTS_ID}, 'sqlschema', 'db/schema_bci_pipeline.sql', 'BCI pipeline stage and edge tables for cross-repo wiring', 'wiring', 'pipeline,stages,edges'),
  (${CONST_CONTRACTS_ID}, 'sqlschema', 'db/schema_field_usage.sql', 'Field usage tracking table for cataloging field locations across repos', 'tooling', 'fieldusage,catalog'),
  (${CONST_CONTRACTS_ID}, 'sqlschema', 'db/constellation_index.sql', 'Constellation index schema for hp_repo, hp_component, and metadata tables', 'tooling', 'index,navigation'),
  (${CONST_CONTRACTS_ID}, 'sqlschema', 'db/schema_constellation_ontology.sql', 'Constellation ontology schema for palette groups and swatch indices', 'palette', 'ontology,swatches'),
  (${CONST_CONTRACTS_ID}, 'doc', 'docs/for-ai-agents.md', 'AI agent navigation guide with decision tree for constellation queries', 'tooling', 'ai-agents,navigation'),
  (${CONST_CONTRACTS_ID}, 'doc', 'docs/constellation-index.json', 'JSON representation of constellation index for lightweight parsing', 'tooling', 'index,json'),
  (${CONST_CONTRACTS_ID}, 'bcipipeline', 'db/queries/constellation-navigation.sql', 'Named query pack for AI agents to navigate constellation without directory walks', 'wiring', 'queries,navigation');

-- Rotting-Visuals-BCI: BCI ingestion and geometry compute
INSERT INTO hp_component (repo_id, kind, path, summary, domain, tags) VALUES
  (${ROT_VIS_ID}, 'schema', 'schemas/ai-bci-geometry-request-v1.json', 'Primary BCI geometry request schema with stressScore and invariants', 'bci', 'request,invariants,metrics'),
  (${ROT_VIS_ID}, 'schema', 'schemas/ai-bci-geometry-response-v1.json', 'BCI geometry response schema with computed visual parameters', 'bci', 'response,geometry,visual'),
  (${ROT_VIS_ID}, 'schema', 'schemas/bci-geometry-binding-v1.json', 'Geometry binding schema for mapping BCI metrics to shader uniforms', 'bci', 'binding,shader,output'),
  (${ROT_VIS_ID}, 'schema', 'schemas/can-token-registry-rot-visuals-v1.json', 'CAN token registry for safety limits and parameter modulation bounds', 'wiring', 'can,safety,registry'),
  (${ROT_VIS_ID}, 'sqlschema', 'db/schema_rottingvisuals_monstermode.sql', 'MonsterMode runtime SQL schema for BCI frame logging and bindings', 'bci', 'runtime,logging,frames'),
  (${ROT_VIS_ID}, 'canregistry', 'schemas/can-token-registry-rot-visuals-v1.json', 'CAN registry defining max_gain and other safety limits for CAN bus tokens', 'wiring', 'can,safety,tokens'),
  (${ROT_VIS_ID}, 'bcipipeline', 'src/monstermode/mod.rs', 'Rust module implementing BCI ingestion, validation, and geometry compute stages', 'bci', 'ingest,compute,rust');

-- HorrorPlace-Dead-Ledger-Network: Persistence and ledger logging
INSERT INTO hp_component (repo_id, kind, path, summary, domain, tags) VALUES
  (${DEAD_LEDGER_ID}, 'sqlschema', 'db/schema_deadledger_events.sql', 'Ledger event tables for persisting BCI sessions and theatre outputs', 'ledger', 'events,persistence'),
  (${DEAD_LEDGER_ID}, 'sqlschema', 'db/schema_deadledger_sessions.sql', 'Session tracking tables for BCI telemetry correlation', 'ledger', 'sessions,telemetry'),
  (${DEAD_LEDGER_ID}, 'schema', 'schemas/ledger-event-v1.json', 'Ledger event schema for structured logging of BCI and theatre events', 'ledger', 'event,logging'),
  (${DEAD_LEDGER_ID}, 'bcipipeline', 'src/ledger/persistence.rs', 'Persistence layer module writing BCI frames and bindings to SQLite', 'ledger', 'persistence,sqlite');

-- HorrorPlace-Neural-Resonance-Lab: Analysis and research
INSERT INTO hp_component (repo_id, kind, path, summary, domain, tags) VALUES
  (${NEURAL_LAB_ID}, 'schema', 'schemas/neural-resonance-profile-v1.json', 'Neural resonance profile schema for BCI pattern analysis', 'bci', 'analysis,patterns'),
  (${NEURAL_LAB_ID}, 'doc', 'docs/neural-resonance-methodology.md', 'Documentation on neural resonance analysis methods for BCI data', 'bci', 'analysis,methodology'),
  (${NEURAL_LAB_ID}, 'bcipipeline', 'src/analysis/resonance_detector.rs', 'Resonance detection module identifying patterns in BCI streams', 'analysis', 'resonance,detection');

-- Spectral-Foundry: Palette and style contracts
INSERT INTO hp_component (repo_id, kind, path, summary, domain, tags) VALUES
  (${SPECTRAL_FOUND_ID}, 'schema', 'schemas/palette-contract-v1.json', 'Palette contract schema defining swatch groups and spectral mappings', 'palette', 'contract,swatches'),
  (${SPECTRAL_FOUND_ID}, 'schema', 'schemas/stylecontract_v1.json', 'Style contract schema for mood and atmosphere definitions', 'palette', 'style,mood'),
  (${SPECTRAL_FOUND_ID}, 'sqlschema', 'db/schema_palette_groups.sql', 'Palette groups SQL schema for spectral PUS and swatch indices', 'palette', 'groups,spectral');

-- Codebase-of-Death: Analysis tooling
INSERT INTO hp_component (repo_id, kind, path, summary, domain, tags) VALUES
  (${CODEBASE_DEATH_ID}, 'doc', 'docs/codebase-analysis-methodology.md', 'Methodology document for static analysis of constellation codebases', 'analysis', 'methodology,static'),
  (${CODEBASE_DEATH_ID}, 'tooling', 'tools/codebase-metrics.py', 'Python script computing codebase metrics for constellation health', 'tooling', 'metrics,health');

-- HorrorPlace-RotCave: Tooling and linters
INSERT INTO hp_component (repo_id, kind, path, summary, domain, tags) VALUES
  (${ROTCVE_ID}, 'tooling', 'tools/lint-bci-schema.py', 'BCI schema linter ensuring schema compliance with constellation standards', 'tooling', 'lint,bci,schema'),
  (${ROTCVE_ID}, 'tooling', 'tools/validate-pipeline-wiring.py', 'Pipeline wiring validator checking bcipipelinestage and bcipipelineedge consistency', 'tooling', 'lint,wiring,pipeline');
SQL_EOF

echo "hp_component populated successfully."
