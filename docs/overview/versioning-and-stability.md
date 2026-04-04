# Versioning and Stability

This document defines the versioning strategy, stability guarantees, and migration practices for schemas, registries, and tooling in `HorrorPlace-Constellation-Contracts`.

## Semantic Versioning for Schemas

All JSON Schemas under `schemas/` follow semantic versioning:

```
<major>.<minor>.<patch>
```

- **Major**: Breaking structural or semantic changes (e.g., removing a required field, changing a type).
- **Minor**: Additive, backward-compatible changes (e.g., adding an optional field, expanding an enum).
- **Patch**: Non-functional corrections (e.g., fixing a typo in `description`, clarifying examples).

Schema files embed their version in the `$id` and `title`:
```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://horrorplace.constellation/schemas/core/invariants-spine.v1.json",
  "title": "Invariants Spine Schema v1.0.0",
  ...
}
```

## Deprecation Policy

When a schema field or entire schema is deprecated:

1. Mark it with `"deprecated": true` and a `"deprecationNote"` in the schema.
2. Add a migration guide to `docs/schema-spine/` explaining the replacement.
3. Maintain the deprecated field for at least two minor versions before removal.
4. Update the spine index to flag deprecated schemas and their consumers.

Example deprecation notice in a schema:
```json
"legacyMetric": {
  "type": "number",
  "deprecated": true,
  "deprecationNote": "Use `adaptiveResonanceRatio` (ARR) instead. Removed in v2.0.0."
}
```

## Registry Format Stability

NDJSON registry formats (`registry/**/*.v1.json`) are versioned separately from core schemas:

- Registry schemas evolve more slowly; breaking changes require a new major version (e.g., `registry-regions.v2.json`).
- Existing registries remain valid under their declared schema version; migration is opt-in.
- The spine index tracks which registry version each consumer repo uses.

## Tooling Compatibility

Python CLI tools (`tooling/python/cli/`) and Lua helpers (`tooling/lua/`) declare compatibility ranges:

```python
# In hpc-validate-schema.py
__compatible_schema_versions__ = ["v1.0.0", "v1.1.0", "v1.2.0"]
```

- Tools fail with a clear error if presented with an incompatible schema version.
- Major tool updates are coordinated with schema major versions to avoid breakage.

## Spine Index Versioning

The schema spine index (`schema-spine-index.json`) is regenerated on schema changes and includes:

```json
{
  "indexVersion": "1.0.0",
  "generatedAt": "2026-01-15T03:00:00Z",
  "schemas": { ... },
  "invariants": { ... },
  "metrics": { ... },
  "consumers": { ... }
}
```

- `indexVersion` follows semantic versioning and increments when the index structure changes.
- Consumers can pin to a specific `indexVersion` for deterministic behavior.

## Migration Workflow for Breaking Changes

When a major schema version is released:

1. **Announce**: Open a `contract-change-proposal.md` issue with rationale and migration steps.
2. **Parallel Support**: Maintain both v1 and v2 schemas in `schemas/` during a transition window (default: 90 days).
3. **Update Tooling**: Ensure CLI tools can validate against both versions and emit migration hints.
4. **Consumer Migration**: Downstream repos update their pinned schema versions and adjust contract cards.
5. **Deprecate v1**: After the transition window, mark v1 schemas as deprecated and remove them in the next major release.

## Stability Guarantees

| Artifact Type | Stability Guarantee | Breaking Change Notice |
|--------------|---------------------|------------------------|
| Core Schemas (`schemas/core/`) | Stable for 12 months after v1.0.0 | 90 days via issue + spine index flag |
| Registry Schemas (`schemas/registry/`) | Stable for 18 months after v1.0.0 | 120 days via issue + registry lint warning |
| Tooling CLI (`tooling/python/cli/`) | Backward-compatible within major version | Release notes + deprecation warnings in output |
| Lua Helpers (`tooling/lua/`) | Best-effort stability; engine-agnostic | Documentation updates + example migrations |

## Pinning and Reproducibility

Downstream repos should pin schema and tooling versions in their CI:

```yaml
# In downstream repo's .github/workflows/ci.yml
- uses: Doctor0Evil/HorrorPlace-Constellation-Contracts/.github/workflows/schema-validate.yml@v1.2.0
  with:
    schema_version: "v1.0.0"
```

- Pinning ensures reproducible validation across environments.
- Automated Dependabot-style updates can be configured for minor/patch versions.

## Related Documents

- `docs/schema-spine/schema-spine-index-spec.md`: Spine index structure and consumer mapping.
- `.github/ISSUE_TEMPLATE/contract-change-proposal.md`: Template for proposing schema changes.
- `research/schema-spine-open-questions.md`: Non-binding discussions on future versioning strategies.
