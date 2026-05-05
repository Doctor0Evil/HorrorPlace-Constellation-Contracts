-- File db/schema/ai_task_index_constellation_contracts.sql
-- Target repo Doctor0Evil/HorrorPlace-Constellation-Contracts
-- Purpose Structured task index for AI agents (SQLite-native, no Rust tools).

PRAGMA foreign_keys = ON;

------------------------------------------------------------
-- 1. Task index table
------------------------------------------------------------

CREATE TABLE IF NOT EXISTS ai_task (
    task_id        INTEGER PRIMARY KEY AUTOINCREMENT,
    category       TEXT NOT NULL,   -- e.g. "constellation-index", "pipeline", "fieldusage", "tooling", "docs"
    name           TEXT NOT NULL,   -- short identifier
    description    TEXT NOT NULL,   -- 1–3 sentence description
    priority       INTEGER NOT NULL DEFAULT 2,  -- 1=high, 2=medium, 3=low
    recommended_tools TEXT NOT NULL,   -- e.g. "sqlite3, sh, grep, sed"
    forbidden_tools   TEXT NOT NULL,   -- e.g. "rustup, cargo"
    files_touched  TEXT NOT NULL      -- comma-separated repo-relative paths
);

CREATE INDEX IF NOT EXISTS idx_ai_task_category
    ON ai_task (category, priority);

CREATE INDEX IF NOT EXISTS idx_ai_task_name
    ON ai_task (name);

------------------------------------------------------------
-- 2. Seed tasks for HorrorPlace-Constellation-Contracts
------------------------------------------------------------

INSERT INTO ai_task (
    category, name, description, priority,
    recommended_tools, forbidden_tools, files_touched
) VALUES
-- Category: constellation-index
(
    'constellation-index',
    'normalize_hp_repo',
    'Validate and normalize hp_repo entries so every HorrorPlace repository is present with a canonical name, git_url, and role.',
    1,
    'sqlite3, sh, grep, sed',
    'rustup, cargo',
    'db/schema/constellation_index.sql, db/queries/constellation-navigation.sql'
),
(
    'constellation-index',
    'populate_hp_component',
    'Populate hp_component with one row per key schema, SQL file, and wiring document across all HorrorPlace repos for cheap navigation.',
    1,
    'sqlite3, sh, find, grep, awk',
    'rustup, cargo',
    'db/schema/constellation_index.sql, db/queries/constellation-navigation.sql'
),
(
    'constellation-index',
    'check_constellation_index_script',
    'Add a shell script that verifies each repo directory has a matching hp_repo row and fails when entries are missing.',
    2,
    'sh, sqlite3, grep',
    'rustup, cargo',
    'scripts/check_constellation_index.sh, db/schema/constellation_index.sql'
),

-- Category: pipeline
(
    'pipeline',
    'complete_bcipipelinestage',
    'Ensure bcipipelinestage has entries for all ingestion, compute, theatre/arcade, and persistence stages across HorrorPlace repos.',
    1,
    'sqlite3, sh, grep, sed',
    'rustup, cargo',
    'db/schema/bci_pipeline.sql, db/queries/constellation-navigation.sql'
),
(
    'pipeline',
    'define_bcipipelineedge_cross_repo',
    'Define bcipipelineedge rows for cross-repo wiring from BCI ingestion through theatre/arcade nodes into ledger and analysis systems.',
    1,
    'sqlite3, sh, grep',
    'rustup, cargo',
    'db/schema/bci_pipeline.sql, db/queries/constellation-navigation.sql'
),
(
    'pipeline',
    'check_bci_pipeline_wiring_script',
    'Add a lint script that finds orphan pipeline stages and verifies at least one path from input types to persistence layers.',
    2,
    'sh, sqlite3',
    'rustup, cargo',
    'scripts/check_bci_pipeline_wiring.sh, db/schema/bci_pipeline.sql'
),

-- Category: fieldusage
(
    'fieldusage',
    'deduplicate_fieldusage',
    'Normalize and deduplicate fieldusage entries so each fieldpath-location pair appears once with a consistent role and note.',
    2,
    'sqlite3, awk, sed',
    'rustup, cargo',
    'db/schema/fieldusage.sql'
),
(
    'fieldusage',
    'ensure_core_fields_covered',
    'Ensure critical BCI fields and invariants have fieldusage rows for JSON schemas, SQL tables, and important code locations.',
    1,
    'sqlite3, grep, awk',
    'rustup, cargo',
    'db/schema/fieldusage.sql'
),
(
    'fieldusage',
    'check_fieldusage_complete_script',
    'Add a script that verifies all must-track fields appear in at least one jsonschema and one sqltable entry in fieldusage.',
    2,
    'sh, sqlite3',
    'rustup, cargo',
    'scripts/check_fieldusage_complete.sh, db/schema/fieldusage.sql'
),

-- Category: tooling
(
    'tooling',
    'enforce_native_tools_policy',
    'Ensure docs/agent-tooling-policy.md explicitly forbids rustup and cargo and prefers sqlite3 plus native Unix tools.',
    1,
    'vi, nano, sed',
    'rustup, cargo',
    'docs/agent-tooling-policy.md, docs/ai-metadata.json'
),
(
    'tooling',
    'wire_tooling_policy_in_metadata',
    'Add or update documentation.toolingPolicy in docs/ai-metadata.json to point to docs/agent-tooling-policy.md for this repo.',
    1,
    'jq, vi, nano',
    'rustup, cargo',
    'docs/ai-metadata.json, docs/agent-tooling-policy.md'
),
(
    'tooling',
    'check_tooling_usage_script',
    'Create a script that scans scripts and CI configs for rustup or cargo usage and fails when such commands are present.',
    2,
    'sh, grep, awk',
    'rustup, cargo',
    'scripts/check_tooling_usage.sh, .github'
),

-- Category: docs
(
    'docs',
    'extend_for_ai_agents_workflows',
    'Extend docs/for-ai-agents.md with step-by-step workflows for constellation wiring, using only sqlite3 and static file inspection.',
    2,
    'vi, nano, sed',
    'rustup, cargo',
    'docs/for-ai-agents.md, db/queries/constellation-navigation.sql'
),
(
    'docs',
    'document_multi_repo_wiring_survey',
    'Document how agents can use hp_repo, hp_component, bcipipelinestage, and bcipipelineedge to understand cross-repo wiring at low token cost.',
    2,
    'vi, nano',
    'rustup, cargo',
    'docs/for-ai-agents.md, db/schema/constellation_index.sql, db/schema/bci_pipeline.sql'
),
(
    'docs',
    'add_low_token_diff_guidance',
    'Add guidance that agents should prefer small, schema-centric diffs in HorrorPlace-Constellation-Contracts before editing runtime repos.',
    3,
    'vi, nano',
    'rustup, cargo',
    'docs/for-ai-agents.md'
);
