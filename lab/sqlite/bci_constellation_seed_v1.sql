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
INSERT INTO constellation_vm_node (vm_name, tier, roles, subscribed_chains, status, created_at, updated_at) VALUES
('HorrorPlace-Constellation-Contracts', 1, '["Core"]', NULL, 'active', datetime('now'), datetime('now')),
('HorrorPlace-Codebase-of-Death', 2, '["Breeder", "Validator"]', '["ARCHIVAL_LINE_ALPHA", "HYDRO_LINE_BETA", "LIMEN_LINE_GAMMA"]', 'active', datetime('now'), datetime('now')),
('HorrorPlace-Neural-Resonance-Lab', 3, '["Experimenter"]', NULL, 'active', datetime('now'), datetime('now')),
('HorrorPlace-Black-Archivum', 2, '["Archivist"]', '["ARCHIVAL_LINE_ALPHA"]', 'active', datetime('now'), datetime('now')),
('HorrorPlace-Spectral-Foundry', 2, '["Persona-Host"]', '["ARCHIVAL_LINE_ALPHA", "HYDRO_LINE_BETA", "LIMEN_LINE_GAMMA"]', 'active', datetime('now'), datetime('now')),
('HorrorPlace-Atrocity-Seeds', 2, '["Event-Host"]', '["ARCHIVAL_LINE_ALPHA", "HYDRO_LINE_BETA", "LIMEN_LINE_GAMMA"]', 'active', datetime('now'), datetime('now')),
('HorrorPlace-Obscura-Nexus', 2, '["Style-Host"]', NULL, 'active', datetime('now'), datetime('now')),
('HorrorPlace-Liminal-Continuum', 2, '["Agent-Router"]', '["LIMEN_LINE_GAMMA"]', 'active', datetime('now'), datetime('now')),
('HorrorPlace-Dead-Ledger-Network', 2, '["Ledger-Keeper"]', '["ARCHIVAL_LINE_ALPHA", "LIMEN_LINE_GAMMA"]', 'active', datetime('now'), datetime('now')),
('HorrorPlace-Redacted-Chronicles', 3, '["Telemetry-Host"]', NULL, 'active', datetime('now'), datetime('now')),
('HorrorPlace-Process-Gods-Research', 3, '["Experimental"]', NULL, 'active', datetime('now'), datetime('now'));

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
