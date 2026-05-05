-- File db/constellation_index.sql
-- Target repo Doctor0Evil/HorrorPlace-Constellation-Contracts
-- Purpose Cross-repo index of Horror$Place constellation.

PRAGMA foreign_keys = ON;

------------------------------------------------------------
-- 1. Repositories known to the constellation.
------------------------------------------------------------

CREATE TABLE IF NOT EXISTS hp_repo (
    repo_id         INTEGER PRIMARY KEY AUTOINCREMENT,
    -- Human-facing name and canonical git URL.
    name            TEXT NOT NULL UNIQUE,  -- e.g. Rotting-Visuals-BCI
    git_url         TEXT NOT NULL,         -- e.g. https://github.com/Doctor0Evil/Rotting-Visuals-BCI

    -- Local checkout information for mono-workspaces or CI sandboxes.
    local_root      TEXT,                  -- relative path from constellation root if present
    local_checkout  INTEGER NOT NULL DEFAULT 0,  -- 1 if present locally, 0 if metadata-only

    -- Role and lifecycle metadata.
    role            TEXT NOT NULL DEFAULT 'runtime', -- runtime, contracts, analysis, tooling, etc.
    is_temporary    INTEGER NOT NULL DEFAULT 0,      -- 1 for per-session / experiment repos
    session_id      TEXT,                            -- optional session key for temporary repos

    -- Basic timestamps; application is responsible for updating them.
    created_at      INTEGER NOT NULL DEFAULT (strftime('%s','now')),
    updated_at      INTEGER NOT NULL DEFAULT (strftime('%s','now'))
);

CREATE INDEX IF NOT EXISTS idx_hp_repo_name
    ON hp_repo (name);

CREATE INDEX IF NOT EXISTS idx_hp_repo_temp
    ON hp_repo (is_temporary, session_id);

------------------------------------------------------------
-- 2. Optional ignore list for non-repo directories.
------------------------------------------------------------

CREATE TABLE IF NOT EXISTS hp_repo_ignore (
    path    TEXT PRIMARY KEY,   -- top-level or relative path to ignore
    reason  TEXT NOT NULL       -- short explanation for AI agents / tooling
);

------------------------------------------------------------
-- 3. Components inside each repo (schemas, SQL, Rust modules, etc.).
------------------------------------------------------------

CREATE TABLE IF NOT EXISTS hp_component (
    component_id INTEGER PRIMARY KEY AUTOINCREMENT,

    -- Foreign key into hp_repo.
    repo_id      INTEGER NOT NULL REFERENCES hp_repo(repo_id) ON DELETE CASCADE,

    -- Component classification.
    kind         TEXT NOT NULL,       -- schema, sqlschema, rustmodule, palette, bcipipeline, canregistry, doc, etc.
    path         TEXT NOT NULL,       -- repo-relative POSIX path

    -- Short 100–200 char description, auto-populated from schemas / docs.
    summary      TEXT NOT NULL,

    -- Optional hints for AI/agents (cheap routing).
    domain       TEXT,                -- bci, palette, ledger, wiring, etc.
    tags         TEXT,                -- comma-separated tags (e.g. "request,response,monstermode")

    -- Timestamps for staleness detection.
    created_at   INTEGER NOT NULL DEFAULT (strftime('%s','now')),
    updated_at   INTEGER NOT NULL DEFAULT (strftime('%s','now'))
);

CREATE INDEX IF NOT EXISTS idx_hp_component_repo
    ON hp_component (repo_id, kind);

CREATE INDEX IF NOT EXISTS idx_hp_component_kind
    ON hp_component (kind);

CREATE INDEX IF NOT EXISTS idx_hp_component_path
    ON hp_component (path);

------------------------------------------------------------
-- 4. Lightweight per-repo metadata cache for JSON sync.
------------------------------------------------------------

CREATE TABLE IF NOT EXISTS hp_repo_meta (
    repo_id          INTEGER PRIMARY KEY REFERENCES hp_repo(repo_id) ON DELETE CASCADE,
    ai_metadata_path TEXT,    -- e.g. docs/ai-metadata.json (repo-relative)
    last_modified    INTEGER, -- Unix seconds: last time this repo's entry was rebuilt into constellation index
    notes            TEXT
);
