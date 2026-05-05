#!/bin/sh
# File: scripts/populate_bci_pipeline.sh
# Purpose: Populate bcipipelinestage and bcipipelineedge tables with cross-repo wiring.
# Constraints: Uses only sqlite3, sh, find, grep, awk, sed (no Rustup/Cargo).

set -e

DB_PATH="${1:-db/constellation-index.db}"

if [ ! -f "${DB_PATH}" ]; then
    echo "Error: Database ${DB_PATH} not found. Run init_constellation_index.sh first."
    exit 1
fi

echo "Populating BCI pipeline stages and edges..."

sqlite3 "${DB_PATH}" << 'SQL_EOF'
-- ============================================================
-- BCI Pipeline Stages across HorrorPlace repos
-- ============================================================

-- Rotting-Visuals-BCI: Ingestion and validation stages
INSERT INTO bci_pipeline_stage (repo, stage_key, name, layer, input_type, output_type, primary_file, note) VALUES
  ('Rotting-Visuals-BCI', 'rvbci_request_in', 'BCI Request Ingestion', 'ingest', 
   'ai-bci-geometry-request-v1', 'BciSummary+Invariants',
   'src/monstermode/mod.rs',
   'Initial ingestion of AI-driven BCI geometry requests'),
  
  ('Rotting-Visuals-BCI', 'rvbci_validate', 'BCI Validation Gate', 'compute',
   'BciSummary+Invariants', 'ValidatedBciFrame',
   'src/monstermode/validation.rs',
   'Validates stressScore, visualOverloadIndex, and invariants CIC/DET/CDL/ARR'),
  
  ('Rotting-Visuals-BCI', 'rvbci_geometry', 'Geometry Compute', 'compute',
   'ValidatedBciFrame', 'GeometryParams',
   'src/monstermode/geometry.rs',
   'Computes maskRadius, motionSmear, and other visual parameters from BCI metrics'),
  
  ('Rotting-Visuals-BCI', 'rvbci_binding', 'Binding Generation', 'compute',
   'GeometryParams', 'BindingRows',
   'src/monstermode/binding.rs',
   'Generates binding rows mapping BCI values to shader uniforms and RTPCs'),
  
  ('Rotting-Visuals-BCI', 'rvviz_render', 'Theatre Render Output', 'render',
   'BindingRows', 'RenderedFrame',
   'src/renderer/mod.rs',
   'Theatre/arcade mode rendering using computed bindings'),
  
  ('Rotting-Visuals-BCI', 'rvbci_log', 'BCI Frame Logging', 'log',
   'ValidatedBciFrame', 'LoggedFrame',
   'db/schema_rottingvisuals_monstermode.sql',
   'Logs BCI frames to SQLite for offline analysis and ledger sync');

-- HorrorPlace-Dead-Ledger-Network: Persistence stages
INSERT INTO bci_pipeline_stage (repo, stage_key, name, layer, input_type, output_type, primary_file, note) VALUES
  ('HorrorPlace-Dead-Ledger-Network', 'ledger_ingest', 'Ledger Event Ingestion', 'ingest',
   'LoggedFrame', 'LedgerEvent',
   'src/ledger/ingest.rs',
   'Ingests logged frames from runtime repos into ledger event queue'),
  
  ('HorrorPlace-Dead-Ledger-Network', 'ledger_persist', 'Session Persistence', 'persistence',
   'LedgerEvent', 'PersistedSession',
   'src/ledger/persistence.rs',
   'Persists BCI sessions and theatre outputs to permanent storage'),
  
  ('HorrorPlace-Dead-Ledger-Network', 'ledger_telemetry', 'Telemetry Correlation', 'persistence',
   'PersistedSession', 'CorrelatedTelemetry',
   'src/ledger/telemetry.rs',
   'Correlates BCI telemetry with session metadata for analysis');

-- HorrorPlace-Neural-Resonance-Lab: Analysis stages
INSERT INTO bci_pipeline_stage (repo, stage_key, name, layer, input_type, output_type, primary_file, note) VALUES
  ('HorrorPlace-Neural-Resonance-Lab', 'neural_resonance', 'Resonance Detection', 'analysis',
   'CorrelatedTelemetry', 'ResonanceProfile',
   'src/analysis/resonance_detector.rs',
   'Detects neural resonance patterns in BCI data streams'),
  
  ('HorrorPlace-Neural-Resonance-Lab', 'neural_pattern', 'Pattern Analysis', 'analysis',
   'ResonanceProfile', 'PatternReport',
   'src/analysis/pattern_analyzer.rs',
   'Analyzes recurring patterns for constellation optimization');

-- HorrorPlace-Spectral-Foundry: Palette lookup stages
INSERT INTO bci_pipeline_stage (repo, stage_key, name, layer, input_type, output_type, primary_file, note) VALUES
  ('HorrorPlace-Spectral-Foundry', 'palette_lookup', 'Palette Swatch Lookup', 'compute',
   'GeometryParams', 'PaletteSwatches',
   'src/palette/lookup.rs',
   'Looks up palette swatch indices based on mood and style contracts');

-- HorrorPlace-Constellation-Contracts: CAN registry stage
INSERT INTO bci_pipeline_stage (repo, stage_key, name, layer, input_type, output_type, primary_file, note) VALUES
  ('HorrorPlace-Constellation-Contracts', 'can_registry', 'CAN Token Registry Check', 'compute',
   'BindingRows', 'ValidatedBindings',
   'schemas/can-token-registry-rot-visuals-v1.json',
   'Validates bindings against CAN token safety limits');

-- ============================================================
-- BCI Pipeline Edges (cross-repo wiring)
-- ============================================================

-- Path 1: Main BCI flow (Rotting-Visuals internal)
INSERT INTO bci_pipeline_edge (from_stage_id, to_stage_id, protocol, description)
SELECT 
  s1.stage_id, s2.stage_id, 'in-process',
  'BCI request ingestion to validation gate'
FROM bci_pipeline_stage s1, bci_pipeline_stage s2
WHERE s1.stage_key = 'rvbci_request_in' AND s2.stage_key = 'rvbci_validate'
  AND s1.repo = 'Rotting-Visuals-BCI' AND s2.repo = 'Rotting-Visuals-BCI';

INSERT INTO bci_pipeline_edge (from_stage_id, to_stage_id, protocol, description)
SELECT 
  s1.stage_id, s2.stage_id, 'in-process',
  'Validation to geometry compute stage'
FROM bci_pipeline_stage s1, bci_pipeline_stage s2
WHERE s1.stage_key = 'rvbci_validate' AND s2.stage_key = 'rvbci_geometry'
  AND s1.repo = 'Rotting-Visuals-BCI' AND s2.repo = 'Rotting-Visuals-BCI';

INSERT INTO bci_pipeline_edge (from_stage_id, to_stage_id, protocol, description)
SELECT 
  s1.stage_id, s2.stage_id, 'in-process',
  'Geometry compute to binding generation'
FROM bci_pipeline_stage s1, bci_pipeline_stage s2
WHERE s1.stage_key = 'rvbci_geometry' AND s2.stage_key = 'rvbci_binding'
  AND s1.repo = 'Rotting-Visuals-BCI' AND s2.repo = 'Rotting-Visuals-BCI';

INSERT INTO bci_pipeline_edge (from_stage_id, to_stage_id, protocol, description)
SELECT 
  s1.stage_id, s2.stage_id, 'in-process',
  'Binding generation to theatre render output'
FROM bci_pipeline_stage s1, bci_pipeline_stage s2
WHERE s1.stage_key = 'rvbci_binding' AND s2.stage_key = 'rvviz_render'
  AND s1.repo = 'Rotting-Visuals-BCI' AND s2.repo = 'Rotting-Visuals-BCI';

-- Path 2: Logging path (Rotting-Visuals -> Dead-Ledger)
INSERT INTO bci_pipeline_edge (from_stage_id, to_stage_id, protocol, description)
SELECT 
  s1.stage_id, s2.stage_id, 'sqlite',
  'BCI frame logging to ledger ingestion (cross-repo)'
FROM bci_pipeline_stage s1, bci_pipeline_stage s2
WHERE s1.stage_key = 'rvbci_log' AND s2.stage_key = 'ledger_ingest'
  AND s1.repo = 'Rotting-Visuals-BCI' AND s2.repo = 'HorrorPlace-Dead-Ledger-Network';

INSERT INTO bci_pipeline_edge (from_stage_id, to_stage_id, protocol, description)
SELECT 
  s1.stage_id, s2.stage_id, 'sqlite',
  'Ledger ingestion to session persistence'
FROM bci_pipeline_stage s1, bci_pipeline_stage s2
WHERE s1.stage_key = 'ledger_ingest' AND s2.stage_key = 'ledger_persist'
  AND s1.repo = 'HorrorPlace-Dead-Ledger-Network' AND s2.repo = 'HorrorPlace-Dead-Ledger-Network';

-- Path 3: Analysis path (Dead-Ledger -> Neural-Resonance-Lab)
INSERT INTO bci_pipeline_edge (from_stage_id, to_stage_id, protocol, description)
SELECT 
  s1.stage_id, s2.stage_id, 'ipc',
  'Session persistence to resonance detection (cross-repo analysis)'
FROM bci_pipeline_stage s1, bci_pipeline_stage s2
WHERE s1.stage_key = 'ledger_persist' AND s2.stage_key = 'neural_resonance'
  AND s1.repo = 'HorrorPlace-Dead-Ledger-Network' AND s2.repo = 'HorrorPlace-Neural-Resonance-Lab';

INSERT INTO bci_pipeline_edge (from_stage_id, to_stage_id, protocol, description)
SELECT 
  s1.stage_id, s2.stage_id, 'in-process',
  'Resonance detection to pattern analysis'
FROM bci_pipeline_stage s1, bci_pipeline_stage s2
WHERE s1.stage_key = 'neural_resonance' AND s2.stage_key = 'neural_pattern'
  AND s1.repo = 'HorrorPlace-Neural-Resonance-Lab' AND s2.repo = 'HorrorPlace-Neural-Resonance-Lab';

-- Path 4: Palette integration (Spectral-Foundry -> Rotting-Visuals)
INSERT INTO bci_pipeline_edge (from_stage_id, to_stage_id, protocol, description)
SELECT 
  s1.stage_id, s2.stage_id, 'in-process',
  'Palette swatch lookup integrated with geometry compute'
FROM bci_pipeline_stage s1, bci_pipeline_stage s2
WHERE s1.stage_key = 'palette_lookup' AND s2.stage_key = 'rvbci_binding'
  AND s1.repo = 'HorrorPlace-Spectral-Foundry' AND s2.repo = 'Rotting-Visuals-BCI';

-- Path 5: CAN registry validation (Constellation-Contracts -> Rotting-Visuals)
INSERT INTO bci_pipeline_edge (from_stage_id, to_stage_id, protocol, description)
SELECT 
  s1.stage_id, s2.stage_id, 'file',
  'CAN token registry validation before binding application'
FROM bci_pipeline_stage s1, bci_pipeline_stage s2
WHERE s1.stage_key = 'can_registry' AND s2.stage_key = 'rvbci_binding'
  AND s1.repo = 'HorrorPlace-Constellation-Contracts' AND s2.repo = 'Rotting-Visuals-BCI';

SQL_EOF

echo "BCI pipeline stages and edges populated successfully."

# Show summary
echo ""
echo "Pipeline summary:"
sqlite3 -column -header "${DB_PATH}" "SELECT repo, COUNT(*) as stages FROM bci_pipeline_stage GROUP BY repo;"
echo ""
echo "Edge summary:"
sqlite3 -column -header "${DB_PATH}" "SELECT protocol, COUNT(*) as edges FROM bci_pipeline_edge GROUP BY protocol;"
