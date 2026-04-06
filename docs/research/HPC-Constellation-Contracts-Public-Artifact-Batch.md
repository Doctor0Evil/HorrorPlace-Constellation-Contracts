---
invariants_used:
  - CIC
  - MDI
  - AOS
  - RRM
  - FCF
  - SPR
  - RWF
  - DET
  - HVF
  - LSG
  - SHCI
metrics_used:
  - UEC
  - EMD
  - STCI
  - CDL
  - ARR
tiers:
  - standard
  - mature
  - research
deadledger_surface:
  - zkpproof_schema
  - verifiers_registry
  - bundle_attestation
  - agent_attestation
  - spectral_seed_attestation
  - bci_state_proof
---

# HorrorPlace-Constellation-Contracts Public Artifact Batch

This document provides a ready-to-commit batch of minimal but structured public artifacts for the `HorrorPlace-Constellation-Contracts` repository. Each section contains one file with a concrete path, brief intent, and a slightly richer stub that is still small and easy to extend.

All JSON examples assume Draft 2020-12 and use canonical `$id` URIs under `https://horror.place/schemas/...`. All Markdown is prose-only and safe for public publication.

---

## 1. QPU Datashard Envelope Schema

**Repo:** `HorrorPlace-Constellation-Contracts`  
**File:** `schemas/qpudatashard-envelope.v1.json`

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://horror.place/schemas/qpu/qpudatashard-envelope.v1.json",
  "title": "QPU Datashard Envelope v1",
  "description": "Minimal envelope for QPU datashards used across the VM-Constellation. One object per NDJSON line.",
  "type": "object",
  "additionalProperties": false,
  "required": [
    "schema",
    "shardId",
    "shardKind",
    "schemaRef",
    "repoTier",
    "repoName",
    "safetyTier",
    "intensityBand",
    "entitlementProfileId",
    "functionKind",
    "invariants",
    "metrics"
  ],
  "properties": {
    "schema": {
      "type": "string",
      "const": "https://horror.place/schemas/qpu/qpudatashard-envelope.v1.json"
    },
    "shardId": {
      "type": "string",
      "description": "Stable identifier for this shard, e.g., qpu.persona.archivist.v1."
    },
    "shardKind": {
      "type": "string",
      "description": "Logical kind of shard, such as persona-impl, spectral-seed, invariant-bundle, policy-profile."
    },
    "schemaRef": {
      "type": "string",
      "description": "Canonical URI of the primary contract schema implemented by this shard."
    },
    "repoTier": {
      "type": "string",
      "enum": ["T1-core", "T2-vault", "T3-lab"],
      "description": "VM-Constellation tier of the producing repository."
    },
    "repoName": {
      "type": "string",
      "description": "Canonical repository name, e.g., Horror.Place or HorrorPlace-Atrocity-Seeds."
    },
    "safetyTier": {
      "type": "string",
      "enum": ["standard", "mature", "research"],
      "description": "High-level safety tier for this shard."
    },
    "intensityBand": {
      "type": "integer",
      "minimum": 0,
      "maximum": 10,
      "description": "Normalized intensity band in the range 0–10."
    },
    "entitlementProfileId": {
      "type": "string",
      "description": "Identifier of the entitlement profile that governs this shard."
    },
    "deadledgerRef": {
      "type": "string",
      "description": "Opaque reference to a Dead-Ledger attestation or proof, if any."
    },
    "functionKind": {
      "type": "string",
      "description": "Functional role, e.g., static-contract, bci-swing, telemetry-slice, policy-card."
    },
    "invariants": {
      "type": "object",
      "description": "Numeric invariant values or bands mapped by invariant code.",
      "additionalProperties": true
    },
    "metrics": {
      "type": "object",
      "description": "Metric descriptors mapped by metric code.",
      "additionalProperties": true
    },
    "bciBinding": {
      "type": "object",
      "description": "Optional binding to a derived BCI summary type.",
      "additionalProperties": true
    },
    "zkp": {
      "type": "object",
      "description": "Optional hints about expected proof types for this shard.",
      "additionalProperties": true
    }
  }
}
```

---

## 2. QPU Datashard Vocabulary

**Repo:** `HorrorPlace-Constellation-Contracts`  
**File:** `docs/qpu-datashard-vocabulary.md`

```markdown
# QPU Datashard Vocabulary

This document defines the minimal vocabulary for QPU datashards emitted as NDJSON lines that validate against `schemas/qpudatashard-envelope.v1.json`. It is safe for Tier-1 publication and never describes raw trauma or BCI payloads.

## 1. Shard kinds

Shard kinds identify the high-level role of a shard.

- `persona-impl` — NDJSON view of persona or agent contracts.
- `spectral-seed` — NDJSON view of seed contracts from vault repos.
- `invariant-bundle` — projection of region or site bundles from Black-Archivum.
- `policy-profile` — safety and entitlement profiles.
- `telemetry-snapshot` — aggregated metric summaries.
- `bci-swing-function` — description of allowed metric adjustments driven by derived BCI summaries.
- `zkp-proof-envelope` — metadata projection of Dead-Ledger proof envelopes.

Each shard MUST set `shardKind` to one of these values.

## 2. Function kinds

Function kinds describe how a shard is expected to be used.

- `static-contract` — static contracts and cards.
- `bci-swing` — metric adjustments based on BCI summaries.
- `telemetry-slice` — metric snapshots for analysis.
- `policy-card` — policy and profile definitions.

Additional function kinds may be added over time, but all MUST be documented here.

## 3. Alignment with schemas

The `schemaRef` field links each shard to a canonical JSON Schema defined in Horror.Place, HorrorPlace-Constellation-Contracts, or HorrorPlace-Dead-Ledger-Network. This ensures that QPU datashards remain thin routing envelopes, not alternate schema definitions.
```

---

## 3. Schema Spine Index Schema

**Repo:** `HorrorPlace-Constellation-Contracts`  
**File:** `schemas/spine/schema-spine-index.schema.json`

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://horror.place/schemas/spine/schema-spine-index.v1.json",
  "title": "Schema Spine Index v1",
  "description": "Machine-readable index of canonical schemas and their consumers across the VM-Constellation.",
  "type": "object",
  "additionalProperties": false,
  "required": ["indexVersion", "generatedAt", "schemas", "consumers"],
  "properties": {
    "indexVersion": {
      "type": "string",
      "description": "Semantic version of the index format."
    },
    "generatedAt": {
      "type": "string",
      "format": "date-time",
      "description": "UTC timestamp when the index was generated."
    },
    "schemas": {
      "type": "object",
      "description": "Map of schema ID to metadata.",
      "additionalProperties": {
        "type": "object",
        "additionalProperties": false,
        "required": ["title", "version", "tier"],
        "properties": {
          "title": { "type": "string" },
          "version": { "type": "string" },
          "tier": { "type": "string" }
        }
      }
    },
    "consumers": {
      "type": "object",
      "description": "Map of consumer repo name to dependency information.",
      "additionalProperties": {
        "type": "object",
        "additionalProperties": false,
        "required": ["tier", "usesSchemas"],
        "properties": {
          "tier": { "type": "string" },
          "usesSchemas": {
            "type": "array",
            "items": { "type": "string" }
          }
        }
      }
    }
  }
}
```

---

## 4. Schema Spine Index Example

**Repo:** `HorrorPlace-Constellation-Contracts`  
**File:** `registry/spine/schema-spine-index.example.json`

```json
{
  "indexVersion": "1.0.0",
  "generatedAt": "2026-01-01T00:00:00Z",
  "schemas": {
    "https://horror.place/schemas/core/invariants-spine.v1.json": {
      "title": "Invariants Spine v1",
      "version": "1.0.0",
      "tier": "T1-core"
    },
    "https://horror.place/schemas/core/entertainment-metrics-spine.v1.json": {
      "title": "Entertainment Metrics Spine v1",
      "version": "1.0.0",
      "tier": "T1-core"
    }
  },
  "consumers": {
    "Horror.Place": {
      "tier": "T1-core",
      "usesSchemas": [
        "https://horror.place/schemas/core/invariants-spine.v1.json",
        "https://horror.place/schemas/core/entertainment-metrics-spine.v1.json"
      ]
    },
    "HorrorPlace-Atrocity-Seeds": {
      "tier": "T2-vault",
      "usesSchemas": [
        "https://horror.place/schemas/core/invariants-spine.v1.json"
      ]
    }
  }
}
```

---

## 5. Schema Spine Index Spec

**Repo:** `HorrorPlace-Constellation-Contracts`  
**File:** `docs/schema-spine/schema-spine-index-spec.md`

```markdown
# Schema Spine Index Specification

The schema spine index is a JSON document that lists canonical schema IDs and the repositories that consume them.

- Producers generate `schema-spine-index.json` by scanning schema files and consumer manifests.
- Consumers read `schema-spine-index.json` to perform impact analysis and drift detection.

The concrete shape of the index is defined in `schemas/spine/schema-spine-index.schema.json`. A minimal, valid example is provided in `registry/spine/schema-spine-index.example.json`.
```

---

## 6. Cross-Repo Consumer Mapping Doc

**Repo:** `HorrorPlace-Constellation-Contracts`  
**File:** `docs/schema-spine/cross-repo-consumer-mapping.md`

```markdown
# Cross-Repo Consumer Mapping

Each VM-Constellation repository declares its schema dependencies in a small manifest file. The schema spine index generator reads these manifests and produces a consolidated `schema-spine-index.json`.

This mapping allows toolchains to answer questions such as:

- Which repos use `invariants-spine.v1`?
- Which schemas are required before updating `policyEnvelope.v1`?

The exact manifest format is implementation-defined, but MUST include the repository name, tier, and a list of schema IDs.
```

---

## 7. AI Authoring Request Schema

**Repo:** `HorrorPlace-Constellation-Contracts`  
**File:** `schemas/tooling/ai-authoring-request.v1.json`

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://horror.place/schemas/tooling/ai-authoring-request.v1.json",
  "title": "AI Authoring Request v1",
  "description": "Structured request for a single AI-generated artifact.",
  "type": "object",
  "additionalProperties": false,
  "required": ["targetRepo", "targetPath", "schemaRef", "tier", "purpose"],
  "properties": {
    "targetRepo": {
      "type": "string",
      "description": "Canonical name of the repository to receive the artifact."
    },
    "targetPath": {
      "type": "string",
      "description": "Repository-relative path of the file to create or update."
    },
    "schemaRef": {
      "type": "string",
      "description": "Canonical schema URI that the payload must validate against."
    },
    "tier": {
      "type": "string",
      "enum": ["T1-core", "T2-vault", "T3-lab"],
      "description": "VM-Constellation tier of the target repository."
    },
    "purpose": {
      "type": "string",
      "description": "Short human-readable description of the change."
    },
    "prismMetaRef": {
      "type": "string",
      "description": "Optional reference to a prismMeta profile."
    }
  }
}
```

---

## 8. AI Authoring Response Schema

**Repo:** `HorrorPlace-Constellation-Contracts`  
**File:** `schemas/tooling/ai-authoring-response.v1.json`

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://horror.place/schemas/tooling/ai-authoring-response.v1.json",
  "title": "AI Authoring Response v1",
  "description": "Structured response containing a single AI-generated artifact.",
  "type": "object",
  "additionalProperties": false,
  "required": ["targetRepo", "targetPath", "schemaRef", "payload"],
  "properties": {
    "targetRepo": {
      "type": "string",
      "description": "Repository that should receive this payload."
    },
    "targetPath": {
      "type": "string",
      "description": "Repository-relative path where this payload should be written."
    },
    "schemaRef": {
      "type": "string",
      "description": "Canonical schema URI that this payload conforms to."
    },
    "payload": {
      "type": "object",
      "description": "Schema-conformant JSON object to write to disk."
    },
    "prismMeta": {
      "type": "object",
      "description": "Optional inline prism metadata describing the generator and constraints."
    }
  }
}
```

---

## 9. PrismMeta and AgentProfile Schemas

**Repo:** `HorrorPlace-Constellation-Contracts`  
**File:** `schemas/tooling/prismMeta.v1.json`

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://horror.place/schemas/tooling/prismMeta.v1.json",
  "title": "Prism Metadata v1",
  "description": "Metadata describing how an AI agent generates and validates artifacts.",
  "type": "object",
  "additionalProperties": false,
  "required": ["generatorId", "profileId", "constraints"],
  "properties": {
    "generatorId": {
      "type": "string",
      "description": "Stable identifier of the AI generator or toolchain."
    },
    "profileId": {
      "type": "string",
      "description": "Identifier of the prism profile in use."
    },
    "constraints": {
      "type": "object",
      "description": "Implementation-defined constraint parameters.",
      "additionalProperties": true
    }
  }
}
```

**Repo:** `HorrorPlace-Constellation-Contracts`  
**File:** `schemas/tooling/agentProfile.v1.json`

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://horror.place/schemas/tooling/agentProfile.v1.json",
  "title": "Agent Profile v1",
  "description": "Minimal description of an AI agent participating in authoring workflows.",
  "type": "object",
  "additionalProperties": false,
  "required": ["agentId", "description", "capabilities"],
  "properties": {
    "agentId": {
      "type": "string",
      "description": "Stable identifier for the agent."
    },
    "description": {
      "type": "string",
      "description": "Short description of the agent's role."
    },
    "capabilities": {
      "type": "array",
      "description": "List of capabilities or surfaces the agent can safely operate on.",
      "items": { "type": "string" }
    }
  }
}
```

---

## 10. AI Authoring One-File-Per-Request Doc

**Repo:** `HorrorPlace-Constellation-Contracts`  
**File:** `docs/tooling/ai-authoring-one-file-per-request.md`

```markdown
# AI Authoring One-File-Per-Request

AI authoring workflows for the VM-Constellation follow a one-file-per-request contract.

1. A client sends an `ai-authoring-request.v1` object describing the target repository, path, schema, tier, and purpose.
2. An AI agent returns exactly one `ai-authoring-response.v1` object whose `payload` validates against the requested `schemaRef`.
3. CI and pre-commit hooks validate the response and either accept the new file or reject it with clear errors.

External platforms that implement this pattern can integrate with HorrorPlace repositories without guessing file paths or schema shapes.
```

---

## 11. NDJSON Registry Conventions

**Repo:** `HorrorPlace-Constellation-Contracts`  
**File:** `docs/registries/ndjson-registry-conventions.md`

```markdown
# NDJSON Registry Conventions

All registries in the VM-Constellation are newline-delimited JSON (NDJSON):

- One valid JSON object per line.
- UTF-8 encoding with LF line endings.
- No comments or trailing commas.
- A final newline at the end of the file.

Each entry MUST include at least:

- `id` — stable, unique identifier.
- `schemaRef` — canonical schema URI.
- `createdAt` — ISO 8601 UTC timestamp.
- `status` — one of `active`, `deprecated`, `archived`, or `draft`.

Type-specific schemas add event, region, style, or persona fields as needed.
```

---

## 12. Registry Entry Base Schema

**Repo:** `HorrorPlace-Constellation-Contracts`  
**File:** `schemas/registry/registry-entry-base.v1.json`

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://horror.place/schemas/registry/registry-entry-base.v1.json",
  "title": "Registry Entry Base v1",
  "description": "Common fields for all NDJSON registry entries.",
  "type": "object",
  "additionalProperties": false,
  "required": ["id", "schemaRef", "createdAt", "status"],
  "properties": {
    "id": {
      "type": "string",
      "description": "Stable, globally unique identifier for the entry."
    },
    "schemaRef": {
      "type": "string",
      "description": "Canonical schema URI for this entry."
    },
    "createdAt": {
      "type": "string",
      "format": "date-time",
      "description": "ISO 8601 UTC timestamp of creation."
    },
    "status": {
      "type": "string",
      "enum": ["active", "deprecated", "archived", "draft"],
      "description": "Lifecycle status of the entry."
    }
  }
}
```

---

## 13. Registry Events Schema and Example

**Repo:** `HorrorPlace-Constellation-Contracts`  
**File:** `schemas/registry/registry-events.v1.json`

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://horror.place/schemas/registry/registry-events.v1.json",
  "title": "Events Registry Entry v1",
  "description": "Registry entry for events, extending the base registry shape.",
  "allOf": [
    {
      "$ref": "registry-entry-base.v1.json"
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "eventType": {
          "type": "string",
          "description": "High-level event type label, e.g., ambient, encounter."
        },
        "regionIds": {
          "type": "array",
          "description": "List of region IDs where this event can occur.",
          "items": { "type": "string" }
        }
      }
    }
  ]
}
```

**Repo:** `HorrorPlace-Constellation-Contracts`  
**File:** `registry/formats/events.ndjson.example`

```text
{"id":"EVT-EXAMPLE-0001","schemaRef":"https://horror.place/schemas/registry/registry-events.v1.json","createdAt":"2026-01-01T00:00:00Z","status":"active","eventType":"ambient","regionIds":["REG-EXAMPLE-0001"]}
```

---

This batch gives `HorrorPlace-Constellation-Contracts` a coherent starting surface for:

- QPU datashards.
- Schema spine index and consumer mapping.
- AI authoring request/response and prism metadata.
- NDJSON registry conventions and an events registry example.

All files are minimal but structured enough that external platforms and AI-chat agents can begin integrating against them without inventing their own shapes.
