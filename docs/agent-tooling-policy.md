# Agent Tooling Policy for HorrorPlace-Constellation-Contracts

This document defines the allowed and prohibited tooling for AI agents and human contributors working in the `HorrorPlace-Constellation-Contracts` repository.

## Purpose

This repository serves as the **schema spine** and **constellation index** for the HorrorPlace ecosystem. It must remain:

- **Lightweight**: No heavy build dependencies.
- **Portable**: Runnable on any system with basic POSIX tools.
- **AI-friendly**: Optimized for low-token, query-driven navigation.

## Allowed Tools

### Primary Tools (Always Permitted)

| Tool | Purpose | Examples |
|------|---------|----------|
| `sqlite3` | All database work, schema creation, queries | Creating constellation index, running navigation queries |
| `sh` / `bash` | Scripting, automation | CI scripts, data population scripts |
| `find` | File discovery | Locating schemas, SQL files, docs |
| `grep` | Text search | Finding field references, pattern matching |
| `awk` | Text processing | Parsing manifests, extracting data |
| `sed` | Stream editing | Normalizing names, transforming text |
| `cat`, `head`, `tail` | File inspection | Reading schema files, viewing logs |

### Secondary Tools (Permitted When Available)

| Tool | Purpose | Notes |
|------|---------|-------|
| `jq` | JSON parsing | Optional; use grep/sed as fallback |
| `rg` (ripgrep) | Fast text search | Optional; grep is always available |

## Prohibited Tools

### Never Install or Invoke

The following tools are **explicitly forbidden** in this repository:

| Tool | Reason |
|------|--------|
| `rustup` | This repo treats Rust code as static text; no compilation required |
| `cargo` | No Rust builds; Rust crates are indexed but not compiled here |
| `npm` / `yarn` / `pnpm` | No Node.js build steps in this repo |
| `pip` (for build) | Python scripts may exist as tools, but no package installation in CI |
| `cmake`, `make` (for builds) | No native compilation in this repo |

### Rationale

1. **Constellation-Contracts is a schema repository**, not a runtime. It defines contracts, wiring, and indexes—not executable code.
2. **AI agents need cheap navigation**. Installing toolchains increases token cost and complexity.
3. **Cross-repo consistency**. Other HorrorPlace repos (Rotting-Visuals-BCI, Dead-Ledger, etc.) handle their own builds. This repo only indexes them.

## How to Work Within These Constraints

### For Database Work

Always use `sqlite3`:

```sh
# Create/update schema
sqlite3 db/constellation-index.db < db/constellation_index.sql

# Run a query
sqlite3 db/constellation-index.db "SELECT * FROM hp_repo WHERE role = 'runtime';"
```

### For Code Analysis

Use shell + text tools:

```sh
# Find all BCI-related schemas
find schemas -name '*bci*.json' -type f

# Grep for field usage
grep -r "stressScore" schemas/ db/

# Extract repo names from manifests
grep '"repoName"' manifests/*.json | sed 's/.*"\([^"]*\)".*/\1/'
```

### For Rust/C++ Code

Treat as **static text only**:

- Do NOT compile Rust crates in this repo.
- Do NOT run `cargo check`, `cargo build`, or `cargo test`.
- Use `grep`, `awk`, or manual inspection to understand code structure.
- Index Rust files in `hp_component` table for AI navigation, but do not build them.

## CI/CD Enforcement

The repository includes CI scripts that enforce this policy:

- `scripts/check_tooling_usage.sh`: Scans for prohibited commands (`rustup`, `cargo`, etc.).
- `scripts/check_constellation_index.sh`: Validates constellation index completeness.
- `scripts/check_bci_pipeline_wiring.sh`: Lints BCI pipeline wiring.
- `scripts/check_fieldusage_complete.sh`: Verifies field coverage.
- `scripts/test_constellation_queries.sh`: Smoke-tests navigation queries.

These scripts run in CI and will fail if prohibited tooling is detected.

## Exceptions

If you believe an exception is necessary:

1. Open a discussion issue explaining the use case.
2. Propose an alternative using allowed tools.
3. Only if no alternative exists, propose a policy amendment with justification.

## Related Documents

- `docs/for-ai-agents.md`: AI agent navigation guide.
- `db/queries/constellation-navigation.sql`: Canonical query vocabulary.
- `docs/constellation-index.json`: JSON representation of constellation index.

---

**Policy Version**: 1.0  
**Last Updated**: 2024  
**Enforced By**: CI workflows and human review
