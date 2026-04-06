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

# CHAT_DIRECTOR v1 Specification

This document defines CHAT_DIRECTOR as a schema-driven authoring service that sits on top of the existing schema spine, repo manifest, and prism contracts for the VM-Constellation. It narrows v1 scope to moods, event contracts, and region/seed contractCards and integrates into CI as a validator/generator that operates entirely through JSON/NDJSON envelopes.

CHAT_DIRECTOR is not a new “tool class”; it is a contract-aware front door that wraps the ai-authoring-request/response/prism envelope flow and enforces invariant and metric bands at authoring time. It emits constellation-compliant artifacts only and never generates engine code directly.

---

## 1. Role and scope of CHAT_DIRECTOR v1

CHAT_DIRECTOR v1 is positioned as the authoring layer between AI-chat systems and the constellation’s contract surface. It owns the process of accepting authoring requests, validating them against the schema spine and repo manifests, and producing contractCards and registry entries that are safe to commit.

### 1.1 Supported contract kinds in v1

CHAT_DIRECTOR v1 supports a narrow set of contract kinds:

- Mood contracts  
- Event contracts  
- Region contractCards  
- Seed contractCards  

Persona contracts are explicitly out of scope until SHCI policies, telemetry, and persona governance are fully encoded in schemas and repo policies.

### 1.2 Responsibilities

For supported kinds, CHAT_DIRECTOR:

- Accepts `ai-authoring-request.v1` envelopes that specify intent, target kind, and constraints.  
- Validates each request against `schema-spine-index` and `repo-manifest.hpc.json`.  
- Emits exactly one `ai-authoring-response.v1` prism envelope per turn.  
- Produces either a new contractCard (mood/event/region/seed) or returns a structured refusal with machine-readable error codes.  
- Optionally emits NDJSON registry diffs (new or updated entries) as part of the response envelope.

CHAT_DIRECTOR does not write files directly; CI and orchestrators consume its envelopes and apply them to repositories.

---

## 2. Schema spine integration

CHAT_DIRECTOR is a consumer of the schema spine index defined in this repository. It treats the spine as a hard gate for acceptable schemas and contract kinds.

### 2.1 Required schema spine entries

The `schema-spine-index.json` MUST include:

- Invariants spine  
- Entertainment metrics spine  
- Contract family schemas  
- Authoring envelope schemas  

At minimum, the following schema IDs must be present:

- `core/invariants-spine.v1`  
- `core/entertainment-metrics-spine.v1`  
- `core/eventContract.v1`  
- `core/regionContractCard.v1`  
- `core/seedContractCard.v1`  
- `core/moodContract.v1`  
- `tooling/ai-authoring-request.v1`  
- `tooling/ai-authoring-response.v1`  
- `tooling/prismMeta.v1`  

Each entry should declare its consumers, including CHAT_DIRECTOR, so impact analysis and drift detection remain consistent.

### 2.2 Spine loading and enforcement

On startup, CHAT_DIRECTOR:

1. Loads `schema-spine-index.json` into typed structures.  
2. Builds a map of allowed `schemaRef` values and contract kinds.  
3. Refuses to accept any authoring request whose `schemaRef` or `kind` is not present in the spine.  

This turns the schema spine from documentation into an enforcement surface.

---

## 3. Repo manifest integration

Per-repo manifests give CHAT_DIRECTOR the routing layer it needs to map contract kinds to concrete repositories and paths.

### 3.1 Manifest structure

Each repo that participates in CHAT_DIRECTOR flows defines a manifest file at its root, tentatively named `repo-manifest.hpc.json`. A minimal shape:

```json
{
  "repoName": "HorrorPlace-Atrocity-Seeds",
  "tier": "T2-vault",
  "allowedSchemas": [
    "https://horror.place/schemas/core/seedContractCard.v1.json"
  ],
  "defaultTargets": {
    "seedContractCard": {
      "basePath": "registry/seeds",
      "idPrefix": "SEED-"
    }
  },
  "authoringRules": {
    "requirePrismEnvelope": true,
    "requireDeadledgerRef": true,
    "allowedKinds": ["seedContractCard"]
  }
}
```

Repositories can extend this schema, but CHAT_DIRECTOR only relies on the fields above.

### 3.2 Routing semantics

When CHAT_DIRECTOR receives a request, it:

- Uses `kind` and `tier` to find the appropriate repo manifest.  
- Confirms that `schemaRef` is in `allowedSchemas` for that repo.  
- Computes a target path using `defaultTargets` (e.g., `basePath` plus ID).  

If any check fails, CHAT_DIRECTOR rejects the request with a structured error rather than attempting generation.

---

## 4. Authoring envelopes

CHAT_DIRECTOR communicates exclusively through the ai-authoring-request and ai-authoring-response/prism envelope schemas defined in this repository.

### 4.1 Authoring request fields

The `ai-authoring-request.v1` schema must support the following conceptual fields:

- `requestId` — stable identifier for traceability.  
- `agentId` — identity of the calling AI agent.  
- `kind` — one of `mood`, `eventContract`, `regionContractCard`, or `seedContractCard`.  
- `targetRepo` — optional explicit repo name; if omitted, CHAT_DIRECTOR selects based on manifests.  
- `schemaRef` — canonical contract schema.  
- `referencedIds` — IDs such as region IDs, seed IDs, or mood IDs that the new contract should relate to.  
- `invariantTargets` — requested invariant bands (e.g., CIC/AOS/LSG).  
- `metricTargets` — requested entertainment metric bands (e.g., UEC/EMD/STCI/CDL/ARR).  

Requests are validated against the corresponding JSON Schema before further processing.

### 4.2 Authoring response / prism envelope

The `ai-authoring-response.v1` schema defines a response envelope with:

- `responseId` and `requestId`  
- `status` — `success` or `refused`  
- `targetRepo` and `targetPath`  
- `schemaRef`  
- `payload` — the contractCard or mood/event contract object  
- `registryDiff` — optional NDJSON lines wrapped as JSON strings  
- `prismMeta` — embedded prism metadata describing the generator, profile, and constraints  

For `status=refused`, `payload` can carry structured error details instead of a contract object.

CHAT_DIRECTOR must never emit raw engine code, Lua, or Rust; it only generates contract-level JSON/NDJSON.

---

## 5. Validation semantics

The primary value of CHAT_DIRECTOR is that it enforces invariant and metric constraints at authoring time, guided by the schema spine and repo manifests.

### 5.1 Invariant and metric ranges

CHAT_DIRECTOR reads the invariants and metrics spines and uses them to validate requested and generated values. Examples:

- CIC, MDI, AOS, RRM, FCF, SPR, RWF, DET, HVF, LSG, SHCI ranges and semantics.  
- UEC, EMD, STCI, CDL, ARR ranges and target bands.

Requests may specify target bands; generators must produce values that fit within both the global spines and any active policy or region contracts.

### 5.2 Request validation flow

For each request:

1. Validate against `ai-authoring-request.v1.json`.  
2. Confirm `kind` is supported by v1.  
3. Check `schemaRef` against the schema spine.  
4. Validate `targetRepo` and `tier` against `repo-manifest.hpc.json` if present.  
5. Ensure requested invariant and metric bands are subsets of global and repo-level policies.  

If any check fails, CHAT_DIRECTOR returns a `refused` response with machine-parsable error codes and does not attempt generation.

### 5.3 Response validation

For generated responses:

1. Validate `payload` against the referenced `schemaRef`.  
2. Validate `prismMeta` against `prismMeta.v1.json`.  
3. Verify that bound invariants and metrics remain within canonical ranges.  
4. If `tier` is higher than public, enforce `deadledgerRef` requirements from the repo manifest.  

CHAT_DIRECTOR must not mark a response as `success` unless all checks pass.

---

## 6. DreadForge and MASTERBLUEPRINT integration

CHAT_DIRECTOR must align with existing DreadForge and MASTERBLUEPRINT contracts and lifecycle assumptions.

### 6.1 Moods and DreadForge

Mood authoring is guided by:

- `moodContract.v1` and associated DreadForge-specific mood contracts.  
- Documentation describing mood bands and how DreadForge interprets CIC/LSG/HVF and UEC/EMD/STCI/CDL/ARR bands.

When generating a mood contract:

- CHAT_DIRECTOR must treat the mood schemas and DreadForge guidelines as canonical.  
- It must constrain invariant and metric bands to subsets of those allowed by the spines and any active policy envelopes and region contracts.  
- It must avoid introducing new implicit definitions that conflict with DreadForge expectations.

### 6.2 MASTERBLUEPRINT phase constraints

MASTERBLUEPRINT defines a phase model:

- Schemas  
- Registries  
- Bundles/seeds  
- Engine scripts (Lua, adapters)  

For v1, CHAT_DIRECTOR is restricted to the first three phases:

- It can create or update contractCards and registries.  
- It may not generate Lua modules, adapters, or other runtime code.  
- Engine layers always remain downstream consumers of accepted contracts.

This keeps CHAT_DIRECTOR focused on declarative contract work rather than runtime behavior.

---

## 7. Rust implementation outline

CHAT_DIRECTOR v1 is implemented as a Rust service built on existing or planned crates such as `hpc-spine`, `hpc-generate-contract`, and the prism compiler.

### 7.1 Core library responsibilities

A core library crate provides:

- Loading and parsing of `schema-spine-index.json` and `repo-manifest.hpc.json`.  
- Query helpers such as “is this `schemaRef` valid?” and “which repo handles `kind=regionContractCard` at tier vault?”.  
- Mapping from authoring request envelopes to target repo/path combinations.

### 7.2 Validator component

The validator:

- Embeds or shells out to a JSON Schema engine.  
- Validates both authoring requests and candidate responses against schemas listed in the spine.  
- Uses the invariants and metrics spines to enforce numeric ranges, rather than duplicating constraints.  

This ensures consistency when spines evolve.

### 7.3 Generator component

The generator:

- Supports the limited v1 set (mood, event, region, seed).  
- Produces skeleton contractCards with required fields, ID prefixes, `schemaRef`, and invariant/metric bands derived from:

  - Global spines  
  - Policy envelopes  
  - Region baselines (for region/seed work)

- Leaves free-form “intent” fields to higher-level AI-chat systems, which must stay within the numeric constraints established.

### 7.4 Service surfaces

The Rust core can appear as:

- A CLI binary for CI (e.g., `chat-director validate` and `chat-director generate`).  
- An HTTP/JSON service for interactive chat front-ends.  
- A minimal FFI wrapper for editor integrations.

All surfaces share the same schema spine and manifest logic.

---

## 8. CI integration and enforcement

CHAT_DIRECTOR integrates into existing CI pipelines as another validation and generation step, not as a separate workflow class.

### 8.1 Pre-merge gate

For pull requests that touch:

- Mood contracts  
- Event contracts  
- Region contractCards  
- Seed contractCards  
- Their associated registries  

CI must:

- Run CHAT_DIRECTOR in “audit mode” on the diff.  
- Validate that each changed file is covered by a valid authoring envelope.  
- Confirm that invariant and metric ranges obey spines, DET normalization, and policy bands.  

If any artifact fails, CI rejects the PR.

### 8.2 Prism envelope requirement

CI can enforce that:

- No new contract file is accepted unless it originated from a valid `ai-authoring-response` prism envelope.  
- Envelopes must be stored or referenced in a way that CI can verify, such as a dedicated directory or commit metadata.

This closes the gap between chat outputs and repository contents.

---

## 9. Impact on AI-chat behavior

With the spine, manifests, and authoring envelopes in place, CHAT_DIRECTOR becomes the operating system for AI-chat integrations touching contracts.

### 9.1 Constraints on external copilots

Any copilot or AI-chat integration that wishes to generate HorrorPlace artifacts must:

- Emit only `ai-authoring-request` and `ai-authoring-response` envelopes that validate against the canonical schemas.  
- Respect the `kind` and `schemaRef` values advertised in the spine.  
- Accept that one request maps to one envelope, which may be a contract or a structured refusal.  

Generative systems are thus constrained to act as deterministic contract compilers.

### 9.2 Benefits for the constellation

Under these rules:

- All AI-authored artifacts conform to shared schemas and spines.  
- Invariant and metric bands remain within safe ranges from the outset.  
- Registries and bundles stay consistent with DreadForge and MASTERBLUEPRINT assumptions.  
- CI can reason about contracts and enforce doctrine programmatically, without relying on ad-hoc text analysis.

CHAT_DIRECTOR v1 therefore acts as a schema-driven, repo-aware authoring surface that keeps AI within the rails defined by the VM-Constellation’s contract spine.
