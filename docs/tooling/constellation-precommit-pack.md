# Constellation Pre-Commit Pack: Reusable Validation Hooks

This document defines the `constellation-precommit-pack`—a reusable set of local pre-commit hooks and CI workflows that enforce HorrorPlace contract rules before code lands in any constellation repository.

## Purpose

The pre-commit pack ensures that:

1. **Schema compliance**: All JSON files conform to their declared canonical schemas.
2. **Registry integrity**: NDJSON entries have required fields, valid IDs, and proper references.
3. **AI authoring discipline**: Generated contract cards embed valid `prismMeta` and respect one-file-per-request.
4. **Cross-repo consistency**: Changes propagate validation signals to dependent repositories via the spine index.

By bundling these checks into a standard, importable package, downstream repos can adopt constellation governance with minimal configuration.

## Components

The pre-commit pack includes:

### 1. Local Pre-Commit Hooks (`tooling/python/cli/`)

Executable Python scripts that run on `git commit`:

| Script | Purpose | Exit Code on Failure |
|--------|---------|---------------------|
| `hpc-validate-schema.py` | Validate JSON files against canonical schemas | 1 |
| `hpc-lint-registry.py` | Lint NDJSON registry entries for required fields and reference formats | 1 |
| `hpc-generate-spine-index.py` | Regenerate spine index locally for impact analysis (optional) | 0 (warns on drift) |

Usage example (`.pre-commit-config.yaml` in downstream repo):

```yaml
repos:
  - repo: https://github.com/Doctor0Evil/HorrorPlace-Constellation-Contracts
    rev: v1.0.0  # Pin to a stable tag
    hooks:
      - id: hpc-validate-schema
        args: [--mode, batch, --root, schemas/]
      - id: hpc-lint-registry
        args: [--mode, batch, --root, registry/]
      - id: hpc-generate-spine-index
        args: [--diff, --warn-only]  # Warn on drift, don't block commit
```

### 2. Reusable GitHub Actions Workflows (`.github/workflows/`)

CI jobs that other repos can call via `uses:`:

| Workflow | Purpose | Inputs |
|----------|---------|--------|
| `schema-validate.yml` | Validate all `.json` files in `schemas/` against canonical schemas | `python_version`, `strict_mode` |
| `registry-lint.yml` | Lint NDJSON registry files for format and reference compliance | `python_version`, `pattern` |
| `ai-authoring-validate.yml` | Validate AI-generated contract cards for prismMeta and invariant compliance | `contract_file_pattern` |
| `constellation-precommit-pack.yml` | Meta-workflow that runs all three above in one job | `validate_schemas`, `lint_registries`, `validate_ai_authored`, `python_version` |

Usage example (downstream repo's `.github/workflows/ci.yml`):

```yaml
name: Constellation Validation
on: [pull_request, push]
jobs:
  validate:
    uses: Doctor0Evil/HorrorPlace-Constellation-Contracts/.github/workflows/constellation-precommit-pack.yml@main
    with:
      validate_schemas: true
      lint_registries: true
      validate_ai_authored: true
      python_version: "3.11"
```

### 3. Configuration Templates

Starter configs for easy adoption:

- `.pre-commit-config.example.yaml`: Local hook configuration template.
- `ci-minimal.example.yml`: Minimal CI workflow that imports the pre-commit pack.
- `.horrorplace-contracts.example.json`: Template for declaring repo dependencies and pinned schema versions.

## Validation Rules Enforced

### Schema Validation (`hpc-validate-schema.py`)

- All `.json` files must have a valid `$schema` declaration.
- Files must parse as valid JSON (no trailing commas, proper quoting).
- If `--mode ai-authoring`, additionally check:
  - `prismMeta` is present and structurally valid.
  - `targetRepo`, `targetPath`, `schemaRef`, `tier` are consistent with `prismMeta.linkage`.
  - Invariant/metric values fall within canonical ranges from the spine index.

### Registry Linting (`hpc-lint-registry.py`)

- Each NDJSON line must parse as valid JSON.
- Required fields (`id`, `schemaref`, `deadledgerref`, `artifactid`, `createdAt`, `status`) must be present.
- `id` must follow the `<TYPE>-<CODE>-<SEQ>` naming convention.
- `schemaref` must point to an existing canonical schema `$id`.
- `deadledgerref` must match the pattern expected for the entry's `tier`.
- No duplicate `id` values within the same registry file.

### AI Authoring Validation (`hpc-validate-schema.py --mode ai-authoring`)

- Contract cards must embed valid `prismMeta` per `schemas/tooling/prismMeta.v1.json`.
- `prismMeta.linkage` fields must match top-level `targetRepo`, `path`, `schemaRef`, `tier`.
- Invariant bindings must reference canonical invariant names and respect ranges.
- If `tier` is `vault` or `lab`, `deadledgerref` must be present and non-empty.

## Integration Guide for Downstream Repos

### Step 1: Declare Dependencies

Create `.horrorplace-contracts.json` at the repo root:

```json
{
  "repoName": "Your-Repo-Name",
  "tier": "public",
  "pins": {
    "schemas": [
      "https://horrorplace.constellation/schemas/core/invariants-spine.v1.json"
    ],
    "invariants": ["CIC", "AOS"],
    "metrics": ["UEC"],
    "contracts": ["regionContractCard"]
  },
  "produces": {
    "registries": ["regions"],
    "telemetry": ["session-metrics-envelope"]
  }
}
```

### Step 2: Install Pre-Commit Hooks

```bash
# In your repo root
pip install pre-commit jsonschema
pre-commit install
```

Ensure `.pre-commit-config.yaml` references the constellation contracts repo.

### Step 3: Add CI Workflow

Copy `examples/minimal-constellation/.github/workflows/minimal-constellation-ci.yml` to your `.github/workflows/` and adjust paths as needed.

### Step 4: Test Locally

```bash
# Validate a schema file
python tooling/python/cli/hpc-validate-schema.py --file schemas/core/regionContractCard.v1.json

# Lint a registry example
python tooling/python/cli/hpc-lint-registry.py --file registry/formats/regions.example.ndjson

# Generate spine index locally (for impact analysis)
python tooling/python/cli/hpc-generate-spine-index.py --root schemas/ --output schema-spine-index.json
```

## Drift Detection and Remediation

The pre-commit pack includes optional drift-checking features:

- `hpc-generate-spine-index.py --diff`: Compare local spine index against canonical to detect schema version mismatches.
- `hpc-lint-registry.py --check-references`: Verify that `schemaref` and `deadledgerref` values resolve to existing artifacts.
- CI workflow `spine-index-generate.yml`: Runs weekly to flag repos with outdated schema pins.

When drift is detected:
1. An issue is auto-created in the drifted repo using `.github/ISSUE_TEMPLATE/schema-drift.md`.
2. The issue includes migration hints from `docs/overview/versioning-and-stability.md`.
3. Maintainers can run `hpc-validate-schema.py --migrate` to auto-update contract cards to the new schema version.

## Privacy and Access Control

- Public repos can use all pre-commit pack features without authentication.
- Vault/lab repos must provide a `deadledgerref` validation token (via GitHub secret) to run certain checks.
- The pre-commit pack never uploads raw content; only metadata and validation results are shared.

## Example: Minimal Constellation Adoption

See `examples/minimal-constellation/` for a fully wired example repo that:

- Declares dependencies in `.horrorplace-contracts.json`.
- Uses `.pre-commit-config.yaml` to import constellation hooks.
- Runs `minimal-constellation-ci.yml` on every PR.
- Contains minimal valid schemas, registries, and contract cards.

Clone and explore:

```bash
git clone https://github.com/Doctor0Evil/HorrorPlace-Constellation-Contracts
cd HorrorPlace-Constellation-Contracts/examples/minimal-constellation
pre-commit run --all-files  # Should pass with no errors
```

## Related Documents

- `tooling/python/cli/hpc-validate-schema.py`: Reference schema validator implementation.
- `tooling/python/cli/hpc-lint-registry.py`: Reference registry linter implementation.
- `tooling/python/cli/hpc-generate-spine-index.py`: Reference spine index generator.
- `.github/workflows/constellation-precommit-pack.yml`: Reusable CI workflow definition.
- `examples/minimal-constellation/`: Worked example of adoption.
- `docs/overview/versioning-and-stability.md`: Deprecation and migration policies for schema evolution.
