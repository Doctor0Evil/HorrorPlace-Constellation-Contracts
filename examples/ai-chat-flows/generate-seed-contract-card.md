# AI Chat Flow: Generate a Seed Contract Card

This walkthrough shows how an AI chat agent can generate a `seedContractCard` that encodes invariant gradients and pacing metrics, using the same schema-first pipeline that powers region contracts.

---

## 1. Discover seed constraints

The AI agent should first confirm:

- `seedContractCard` is a supported objectKind.
- Which repo and tier are appropriate (e.g., a vault or seeds repo).
- What invariant and metric ranges are recommended for seeds.

Commands:

```bash
python tooling/python/cli/hpc-generate-spine-index.py --check
python tooling/python/cli/hpc-validate-schema.py --json
python tooling/python/cli/hpc-lint-registry.py --json --summary
```

The AI reads:

- Seed contract schema under `schemas/core/seed-contract-card-v1.json`.
- Spine entries for invariants and metrics relevant to `seedContractCard`.

---

## 2. Draft an ai-authoring-request for a seed

Example request:

```json
{
  "intent": "create a slow-burn seed that raises dread in high-CIC regions",
  "objectKind": "seedContractCard",
  "targetRepo": "HorrorPlace-Atrocity-Seeds",
  "tier": "vault",
  "schemaRef": "https://horror.place/schemas/core/seed-contract-card-v1.json",
  "referencedIds": [
    "REGION-SUBWAY-0001"
  ],
  "intendedInvariants": {
    "CIC": 0.75,
    "AOS": 0.8,
    "LSG": 0.7,
    "HVF": 0.6,
    "SHCI": 0.9
  },
  "intendedMetrics": {
    "UEC": [0.6, 0.95],
    "ARR": [0.7, 0.9],
    "CDL": [0.4, 0.7]
  },
  "manifestTier": "vault"
}
```

The AI must keep invariant and metric bands within the spine’s vault-tier ranges.

---

## 3. Normalize with CHAT_DIRECTOR plan

```bash
hpc-chat-director plan \
  --from-file path/to/ai-authoring-request.seed.json \
  --format json \
  > .ai/seed-contract-plan.json
```

The plan clarifies:

- Final `targetRepo` and `targetPath`.
- Schemas and manifests to use.
- Whether requested bands are compatible with policies for seeds.

---

## 4. Generate the seed contract card artifact

Based on the plan and seed schema, the AI generates the contract artifact.

Example artifact payload:

```json
{
  "id": "SEED-SUBWAY-SLOWBURN-0001",
  "objectKind": "seedContractCard",
  "schemaRef": "https://horror.place/schemas/core/seed-contract-card-v1.json",
  "seedName": "Subway Slowburn",
  "appliesToRegionId": "REGION-SUBWAY-0001",
  "invariantBindings": {
    "CIC": 0.78,
    "AOS": 0.82,
    "LSG": 0.73,
    "HVF": 0.61,
    "SHCI": 0.92
  },
  "metricTargets": {
    "UEC": [0.65, 0.93],
    "ARR": [0.72, 0.88],
    "CDL": [0.42, 0.68]
  },
  "gradientProfile": "slow-burn"
}
```

The AI wraps this artifact in an `ai-authoring-response` envelope.

---

## 5. Envelope pre-flight with aiauthoringvalidator.py

```bash
python tooling/python/schemaspine/aiauthoringvalidator.py response \
  path/to/ai-authoring-response.seed.json \
  > .ai/seed-contract-validation.jsonl
```

The validator checks:

- Envelope structure and provenance fields.
- The presence and band validity of `invariantBindings` and `metricTargets`.
- Self-consistency of tier and manifest-tier declarations.

The AI iterates until the script exits with code 0.

---

## 6. Full validation and apply

```bash
hpc-chat-director validate-response \
  --from-file path/to/ai-authoring-response.seed.json \
  --format json \
  > .ai/seed-contract-validated.json
```

On success, apply:

```bash
hpc-chat-director apply \
  --from-file .ai/seed-contract-validated.json \
  --format json
```

This appends the seed entry to the appropriate NDJSON registry (e.g., `registry/seeds.ndjson` in the seeds repo) in a manifest-approved location.
