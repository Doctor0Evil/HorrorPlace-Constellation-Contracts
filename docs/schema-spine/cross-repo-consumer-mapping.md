# Cross-Repo Consumer Mapping

This document describes how the schema spine index tracks which repositories, tools, and registries consume each schema, invariant, metric, or contract type—and how this mapping enables bidirectional validation propagation across the VM-constellation.

## Purpose

In a multi-repo constellation, changes to a canonical schema can silently break downstream consumers. The consumer mapping solves this by:

1. **Discoverability**: Answering "who uses this schema?" without manual code search.
2. **Impact Analysis**: Flagging affected repos before a schema change is merged.
3. **Validation Propagation**: Triggering CI checks in dependent repos when a schema evolves.
4. **Drift Prevention**: Detecting when a consumer repo diverges from the canonical schema version.

## Consumer Registration

Each repository in the constellation declares its dependencies in a `.horrorplace-contracts.json` file at its root:

```json
{
  "repoName": "HorrorPlace-Black-Archivum",
  "tier": "vault",
  "pins": {
    "schemas": [
      "https://horrorplace.constellation/schemas/core/invariants-spine.v1.json",
      "https://horrorplace.constellation/schemas/registry/registry-regions.v1.json"
    ],
    "invariants": ["CIC", "AOS", "DET"],
    "metrics": ["UEC", "ARR"],
    "contracts": ["regionContractCard", "seedContractCard"]
  },
  "produces": {
    "registries": ["regions", "events"],
    "telemetry": ["session-metrics-envelope"]
  }
}
```

This file is:
- **Mandatory** for all constellation repos (public, vault, lab).
- **Validated** by `registry-lint.yml` on every PR.
- **Parsed** by `hpc-generate-spine-index.py` to populate the `consumers` object in the spine index.

## Mapping Structure in Spine Index

The spine index's `consumers` object organizes dependencies by consumer repo:

```json
{
  "consumers": {
    "HorrorPlace-Black-Archivum": {
      "type": "vault-repo",
      "tier": "vault",
      "usesSchemas": [
        "https://horrorplace.constellation/schemas/core/invariants-spine.v1.json"
      ],
      "usesInvariants": ["CIC", "AOS"],
      "usesMetrics": ["UEC"],
      "usesContracts": ["regionContractCard"],
      "producesRegistries": ["regions"],
      "lastValidated": "2026-01-15T03:00:00Z",
      "schemaVersions": {
        "invariants-spine.v1.json": "1.0.0",
        "registry-regions.v1.json": "1.0.0"
      }
    }
  }
}
```

### Key Fields

| Field | Description |
|-------|-------------|
| `type` | Repo role: `public-core`, `vault-repo`, `lab-repo`, `tooling`, `example`. |
| `tier` | Access tier: `public`, `vault`, `lab`. Gates validation rigor and content policies. |
| `usesSchemas` | List of canonical schema `$id` values this repo depends on. |
| `usesInvariants` / `usesMetrics` | Flat lists of invariant/metric names consumed. Enables quick impact queries. |
| `producesRegistries` | Which NDJSON registries this repo maintains (for cross-repo linking). |
| `lastValidated` | UTC timestamp of last successful CI validation against pinned schemas. |
| `schemaVersions` | Map of schema filename → pinned version. Enables drift detection. |

## Bidirectional Validation Propagation

When a schema changes, the spine index enables a two-phase validation workflow:

### Phase 1: Pre-Merge Impact Analysis
1. Developer proposes a change to `invariants-spine.v1.json` in a PR.
2. CI runs `hpc-generate-spine-index.py --diff` to compute affected consumers.
3. The PR description is auto-updated with a table:

```markdown
## Affected Consumers (Auto-Generated)

| Repo | Tier | Schemas Used | Action Required |
|------|------|--------------|----------------|
| HorrorPlace-Black-Archivum | vault | invariants-spine.v1.json | Review CIC/AOS usage |
| HorrorPlace-Atrocity-Seeds | vault | invariants-spine.v1.json | Update PCG seed logic |
```

4. Maintainers of affected repos are auto-requested as reviewers.

### Phase 2: Post-Merge Propagation
1. After the schema PR merges, a `repository_dispatch` event is sent to each affected repo.
2. Each repo's CI runs `hpc-validate-schema.py` against its pinned schema version.
3. If validation fails, the repo opens an auto-PR with migration hints from `docs/schema-spine/`.

This ensures changes propagate safely and explicitly, never silently.

## Drift Detection and Remediation

The spine index tracks `schemaVersions` per consumer. Drift occurs when:

- A consumer repo pins an outdated schema version.
- A consumer uses an invariant/metric not declared in its `.horrorplace-contracts.json`.

Detection workflow:
1. Weekly CI job (`spine-index-generate.yml`) regenerates the index.
2. Compares `consumers[*].schemaVersions` against current canonical versions.
3. Opens issues in drifted repos with template:

```markdown
## Schema Drift Detected

Your repo `HorrorPlace-Black-Archivum` pins `invariants-spine.v1.json@1.0.0`, but the canonical version is `1.1.0`.

### Changes in v1.1.0
- Added optional field `historicalConfidence` to CIC definition.
- Expanded DET range documentation.

### Migration Steps
1. Update your `.horrorplace-contracts.json` to pin v1.1.0.
2. Run `hpc-validate-schema.py --migrate` to update contract cards.
3. Verify registry entries still validate.

See `docs/schema-spine/invariants-and-metrics-spine.md` for details.
```

## Tooling Support

### `hpc-generate-spine-index.py` Flags
- `--diff`: Compare current index against HEAD to compute affected consumers.
- `--propagate`: After index update, send `repository_dispatch` events to affected repos.
- `--drift-check`: Scan consumer repos for version mismatches and emit issues.

### CI Integration Example
Downstream repos include this in their `.github/workflows/ci.yml`:

```yaml
- name: Validate against constellation contracts
  uses: Doctor0Evil/HorrorPlace-Constellation-Contracts/.github/workflows/constellation-precommit-pack.yml@main
  with:
    validate_schemas: true
    lint_registries: true
    contract_file_pattern: "**/*ContractCard*.json"
```

## Privacy and Access Control

- Public repos (`Horror.Place`) appear in the public spine index.
- Vault/lab repos (`HorrorPlace-Black-Archivum`) are included only in vault-tier spine index builds, gated by `deadledgerref` validation.
- The spine index itself never contains raw content, only schema references and consumer metadata.

## Example: Querying the Spine Index

```python
# Pseudocode: Find all repos using the CIC invariant
index = load_spine_index("schema-spine-index.json")
cic_consumers = [
    repo for repo, data in index.consumers.items()
    if "CIC" in data.get("usesInvariants", [])
]
print(f"CIC is used by: {cic_consumers}")
# → ["HorrorPlace-Black-Archivum", "HorrorPlace-Atrocity-Seeds", ...]
```

## Related Documents

- `registry/spine/schema-spine-index.example.json`: Minimal spine index instance.
- `docs/overview/versioning-and-stability.md`: Deprecation and migration policies.
- `tooling/python/schema_spine/spine_index_builder.py`: Reference index generator.
- `.github/ISSUE_TEMPLATE/schema-drift.md`: Template for reporting drift.
