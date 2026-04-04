# Integration with Horror.Place Core

This document describes how `HorrorPlace-Constellation-Contracts` aligns with and extends the `Horror.Place` sovereign core repository. The two repos work in tandem: `Horror.Place` defines horror-specific domain logic, while this repo provides cross-platform, AI-engineering-friendly contracts.

## Relationship Overview

```
Horror.Place (Domain Core)          HorrorPlace-Constellation-Contracts (Contract Layer)
─────────────────────────          ─────────────────────────────────────
• Horror-specific invariants       • Generic invariant/metric schema spine
• Style contracts for implication  • Reusable contract card templates
• Doctrinal docs (no raw horror)   • AI-authoring contracts (prismMeta, agentProfile)
• Public, GitHub-compliant         • Public, platform-agnostic, contract-only
```

### Key Alignment Points

1. **Schema Inheritance**: Contracts in this repo reference canonical schemas from `Horror.Place` via `$ref`:
   ```json
   {
     "$schema": "https://json-schema.org/draft/2020-12/schema",
     "$id": "https://horrorplace.constellation/schemas/core/regionContractCard.v1.json",
     "properties": {
       "invariantBindings": {
         "$ref": "https://raw.githubusercontent.com/Doctor0Evil/Horror.Place/main/schemas/invariants-spine.v1.json#/properties/CIC"
       }
     }
   }
   ```

2. **Shared Invariant Definitions**: Both repos use the same canonical invariant names (`CIC`, `AOS`, `DET`, `SHCI`) and metric names (`UEC`, `EMD`, `STCI`, `CDL`, `ARR`). This repo's `invariants-spine.v1.json` mirrors `Horror.Place`'s definitions to ensure cross-repo validation.

3. **Tier Gating Consistency**: The `tier` field (`public`, `vault`, `lab`) has identical semantics in both repos. Public-tier contracts must comply with GitHub content policies; vault/lab tiers require `deadledgerref` attestation.

4. **Orchestrator Coordination**: `Horror.Place-Orchestrator` polls vault artifacts, verifies hashes, and updates public registries. This repo's `registry-lint.yml` ensures registry entries are valid before the orchestrator processes them.

## Integration Workflow

### Step 1: Pin Schema Versions

Downstream repos (including `Horror.Place` itself) should pin to specific schema versions from this repo:

```json
// .horrorplace-contracts.json in Horror.Place repo
{
  "repoName": "Horror.Place",
  "tier": "public",
  "pins": {
    "schemas": [
      "https://horrorplace.constellation/schemas/core/invariants-spine.v1.json",
      "https://horrorplace.constellation/schemas/tooling/prismMeta.v1.json"
    ],
    "invariants": ["CIC", "AOS", "DET"],
    "metrics": ["UEC", "ARR"]
  }
}
```

### Step 2: Import CI Workflows

`Horror.Place` can reuse validation workflows from this repo:

```yaml
# In Horror.Place/.github/workflows/ci.yml
validate-contracts:
  uses: Doctor0Evil/HorrorPlace-Constellation-Contracts/.github/workflows/constellation-precommit-pack.yml@v1.0.0
  with:
    validate_schemas: true
    lint_registries: true
    python_version: "3.11"
```

### Step 3: Emit Contract Cards

When `Horror.Place` generates new content (e.g., a new region definition), it should emit a `regionContractCard` conforming to this repo's schema:

```json
{
  "prismMeta": {
    "version": "1.0.0",
    "generatedBy": {
      "agentId": "horror-place-core-v3.2",
      "agentProfileRef": "https://horrorplace.constellation/schemas/tooling/agentProfile.v1.json",
      "timestamp": "2026-01-15T03:00:00Z"
    },
    "linkage": {
      "targetRepo": "HorrorPlace-Black-Archivum",
      "targetPath": "registry/regions/REG-ARAL-0001.ndjson",
      "schemaRef": "https://horrorplace.constellation/schemas/registry/registry-regions.v1.json",
      "tier": "vault"
    },
    "dependencies": {
      "invariants": ["CIC", "AOS"],
      "metrics": ["UEC"],
      "contracts": ["regionContractCard"],
      "registryIds": []
    },
    "validationPropagation": {
      "onSchemaChange": ["registry-lint", "spine-index-update"],
      "onInvariantDrift": ["telemetry-review"],
      "notifyRepos": ["Horror.Place-Orchestrator"]
    },
    "prismHash": "sha256:..."
  },
  "id": "REG-ARAL-0001",
  "schemaVersion": "1.0.0",
  "targetRepo": "HorrorPlace-Black-Archivum",
  "path": "registry/regions/REG-ARAL-0001.ndjson",
  "tier": "vault",
  "invariantBindings": { "CIC": 0.92, "AOS": 8.1 },
  "deadledgerref": "zkp:sha256:..."
}
```

### Step 4: Validate and Propagate

1. `Horror.Place` CI runs `hpc-validate-schema.py` on the generated contract card.
2. If valid, the card is committed to the target repo (`HorrorPlace-Black-Archivum`).
3. `Horror.Place-Orchestrator` detects the new registry entry, verifies `deadledgerref`, and updates public-facing indices.
4. The spine index is regenerated, flagging dependent repos for review.

## Version Synchronization

Because `Horror.Place` and this repo evolve independently, version synchronization is critical:

| Scenario | Resolution |
|----------|-----------|
| `Horror.Place` updates an invariant definition | This repo mirrors the change in `invariants-spine.v1.json` with a new minor version; consumers pin to specific versions. |
| This repo adds a new contract type | `Horror.Place` can adopt it optionally; no breaking changes to existing logic. |
| Breaking schema change in either repo | Follow deprecation policy in `docs/overview/versioning-and-stability.md`; maintain parallel support for 90 days. |

Automated tooling (`hpc-generate-spine-index.py --diff`) detects version mismatches and opens issues with migration guidance.

## Content Policy Alignment

Both repos enforce the same content rules:

- ❌ No raw horror scenes, gore, or graphic content.
- ❌ No embedded assets (images, audio, binaries); use opaque references only.
- ✅ All horror is expressed via invariants, metrics, style descriptors, and implication.
- ✅ Telemetry and identity data are privacy-aware and contract-only.

This alignment ensures that `Horror.Place` remains GitHub-compliant while this repo stays platform-agnostic.

## Testing and Validation

To verify integration locally:

```bash
# Clone both repos side-by-side
git clone https://github.com/Doctor0Evil/Horror.Place
git clone https://github.com/Doctor0Evil/HorrorPlace-Constellation-Contracts

# Validate Horror.Place schemas against constellation contracts
python HorrorPlace-Constellation-Contracts/tooling/python/cli/hpc-validate-schema.py \
  --root Horror.Place/schemas/ \
  --mode batch \
  --strict

# Lint Horror.Place registry examples
python HorrorPlace-Constellation-Contracts/tooling/python/cli/hpc-lint-registry.py \
  --file Horror.Place/registry/formats/regions.example.ndjson \
  --strict
```

Both commands should exit with code 0 if integration is correct.

## Related Documents

- `Horror.Place/README.md`: Core repository overview and doctrine.
- `schemas/core/invariants-spine.v1.json`: Canonical invariant definitions shared by both repos.
- `docs/overview/versioning-and-stability.md`: Deprecation and migration policies.
- `tooling/python/schema_spine/spine_index_builder.py`: Reference implementation for spine index generation.
- `.github/ISSUE_TEMPLATE/schema-drift.md`: Template for reporting version mismatches.
