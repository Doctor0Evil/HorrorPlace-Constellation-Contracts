#!/bin/sh
# File: scripts/populate_field_usage.sh
# Purpose: Populate field_usage table with critical BCI fields and their locations.
# Constraints: Uses only sqlite3, sh, find, grep, awk, sed (no Rustup/Cargo).

set -e

DB_PATH="${1:-db/constellation-index.db}"

if [ ! -f "${DB_PATH}" ]; then
    echo "Error: Database ${DB_PATH} not found. Run init_constellation_index.sh first."
    exit 1
fi

echo "Populating field_usage table with critical BCI fields..."

sqlite3 "${DB_PATH}" << 'SQL_EOF'
-- ============================================================
-- Critical BCI Summary fields
-- ============================================================

INSERT INTO field_usage (field_path, repo, location_type, location_path, container_name, container_field, role, note) VALUES
  -- stressScore: primary horror driver
  ('bciSummary.stressScore', 'Rotting-Visuals-BCI', 'json_schema', 'schemas/ai-bci-geometry-request-v1.json', 'ai-bci-geometry-request-v1', 'stressScore', 'input', 'Normalized 0-1 scalar stress, primary horror driver'),
  ('bciSummary.stressScore', 'Rotting-Visuals-BCI', 'sql_table', 'db/schema_rottingvisuals_monstermode.sql', 'bcirequestframe', 'stressscore', 'log_column', 'Logged per frame for offline analysis'),
  ('bciSummary.stressScore', 'HorrorPlace-Dead-Ledger-Network', 'sql_table', 'db/schema_deadledger_events.sql', 'ledgerevent', 'stress_score', 'persisted', 'Persisted in ledger for session correlation'),
  
  -- visualOverloadIndex: secondary horror metric
  ('bciSummary.visualOverloadIndex', 'Rotting-Visuals-BCI', 'json_schema', 'schemas/ai-bci-geometry-request-v1.json', 'ai-bci-geometry-request-v1', 'visualOverloadIndex', 'input', 'Normalized 0-1 visual overload metric'),
  ('bciSummary.visualOverloadIndex', 'Rotting-Visuals-BCI', 'sql_table', 'db/schema_rottingvisuals_monstermode.sql', 'bcirequestframe', 'visualoverloadindex', 'log_column', 'Logged for visual intensity analysis'),
  
  -- startleSpike: acute fear response
  ('bciSummary.startleSpike', 'Rotting-Visuals-BCI', 'json_schema', 'schemas/ai-bci-geometry-request-v1.json', 'ai-bci-geometry-request-v1', 'startleSpike', 'input', 'Boolean flag for acute startle response detection'),
  ('bciSummary.startleSpike', 'Rotting-Visuals-BCI', 'sql_table', 'db/schema_rottingvisuals_monstermode.sql', 'bcirequestframe', 'startlespike', 'log_column', 'Logged for surprise timing analysis');

-- ============================================================
-- Invariant fields (CIC, DET, CDL, ARR)
-- ============================================================

INSERT INTO field_usage (field_path, repo, location_type, location_path, container_name, container_field, role, note) VALUES
  -- CIC: Constellation Integrity Coefficient
  ('invariants.CIC', 'Rotting-Visuals-BCI', 'json_schema', 'schemas/ai-bci-geometry-request-v1.json', 'ai-bci-geometry-request-v1', 'CIC', 'invariant_gate', 'Constellation Integrity Coefficient, 0-1 safety factor'),
  ('invariants.CIC', 'Rotting-Visuals-BCI', 'rust_struct', 'src/monstermode/mod.rs', 'Invariants', 'CIC', 'invariant_gate', 'Used to dampen extreme visual and audio effects'),
  ('invariants.CIC', 'HorrorPlace-Constellation-Contracts', 'sql_table', 'db/schema_constellation_ontology.sql', 'invariantslog', 'cic_value', 'audit_column', 'Audited for governance compliance'),
  
  -- DET: Dread Engagement Threshold
  ('invariants.DET', 'Rotting-Visuals-BCI', 'json_schema', 'schemas/ai-bci-geometry-request-v1.json', 'ai-bci-geometry-request-v1', 'DET', 'invariant_gate', 'Dread Engagement Threshold for sustained fear tracking'),
  ('invariants.DET', 'Rotting-Visuals-BCI', 'rust_struct', 'src/monstermode/mod.rs', 'Invariants', 'DET', 'invariant_gate', 'Controls pacing of horror escalation'),
  
  -- CDL: Cognitive Dissonance Level
  ('invariants.CDL', 'Rotting-Visuals-BCI', 'json_schema', 'schemas/ai-bci-geometry-request-v1.json', 'ai-bci-geometry-request-v1', 'CDL', 'invariant_gate', 'Cognitive Dissonance Level for reality distortion'),
  ('invariants.CDL', 'HorrorPlace-Neural-Resonance-Lab', 'sql_table', 'db/schema_neural_analysis.sql', 'resonanceprofile', 'cdl_avg', 'analysis_metric', 'Averaged for resonance pattern detection'),
  
  -- ARR: Arousal Regulation Ratio
  ('invariants.ARR', 'Rotting-Visuals-BCI', 'json_schema', 'schemas/ai-bci-geometry-request-v1.json', 'ai-bci-geometry-request-v1', 'ARR', 'invariant_gate', 'Arousal Regulation Ratio for player safety'),
  ('invariants.ARR', 'HorrorPlace-Dead-Ledger-Network', 'sql_table', 'db/schema_deadledger_sessions.sql', 'sessionmetrics', 'arr_min', 'safety_threshold', 'Minimum ARR tracked for consent compliance');

-- ============================================================
-- Visual geometry parameters
-- ============================================================

INSERT INTO field_usage (field_path, repo, location_type, location_path, container_name, container_field, role, note) VALUES
  -- visual.maskRadius
  ('visual.maskRadius', 'Rotting-Visuals-BCI', 'json_schema', 'schemas/bci-geometry-binding-v1.json', 'bci-geometry-binding-v1', 'maskRadius', 'output', 'Shrinking mask radius driven by stress and overload'),
  ('visual.maskRadius', 'Rotting-Visuals-BCI', 'sql_table', 'db/schema_rottingvisuals_monstermode.sql', 'bcibindinggeometry', 'maskradius', 'log_column', 'Logged per binding for reconstruction'),
  ('visual.maskRadius', 'Rotting-Visuals-BCI', 'rust_struct', 'src/monstermode/mod.rs', 'VisualParams', 'maskRadius', 'derived_param', 'Computed from BciSummary and Invariants'),
  
  -- visual.motionSmear
  ('visual.motionSmear', 'Rotting-Visuals-BCI', 'json_schema', 'schemas/bci-geometry-binding-v1.json', 'bci-geometry-binding-v1', 'motionSmear', 'output', 'Motion blur intensity based on panic level'),
  ('visual.motionSmear', 'Rotting-Visuals-BCI', 'sql_table', 'db/schema_rottingvisuals_monstermode.sql', 'bcibindinggeometry', 'motionsmear', 'log_column', 'Logged for visual fidelity tuning'),
  
  -- visual.chromaticAberration
  ('visual.chromaticAberration', 'Rotting-Visuals-BCI', 'json_schema', 'schemas/bci-geometry-binding-v1.json', 'bci-geometry-binding-v1', 'chromaticAberration', 'output', 'Chromatic aberration for disorientation effect');

-- ============================================================
-- Audio RTPC parameters
-- ============================================================

INSERT INTO field_usage (field_path, repo, location_type, location_path, container_name, container_field, role, note) VALUES
  -- audio.heartbeatGain
  ('audio.heartbeatGain', 'Rotting-Visuals-BCI', 'json_schema', 'schemas/bci-geometry-binding-v1.json', 'bci-geometry-binding-v1', 'heartbeatGain', 'output', 'Heartbeat audio gain modulation'),
  ('audio.heartbeatGain', 'HorrorPlace-Spectral-Foundry', 'sql_table', 'db/schema_palette_groups.sql', 'audiortpcmapping', 'heartbeat_gain', 'rtpc_target', 'Mapped to Wwise RTPC'),
  
  -- audio.ringingLevel
  ('audio.ringingLevel', 'Rotting-Visuals-BCI', 'json_schema', 'schemas/bci-geometry-binding-v1.json', 'bci-geometry-binding-v1', 'ringingLevel', 'output', 'Tinnitus/ringing effect intensity'),
  
  -- audio.lowFreqBoost
  ('audio.lowFreqBoost', 'Rotting-Visuals-BCI', 'json_schema', 'schemas/bci-geometry-binding-v1.json', 'bci-geometry-binding-v1', 'lowFreqBoost', 'output', 'Low frequency boost for dread atmosphere');

-- ============================================================
-- CAN registry fields
-- ============================================================

INSERT INTO field_usage (field_path, repo, location_type, location_path, container_name, container_field, role, note) VALUES
  -- can.token.max_gain
  ('can.token.max_gain', 'Rotting-Visuals-BCI', 'json_schema', 'schemas/can-token-registry-rot-visuals-v1.json', 'ai-can-token-registry-v1', 'max_effective_gain', 'safety_limit', 'Upper bound for CAN-driven parameter modulation'),
  ('can.token.max_gain', 'Rotting-Visuals-BCI', 'sql_table', 'db/schema_can_tokens.sql', 'can_token', 'max_gain', 'safety_limit', 'Mirrors JSON max_effective_gain for queryable audits'),
  
  -- can.token.min_floor
  ('can.token.min_floor', 'Rotting-Visuals-BCI', 'json_schema', 'schemas/can-token-registry-rot-visuals-v1.json', 'ai-can-token-registry-v1', 'min_floor', 'safety_limit', 'Lower bound preventing complete signal loss');

-- ============================================================
-- Palette fields
-- ============================================================

INSERT INTO field_usage (field_path, repo, location_type, location_path, container_name, container_field, role, note) VALUES
  -- palette.swatchIndex
  ('palette.swatchIndex', 'HorrorPlace-Spectral-Foundry', 'json_schema', 'schemas/palette-contract-v1.json', 'palette-contract-v1', 'swatchIndex', 'lookup_key', 'Palette swatch index for color grading'),
  ('palette.swatchIndex', 'HorrorPlace-Spectral-Foundry', 'sql_table', 'db/schema_palette_groups.sql', 'paletteswatch', 'swatch_index', 'primary_key', 'Primary key for swatch lookup'),
  
  -- palette.groupId
  ('palette.groupId', 'HorrorPlace-Spectral-Foundry', 'json_schema', 'schemas/palette-contract-v1.json', 'palette-contract-v1', 'groupId', 'classification', 'Palette group identifier for mood classification');

SQL_EOF

echo "field_usage populated successfully."

# Show summary
echo ""
echo "Field usage summary:"
sqlite3 -column -header "${DB_PATH}" "SELECT role, COUNT(*) as count FROM field_usage GROUP BY role ORDER BY count DESC;"
echo ""
echo "Fields by repo:"
sqlite3 -column -header "${DB_PATH}" "SELECT repo, COUNT(*) as count FROM field_usage GROUP BY repo ORDER BY count DESC;"
