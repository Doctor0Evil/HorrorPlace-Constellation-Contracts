# AI Chat Flow: Generate a Mood Contract

This walkthrough explains how an AI chat agent can generate a `moodContract` that defines atmosphere and pacing over a region or tile class, using the same schema-first approach as regions and seeds.

---

## 1. Inspect mood capabilities

The AI agent should:

- Confirm `moodContract` is a valid objectKind.
- Understand tier and phase constraints for mood contracts.
- Review canonical bands for mood-relevant invariants and metrics (e.g., UEC, EMD, ARR, CDL).

Discovery commands:

```bash
python tooling/python/cli/hpc-generate-spine-index.py --check
python tooling/python/cli/hpc-validate-schema.py --json
python tooling/python/cli/hpc-lint-registry.py --json --summary
```

The AI reads:

- `schemas/core/mood-contract-v1.json`.
- `schema-spine-index-v1.json` entries that mention `moodContract`.

---

## 2. Draft an ai-authoring-request for a mood

Example request:

```json
{
  "intent": "create a liminal mood for the subway junction, emphasizing uncertainty with moderate dread",
  "objectKind": "moodContract",
  "targetRepo": "HorrorPlace-Constellation-Contracts",
  "tier": "standard",
  "schemaRef": "https://horror.place/schemas/core/mood-contract-v1.json",
  "referencedIds": [
    "REGION-SUBWAY-0001"
  ],
  "intendedInvariants": {
    "CIC": 0.6,
    "AOS": 0.75,
    "LSG": 0.8
  },
  "intendedMetrics": {
    "UEC": [0.7, 0.9],
    "ARR": [0.7, 0.9],
    "CDL": [0.3, 0.6],
    "EMD": [0.7, 0.9]
  },
  "manifestTier": "standard"
}
```

These bands should fall within standard-tier mood bounds from the spine.

---

## 3. Normalize with CHAT_DIRECTOR

```bash
hpc-chat-director plan \
  --from-file path/to/ai-authoring-request.mood.json \
  --format json \
  > .ai/mood-contract-plan.json
```

The plan confirms:

- Legal placement for the mood contract.
- Which region/registry references must resolve.
- Any coupling between this mood and existing seeds/events.

---

## 4. Generate the mood contract artifact

Based on the plan and schema, the AI constructs a `moodContract` artifact.

Example:

```json
{
  "id": "MOOD-SUBWAY-0001",
  "objectKind": "moodContract",
  "schemaRef": "https://horror.place/schemas/core/mood-contract-v1.json",
  "moodName": "Subway Liminal Drift",
  "appliesToRegionId": "REGION-SUBWAY-0001",
  "tileClassBands": {
    "junction": {
      "invariantBindings": {
        "CIC": 0.61,
        "AOS": 0.77,
        "LSG": 0.82
      },
      "metricTargets": {
        "UEC": [0.72, 0.9],
        "ARR": [0.7, 0.88],
        "CDL": [0.32, 0.55],
        "EMD": [0.73, 0.9]
      }
    }
  }
}
```

This artifact is wrapped in an `ai-authoring-response` envelope.

---

## 5. Pre-flight envelope validation

```bash
python tooling/python/schemaspine/aiauthoringvalidator.py response \
  path/to/ai-authoring-response.mood.json \
  > .ai/mood-contract-validation.jsonl
```

The script verifies:

- One primary artifact.
- Envelope provenance (agent, session, repo path).
- Mood invariants and metrics within spine bands for the given tier.

The AI uses diagnostic output to correct any out-of-band values or missing fields.

---

## 6. Full validation and application

```bash
hpc-chat-director validate-response \
  --from-file path/to/ai-authoring-response.mood.json \
  --format json \
  > .ai/mood-contract-validated.json
```

If successful, apply:

```bash
hpc-chat-director apply \
  --from-file .ai/mood-contract-validated.json \
  --format json
```

This adds the mood contract to the appropriate registry, ready for integration with runtime systems or Lua-based tools (e.g., mood inspectors, band checkers).
