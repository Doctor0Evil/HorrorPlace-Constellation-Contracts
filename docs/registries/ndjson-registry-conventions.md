# NDJSON Registry Conventions

This document defines the standardized format, naming rules, and validation expectations for all NDJSON registry files in the HorrorPlace constellation. Registries are the machine-navigable index that replaces ad-hoc file discovery.

## Format Rules

All registry files must:

1. Use **newline-delimited JSON (NDJSON)**: one valid JSON object per line, no commas between objects.
2. Encode in **UTF-8** with **LF line endings** (no CRLF).
3. Contain **no comments**, **no trailing commas**, and **no pretty-printing** (compact JSON per line).
4. End with a **final newline** (POSIX-compliant).
5. Use **opaque references only** for assets: `artifactid`, `cid`, or `deadledgerref`—never raw URLs, paths, or content.

Example valid line:
```json
{"id":"EVT-ARAL-0001","schemaref":"https://horrorplace.constellation/schemas/core/event-contract.v1.json","deadledgerref":"zkp:sha256:abc123...","artifactid":"ipfs:bafy...","createdAt":"2026-01-15T03:00:00Z","status":"active"}
```

## ID Naming Conventions

Every registry entry must have a stable, globally unique `id` following this pattern:

```
<TYPE>-<REGION-CODE>-<SEQUENCE>
```

| Component | Format | Example |
|-----------|--------|---------|
| `TYPE` | 3–4 uppercase letters: `REG`, `EVT`, `STY`, `PER` | `EVT` |
| `REGION-CODE` | 3–6 uppercase letters/digits, derived from geography or concept | `ARAL`, `DRYMDW`, `LAB7` |
| `SEQUENCE` | 4-digit zero-padded integer | `0001` |

Full example: `REG-ARAL-0042`, `EVT-LAB7-0001`.

IDs are **immutable** once published to a registry. Deprecation requires a new entry with a new ID and a `deprecatedReplacedBy` field.

## Required Fields (All Registries)

Every registry entry, regardless of type, must include:

| Field | Type | Description |
|-------|------|-------------|
| `id` | string | Globally unique ID per naming convention above. |
| `schemaref` | string (URI) | Canonical `$id` of the schema this entry conforms to. |
| `deadledgerref` | string | Opaque cryptographic or governance proof identifier. |
| `artifactid` | string | Content-addressable or implementation-agnostic reference to the actual asset/implementation. |
| `createdAt` | string (ISO 8601) | UTC timestamp of entry creation. |
| `status` | enum | One of: `"active"`, `"deprecated"`, `"archived"`, `"draft"`. |

## Optional but Recommended Fields

| Field | Type | Description |
|-------|------|-------------|
| `tags` | array[string] | Lowercase, hyphenated keywords for filtering (e.g., `["biohazard","soviet-era"]`). |
| `tier` | enum | Access tier: `"public"`, `"vault"`, `"lab"`. Defaults to `"public"` if omitted. |
| `invariantBindings` | object | Map of invariant name → value or range (e.g., `{"CIC":0.8,"AOS":7.2}`). |
| `metricTargets` | object | Map of metric name → expected range or threshold (e.g., `{"UEC":[40,80]}`). |
| `prismMetaRef` | string | Reference to a `prismMeta` document describing linkage and dependency graph. |

## Validation and Linting

All registry files are validated by `hpc-lint-registry.py` against their declared `schemaref`. The linter enforces:

- Presence and format of all required fields.
- `schemaref` points to an existing canonical schema in `schemas/`.
- `deadledgerref` matches the pattern expected for the entry's `tier`.
- `id` uniqueness within the registry file (no duplicates).
- `status` transitions follow allowed paths (e.g., `draft` → `active`, not `active` → `draft`).

CI job `registry-lint.yml` runs this linter on every PR that modifies `registry/**/*.ndjson*` or `registry/**/*.example`.

## Registry File Naming

Registry files must be named:

```
<plural-type>.ndjson      # Production registries (gitignored by default)
<plural-type>.example.ndjson  # Public examples in this repo
```

Examples:
- `regions.ndjson` (production, gitignored)
- `regions.example.ndjson` (public example)
- `events.example.ndjson`

## Cross-Registry Linking

Registries reference each other via ID, not path. Example: a `regionContractCard` may list `eventIds: ["EVT-ARAL-0001","EVT-ARAL-0002"]`. Tools resolve these IDs by querying the appropriate registry (`events.ndjson`), not by guessing file locations.

To enable this, every constellation repo that maintains a registry must:

1. Publish its registry schema under `schemas/registry/`.
2. Declare its registry production in `.horrorplace-contracts.json`.
3. Ensure its registry entries are included in the spine index via `hpc-generate-spine-index.py`.

## Privacy and Tier Gating

- `public` tier registries may be fully visible and queryable.
- `vault`/`lab` tier registries must have `deadledgerref` validation before any tool exposes entry details.
- The spine index includes tier metadata so AI agents and CI can enforce access rules without embedding secrets.

## Example: Minimal Registry Entry

```json
{"id":"REG-ARAL-0001","schemaref":"https://horrorplace.constellation/schemas/registry/registry-regions.v1.json","deadledgerref":"zkp:sha256:7f8a9b...","artifactid":"ipfs:bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi","createdAt":"2026-01-15T03:00:00Z","status":"active","tags":["aral-sea","post-soviet","biohazard"],"tier":"vault","invariantBindings":{"CIC":0.92,"AOS":8.1}}
```

## Related Documents

- `schemas/registry/registry-entry-base.v1.json`: Base schema for all registry entries.
- `registry/formats/*.example`: Concrete NDJSON examples for each registry type.
- `docs/schema-spine/cross-repo-consumer-mapping.md`: How registries are tracked in the spine index.
- `tooling/python/schema_spine/registry_linter.py`: Reference implementation of the registry linter.
