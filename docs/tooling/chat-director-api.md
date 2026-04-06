# CHAT_DIRECTOR API

This document describes the external-facing API for CHAT_DIRECTOR as seen by AI agents, CLIs, and CI workflows. It focuses on JSON envelopes, CLI flags, and expected responses rather than internal Rust details.

---

## Envelope APIs

### ai-authoring-request

**Shape (simplified):**

```json
{
  "intent": "string",
  "objectKind": "regionContractCard | seedContractCard | moodContract | ...",
  "targetRepo": "string",
  "tier": "standard | mature | vault | ...",
  "schemaRef": "string",
  "referencedIds": ["ID-STRING"],
  "intendedInvariants": {
    "CIC": 0.5,
    "AOS": 0.6
  },
  "intendedMetrics": {
    "UEC": [0.6, 0.9],
    "ARR": [0.7, 0.9]
  },
  "manifestTier": "string"
}
```

**Contract:**

- Must conform to `ai-authoring-request-v1.json`.
- `objectKind` must match a known schema in the spine.
- `intendedInvariants` and `intendedMetrics` should fall within canonical bands for the specified tier.

### ai-authoring-response

**Shape (simplified):**

```json
{
  "tier": "standard",
  "manifestTier": "standard",
  "objectKind": "regionContractCard",
  "prismMeta": {
    "prismId": "string",
    "sessionId": "string",
    "agentId": "string",
    "agentProfileId": "string",
    "targetRepo": "string",
    "path": "string",
    "tier": "standard"
  },
  "artifact": {
    "id": "REGION-XXX-0001",
    "objectKind": "regionContractCard",
    "schemaRef": "https://.../region-contract-card-v1.json",
    "invariantBindings": {
      "CIC": 0.6,
      "AOS": 0.7
    },
    "metricTargets": {
      "UEC": [0.7, 0.9],
      "ARR": [0.7, 0.9]
    },
    "...": "..."
  }
}
```

**Contract:**

- Must conform to `ai-authoring-response-v1.json`.
- Exactly one primary artifact.
- Invariants and metrics must be present and within spine bands for the objectKind/tier.
- Envelope provenance fields must be complete.

---

## CLI endpoints

### `hpc-chat-director plan`

**Purpose:** Normalize an authoring request and compute routing/validation context.

**Example:**

```bash
hpc-chat-director plan \
  --from-file path/to/ai-authoring-request.region.json \
  --format json
```

**Inputs:**

- `--from-file` – path to JSON `ai-authoring-request`.
- `--format json` – emit machine-readable JSON response.

**Outputs:**

- JSON object with:
  - `status`: "ok" or "error".
  - `plan`: resolved `targetRepo`, `targetPath`, `schemaRef`, and tier.
  - `bandHints`: optional suggestions if intended bands are borderline.
  - `diagnostics[]`: detailed error objects on failure.

### `hpc-chat-director validate-response`

**Purpose:** Perform full validation of an `ai-authoring-response`.

**Example:**

```bash
hpc-chat-director validate-response \
  --from-file path/to/ai-authoring-response.region.json \
  --format json
```

**Outputs:**

- JSON object with:
  - `status`: "ok" or "error".
  - `diagnostics[]`: list of errors/warnings with:
    - `code`
    - `message`
    - `jsonPointer`
    - `severity`
    - `hint` (optional)

Exit code is non-zero when `status` is "error".

### `hpc-chat-director apply`

**Purpose:** Apply a validated plan or response to registries.

**Example:**

```bash
hpc-chat-director apply \
  --from-file path/to/region-contract-validated.json \
  --format json
```

**Behavior:**

- Checks manifests and routing rules.
- Writes new or updated entries into NDJSON registries.
- Emits JSON describing what changed (e.g., which registry, which ID).

---

## Python CLI APIs

Key Python CLIs used by CI and AI agents:

- `hpc-generate-spine-index.py`:
  - `--check` – compare regenerated index with committed one.
  - `--json` – emit status as JSON.

- `hpc-validate-schema.py`:
  - `--json` – emit diagnostics as JSONL.
  - Accepts optional file paths to limit scope.

- `hpc-lint-registry.py`:
  - `--json` – JSONL diagnostics.
  - `--summary` – human-readable summary.

- `aiauthoringvalidator.py`:
  - `request <file>` – validate request envelopes.
  - `response <file>` – validate response envelopes.

These commands are designed to be called from automation or AI agents without requiring deep Python knowledge.

---

## Lua APIs

Lua helpers provide read-only access to the same artifacts:

- `hpccontractcards.lua`:
  - `load_spine(path?)`
  - `load_contract(path)`
  - `check_contract(spine, contract, object_kind?, tier?) -> summary`
  - `print_summary(summary)`

- `hpcregistryclient.lua`:
  - `load_registry(path) -> registry`
  - `resolve_id(registry, id)`
  - `filter_by_tier(registry, tier)`
  - `filter_by_tag(registry, tag)`
  - `validate_references(source_registry, target_registry, opts?) -> diagnostics`

These functions support in-editor inspectors and game runtime checks for contract integrity.
