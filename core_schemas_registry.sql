-- File: core_schemas_registry.sql
-- Target repo: Doctor0Evil/HorrorPlace-Constellation-Contracts
-- Destination: core/schemas/db/core_schemas_registry.sql
-- Purpose: SQLite database schema for indexing and tracking all core schemas in the Horror$Place constellation

PRAGMA foreign_keys = ON;
PRAGMA journal_mode = WAL;

-- =============================================================================
-- TABLE: schema_registry
-- Tracks all schema files in core/schemas, their versions, and metadata
-- =============================================================================
CREATE TABLE IF NOT EXISTS schema_registry (
    schema_id           INTEGER PRIMARY KEY AUTOINCREMENT,
    schema_name         TEXT NOT NULL UNIQUE,
    schema_version      TEXT NOT NULL,
    file_path           TEXT NOT NULL,
    schema_type         TEXT NOT NULL CHECK (
        schema_type IN (
            'manifest',
            'invariant',
            'metric',
            'contract',
            'binding',
            'registry',
            'workflow',
            'governance'
        )
    ),
    json_schema_id      TEXT NOT NULL,
    title               TEXT,
    description         TEXT,
    created_timestamp   INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_timestamp   INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    is_active           INTEGER NOT NULL DEFAULT 1 CHECK (is_active IN (0, 1)),
    constellation_scope TEXT NOT NULL DEFAULT 'Horror$Place',
    UNIQUE(schema_name, schema_version)
);

-- =============================================================================
-- TABLE: schema_dependencies
-- Tracks $ref dependencies between schemas
-- =============================================================================
CREATE TABLE IF NOT EXISTS schema_dependencies (
    dependency_id       INTEGER PRIMARY KEY AUTOINCREMENT,
    parent_schema_id    INTEGER NOT NULL REFERENCES schema_registry(schema_id)
                          ON DELETE CASCADE,
    referenced_schema   TEXT NOT NULL,
    reference_path      TEXT NOT NULL,
    is_required         INTEGER NOT NULL DEFAULT 1 CHECK (is_required IN (0, 1)),
    created_timestamp   INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

-- =============================================================================
-- TABLE: schema_properties
-- Flattened key-value index of all top-level and nested properties
-- =============================================================================
CREATE TABLE IF NOT EXISTS schema_properties (
    property_id         INTEGER PRIMARY KEY AUTOINCREMENT,
    schema_id           INTEGER NOT NULL REFERENCES schema_registry(schema_id)
                          ON DELETE CASCADE,
    property_path       TEXT NOT NULL,
    property_name       TEXT NOT NULL,
    property_type       TEXT NOT NULL,
    is_required         INTEGER NOT NULL DEFAULT 0 CHECK (is_required IN (0, 1)),
    enum_values         TEXT,
    description         TEXT,
    default_value       TEXT
);

-- =============================================================================
-- TABLE: schema_validation_rules
-- Stores validation constraints (min, max, pattern, etc.)
-- =============================================================================
CREATE TABLE IF NOT EXISTS schema_validation_rules (
    rule_id             INTEGER PRIMARY KEY AUTOINCREMENT,
    schema_id           INTEGER NOT NULL REFERENCES schema_registry(schema_id)
                          ON DELETE CASCADE,
    property_path       TEXT NOT NULL,
    rule_type           TEXT NOT NULL CHECK (
        rule_type IN (
            'minimum',
            'maximum',
            'pattern',
            'minLength',
            'maxLength',
            'minItems',
            'maxItems',
            'enum',
            'const',
            'format'
        )
    ),
    rule_value          TEXT NOT NULL,
    description         TEXT
);

-- =============================================================================
-- TABLE: constellation_repos
-- Links schemas to repositories that implement/use them
-- =============================================================================
CREATE TABLE IF NOT EXISTS constellation_repos (
    repo_id             INTEGER PRIMARY KEY AUTOINCREMENT,
    repo_name           TEXT NOT NULL UNIQUE,
    repo_url            TEXT NOT NULL,
    repo_type           TEXT NOT NULL CHECK (
        repo_type IN (
            'core-contracts',
            'bci-engine',
            'ledger',
            'neural-lab',
            'foundry',
            'rotcave',
            'death-engine',
            'experience-pack',
            'toolkit'
        )
    ),
    manifest_schema_id  INTEGER REFERENCES schema_registry(schema_id)
                          ON DELETE SET NULL,
    is_active           INTEGER NOT NULL DEFAULT 1 CHECK (is_active IN (0, 1)),
    created_timestamp   INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_timestamp   INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

-- =============================================================================
-- TABLE: repo_schema_usage
-- Tracks which repos use which schemas (many-to-many)
-- =============================================================================
CREATE TABLE IF NOT EXISTS repo_schema_usage (
    usage_id            INTEGER PRIMARY KEY AUTOINCREMENT,
    repo_id             INTEGER NOT NULL REFERENCES constellation_repos(repo_id)
                          ON DELETE CASCADE,
    schema_id           INTEGER NOT NULL REFERENCES schema_registry(schema_id)
                          ON DELETE CASCADE,
    usage_type          TEXT NOT NULL CHECK (
        usage_type IN (
            'implements',
            'references',
            'extends',
            'validates-against',
            'generates'
        )
    ),
    usage_context       TEXT,
    created_timestamp   INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    UNIQUE(repo_id, schema_id, usage_type)
);

-- =============================================================================
-- TABLE: schema_evolution_log
-- Audit trail for schema changes and versioning
-- =============================================================================
CREATE TABLE IF NOT EXISTS schema_evolution_log (
    log_id              INTEGER PRIMARY KEY AUTOINCREMENT,
    schema_id           INTEGER NOT NULL REFERENCES schema_registry(schema_id)
                          ON DELETE CASCADE,
    previous_version    TEXT,
    new_version         TEXT NOT NULL,
    change_type         TEXT NOT NULL CHECK (
        change_type IN (
            'created',
            'property-added',
            'property-removed',
            'property-modified',
            'constraint-added',
            'constraint-removed',
            'deprecated',
            'breaking-change'
        )
    ),
    change_description  TEXT NOT NULL,
    author              TEXT,
    timestamp           INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

-- =============================================================================
-- TABLE: invariant_gates
-- Stores constellation invariant definitions (CIC, AOS, DET, LSG, etc.)
-- =============================================================================
CREATE TABLE IF NOT EXISTS invariant_gates (
    invariant_id        INTEGER PRIMARY KEY AUTOINCREMENT,
    invariant_code      TEXT NOT NULL UNIQUE,
    invariant_name      TEXT NOT NULL,
    category            TEXT NOT NULL CHECK (
        category IN (
            'core',
            'entertainment',
            'safety',
            'performance',
            'governance'
        )
    ),
    value_range_min     REAL NOT NULL DEFAULT 0.0,
    value_range_max     REAL NOT NULL DEFAULT 1.0,
    is_required         INTEGER NOT NULL DEFAULT 0 CHECK (is_required IN (0, 1)),
    description         TEXT NOT NULL,
    schema_id           INTEGER REFERENCES schema_registry(schema_id)
                          ON DELETE SET NULL,
    created_timestamp   INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

-- =============================================================================
-- INDEXES for performance optimization
-- =============================================================================
CREATE INDEX IF NOT EXISTS idx_schema_registry_type
    ON schema_registry(schema_type);

CREATE INDEX IF NOT EXISTS idx_schema_registry_active
    ON schema_registry(is_active);

CREATE INDEX IF NOT EXISTS idx_schema_registry_name_version
    ON schema_registry(schema_name, schema_version);

CREATE INDEX IF NOT EXISTS idx_schema_dependencies_parent
    ON schema_dependencies(parent_schema_id);

CREATE INDEX IF NOT EXISTS idx_schema_dependencies_ref
    ON schema_dependencies(referenced_schema);

CREATE INDEX IF NOT EXISTS idx_schema_properties_schema
    ON schema_properties(schema_id);

CREATE INDEX IF NOT EXISTS idx_schema_properties_path
    ON schema_properties(property_path);

CREATE INDEX IF NOT EXISTS idx_schema_properties_name
    ON schema_properties(property_name);

CREATE INDEX IF NOT EXISTS idx_schema_validation_schema
    ON schema_validation_rules(schema_id);

CREATE INDEX IF NOT EXISTS idx_constellation_repos_type
    ON constellation_repos(repo_type);

CREATE INDEX IF NOT EXISTS idx_repo_schema_usage_repo
    ON repo_schema_usage(repo_id);

CREATE INDEX IF NOT EXISTS idx_repo_schema_usage_schema
    ON repo_schema_usage(schema_id);

CREATE INDEX IF NOT EXISTS idx_schema_evolution_schema
    ON schema_evolution_log(schema_id);

CREATE INDEX IF NOT EXISTS idx_schema_evolution_timestamp
    ON schema_evolution_log(timestamp DESC);

CREATE INDEX IF NOT EXISTS idx_invariant_gates_code
    ON invariant_gates(invariant_code);

CREATE INDEX IF NOT EXISTS idx_invariant_gates_category
    ON invariant_gates(category);

-- =============================================================================
-- VIEWS for common queries
-- =============================================================================

-- View: Active schemas with their dependency counts
CREATE VIEW IF NOT EXISTS v_active_schemas AS
SELECT 
    sr.schema_id,
    sr.schema_name,
    sr.schema_version,
    sr.schema_type,
    sr.file_path,
    sr.json_schema_id,
    sr.title,
    COUNT(DISTINCT sd.dependency_id) AS dependency_count,
    COUNT(DISTINCT rsu.usage_id) AS repo_usage_count,
    sr.created_timestamp,
    sr.updated_timestamp
FROM schema_registry sr
LEFT JOIN schema_dependencies sd ON sr.schema_id = sd.parent_schema_id
LEFT JOIN repo_schema_usage rsu ON sr.schema_id = rsu.schema_id
WHERE sr.is_active = 1
GROUP BY sr.schema_id;

-- View: Repository schema implementation matrix
CREATE VIEW IF NOT EXISTS v_repo_schema_matrix AS
SELECT 
    cr.repo_name,
    cr.repo_type,
    sr.schema_name,
    sr.schema_version,
    rsu.usage_type,
    rsu.usage_context
FROM constellation_repos cr
JOIN repo_schema_usage rsu ON cr.repo_id = rsu.repo_id
JOIN schema_registry sr ON rsu.schema_id = sr.schema_id
WHERE cr.is_active = 1 AND sr.is_active = 1;

-- View: Schema evolution timeline
CREATE VIEW IF NOT EXISTS v_schema_timeline AS
SELECT 
    sr.schema_name,
    sel.previous_version,
    sel.new_version,
    sel.change_type,
    sel.change_description,
    sel.author,
    sel.timestamp
FROM schema_evolution_log sel
JOIN schema_registry sr ON sel.schema_id = sr.schema_id
ORDER BY sel.timestamp DESC;

-- =============================================================================
-- SEED DATA: Current repos-manifest-v1.json schema
-- =============================================================================
INSERT OR IGNORE INTO schema_registry (
    schema_name,
    schema_version,
    file_path,
    schema_type,
    json_schema_id,
    title,
    description,
    constellation_scope
) VALUES (
    'repos-manifest-v1',
    'v1',
    'core/schemas/repos-manifest-v1.json',
    'manifest',
    'repos-manifest-v1.json',
    'Horror$Place Repository Manifest Schema',
    'Defines the repository manifest contract for the Horror$Place constellation, establishing how repositories declare their identity, capabilities, dependencies, and wiring points across the multi-repo VM-constellation.',
    'Horror$Place'
);

-- Link repos-manifest-v1 to HorrorPlace-Constellation-Contracts repo
INSERT OR IGNORE INTO constellation_repos (
    repo_name,
    repo_url,
    repo_type
) VALUES (
    'HorrorPlace-Constellation-Contracts',
    'https://github.com/Doctor0Evil/HorrorPlace-Constellation-Contracts',
    'core-contracts'
);

INSERT OR IGNORE INTO repo_schema_usage (
    repo_id,
    schema_id,
    usage_type,
    usage_context
) VALUES (
    (SELECT repo_id FROM constellation_repos WHERE repo_name = 'HorrorPlace-Constellation-Contracts'),
    (SELECT schema_id FROM schema_registry WHERE schema_name = 'repos-manifest-v1'),
    'implements',
    'Root manifest schema for all constellation repositories'
);

-- Seed core invariants (CIC, AOS, DET, LSG)
INSERT OR IGNORE INTO invariant_gates (invariant_code, invariant_name, category, is_required, description) VALUES
    ('CIC', 'Constellation Integrity Coefficient', 'core', 1, 'Measures overall system health and schema compliance across constellation'),
    ('AOS', 'Attention Orientation Score', 'core', 1, 'Tracks user attention focus and cognitive load state'),
    ('DET', 'Detection Reliability', 'core', 1, 'Signal detection confidence and validity metric'),
    ('LSG', 'Latency Safety Guard', 'core', 1, 'Ensures safe response times and prevents dangerous lag spikes');

-- Seed optional entertainment invariants
INSERT OR IGNORE INTO invariant_gates (invariant_code, invariant_name, category, is_required, description) VALUES
    ('UEC', 'User Engagement Coefficient', 'entertainment', 0, 'Measures sustained user engagement and immersion'),
    ('EMD', 'Emotional Modulation Depth', 'entertainment', 0, 'Tracks emotional intensity and dynamic range'),
    ('STCI', 'Stress-To-Challenge Index', 'entertainment', 0, 'Balances stress response against perceived challenge level'),
    ('CDL', 'Cognitive Demand Level', 'entertainment', 0, 'Measures mental workload and processing requirements'),
    ('ARR', 'Adaptive Response Rate', 'entertainment', 0, 'Tracks system responsiveness to state changes');

-- Log creation event
INSERT INTO schema_evolution_log (
    schema_id,
    previous_version,
    new_version,
    change_type,
    change_description,
    author
) VALUES (
    (SELECT schema_id FROM schema_registry WHERE schema_name = 'repos-manifest-v1'),
    NULL,
    'v1',
    'created',
    'Initial schema created for Horror$Place constellation repository manifest contract',
    'Doctor0Evil'
);

-- =============================================================================
-- TRIGGERS for automated timestamp updates
-- =============================================================================
CREATE TRIGGER IF NOT EXISTS update_schema_registry_timestamp
AFTER UPDATE ON schema_registry
FOR EACH ROW
BEGIN
    UPDATE schema_registry 
    SET updated_timestamp = strftime('%s', 'now')
    WHERE schema_id = NEW.schema_id;
END;

CREATE TRIGGER IF NOT EXISTS update_constellation_repos_timestamp
AFTER UPDATE ON constellation_repos
FOR EACH ROW
BEGIN
    UPDATE constellation_repos 
    SET updated_timestamp = strftime('%s', 'now')
    WHERE repo_id = NEW.repo_id;
END;
