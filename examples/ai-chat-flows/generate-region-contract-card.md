# AI Chat Flow: Generate a Region Contract Card

This walkthrough shows how an AI chat (or human operator) can generate and validate a `regionContractCard` using the CHAT_DIRECTOR stack and minimal Python tooling.

The flow is:

1. Discover context and constraints.
2. Plan an `ai-authoring-request`.
3. Call `hpc-chat-director plan` (or equivalent) to normalize the request.
4. Generate a region contract card payload.
5. Validate the envelope with `aiauthoringvalidator.py`.
6. Run `hpc-chat-director validate-response` and, if clean, apply.

---

## 1. Discover context and constraints

Before generating anything, the AI agent should learn:

- Which objectKinds are allowed.
- Which repos and tiers are valid for the current constellation.
- Basic invariant/metric bands for the chosen objectKind and tier.

Typical discovery commands (from repo root):

```bash
python tooling/python/cli/hpc-generate-spine-index.py --check
python tooling/python/cli/hpc-validate-schema.py --json
python tooling/python/cli/hpc-lint-registry.py --json --summary
```

The agent can also inspect:

- `schemas/core/region-contract-card-v1.json` for structural rules.
- `schemas/core/schema-spine-index-v1.json` for numeric ranges.

---

## 2. Draft an ai-authoring-request

The AI agent proposes an `ai-authoring-request` describing the intent and constraints for a new region contract card.

Example request (conceptual JSON):

```json
{
  "intent": "create a new region contract card for a liminal subway junction",
  "objectKind": "regionContractCard",
  "targetRepo": "HorrorPlace-Constellation-Contracts",
  "tier": "standard",
  "schemaRef": "https://horror.place/schemas/core/region-contract-card-v1.json",
  "referencedIds": [
    "MOOD-SUBWAY-0001"
  ],
  "intendedInvariants": {
    "CIC": 0.6,
    "AOS": 0.7,
    "LSG": 0.8,
    "HVF": 0.5
  },
  "intendedMetrics": {
    "UEC": [0.6, 0.9],
    "ARR": [0.6, 0.8],
    "CDL": [0.3, 0.5]
  },
  "manifestTier": "standard"
}
```

This JSON must conform to the `ai-authoring-request` schema and respect spine bands for the chosen tier.

---

## 3. Normalize the request with CHAT_DIRECTOR

The AI submits the request to `hpc-chat-director` to obtain a normalized, machine-checked plan.

Example command:

```bash
hpc-chat-director plan \
  --from-file path/to/ai-authoring-request.region.json \
  --format json \
  > .ai/region-contract-plan.json
```

The plan:

- Resolves the final `targetRepo`, `targetPath`, and `schemaref`.
- Confirms phase, tier, and manifest routing are valid.
- Optionally includes hints for invariants and metrics if the request is borderline.

The AI reads `.ai/region-contract-plan.json` and uses it as the authoritative plan for the next step.

---

## 4. Generate the region contract card

Using the plan and the region schema, the AI produces a concrete region contract card JSON that matches `schemaRef` and aligns with intended invariants/metrics.

Example skeleton of the artifact (omitting many fields for brevity):

```json
{
  "id": "REGION-SUBWAY-0001",
  "objectKind": "regionContractCard",
  "schemaRef": "https://horror.place/schemas/core/region-contract-card-v1.json",
  "regionName": "Subterranean Transfer Platform",
  "topology": "junction",
  "invariantBindings": {
    "CIC": 0.62,
    "AOS": 0.71,
    "LSG": 0.79,
    "HVF": 0.52
  },
  "metricTargets": {
    "UEC": [0.65, 0.88],
    "ARR": [0.64, 0.78],
    "CDL": [0.32, 0.46]
  },
  "registryReady": true,
  "linkedMoodId": "MOOD-SUBWAY-0001"
}
```

The AI wraps this payload in an `ai-authoring-response` envelope as expected by CHAT_DIRECTOR.

---

## 5. Pre-flight check with aiauthoringvalidator.py

Before invoking the Rust CLI, run the fast Python envelope validator to catch structural issues early.

```bash
python tooling/python/schemaspine/aiauthoringvalidator.py response \
  path/to/ai-authoring-response.region.json \
  > .ai/region-contract-validation.jsonl
```

This step checks:

- Envelope structure (one primary artifact).
- Required fields (intent, objectKind, targetRepo, tier, schemaRef, referencedIds).
- Invariants and metrics present and within spine bands.
- Basic tier/manifest coherence.

If the script exits non-zero, the AI should parse the JSONL diagnostics, fix the envelope or artifact, and retry.

---

## 6. Full validation with CHAT_DIRECTOR

Once the fast checks pass, run the full Rust validator:

```bash
hpc-chat-director validate-response \
  --from-file path/to/ai-authoring-response.region.json \
  --format json \
  > .ai/region-contract-validated.json
```

This performs:

- JSON Schema validation.
- Invariant and metric enforcement.
- Manifest routing and tier rules.
- Envelope checks and dead-ledger hooks (if configured).

If this succeeds, the AI or human can call `apply`:

```bash
hpc-chat-director apply \
  --from-file .ai/region-contract-validated.json \
  --format json
```

This writes the region contract into the appropriate `registry/*.ndjson` file as specified by manifests, completing the region contract creation flow.
