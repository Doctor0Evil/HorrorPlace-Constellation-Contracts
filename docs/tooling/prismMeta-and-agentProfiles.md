# prismMeta and agentProfile Schemas

This document explains two core tooling contracts in the HorrorPlace‑Constellation‑Contracts repository:

- `prismMeta.v1` – how every AI‑generated contract card declares provenance, dependencies, and validation state.
- `agentProfile.v1` – how the system describes an AI/chat agent’s rights (clamp, schema upgrade, registry writes, policy edits).

Together, these schemas let the VM‑constellation treat AI‑chat as a deterministic compiler: each turn emits one contract card plus a machine‑checkable record of **who** generated it, **what** it depends on, and **what** the agent was allowed to do.

---

## 1. prismMeta: provenance and validation for a single file

### Purpose

`prismMeta` is attached to every contract card (policy, region, seed, etc.). It answers four questions:

1. Which session and agent generated this file?
2. Which repository and path is it intended for?
3. Which upstream contracts and proofs does it depend on?
4. What was the validation result (schema, subset rules, ledger resolution), and is there a signed delta?

This makes each file self‑describing and auditable, without re‑running the whole conversation that produced it.

### Location

- File: `schemas/tooling/prismMeta.v1.json`
- Type: JSON Schema Draft‑07

### Key fields (human overview)

- `prismId`  
  Stable identifier for this prism artifact. Often equals the contract card `id`, but can differ if you want multiple prisms per logical card.

- `schemaVersion`  
  Version of the prism schema itself (e.g., `prismMeta.v1`). Used to migrate older prism records.

- `sessionId`  
  Opaque id tying this file back to the AI‑chat / tool session that generated it.

- `agentId` and `agentProfileId`  
  - `agentId`: logical name of the agent or tool (e.g., `archivist-copilot`, `seed-refactor-bot`).
  - `agentProfileId`: reference into an `agentProfile` document (see below) that declares what this agent is allowed to do.

- `targetRepo` and `path`  
  - `targetRepo`: which repository this file is meant to live in (e.g., `Doctor0Evil/HorrorPlace-Atrocity-Seeds`).
  - `path`: the repo‑relative path where the file should be written.

- `tier`  
  Tier label (e.g., `standard`, `mature`, `research`), used to enforce access and proof requirements.

- `hash`  
  Content hash of the generated file (e.g., `sha256:<hex>`). Downstream systems use this to verify they are validating the same bytes the agent signed.

- `dependencies[]`  
  A list of upstream contracts and proofs this file depends on, each with:
  - `id` – the referenced contract or proof id (e.g., `region.aral-basin.v1`, `policy.global-standard.v1`).
  - `kind` – type label (e.g., `policy`, `region`, `seed`, `proof`, `style`).
  - Optional `repo` and `path` – where that dependency lives.

  This enables impact analysis: if a region card changes, the spine and CI can discover all dependent seeds by walking `dependencies`.

- `validation`  
  Summary of the last validation pass at generation time:
  - `schemaValidated` – JSON Schema validation for the card.
  - `subsetValidated` – invariant/metric subset rules (seed vs. region vs. policy).
  - `ledgerValidated` – Dead‑Ledger / proof resolution for `deadledgerref`.
  - `clamped` – whether any values were clamped.
  - `clampReasons[]` – optional list of human‑readable notes (e.g., `DET clamped from 12.0 to 10.0`).
  - `timestamp` – when this validation was performed.

  These fields are not a replacement for CI; they are a snapshot of what the agent believed was true when it emitted the file.

- `signedDelta`  
  Cryptographic attestation over the file content and its dependencies:
  - `algorithm` – signature algorithm (e.g., `ed25519`).
  - `signature` – encoded signature bytes (e.g., base64url).
  - `publicKeyRef` – reference to a public key managed by Dead‑Ledger or another key authority.
  - Optional `proofEnvelopeId` – id of a zkpproof envelope that attests this delta.

  This allows Dead‑Ledger and orchestrators to verify provenance and prevent tampering between generation and ingestion.

### How engines and CI should use prismMeta

1. **On generation (AI‑chat / tools):**
   - Fill in all required prism fields.
   - Run a local validator for the contract card and record results in `validation`.
   - Sign the file + dependency list and store the signature in `signedDelta`.

2. **On CI / ingestion:**
   - Verify the card against its own JSON Schema.
   - Verify `prismMeta` against `prismMeta.v1.json`.
   - Recompute the hash and verify the signature.
   - Cross‑check `dependencies` against the schema spine index and registries.

3. **On drift / refactor:**
   - When an upstream contract changes, use `dependencies` to locate affected cards and queue re‑validation or refactor tasks.

---

## 2. agentProfile: who is allowed to clamp, migrate, and edit

### Purpose

`agentProfile` defines the **capabilities** of an AI/chat agent or tool:

- Can it clamp invariants and metrics to fit region/policy caps?
- Can it upgrade schema versions on existing cards?
- Can it write to registries?
- Can it edit policies and regions, or only seeds?

This turns clamping and migration into explicit, auditable entitlements rather than ad‑hoc behavior.

### Location

- Primary definition: `schemas/tooling/agentProfile.v1.json` in HorrorPlace‑Constellation‑Contracts.
- Optionally mirrored in:
  - `HorrorPlace-Dead-Ledger-Network/schemas/agentProfile.v1.json`
  - Other constellation repos that perform local agent checks.

### Key fields (human overview)

- `id`  
  Stable id for the profile, referenced by `prismMeta.agentProfileId` (e.g., `agentProfile.seed-generator.standard`).

- `schemaVersion`  
  Version of this profile schema (e.g., `agentProfile.v1`).

- `displayName`  
  Human‑readable name (e.g., “Seed Generator – Standard Tier”).

- `kind`  
  Conceptual category: `generator`, `refactor`, `auditor`, `orchestrator`, etc.

- `tiers[]`  
  List of tiers the agent is allowed to operate in. Examples:
  - `["standard"]` – public GitHub‑safe generation only.
  - `["standard","mature"]` – can touch Tier 2 vaults.
  - `["research"]` – restricted to Tier 3 labs.

- `permissions`  
  Fine‑grained rights:
  - `allowsClamp` – may the validator clamp invariants/metrics on behalf of this agent?
  - `allowsSchemaUpgrade` – may it propose `schemaVersion` upgrades for existing cards?
  - `allowsRegistryWrite` – can it add or modify registry entries?
  - `allowsPolicyEdit` – can it modify `policyEnvelope` and `regionContractCard` caps/bounds?

  These flags are directly consulted by validation tools and CI when deciding whether to accept clamped or structural changes.

- `allowedSchemas[]`  
  List of schema ids/types the agent may generate or edit. Examples:
  - `["seedContractCard.v1"]` – seed‑only generator.
  - `["seedContractCard.v1","regionContractCard.v1"]` – region‑capable refactor tool.

- `rateLimits` (optional)  
  Soft limits to throttle the agent:
  - `maxContractsPerHour`
  - `maxSeedsPerRegion`

- `meta`  
  Freeform metadata: team ownership, purpose, notes, etc.

### How engines and CI should use agentProfile

1. **Token creation and mapping:**
   - Each agent (or class of agents) is issued an `agentProfile` and a token that encodes `agentProfileId`.
   - AI‑chat systems use that profile to decide which operations they may attempt.

2. **Validation and clamping:**
   - When validating a card, tools look up the `agentProfile` by `prismMeta.agentProfileId`.
   - If values exceed caps and `allowsClamp` is `true`, clamping is allowed and recorded in `prismMeta.validation`.
   - If `allowsClamp` is `false`, out‑of‑range proposals cause validation errors instead.

3. **Registry and policy changes:**
   - CI enforces that:
     - Only agents with `allowsRegistryWrite` may introduce registry deltas.
     - Only agents with `allowsPolicyEdit` may change region or policy caps.

4. **Tier enforcement:**
   - If `prismMeta.tier` is not present in `agentProfile.tiers`, CI rejects the change.
   - Dead‑Ledger can further require proof envelopes tying `agentProfile` to ZKP‑based age/charter checks.

---

## 3. How these contracts change AI‑chat behavior

With `prismMeta` and `agentProfile` in place, AI‑chat moves from “free‑form answer” to “contract‑aware compiler step”:

- Every answer that creates or edits a contract card:
  - Declares target repo, path, tier, and schema.
  - Records who generated it and what that agent is allowed to do.
  - Lists upstream dependencies and validation status.
  - Optionally carries a signed delta and proof envelope id.

- Validators and CI pipelines:
  - Are no longer guessing about context—they read `prismMeta` and `agentProfile` directly.
  - Decide whether to clamp, reject, or accept based on explicit rights, not heuristics.
  - Can trace and re‑validate all downstream cards when a region/policy/proof changes.

For designers and operators, this means the system behaves like a VM‑constellation copilot: one file per request, fully wired, with a clear audit trail of **who**, **what**, and **why** behind each contract.

---
