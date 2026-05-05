# AI Task Index – HorrorPlace-Constellation-Contracts

This document lists structured coding tasks that improve the stability and AI-chat usability of the HorrorPlace-Constellation-Contracts repository. All tasks must be executed using SQLite and native tools only. Do not install or invoke Rustup or Cargo.

---

## Conventions

- Priority:
  - 1 = High (do these first)
  - 2 = Medium
  - 3 = Low
- Recommended tools:
  - sqlite3
  - sh / bash
  - find, grep, sed, awk, sort, xargs
  - jq (when available)
  - vi, nano, or other system editors
- Forbidden tools:
  - rustup
  - cargo
  - any global language toolchain installers

---

## Category: constellation-index

### Task: normalize_hp_repo

- Priority: 1  
- Goal: Ensure `hp_repo` accurately lists all HorrorPlace repositories with canonical names, URLs, and roles.
- Description:
  - Use sqlite3 to:
    - Confirm one row per HorrorPlace repo (e.g. Rotting-Visuals-BCI, HorrorPlace-Dead-Ledger-Network, HorrorPlace-Neural-Resonance-Lab, HorrorPlace-RotCave, HorrorPlace-Spectral-Foundry, HorrorPlace-Codebase-of-Death, HorrorPlace-Constellation-Contracts).
    - Normalize the `name` column for consistent casing and punctuation.
    - Ensure `git_url` points to the canonical GitHub URLs.
    - Set appropriate `role` values (runtime, contracts, ledger, analysis, tooling).
- Recommended tools: sqlite3, sh, grep, sed  
- Forbidden tools: rustup, cargo  
- Files touched:
  - db/schema/constellation_index.sql
  - db/queries/constellation-navigation.sql

---

### Task: populate_hp_component

- Priority: 1  
- Goal: Make `hp_component` a complete index of key schemas, SQL files, wiring documents, and core modules across HorrorPlace repos.
- Description:
  - For each HorrorPlace repo:
    - Use find/grep to locate important files under `schemas/`, `db/`, and `docs/`.
    - Insert one `hp_component` row per key file with:
      - `kind` (schema, sqlschema, bcipipeline, canregistry, doc, etc.).
      - `domain` (bci, palette, ledger, wiring, tooling).
      - A concise `summary` (100–200 characters).
    - Keep paths repo-relative and POSIX-style.
- Recommended tools: sqlite3, sh, find, grep, awk  
- Forbidden tools: rustup, cargo  
- Files touched:
  - db/schema/constellation_index.sql
  - db/queries/constellation-navigation.sql

---

### Task: check_constellation_index_script

- Priority: 2  
- Goal: Automatically detect missing `hp_repo` entries and keep the constellation index in sync with the filesystem.
- Description:
  - Create `scripts/check_constellation_index.sh` that:
    - Lists top-level repo directories for the constellation.
    - Uses sqlite3 to query `hp_repo`.
    - Fails (non-zero exit) when any repo directory has no matching `hp_repo` row.
  - Integrate this script into CI for HorrorPlace-Constellation-Contracts.
- Recommended tools: sh, sqlite3, grep  
- Forbidden tools: rustup, cargo  
- Files touched:
  - scripts/check_constellation_index.sh
  - db/schema/constellation_index.sql

---

## Category: pipeline

### Task: complete_bcipipelinestage

- Priority: 1  
- Goal: Ensure `bcipipelinestage` fully describes all major BCI-related stages across HorrorPlace repos.
- Description:
  - Identify pipeline stages for:
    - BCI ingestion / validation.
    - Geometry computation.
    - Theatre / arcade outputs.
    - Ledger logging / persistence.
  - Insert or update `bcipipelinestage` rows with:
    - `repo`, `stagekey`, `name`, `layer`, `inputtype`, `outputtype`, `primaryfile`.
  - Use short, stable `stagekey` values to make querying easier.
- Recommended tools: sqlite3, sh, grep, sed  
- Forbidden tools: rustup, cargo  
- Files touched:
  - db/schema/bci_pipeline.sql
  - db/queries/constellation-navigation.sql

---

### Task: define_bcipipelineedge_cross_repo

- Priority: 1  
- Goal: Capture cross-repo BCI wiring in `bcipipelineedge` so agents can see end-to-end flows via SQL only.
- Description:
  - For each known cross-repo hop (e.g. BCI server → theatre node → ledger):
    - Create or update `bcipipelineedge` entries with:
      - `fromstageid`, `tostageid`, `protocol`, `description`.
  - Ensure at least one path exists from canonical BCI input types to persistence layers.
- Recommended tools: sqlite3, sh, grep  
- Forbidden tools: rustup, cargo  
- Files touched:
  - db/schema/bci_pipeline.sql
  - db/queries/constellation-navigation.sql

---

### Task: check_bci_pipeline_wiring_script

- Priority: 2  
- Goal: Automatically detect orphan pipeline stages and missing end-to-end paths.
- Description:
  - Create `scripts/check_bci_pipeline_wiring.sh` that:
    - Uses sqlite3 to:
      - Find stages with no incoming or outgoing edges.
      - Verify at least one pipeline path from BCI input layers to persistence layers (e.g. ledger, analysis).
    - Prints a compact report and returns non-zero when wiring is incomplete.
- Recommended tools: sh, sqlite3  
- Forbidden tools: rustup, cargo  
- Files touched:
  - scripts/check_bci_pipeline_wiring.sh
  - db/schema/bci_pipeline.sql

---

## Category: fieldusage

### Task: deduplicate_fieldusage

- Priority: 2  
- Goal: Clean and normalize `fieldusage` so each field-location pair is unique and consistent.
- Description:
  - Use sqlite3 to:
    - Group `fieldusage` by `fieldpath`, `repo`, `locationtype`, and `locationpath`.
    - Identify duplicate rows or inconsistent `role` / `note` combinations.
  - Remove duplicates and standardize `role` and `note` texts for clarity.
- Recommended tools: sqlite3, awk, sed  
- Forbidden tools: rustup, cargo  
- Files touched:
  - db/schema/fieldusage.sql

---

### Task: ensure_core_fields_covered

- Priority: 1  
- Goal: Guarantee that critical BCI metrics and invariants are fully mapped in `fieldusage`.
- Description:
  - For key fields like:
    - `bciSummary.stressScore`
    - `bciSummary.visualOverloadIndex`
    - `bciSummary.startleSpike`
    - `invariants.CIC`, `invariants.DET`, `invariants.CDL`, `invariants.ARR`
  - Ensure `fieldusage` contains entries for:
    - JSON schemas (`locationtype = 'jsonschema'`).
    - SQL tables (`locationtype = 'sqltable'`).
    - Important code locations (`locationtype` like `ruststruct`, `cppstruct`, `shader`).
- Recommended tools: sqlite3, grep, awk  
- Forbidden tools: rustup, cargo  
- Files touched:
  - db/schema/fieldusage.sql

---

### Task: check_fieldusage_complete_script

- Priority: 2  
- Goal: Provide a quick health check that all must-track fields are represented in `fieldusage`.
- Description:
  - Create `scripts/check_fieldusage_complete.sh` that:
    - Contains a curated list of must-track `fieldpath` values.
    - Uses sqlite3 to verify each appears in at least one `jsonschema` and one `sqltable` entry.
    - Fails when any required field is missing.
- Recommended tools: sh, sqlite3  
- Forbidden tools: rustup, cargo  
- Files touched:
  - scripts/check_fieldusage_complete.sh
  - db/schema/fieldusage.sql

---

## Category: tooling

### Task: enforce_native_tools_policy

- Priority: 1  
- Goal: Make the native-tools-only constraint explicit and enforceable for coding agents.
- Description:
  - Ensure `docs/agent-tooling-policy.md`:
    - Explicitly forbids using Rustup and Cargo.
    - Explicitly prefers `sqlite3`, shell, and standard text tools.
    - States that Rust and other language files should be treated as static text (no compilation required).
- Recommended tools: vi, nano, sed  
- Forbidden tools: rustup, cargo  
- Files touched:
  - docs/agent-tooling-policy.md
  - docs/ai-metadata.json

---

### Task: wire_tooling_policy_in_metadata

- Priority: 1  
- Goal: Make sure AI agents discover the tooling policy early via the metadata file.
- Description:
  - Update `docs/ai-metadata.json` so it includes:
    - `documentation.toolingPolicy`: "docs/agent-tooling-policy.md"
  - Validate JSON syntax with jq (when available).
- Recommended tools: jq, vi, nano  
- Forbidden tools: rustup, cargo  
- Files touched:
  - docs/ai-metadata.json
  - docs/agent-tooling-policy.md

---

### Task: check_tooling_usage_script

- Priority: 2  
- Goal: Prevent the introduction of Rustup/Cargo usage into scripts or CI configs.
- Description:
  - Create `scripts/check_tooling_usage.sh` that:
    - Scans `scripts/` and CI configuration files for:
      - "rustup"
      - "cargo"
    - Fails if such commands are found.
  - Integrate this script into CI for HorrorPlace-Constellation-Contracts.
- Recommended tools: sh, grep, awk  
- Forbidden tools: rustup, cargo  
- Files touched:
  - scripts/check_tooling_usage.sh
  - .github (or other CI configuration directories)

---

## Category: docs

### Task: extend_for_ai_agents_workflows

- Priority: 2  
- Goal: Provide clear, repeatable workflows in `docs/for-ai-agents.md` tailored to this repository.
- Description:
  - Extend `docs/for-ai-agents.md` with:
    - Step-by-step workflows for:
      - Constellation navigation using `hp_repo` and `hp_component`.
      - BCI pipeline inspection using `bcipipelinestage` and `bcipipelineedge`.
      - Field tracking using `fieldusage`.
    - Example `sqlite3` commands to run the named queries in `db/queries/constellation-navigation.sql`.
- Recommended tools: vi, nano, sed  
- Forbidden tools: rustup, cargo  
- Files touched:
  - docs/for-ai-agents.md
  - db/queries/constellation-navigation.sql

---

### Task: document_multi_repo_wiring_survey

- Priority: 2  
- Goal: Make cross-repo wiring understandable without opening source code.
- Description:
  - Add a section to `docs/for-ai-agents.md` that explains:
    - How to use `hp_repo` and `hp_component` to list all repos participating in BCI, theatre, and ledger flows.
    - How to use `bcipipelinestage` and `bcipipelineedge` to reconstruct end-to-end wiring.
    - How to answer questions like “where does this BCI field end up?” using SQL only.
- Recommended tools: vi, nano  
- Forbidden tools: rustup, cargo  
- Files touched:
  - docs/for-ai-agents.md
  - db/schema/constellation_index.sql
  - db/schema/bci_pipeline.sql

---

### Task: add_low_token_diff_guidance

- Priority: 3  
- Goal: Guide AI agents toward small, schema-first changes that are cheap and safe.
- Description:
  - In `docs/for-ai-agents.md`, add guidance that:
    - Agents should prefer small, schema- and SQL-centric diffs in HorrorPlace-Constellation-Contracts when improving wiring and observability.
    - Only after schemas and indices are updated should agents suggest changes in runtime repos.
- Recommended tools: vi, nano  
- Forbidden tools: rustup, cargo  
- Files touched:
  - docs/for-ai-agents.md
