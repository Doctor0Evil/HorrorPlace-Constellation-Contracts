# prismMeta and agentProfile: Metadata Contracts for AI Integration

This document defines the structure and usage of `prismMeta` and `agentProfile`—two machine-readable metadata contracts that enable provenance, dependency tracking, bidirectional validation, and deterministic AI authoring in the HorrorPlace constellation.

## 1. prismMeta: provenance and validation for a single artifact

### Purpose

`prismMeta` is attached to every contract card and registry-backed artifact (for example, `policyEnvelope`, `regionContractCard`, `seedContractCard`). It answers four questions:

1. Which session and agent generated this file?
2. Which repository and path is it intended for?
3. Which upstream contracts and proofs does it depend on?
4. What was the validation result (schema, subset rules, ledger resolution), and is there a signed delta?

This makes each file self-describing and auditable, without replaying the conversation that produced it.

### Location

- Schema: `schemas/tooling/prismMeta.v1.json`
- Type: JSON Schema Draft‑07

### Key fields (human overview)

- `prismId`  
  Stable identifier for this prism artifact. Often equals the contract card `id`, but can differ if multiple prisms exist for a logical card.

- `schemaVersion`  
  Version of the prism schema itself (for example, `prismMeta.v1`). Used to migrate older prism records.

- `sessionId`  
  Opaque identifier tying this file back to the AI‑chat or tool session that generated it.

- `agentId` and `agentProfileId`  
  - `agentId`: logical name of the agent or tool (for example, `archivist-copilot`, `seed-refactor-bot`).
  - `agentProfileId`: reference into an `agentProfile` document that declares what this agent is allowed to do.

- `targetRepo` and `path`  
  - `targetRepo`: which repository this file is meant to live in (for example, `Doctor0Evil/HorrorPlace-Atrocity-Seeds`).
  - `path`: the repo-relative path where the file should be written.

- `tier`  
  Tier label (for example, `public`, `vault`, `lab`), used to enforce access and proof requirements.

- `hash`  
  Content hash of the generated file (for example, `sha256:<hex>`). Downstream systems use this to verify they are validating the same bytes the agent signed.

- `dependencies[]`  
  List of upstream contracts and proofs this file depends on, each entry containing:
  - `id` – referenced contract or proof id (for example, `REG-ARAL-0001`, `policy.global-standard.v1`).
  - `kind` – type label (for example, `policy`, `region`, `seed`, `proof`, `style`).
  - Optional `repo` and `path` – where that dependency lives.

  This enables impact analysis: if a region card changes, the spine and CI can discover all dependent seeds by walking `dependencies`.

- `validation`  
  Summary of the last validation pass at generation time:
  - `schemaValidated` – JSON Schema validation result for the card.
  - `subsetValidated` – invariant and metric subset rules (seed vs. region vs. policy).
  - `ledgerValidated` – Dead‑Ledger or proof resolution for `deadledgerref`.
  - `clamped` – whether any values were clamped.
  - `clampReasons[]` – optional list of notes (for example, `DET clamped from 12.0 to 10.0`).
  - `timestamp` – when this validation was performed.

  These fields are a snapshot of what the agent believed was true when it emitted the file; CI still revalidates.

- `signedDelta`  
  Cryptographic attestation over the file content and its dependencies:
  - `algorithm` – signature algorithm (for example, `ed25519`).
  - `signature` – encoded signature bytes (for example, base64url).
  - `publicKeyRef` – reference to a public key managed by Dead‑Ledger or another key authority.
  - Optional `proofEnvelopeId` – identifier of a proof envelope that attests this delta.

### Example structure

Example (illustrative; actual shape is governed by the JSON Schema):

```json
{
  "prismMeta": {
    "prismId": "prism:reg-aral-0001",
    "schemaVersion": "prismMeta.v1",
    "sessionId": "sess-20260115-030000Z-01",
    "agentId": "copilot-horror-v2.1",
    "agentProfileId": "agentProfile.seed-generator.standard",
    "targetRepo": "HorrorPlace-Black-Archivum",
    "path": "registry/regions/regions.ndjson",
    "tier": "vault",
    "hash": "sha256:abc123...",
    "dependencies": [
      {
        "id": "REG-ARAL-0001",
        "kind": "region"
      },
      {
        "id": "EVT-ARAL-0001",
        "kind": "event"
      }
    ],
    "validation": {
      "schemaValidated": true,
      "subsetValidated": true,
      "ledgerValidated": true,
      "clamped": false,
      "clampReasons": [],
      "timestamp": "2026-01-15T03:00:00Z"
    },
    "signedDelta": {
      "algorithm": "ed25519",
      "signature": "base64url-encoded-signature",
      "publicKeyRef": "deadledger:pubkey:agent-archivist-01",
      "proofEnvelopeId": "zkp-envelope-0001"
    }
  }
}
```

### How engines and CI should use prismMeta

1. **On generation (AI‑chat or tools):**
   - Fill in required prism fields.
   - Run a local validator for the contract card and record results in `validation`.
   - Compute the content hash and create a `signedDelta`.

2. **On CI or ingestion:**
   - Validate the card against its JSON Schema.
   - Validate `prismMeta` against `prismMeta.v1.json`.
   - Recompute the hash and verify the signature.
   - Cross‑check `dependencies` against the schema spine index and registries.

3. **On drift or refactor:**
   - When an upstream contract changes, use `dependencies` to locate affected cards and queue revalidation or refactor tasks.

---

## 2. agentProfile: who is allowed to clamp, migrate, and edit

### Purpose

`agentProfile` defines the capabilities of an AI/chat agent or tool:

- Whether it may clamp invariants and metrics to fit region or policy caps.
- Whether it may upgrade schema versions on existing cards.
- Whether it may write to registries.
- Whether it may edit policies and regions, or only seeds.

This turns clamping, migration, and registry updates into explicit, auditable entitlements instead of ad‑hoc behavior.

### Location

- Schema: `schemas/tooling/agentProfile.v1.json`
- Optionally mirrored in other constellation repos (for example, Dead‑Ledger or orchestrator nodes) that need local agent checks.

### Key fields (human overview)

- `id`  
  Stable identifier for the profile, referenced by `prismMeta.agentProfileId` (for example, `agentProfile.seed-generator.standard`).

- `schemaVersion`  
  Version of this profile schema (for example, `agentProfile.v1`).

- `displayName`  
  Human‑readable name (for example, `Seed Generator – Standard Tier`).

- `kind`  
  Conceptual category: `generator`, `refactor`, `auditor`, `orchestrator`, and similar roles.

- `tiers[]`  
  List of tiers the agent is allowed to operate in. For example:
  - `["public"]` – public, GitHub‑safe generation only.
  - `["public","vault"]` – can touch Tier 2 vaults.
  - `["lab"]` – restricted to Tier 3 labs.

- `permissions`  
  Fine‑grained rights:
  - `allowsClamp` – whether the validator may clamp invariants or metrics on behalf of this agent.
  - `allowsSchemaUpgrade` – whether it may propose `schemaVersion` upgrades for existing cards.
  - `allowsRegistryWrite` – whether it may add or modify registry entries.
  - `allowsPolicyEdit` – whether it may modify `policyEnvelope` and `regionContractCard` caps and bounds.

- `allowedSchemas[]`  
  List of schema identifiers or types the agent may generate or edit. For example:
  - `["seedContractCard.v1"]` – seed‑only generator.
  - `["seedContractCard.v1","regionContractCard.v1"]` – region‑capable refactor tool.

- `rateLimits` (optional)  
  Soft limits to throttle the agent, such as:
  - `maxContractsPerHour`
  - `maxSeedsPerRegion`

- `meta`  
  Freeform metadata such as team ownership, purpose, or notes.

### Example structure

Example (illustrative):

```json
{
  "agentProfile": {
    "id": "agentProfile.seed-generator.standard",
    "schemaVersion": "agentProfile.v1",
    "displayName": "Seed Generator – Standard Tier",
    "kind": "generator",
    "tiers": ["public", "vault"],
    "permissions": {
      "allowsClamp": true,
      "allowsSchemaUpgrade": false,
      "allowsRegistryWrite": true,
      "allowsPolicyEdit": false
    },
    "allowedSchemas": [
      "seedContractCard.v1"
    ],
    "rateLimits": {
      "maxContractsPerHour": 20,
      "maxSeedsPerRegion": 5
    },
    "meta": {
      "owner": "HorrorPlace Core Team",
      "notes": "Default profile for seed-generation copilot agents."
    }
  }
}
```

### How engines and CI should use agentProfile

1. **Token creation and mapping:**
   - Each agent or class of agents is issued an `agentProfile` and a token that encodes `agentProfileId`.
   - AI‑chat systems use the profile to decide which operations they may attempt.

2. **Validation and clamping:**
   - When validating a card, tools look up the `agentProfile` by `prismMeta.agentProfileId`.
   - If values exceed caps and `allowsClamp` is `true`, clamping is allowed and recorded in `prismMeta.validation`.
   - If `allowsClamp` is `false`, out‑of‑range proposals cause validation errors instead.

3. **Registry and policy changes:**
   - CI enforces that only agents with `allowsRegistryWrite` may introduce registry deltas.
   - Only agents with `allowsPolicyEdit` may change region or policy caps and bounds.

4. **Tier enforcement:**
   - If `prismMeta.tier` is not present in `agentProfile.tiers`, CI rejects the change.
   - Dead‑Ledger or other governance layers can require additional proof envelopes tying `agentProfile` to age or charter checks.

---

## 3. Interaction with the one‑file‑per‑request rule

`prismMeta` and `agentProfile` combine with the one‑file‑per‑request rule to turn AI‑chat into a contract‑aware compiler step:

- Every generation step that creates or edits a contract card:
  - Declares target repository, path, tier, and schema.
  - Records which agent generated it and what that agent is allowed to do.
  - Lists upstream dependencies and validation status.
  - Optionally carries a signed delta and proof envelope identifier.

- Validators and CI pipelines:
  - Read `prismMeta` and `agentProfile` directly instead of guessing context.
  - Decide whether to clamp, reject, or accept based on explicit rights, not heuristics.
  - Trace and revalidate downstream cards when a region, policy, or proof changes.

For designers and operators, this makes the constellation behave like a controlled compiler pipeline: one request, one file, fully wired, with a clear audit trail of who, what, and why behind each contract.
