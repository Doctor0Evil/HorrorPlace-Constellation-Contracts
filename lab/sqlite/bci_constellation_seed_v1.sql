-- ============================================================================
-- BCI Constellation Wiring Seed Data v1
-- Purpose: Populate constellation wiring tables with initial VM node registry
--          and chain family definitions for AI-agent reference.
-- Tier: 1 (Public, Canonical)
-- Repository: HorrorPlace-Constellation-Contracts
-- ============================================================================

-- ----------------------------------------------------------------------------
-- Seed: constellation_vm_node
-- ----------------------------------------------------------------------------
INSERT INTO constellation_vm_node (vm_name, repository_url, tier, roles, subscribed_chains, status, created_at, updated_at) VALUES
('HorrorPlace-Constellation-Contracts', 'https://github.com/HorrorPlace/Constellation-Contracts', 1, '["Core"]', NULL, 'active', datetime('now'), datetime('now')),
('HorrorPlace-Codebase-of-Death', 'https://github.com/HorrorPlace/Codebase-of-Death', 2, '["Breeder", "Validator"]', '["ARCHIVAL_LINE_ALPHA", "HYDRO_LINE_BETA", "LIMEN_LINE_GAMMA"]', 'active', datetime('now'), datetime('now')),
('HorrorPlace-Neural-Resonance-Lab', 'https://github.com/HorrorPlace/Neural-Resonance-Lab', 3, '["Experimenter"]', NULL, 'active', datetime('now'), datetime('now')),
('HorrorPlace-Black-Archivum', 'https://github.com/HorrorPlace/Black-Archivum', 2, '["Archivist"]', '["ARCHIVAL_LINE_ALPHA"]', 'active', datetime('now'), datetime('now')),
('HorrorPlace-Spectral-Foundry', 'https://github.com/HorrorPlace/Spectral-Foundry', 2, '["Persona-Host"]', '["ARCHIVAL_LINE_ALPHA", "HYDRO_LINE_BETA", "LIMEN_LINE_GAMMA"]', 'active', datetime('now'), datetime('now')),
('HorrorPlace-Atrocity-Seeds', 'https://github.com/HorrorPlace/Atrocity-Seeds', 2, '["Event-Host"]', '["ARCHIVAL_LINE_ALPHA", "HYDRO_LINE_BETA", "LIMEN_LINE_GAMMA"]', 'active', datetime('now'), datetime('now')),
('HorrorPlace-Obscura-Nexus', 'https://github.com/HorrorPlace/Obscura-Nexus', 2, '["Style-Host"]', NULL, 'active', datetime('now'), datetime('now')),
('HorrorPlace-Liminal-Continuum', 'https://github.com/HorrorPlace/Liminal-Continuum', 2, '["Agent-Router"]', '["LIMEN_LINE_GAMMA"]', 'active', datetime('now'), datetime('now')),
('HorrorPlace-Dead-Ledger-Network', 'https://github.com/HorrorPlace/Dead-Ledger-Network', 2, '["Ledger-Keeper"]', '["ARCHIVAL_LINE_ALPHA", "LIMEN_LINE_GAMMA"]', 'active', datetime('now'), datetime('now')),
('HorrorPlace-Redacted-Chronicles', 'https://github.com/HorrorPlace/Redacted-Chronicles', 3, '["Telemetry-Host"]', NULL, 'active', datetime('now'), datetime('now')),
('HorrorPlace-Process-Gods-Research', 'https://github.com/HorrorPlace/Process-Gods-Research', 3, '["Experimental"]', NULL, 'active', datetime('now'), datetime('now'));

-- ----------------------------------------------------------------------------
-- Seed: constellation_chain_family
-- ----------------------------------------------------------------------------
INSERT INTO constellation_chain_family (chain_family_key, description, invariant_weights, metric_weights, tier, status, created_at, updated_at) VALUES
('ARCHIVAL_LINE_ALPHA', 'Archival Drip → Contradiction Spike → Silent Redaction',
 '{"CIC": 0.6, "AOS": 1.0, "RWF": 0.4, "SHCI": 0.9}',
 '{"UEC": 0.7, "EMD": 0.9, "STCI": 0.4, "CDL": 0.7, "ARR": 0.85}',
 'mature', 'live', datetime('now'), datetime('now')),
('HYDRO_LINE_BETA', 'Mundane Water Ritual → Spectral Whisper → Spatial Mistrust',
 '{"RRM": 0.8, "SPR": 0.9, "DET": 0.6, "LSG": 0.5}',
 '{"UEC": 0.65, "EMD": 0.8, "STCI": 0.8, "CDL": 0.6, "ARR": 0.8}',
 'mature', 'live', datetime('now'), datetime('now')),
('LIMEN_LINE_GAMMA', 'Liminal Corridor → Vanish.Dissipation! → Dead Ledger Entry',
 '{"LSG": 1.0, "HVF": 0.9, "DET": 0.7, "SHCI": 0.85}',
 '{"UEC": 0.75, "EMD": 0.75, "STCI": 0.85, "CDL": 0.7, "ARR": 0.8}',
 'mature', 'live', datetime('now'), datetime('now'));

-- ----------------------------------------------------------------------------
-- Seed: constellation_transition_channel
-- ----------------------------------------------------------------------------
INSERT INTO constellation_transition_channel (channel_name, source_vm_node_id, target_vm_node_id, data_schema_id, direction, priority) VALUES
('history_seed', 4, 2, 'archive_record_v1', 'unidirectional', 80),
('pcg_binding', 6, 2, 'event_contract_v1', 'unidirectional', 75),
('persona_binding', 5, 2, 'persona_contract_v1', 'bidirectional', 90),
('dead_ledger', 2, 9, 'dead_ledger_entry_v1', 'unidirectional', 95),
('telemetry_export', 2, 10, 'bci-mapping-activation-v1', 'unidirectional', 70),
('style_envelope', 7, 2, 'style_contract_v1', 'unidirectional', 65);

-- ----------------------------------------------------------------------------
-- Seed: constellation_schema_registry
-- ----------------------------------------------------------------------------
INSERT INTO constellation_schema_registry (schema_key, schema_version, tier, repository, path, description, status, created_at, updated_at) VALUES
('bci-feature-envelope-v1', '1.0.0', 1, 'HorrorPlace-Constellation-Contracts', 'schemas/bci/bci-feature-envelope-v1.json', 'Raw BCI feature envelope', 'active', datetime('now'), datetime('now')),
('bci-metrics-envelope-v1', '1.0.0', 1, 'HorrorPlace-Constellation-Contracts', 'schemas/bci/bci-metrics-envelope-v1.json', 'Normalized entertainment metrics', 'active', datetime('now'), datetime('now')),
('bci-summary-v1', '1.0.0', 1, 'HorrorPlace-Constellation-Contracts', 'schemas/bci/bci-summary-v1.json', 'High-level runtime state summary', 'active', datetime('now'), datetime('now')),
('bci-geometry-binding-v1', '1.0.0', 1, 'HorrorPlace-Constellation-Contracts', 'schemas/bci/bci-geometry-binding-v1.json', 'Maps BCI state to output parameters', 'active', datetime('now'), datetime('now')),
('bci-safety-profile-v1', '1.0.0', 1, 'HorrorPlace-Constellation-Contracts', 'schemas/bci/bci-safety-profile-v1.json', 'Safety caps and recovery policies', 'active', datetime('now'), datetime('now')),
('bci-mapping-request-v1', '1.0.0', 1, 'HorrorPlace-Constellation-Contracts', 'schemas/bci/bci-mapping-request-v1.json', 'Unified mapping request', 'active', datetime('now'), datetime('now')),
('bci-mapping-response-v1', '1.0.0', 1, 'HorrorPlace-Constellation-Contracts', 'schemas/bci/bci-mapping-response-v1.json', 'Unified mapping response', 'active', datetime('now'), datetime('now')),
('bci-intervention-rule-v1', '1.0.0', 1, 'HorrorPlace-Constellation-Contracts', 'schemas/runtime/bci-intervention-rule-v1.json', 'Prescriptive tuning rule', 'active', datetime('now'), datetime('now')),
('bci-tuning-playbook-v1', '1.0.0', 1, 'HorrorPlace-Constellation-Contracts', 'schemas/runtime/bci-tuning-playbook-v1.json', 'Collection of tuning rules', 'active', datetime('now'), datetime('now')),
('event_contract_v1', '1.0.0', 1, 'HorrorPlace-Constellation-Contracts', 'schemas/event_contract_v1.json', 'Event contract schema', 'active', datetime('now'), datetime('now')),
('persona_contract_v1', '1.0.0', 1, 'HorrorPlace-Constellation-Contracts', 'schemas/persona_contract_v1.json', 'Persona contract schema', 'active', datetime('now'), datetime('now'));

-- ----------------------------------------------------------------------------
-- Seed: constellation_file_manifest (Sample generated files)
-- ----------------------------------------------------------------------------
INSERT INTO constellation_file_manifest (file_path, file_name, repository, tier, file_type, schema_ref, depends_on, status, generated_at, updated_at) VALUES
('schemas/bci/bci-feature-envelope-v1.json', 'bci-feature-envelope-v1.json', 'HorrorPlace-Constellation-Contracts', 1, 'schema', 'bci-feature-envelope-v1', NULL, 'generated', datetime('now'), datetime('now')),
('schemas/bci/bci-metrics-envelope-v1.json', 'bci-metrics-envelope-v1.json', 'HorrorPlace-Constellation-Contracts', 1, 'schema', 'bci-metrics-envelope-v1', NULL, 'generated', datetime('now'), datetime('now')),
('schemas/bci/bci-safety-profile-v1.json', 'bci-safety-profile-v1.json', 'HorrorPlace-Constellation-Contracts', 1, 'schema', 'bci-safety-profile-v1', NULL, 'generated', datetime('now'), datetime('now')),
('scripts/bci/hpc_bci_adapter.lua', 'hpc_bci_adapter.lua', 'HorrorPlace-Codebase-of-Death', 2, 'script', NULL, '["bci-mapping-request-v1", "bci-mapping-response-v1"]', 'generated', datetime('now'), datetime('now')),
('src/bci/safety_kernel.rs', 'safety_kernel.rs', 'HorrorPlace-Codebase-of-Death', 2, 'script', 'bci-safety-profile-v1', NULL, 'generated', datetime('now'), datetime('now')),
('configs/bci/bci-lab-config-phase-2-default.json', 'bci-lab-config-phase-2-default.json', 'HorrorPlace-Neural-Resonance-Lab', 3, 'config', 'bci-lab-config-v1', NULL, 'generated', datetime('now'), datetime('now')),
('lab/sqlite/bci_phase1_schema.sql', 'bci_phase1_schema.sql', 'HorrorPlace-Constellation-Contracts', 1, 'sql', NULL, NULL, 'generated', datetime('now'), datetime('now')),
('lab/sqlite/bci_phase1_seed.sql', 'bci_phase1_seed.sql', 'HorrorPlace-Constellation-Contracts', 1, 'sql', NULL, 'bci_phase1_schema.sql', 'generated', datetime('now'), datetime('now')),
('contracts/events/chain_archival_line_alpha_v1.json', 'chain_archival_line_alpha_v1.json', 'HorrorPlace-Codebase-of-Death', 2, 'config', 'event_contract_v1', NULL, 'generated', datetime('now'), datetime('now')),
('contracts/personas/archivist_chainreactor_v1.json', 'archivist_chainreactor_v1.json', 'HorrorPlace-Spectral-Foundry', 2, 'config', 'persona_contract_v1', NULL, 'generated', datetime('now'), datetime('now'));

-- ----------------------------------------------------------------------------
-- Seed: constellation_progress_tracker (12-month investigation milestones)
-- ----------------------------------------------------------------------------
INSERT INTO constellation_progress_tracker (phase_id, milestone_name, repository, target_files, completed_files, validation_status, blocking_issues, assigned_to, due_date, created_at, updated_at) VALUES
('phase-1', 'BCI Schema Spine Complete', 'HorrorPlace-Constellation-Contracts', 11, 11, 'validated', '[]', 'AI-Chat-Agent', '2026-03-31', datetime('now'), datetime('now')),
('phase-1', 'Lab SQL Schema Complete', 'HorrorPlace-Constellation-Contracts', 2, 2, 'validated', '[]', 'AI-Chat-Agent', '2026-03-31', datetime('now'), datetime('now')),
('phase-1', 'Rust Safety Kernel', 'HorrorPlace-Codebase-of-Death', 1, 1, 'validated', '[]', 'AI-Chat-Agent', '2026-03-31', datetime('now'), datetime('now')),
('phase-1', 'Lua BCI Adapter', 'HorrorPlace-Codebase-of-Death', 1, 1, 'validated', '[]', 'AI-Chat-Agent', '2026-03-31', datetime('now'), datetime('now')),
('phase-2', 'Penum-Cube Analysis', 'HorrorPlace-Neural-Resonance-Lab', 3, 0, 'not_started', '[]', 'TBD', '2026-05-31', datetime('now'), datetime('now')),
('phase-2', 'Lab-Plague Ethics', 'HorrorPlace-Neural-Resonance-Lab', 3, 0, 'not_started', '[]', 'TBD', '2026-05-31', datetime('now'), datetime('now')),
('phase-2', 'Dead-City-Ruins Vignettes', 'HorrorPlace-Neural-Resonance-Lab', 3, 0, 'not_started', '[]', 'TBD', '2026-05-31', datetime('now'), datetime('now')),
('phase-3', 'Intervention Rules', 'HorrorPlace-Codebase-of-Death', 4, 4, 'validated', '[]', 'AI-Chat-Agent', '2026-09-30', datetime('now'), datetime('now')),
('phase-3', 'Tuning Playbook', 'HorrorPlace-Codebase-of-Death', 1, 1, 'validated', '[]', 'AI-Chat-Agent', '2026-09-30', datetime('now'), datetime('now')),
('phase-3', 'T3 Lab Experiments', 'HorrorPlace-Neural-Resonance-Lab', 5, 0, 'not_started', '[]', 'TBD', '2026-12-31', datetime('now'), datetime('now'));

-- ----------------------------------------------------------------------------
-- Seed: constellation_tuning_playbook
-- ----------------------------------------------------------------------------
INSERT INTO constellation_tuning_playbook (playbook_key, playbook_version, generated_date, phase_experiment_refs, rule_count, enabled_rule_count, global_constraints, approval_status, approved_by, approved_date, next_review_date, created_at, updated_at) VALUES
('PLAYBOOK.PHASE3.V1.0', '1.0.0', '2026-01-15T00:00:00Z',
 '["EXP.PHASE3.UEC.BOOSTER.2026.Q1", "EXP.PHASE3.HINT.CDL.2026.Q2", "EXP.PHASE3.HAPTIC.EMD.2026.Q3", "EXP.PHASE3.GATE.DET.2026.Q2"]',
 4, 4,
 '{"maxConcurrentInterventions": 3, "minInterventionSpacingSeconds": 30.0, "safetyProfileId": "bci-safety-profile-v1.monster-mode-standard"}',
 'APPROVED', 'DEAD-LEDGER.SIG.2026.Q1.BCI.PLAYBOOK', '2026-01-15T00:00:00Z', '2026-07-15T00:00:00Z',
 datetime('now'), datetime('now'));

-- ----------------------------------------------------------------------------
-- Seed: constellation_intervention_rule
-- ----------------------------------------------------------------------------
INSERT INTO constellation_intervention_rule (rule_key, rule_name, intervention_type, tier, trigger_conditions, action_parameters, safety_constraints, validation_metrics, playbook_id, enabled, created_at, updated_at) VALUES
('INT.UEC.BOOSTER.001', 'Underengagement Booster Profile', 'SPAWN_DENSITY', 'mature',
 '{"metricConditions": [{"metric": "UEC", "operator": "LT", "threshold": 0.30, "durationSeconds": 60.0}], "stressBand": "UNDERSTIMULATED"}',
 '{"spawnDensity": 0.8, "durationSeconds": 120.0, "rampUpSeconds": 10.0}',
 '{"maxDetIncrease": 1.5, "requiresCsiAbove": 0.70, "blockedInStressBands": ["OVERWHELMED"]}',
 '{"phase3ExperimentId": "EXP.PHASE3.UEC.BOOSTER.2026.Q1", "effectSize": 0.12, "generalizablePresets": ["penum-cube-lab-v1", "dead-city-ruins-vignette-v1", "city-quarantine-blackout-v1"], "safetyDegradationObserved": false}',
 'PLAYBOOK.PHASE3.V1.0', 1, datetime('now'), datetime('now')),
('INT.HINT.CDL.001', 'High CDL Evidence Hint System', 'HINT_SYSTEM', 'mature',
 '{"metricConditions": [{"metric": "CDL", "operator": "GT", "threshold": 0.70, "durationSeconds": 45.0}], "stressBand": "OPTIMALSTRESS"}',
 '{"hintIntensity": 0.3, "hintType": "evidence-highlight", "maxHintsPerSession": 5}',
 '{"maxDetIncrease": 0.5, "requiresCsiAbove": 0.60, "blockedInStressBands": ["OVERWHELMED"]}',
 '{"phase3ExperimentId": "EXP.PHASE3.HINT.CDL.2026.Q2", "effectSize": 0.08, "generalizablePresets": ["lab-plague-ethics-v1", "dead-city-ruins-vignette-v1", "asylum-replay-degradation-v1"], "safetyDegradationObserved": false}',
 'PLAYBOOK.PHASE3.V1.0', 1, datetime('now'), datetime('now')),
('INT.HAPTIC.EMD.001', 'Delta/Theta Haptic Synchronization', 'HAPTIC_SYNC', 'research',
 '{"metricConditions": [{"metric": "EMD", "operator": "LT", "threshold": 0.40, "durationSeconds": 30.0}], "stressBand": "OPTIMALSTRESS"}',
 '{"patternId": "pulse_slow", "hapticDrive": 0.5, "hapticRoutingBias": "center", "syncToBand": "theta"}',
 '{"maxDetIncrease": 0.75, "requiresCsiAbove": 0.65, "blockedInStressBands": ["OVERWHELMED", "UNDERSTIMULATED"]}',
 '{"phase3ExperimentId": "EXP.PHASE3.HAPTIC.EMD.2026.Q3", "effectSize": 0.08, "generalizablePresets": ["penum-cube-lab-v1", "lab-plague-ethics-v1", "city-quarantine-blackout-v1"], "safetyDegradationObserved": false}',
 'PLAYBOOK.PHASE3.V1.0', 1, datetime('now'), datetime('now')),
('INT.GATE.DET.001', 'DET-Based Content Gating for Tier-2', 'CONTENT_GATING', 'mature',
 '{"metricConditions": [{"metric": "DET", "operator": "GT", "threshold": 7.5, "durationSeconds": 0.0}], "stressBand": "OPTIMALSTRESS"}',
 '{"contentTier": "tier-2", "gateAction": "suppress", "fallbackTier": "tier-1", "recoveryThresholdDET": 6.0}',
 '{"maxDetIncrease": 0.0, "requiresCsiAbove": 0.50, "blockedInStressBands": []}',
 '{"phase3ExperimentId": "EXP.PHASE3.GATE.DET.2026.Q2", "effectSize": 0.22, "generalizablePresets": ["lab-plague-ethics-v1", "dead-city-ruins-vignette-v1", "asylum-replay-degradation-v1"], "safetyDegradationObserved": false}',
 'PLAYBOOK.PHASE3.V1.0', 1, datetime('now'), datetime('now'));
