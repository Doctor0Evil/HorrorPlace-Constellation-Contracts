-- ============================================================================
-- BCI Constellation Wiring Tables v1
-- Purpose: Enable AI-Chat and coding-agents to interpret advanced wiring logic
--          across the HorrorPlace VM-Constellation.
-- Tier: 1 (Public, Canonical)
-- Repository: HorrorPlace-Constellation-Contracts
-- ============================================================================

-- ----------------------------------------------------------------------------
-- Table: constellation_vm_node
-- Purpose: Registry of all VM nodes in the constellation with tier and role info.
-- ----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS constellation_vm_node (
    vm_node_id          INTEGER PRIMARY KEY,
    vm_name             TEXT NOT NULL UNIQUE,      -- e.g., "Codebase-of-Death"
    repository_url      TEXT,                      -- GitHub or IPFS reference
    tier                INTEGER NOT NULL,          -- 1, 2, or 3
    roles               TEXT NOT NULL,             -- JSON array: ["Breeder", "Validator"]
    subscribed_chains   TEXT,                      -- JSON array of chain family IDs
    status              TEXT NOT NULL,             -- "active", "deprecated", "maintenance"
    last_heartbeat      TEXT,                      -- ISO 8601 timestamp
    created_at          TEXT NOT NULL,
    updated_at          TEXT NOT NULL
);

-- ----------------------------------------------------------------------------
-- Table: constellation_transition_channel
-- Purpose: Maps transition channels between VM nodes for chain-reaction events.
-- ----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS constellation_transition_channel (
    channel_id          INTEGER PRIMARY KEY,
    channel_name        TEXT NOT NULL UNIQUE,      -- e.g., "history_seed", "dead_ledger"
    source_vm_node_id   INTEGER NOT NULL,
    target_vm_node_id   INTEGER NOT NULL,
    data_schema_id      TEXT NOT NULL,             -- Reference to schema ID
    direction           TEXT NOT NULL,             -- "unidirectional", "bidirectional"
    priority            INTEGER NOT NULL DEFAULT 50,
    enabled             INTEGER NOT NULL DEFAULT 1,
    FOREIGN KEY (source_vm_node_id) REFERENCES constellation_vm_node(vm_node_id),
    FOREIGN KEY (target_vm_node_id) REFERENCES constellation_vm_node(vm_node_id)
);

-- ----------------------------------------------------------------------------
-- Table: constellation_chain_family
-- Purpose: Registry of chain-reaction families with invariant/metric weights.
-- ----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS constellation_chain_family (
    chain_family_id     INTEGER PRIMARY KEY,
    chain_family_key    TEXT NOT NULL UNIQUE,      -- e.g., "ARCHIVAL_LINE_ALPHA"
    description         TEXT,
    invariant_weights   TEXT NOT NULL,             -- JSON object: {"CIC": 0.6, "AOS": 1.0, ...}
    metric_weights      TEXT NOT NULL,             -- JSON object: {"UEC": 0.7, "EMD": 0.9, ...}
    tier                TEXT NOT NULL,             -- "standard", "mature", "research"
    status              TEXT NOT NULL,             -- "experimental", "candidate", "live"
    created_at          TEXT NOT NULL,
    updated_at          TEXT NOT NULL
);

-- ----------------------------------------------------------------------------
-- Table: constellation_event_contract
-- Purpose: Registry of event contracts with VM routing information.
-- ----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS constellation_event_contract (
    contract_id         INTEGER PRIMARY KEY,
    event_id            TEXT NOT NULL UNIQUE,      -- e.g., "EVT.CHAIN.ARCHIVAL_LINE_ALPHA.V1"
    event_name          TEXT NOT NULL,
    chain_family_id     INTEGER NOT NULL,
    tier                TEXT NOT NULL,
    vm_routing_map      TEXT NOT NULL,             -- JSON object mapping roles to VM nodes
    transition_channels TEXT,                      -- JSON array of channel IDs
    safety_profile_id   TEXT,
    status              TEXT NOT NULL,             -- "draft", "review", "approved", "deprecated"
    created_at          TEXT NOT NULL,
    updated_at          TEXT NOT NULL,
    FOREIGN KEY (chain_family_id) REFERENCES constellation_chain_family(chain_family_id)
);

-- ----------------------------------------------------------------------------
-- Table: constellation_persona_binding
-- Purpose: Maps persona contracts to VM nodes and chain families.
-- ----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS constellation_persona_binding (
    binding_id          INTEGER PRIMARY KEY,
    persona_id          TEXT NOT NULL,             -- e.g., "PERSONA.ARCHIVIST.CHAINREACTOR.V1"
    persona_name        TEXT NOT NULL,
    vm_node_id          INTEGER NOT NULL,
    chain_families      TEXT,                      -- JSON array of chain family keys
    invariant_hooks     TEXT,                      -- JSON object of invariant thresholds
    telemetry_bindings  TEXT,                      -- JSON object of telemetry schema refs
    status              TEXT NOT NULL,
    created_at          TEXT NOT NULL,
    updated_at          TEXT NOT NULL,
    FOREIGN KEY (vm_node_id) REFERENCES constellation_vm_node(vm_node_id)
);

-- ----------------------------------------------------------------------------
-- Table: constellation_intervention_rule
-- Purpose: Registry of intervention rules with validation metrics.
-- ----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS constellation_intervention_rule (
    rule_id             INTEGER PRIMARY KEY,
    rule_key            TEXT NOT NULL UNIQUE,      -- e.g., "INT.UEC.BOOSTER.001"
    rule_name           TEXT NOT NULL,
    intervention_type   TEXT NOT NULL,             -- e.g., "SPAWN_DENSITY", "HINT_SYSTEM"
    tier                TEXT NOT NULL,
    trigger_conditions  TEXT NOT NULL,             -- JSON object
    action_parameters   TEXT NOT NULL,             -- JSON object
    safety_constraints  TEXT NOT NULL,             -- JSON object
    validation_metrics  TEXT,                      -- JSON object
    playbook_id         TEXT,                      -- Reference to tuning playbook
    enabled             INTEGER NOT NULL DEFAULT 1,
    created_at          TEXT NOT NULL,
    updated_at          TEXT NOT NULL
);

-- ----------------------------------------------------------------------------
-- Table: constellation_tuning_playbook
-- Purpose: Registry of tuning playbooks with rule collections.
-- ----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS constellation_tuning_playbook (
    playbook_id         INTEGER PRIMARY KEY,
    playbook_key        TEXT NOT NULL UNIQUE,      -- e.g., "PLAYBOOK.PHASE3.V1.0"
    playbook_version    TEXT NOT NULL,
    generated_date      TEXT NOT NULL,             -- ISO 8601 timestamp
    phase_experiment_refs TEXT NOT NULL,           -- JSON array of experiment IDs
    rule_count          INTEGER NOT NULL,
    enabled_rule_count  INTEGER NOT NULL,
    global_constraints  TEXT NOT NULL,             -- JSON object
    approval_status     TEXT NOT NULL,             -- "DRAFT", "REVIEW", "APPROVED", "DEPRECATED"
    approved_by         TEXT,
    approved_date       TEXT,
    next_review_date    TEXT,
    created_at          TEXT NOT NULL,
    updated_at          TEXT NOT NULL
);

-- ----------------------------------------------------------------------------
-- Table: constellation_schema_registry
-- Purpose: Master registry of all schemas across the constellation.
-- ----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS constellation_schema_registry (
    schema_id           INTEGER PRIMARY KEY,
    schema_key          TEXT NOT NULL UNIQUE,      -- e.g., "bci-mapping-request-v1"
    schema_version      TEXT NOT NULL,
    tier                INTEGER NOT NULL,          -- 1, 2, or 3
    repository          TEXT NOT NULL,             -- e.g., "HorrorPlace-Constellation-Contracts"
    path                TEXT NOT NULL,             -- Relative path within repo
    description         TEXT,
    status              TEXT NOT NULL,             -- "active", "deprecated", "draft"
    created_at          TEXT NOT NULL,
    updated_at          TEXT NOT NULL
);

-- ----------------------------------------------------------------------------
-- Table: constellation_wiring_audit_log
-- Purpose: Audit trail for all wiring changes across the constellation.
-- ----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS constellation_wiring_audit_log (
    audit_id            INTEGER PRIMARY KEY,
    change_type         TEXT NOT NULL,             -- "CREATE", "UPDATE", "DELETE"
    entity_type         TEXT NOT NULL,             -- e.g., "vm_node", "event_contract"
    entity_id           INTEGER NOT NULL,
    old_value           TEXT,                      -- JSON of previous state
    new_value           TEXT,                      -- JSON of new state
    changed_by          TEXT NOT NULL,             -- User or system identifier
    change_reason       TEXT,
    changed_at          TEXT NOT NULL              -- ISO 8601 timestamp
);

-- ----------------------------------------------------------------------------
-- Table: constellation_file_manifest
-- Purpose: Tracks all generated files across repositories with status and dependencies.
-- ----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS constellation_file_manifest (
    file_id             INTEGER PRIMARY KEY,
    file_path           TEXT NOT NULL,             -- Relative path from repo root
    file_name           TEXT NOT NULL,             -- Filename with extension
    repository          TEXT NOT NULL,             -- Target repository name
    tier                INTEGER NOT NULL,          -- 1, 2, or 3
    file_type           TEXT NOT NULL,             -- "schema", "config", "script", "sql", "doc"
    schema_ref          TEXT,                      -- Reference to schema_key if applicable
    depends_on          TEXT,                      -- JSON array of file_id dependencies
    status              TEXT NOT NULL,             -- "generated", "pending", "deprecated"
    checksum_sha256     TEXT,                      -- File integrity hash
    generated_at        TEXT NOT NULL,
    updated_at          TEXT NOT NULL
);

-- ----------------------------------------------------------------------------
-- Table: constellation_progress_tracker
-- Purpose: Tracks 12-month investigation progress across all phases and repositories.
-- ----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS constellation_progress_tracker (
    tracker_id          INTEGER PRIMARY KEY,
    phase_id            TEXT NOT NULL,             -- "phase-1", "phase-2", "phase-3"
    milestone_name      TEXT NOT NULL,
    repository          TEXT NOT NULL,
    target_files        INTEGER NOT NULL,          -- Expected file count
    completed_files     INTEGER NOT NULL DEFAULT 0,
    validation_status   TEXT NOT NULL,             -- "not_started", "in_progress", "validated"
    blocking_issues     TEXT,                      -- JSON array of issue descriptions
    assigned_to         TEXT,
    due_date            TEXT,                      -- ISO 8601 date
    completed_at        TEXT,
    created_at          TEXT NOT NULL,
    updated_at          TEXT NOT NULL
);

-- ----------------------------------------------------------------------------
-- View: v_constellation_overview
-- Purpose: High-level overview of constellation state for AI-agent queries.
-- ----------------------------------------------------------------------------
CREATE VIEW IF NOT EXISTS v_constellation_overview AS
SELECT
    vn.vm_name,
    vn.tier,
    vn.roles,
    COUNT(DISTINCT cf.chain_family_key) AS chain_families_supported,
    COUNT(DISTINCT ec.event_id) AS event_contracts,
    COUNT(DISTINCT pb.persona_id) AS persona_bindings,
    COUNT(DISTINCT ir.rule_key) AS intervention_rules
FROM constellation_vm_node vn
LEFT JOIN constellation_persona_binding pb ON vn.vm_node_id = pb.vm_node_id
LEFT JOIN constellation_event_contract ec ON vn.vm_node_id = json_extract(ec.vm_routing_map, '$.vault')
LEFT JOIN constellation_chain_family cf ON ec.chain_family_id = cf.chain_family_id
LEFT JOIN constellation_intervention_rule ir ON 1=1
WHERE vn.status = 'active'
GROUP BY vn.vm_node_id;

-- ----------------------------------------------------------------------------
-- View: v_chain_family_weights
-- Purpose: Queryable view of chain family invariant/metric weights for AI agents.
-- ----------------------------------------------------------------------------
CREATE VIEW IF NOT EXISTS v_chain_family_weights AS
SELECT
    chain_family_key,
    description,
    tier,
    status,
    json_extract(invariant_weights, '$.CIC') AS cic_weight,
    json_extract(invariant_weights, '$.AOS') AS aos_weight,
    json_extract(invariant_weights, '$.LSG') AS lsg_weight,
    json_extract(invariant_weights, '$.SHCI') AS shci_weight,
    json_extract(metric_weights, '$.UEC') AS uec_weight,
    json_extract(metric_weights, '$.EMD') AS emd_weight,
    json_extract(metric_weights, '$.CDL') AS cdl_weight,
    json_extract(metric_weights, '$.ARR') AS arr_weight
FROM constellation_chain_family;

-- ----------------------------------------------------------------------------
-- View: v_intervention_rule_summary
-- Purpose: Quick reference for intervention rules and their trigger conditions.
-- ----------------------------------------------------------------------------
CREATE VIEW IF NOT EXISTS v_intervention_rule_summary AS
SELECT
    rule_key,
    rule_name,
    intervention_type,
    tier,
    json_extract(trigger_conditions, '$.stressBand') AS stress_band_trigger,
    json_extract(action_parameters, '$.spawnDensity') AS spawn_density,
    json_extract(action_parameters, '$.hintIntensity') AS hint_intensity,
    json_extract(safety_constraints, '$.maxDetIncrease') AS max_det_increase,
    enabled,
    updated_at
FROM constellation_intervention_rule;

-- ----------------------------------------------------------------------------
-- View: v_file_manifest_by_repository
-- Purpose: AI-agent query for file generation status per repository.
-- ----------------------------------------------------------------------------
CREATE VIEW IF NOT EXISTS v_file_manifest_by_repository AS
SELECT
    repository,
    tier,
    file_type,
    COUNT(*) AS total_files,
    SUM(CASE WHEN status = 'generated' THEN 1 ELSE 0 END) AS generated_count,
    SUM(CASE WHEN status = 'pending' THEN 1 ELSE 0 END) AS pending_count,
    SUM(CASE WHEN status = 'deprecated' THEN 1 ELSE 0 END) AS deprecated_count
FROM constellation_file_manifest
GROUP BY repository, tier, file_type;

-- ----------------------------------------------------------------------------
-- View: v_phase_progress_summary
-- Purpose: Track 12-month investigation progress across all phases.
-- ----------------------------------------------------------------------------
CREATE VIEW IF NOT EXISTS v_phase_progress_summary AS
SELECT
    phase_id,
    COUNT(*) AS total_milestones,
    SUM(target_files) AS total_target_files,
    SUM(completed_files) AS total_completed_files,
    ROUND(CAST(SUM(completed_files) AS FLOAT) / SUM(target_files) * 100, 2) AS completion_percentage,
    SUM(CASE WHEN validation_status = 'validated' THEN 1 ELSE 0 END) AS validated_milestones,
    SUM(CASE WHEN validation_status = 'in_progress' THEN 1 ELSE 0 END) AS in_progress_milestones,
    SUM(CASE WHEN validation_status = 'not_started' THEN 1 ELSE 0 END) AS not_started_milestones
FROM constellation_progress_tracker
GROUP BY phase_id;

-- ----------------------------------------------------------------------------
-- Indexes for AI-agent query performance.
-- ----------------------------------------------------------------------------
CREATE INDEX IF NOT EXISTS idx_vm_node_tier ON constellation_vm_node(tier);
CREATE INDEX IF NOT EXISTS idx_chain_family_tier ON constellation_chain_family(tier);
CREATE INDEX IF NOT EXISTS idx_event_contract_chain ON constellation_event_contract(chain_family_id);
CREATE INDEX IF NOT EXISTS idx_persona_binding_vm ON constellation_persona_binding(vm_node_id);
CREATE INDEX IF NOT EXISTS idx_intervention_rule_type ON constellation_intervention_rule(intervention_type);
CREATE INDEX IF NOT EXISTS idx_schema_registry_tier ON constellation_schema_registry(tier);
CREATE INDEX IF NOT EXISTS idx_schema_registry_repository ON constellation_schema_registry(repository);
CREATE INDEX IF NOT EXISTS idx_audit_log_entity ON constellation_wiring_audit_log(entity_type, entity_id);
CREATE INDEX IF NOT EXISTS idx_audit_log_changed_at ON constellation_wiring_audit_log(changed_at);
CREATE INDEX IF NOT EXISTS idx_file_manifest_repository ON constellation_file_manifest(repository);
CREATE INDEX IF NOT EXISTS idx_file_manifest_tier ON constellation_file_manifest(tier);
CREATE INDEX IF NOT EXISTS idx_file_manifest_status ON constellation_file_manifest(status);
CREATE INDEX IF NOT EXISTS idx_progress_tracker_phase ON constellation_progress_tracker(phase_id);
CREATE INDEX IF NOT EXISTS idx_progress_tracker_status ON constellation_progress_tracker(validation_status);
