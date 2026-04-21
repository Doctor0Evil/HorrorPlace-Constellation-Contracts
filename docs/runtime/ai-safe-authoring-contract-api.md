## AI‑Safe Authoring Session Contract API

Repository: `HorrorPlace-Constellation-Contracts`  
Path: `docs/runtime/ai-safe-authoring-contract-api.md` [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/c7e27f07-7ccf-4687-aacf-762464846578/this-research-focuses-on-creat-b.QYu2gNRjaV4nBZ03r88w.md)

This document defines how AI‑Chat tools and coding agents construct and use the **AI‑safe authoring session contract** as the front door to any authoring work in the VM‑constellation. It sits above `ai-authoring-request`/`ai-authoring-response` and is enforced by CHATDIRECTOR. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)

The contract object itself is the JSON shape described by `schemastooling/ai-safe-authoring-contract.v1.json`. Agents are responsible for building one instance per logical authoring session and keeping their requests aligned to it. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/c7e27f07-7ccf-4687-aacf-762464846578/this-research-focuses-on-creat-b.QYu2gNRjaV4nBZ03r88w.md)

***

### 1. Purpose and doctrine

An AI‑safe authoring session contract is a single JSON document that describes: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)

- Which **agent** is active and which **ai‑safe authoring profile** governs it.  
- Which **spine**, **manifests**, and **registries** have been loaded for discovery.  
- Which **artifacts** the session plans to emit (object kinds, target repos, tiers, schemarefs).  
- Which **invariants** and **entertainment metrics** those artifacts will touch.  
- Which **safety bands** and **Dead‑Ledger surfaces** are allowed for this session.

Doctrine:

- No AI authoring without a session contract. CHATDIRECTOR must see and validate a contract before accepting any `ai-authoring-request`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/c7e27f07-7ccf-4687-aacf-762464846578/this-research-focuses-on-creat-b.QYu2gNRjaV4nBZ03r88w.md)
- Contracts are **structural**: they describe *where* and *what*, not the prose body. Numeric ranges stay in the spine and profile schemas. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/c7e27f07-7ccf-4687-aacf-762464846578/this-research-focuses-on-creat-b.QYu2gNRjaV4nBZ03r88w.md)
- One contract governs many turns of chat, but all turns must stay inside its declared plan and safety envelope. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)

***

### 2. Contract shape (summary)

The full JSON Schema lives at `schemastooling/ai-safe-authoring-contract.v1.json`. The logical top‑level layout: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/c7e27f07-7ccf-4687-aacf-762464846578/this-research-focuses-on-creat-b.QYu2gNRjaV4nBZ03r88w.md)

- `id: string` – stable session‑contract ID.  
- `schemaVersion: string` – e.g. `"ai-safe-authoring-contract.v1"`.  
- `agent` – who is speaking.  
- `profile` – which ai‑safe authoring profile governs this session.  
- `discovery` – which spines/manifests/registries are loaded.  
- `plan` – planned artifacts for this session.  
- `safety` – consent tier, max intensity band, allowed Dead‑Ledger surfaces.  
- `telemetry` – session ID and optional target metric hints.

Key sub‑objects: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/c7e27f07-7ccf-4687-aacf-762464846578/this-research-focuses-on-creat-b.QYu2gNRjaV4nBZ03r88w.md)

**Agent block**

- `agent.agentId: string` – logical tool or agent name.  
- `agent.agentProfileId: string` – reference to an `ai-safe-authoring-profile-v1` or `agentProfile.v1` card.  
- `agent.roles: ["Architect" | "Implementer" | "Auditor", ...]`.

**Profile block**

- `profile.profileId: string` – active `ai-safe-authoring-profile-v1` ID.  
- `profile.tiers: ["standard" | "mature" | "research", ...]` – tiers allowed in this session.  
- `profile.repos: string[]` – canonical repo names this contract may target.

**Discovery block**

- `discovery.spineLoaded: boolean` – invariants, entertainment metrics, and schema spine are loaded.  
- `discovery.manifestsLoaded: boolean` – repo manifests for all target repos are loaded.  
- `discovery.registriesLoaded: boolean` – required NDJSON registries are loaded.  
- `discovery.sources.spineIndexRef: string` – optional reference to the spine index document.  
- `discovery.sources.manifestRefs: string[]` – manifest IDs.  
- `discovery.sources.registryRefs: string[]` – registry IDs.

**Plan block**

- `plan.artifacts: ArtifactPlan[]` – at least one entry.  
- `plan.maxFilesInBundle: integer` – upper bound (usually 1–3). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/c7e27f07-7ccf-4687-aacf-762464846578/this-research-focuses-on-creat-b.QYu2gNRjaV4nBZ03r88w.md)

Each `ArtifactPlan` contains: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/c7e27f07-7ccf-4687-aacf-762464846578/this-research-focuses-on-creat-b.QYu2gNRjaV4nBZ03r88w.md)

- `objectKind: string` – e.g. `regionContractCard`, `aiChatHorrorProfile`.  
- `targetRepo: string` – canonical repo name.  
- `targetPath: string` – repo‑relative path.  
- `schemaRef: string` – canonical JSON Schema URI.  
- `tier: "T1-core" | "T2-vault" | "T3-lab"` – constellated tier.  
- `invariantsTouched: [ "CIC" | "MDI" | "AOS" | "RRM" | "FCF" | "SPR" | "RWF" | "DET" | "HVF" | "LSG" | "SHCI", ... ]`.  
- `metricsTouched: [ "UEC" | "EMD" | "STCI" | "CDL" | "ARR", ... ]`.

**Safety block**

- `safety.consentTier: string` – effective consent tier (e.g. `"adult-basic"`).  
- `safety.maxIntensityBand: integer` – ceiling on intensity bands (0–10).  
- `safety.deadLedgerSurfaces: [ "zkp-proof-schema" | "verifiers-registry" | "bundle-attestation" | "agent-attestation" | "spectral-seed-attestation" | "bci-state-proof", ... ]`.

**Telemetry block**

- `telemetry.sessionId: string` – binding to runtime metrics envelopes. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/c7e27f07-7ccf-4687-aacf-762464846578/this-research-focuses-on-creat-b.QYu2gNRjaV4nBZ03r88w.md)
- `telemetry.uecTarget?: number` – optional session‑level UEC target band.  
- `telemetry.arrTarget?: number` – optional session‑level ARR target band.

***

### 3. Lifecycle and CHATDIRECTOR integration

CHATDIRECTOR treats the session contract as a **pre‑plan envelope** that must be present and valid before any artifact‑level requests are considered. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/c7e27f07-7ccf-4687-aacf-762464846578/this-research-focuses-on-creat-b.QYu2gNRjaV4nBZ03r88w.md)

1. **Discovery phase**

   - Agent or tool calls a discovery helper, loads:  
     - schema spine, invariants spine, entertainment metrics spine,  
     - manifests for intended repos,  
     - required registries (regions, events, personas, styles, seeds). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)
   - Agent sets `discovery.spineLoaded`, `manifestsLoaded`, `registriesLoaded` to `true` and fills `sources.*` with the IDs used. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/c7e27f07-7ccf-4687-aacf-762464846578/this-research-focuses-on-creat-b.QYu2gNRjaV4nBZ03r88w.md)

2. **Plan phase**

   - Agent constructs `plan.artifacts` from the user’s intent and routing rules.  
   - For each planned artifact, `ManifestIndex` / `ManifestClient` (or equivalent Lua helper) should define `targetRepo`, `targetPath`, `schemaRef`, and `tier`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)
   - Agent declares which invariants and metrics will be touched (`invariantsTouched`, `metricsTouched`) based on the schema and spine. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/c7e27f07-7ccf-4687-aacf-762464846578/this-research-focuses-on-creat-b.QYu2gNRjaV4nBZ03r88w.md)

3. **Contract validation**

   - Agent submits the session contract JSON to `chat-director contract-validate` (CLI) or a corresponding API endpoint. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/c7e27f07-7ccf-4687-aacf-762464846578/this-research-focuses-on-creat-b.QYu2gNRjaV4nBZ03r88w.md)
   - CHATDIRECTOR validates:  
     - JSON Schema correctness (`ai-safe-authoring-contract.v1.json`).  
     - Compatibility with the referenced `ai-safe-authoring-profile-v1`.  
     - Manifest routing (objectKind + tier → correct repo/path).  
     - That invariants/metrics touched are allowed for this agent and profile. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)
   - On success, CHATDIRECTOR records the contract and returns a normalized representation or simple “accepted” response with the contract ID.

4. **Authoring phase**

   - Every `AiAuthoringRequest` sent by the agent must reference the active contract ID and select a subset of `plan.artifacts` as targets. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/c7e27f07-7ccf-4687-aacf-762464846578/this-research-focuses-on-creat-b.QYu2gNRjaV4nBZ03r88w.md)
   - Every `AiAuthoringResponse` must:  
     - Emit at most `plan.maxFilesInBundle` artifacts.  
     - Wrap each output in a prism envelope whose `targetRepo`, `targetPath`, `schemaRef`, and `tier` match exactly one `plan.artifacts` entry.  
     - Stay within the invariant and metric bands implied by the active `ai-safe-authoring-profile` and the planned `invariantsTouched`/`metricsTouched`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/c7e27f07-7ccf-4687-aacf-762464846578/this-research-focuses-on-creat-b.QYu2gNRjaV4nBZ03r88w.md)

5. **CI and telemetry**

   - CI jobs read the contract and verify that:  
     - All emitted artifacts are covered by the contract’s plan.  
     - No out‑of‑profile invariants or metrics appear. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/c7e27f07-7ccf-4687-aacf-762464846578/this-research-focuses-on-creat-b.QYu2gNRjaV4nBZ03r88w.md)
   - Telemetry binds session metrics envelopes to `telemetry.sessionId` for evaluation of CI health, schema drift, and entertainment metric adherence across sessions. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)

***

### 4. Minimal API surface for tools

This section describes the expected helper calls for AI‑Chat and coding agents; the exact transport (CLI vs HTTP) is implementation‑specific.

#### 4.1 Tool → CHATDIRECTOR

**`CreateAndValidateContract`**

- **Input**:  
  - Natural language high‑level goal (out‑of‑band).  
  - Draft `AiSafeAuthoringContract` JSON document.  
- **Behavior**:  
  - Validates JSON against `ai-safe-authoring-contract.v1.json`.  
  - Resolves routing via manifests and spine if `targetRepo`/`targetPath` are incomplete.  
  - Validates compatibility with `ai-safe-authoring-profile-v1` referenced in `profile.profileId` and `agent.agentProfileId`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/c7e27f07-7ccf-4687-aacf-762464846578/this-research-focuses-on-creat-b.QYu2gNRjaV4nBZ03r88w.md)
- **Output**:  
  - Normalized contract JSON with canonicalized `targetRepo`, `targetPath`, `schemaRef`, `tier`.  
  - Or structured errors pointing at schema violations, routing issues, or profile conflicts.

**`ContractToAuthoringRequests`**

- **Input**:  
  - Validated `AiSafeAuthoringContract` JSON.  
- **Behavior**:  
  - Expands `plan.artifacts` into a set of `AiAuthoringRequest` envelopes, each referencing the parent contract and one planned artifact. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/c7e27f07-7ccf-4687-aacf-762464846578/this-research-focuses-on-creat-b.QYu2gNRjaV4nBZ03r88w.md)
- **Output**:  
  - List of `AiAuthoringRequest` objects.

#### 4.2 Tool behavior contract

AI‑Chat and coding agents must:

- Refuse to generate artifacts if `discovery.spineLoaded`, `manifestsLoaded`, or `registriesLoaded` are `false`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/c7e27f07-7ccf-4687-aacf-762464846578/this-research-focuses-on-creat-b.QYu2gNRjaV4nBZ03r88w.md)
- Never propose artifacts outside `plan.artifacts`; all new work must be preceded by a contract update and re‑validation. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/c7e27f07-7ccf-4687-aacf-762464846578/this-research-focuses-on-creat-b.QYu2gNRjaV4nBZ03r88w.md)
- Treat `safety.maxIntensityBand` and `safety.deadLedgerSurfaces` as hard caps when planning cross‑repo work (e.g., no new Dead‑Ledger surface types beyond this list). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)
- Honour the “max three artifacts per bundle” doctrine by not exceeding `plan.maxFilesInBundle` in any single response. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/c7e27f07-7ccf-4687-aacf-762464846578/this-research-focuses-on-creat-b.QYu2gNRjaV4nBZ03r88w.md)

***

### 5. Example (abbreviated)

Below is an abbreviated example of a session contract for an AI‑Chat agent that plans to add a single `aiChatHorrorProfile` card and a supporting template contract in one repo.

```json
{
  "id": "contract-2026-04-20-001",
  "schemaVersion": "ai-safe-authoring-contract.v1",
  "agent": {
    "agentId": "hp-chat-authoring-bot",
    "agentProfileId": "profile:ai-safe-authoring-standard-v1",
    "roles": ["Architect", "Implementer"]
  },
  "profile": {
    "profileId": "profile:ai-safe-authoring-standard-v1",
    "tiers": ["standard"],
    "repos": ["HorrorPlace-Constellation-Contracts"]
  },
  "discovery": {
    "spineLoaded": true,
    "manifestsLoaded": true,
    "registriesLoaded": true,
    "sources": {
      "spineIndexRef": "spine-index:2026-04-20",
      "manifestRefs": ["manifest:Constellation-Contracts:v1"],
      "registryRefs": ["registry:entertainment-metrics:v1"]
    }
  },
  "plan": {
    "maxFilesInBundle": 2,
    "artifacts": [
      {
        "objectKind": "aiChatHorrorProfile",
        "targetRepo": "HorrorPlace-Constellation-Contracts",
        "targetPath": "bundles/runtime/ai-chat/horror-profiles/slowburn-dread.v1.json",
        "schemaRef": "https://horror.place/schemas/runtime/ai-chat-horror-profile.v1.json",
        "tier": "T2-vault",
        "invariantsTouched": ["CIC", "DET", "SHCI"],
        "metricsTouched": ["UEC", "EMD", "STCI", "CDL", "ARR"]
      },
      {
        "objectKind": "aiChatTemplateContract",
        "targetRepo": "HorrorPlace-Constellation-Contracts",
        "targetPath": "bundles/runtime/ai-chat/templates/slowburn-dread-main.v1.json",
        "schemaRef": "https://horror.place/schemas/runtime/ai-chat-template-contract.v1.json",
        "tier": "T2-vault",
        "invariantsTouched": [],
        "metricsTouched": ["UEC", "ARR"]
      }
    ]
  },
  "safety": {
    "consentTier": "adult-basic",
    "maxIntensityBand": 6,
    "deadLedgerSurfaces": ["bundle-attestation", "agent-attestation"]
  },
  "telemetry": {
    "sessionId": "session-7c1a0f24-5f05-4f93-9f74-9c4b0b61a201",
    "uecTarget": 0.7,
    "arrTarget": 0.75
  }
}
```

In this example, CHATDIRECTOR can derive all routing and safety decisions before any file is emitted, and AI‑Chat knows exactly which invariants and metrics it is allowed to touch during this authoring session. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)
