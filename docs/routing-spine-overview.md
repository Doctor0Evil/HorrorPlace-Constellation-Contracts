# Routing Spine Overview v1

## 1. Purpose and Position

The routing spine is the specialization layer that turns the existing schema spine and repo manifest into a single, machine-readable router over the constellation. It defines how each `(objectKind, tier, repo)` triple is bound to a unique contract family, schema, and target path, so that both AI systems and CI pipelines can agree on where artifacts live and which constraints apply.

This document is a thin human-facing overview of the JSON artifacts:

- `schemas/hpc-routing-spine.schema.json` — structural schema for the routing spine.
- `schemas/hpc-routing-spine-v1.json` — first populated routing table instance.

The routing spine does not replace `schema-spine-index-v1.json`. Instead, it sits on top of it:

- `schema-spine-index-v1.json` remains the canonical catalog of invariants, metrics, schemas, and repos.
- `hpc-routing-spine-v1.json` adds a routing axis: `objectKind → tier → repo → {schemaRef, invariants, metrics, math constraints, phase}`.

Together, these files give CHAT_DIRECTOR, Wandering-entity guards, and external AI integrations a single source of truth for “what can be generated, where it goes, and which contracts bind it”.

---

## 2. Core Data Model

### 2.1 Top-level structure

At the top level, the routing spine instance exposes:

- `routingSpineVersion` — semantic version of the routing table (e.g. `1.0.0`).
- `updatedAt` — ISO-8601 timestamp for the current instance.
- `docHints` — optional, human-readable notes used by documentation generators (overview text and per-`objectKind` descriptions).
- `mathConstraints` — catalog of named math constraints that can be attached to routes.
- `objectKinds` — array of `objectKindEntry` objects, one per routed contract family.

Each of these elements is fully specified in `hpc-routing-spine.schema.json` with `additionalProperties: false` at every object layer, aligning with constellation doctrine and keeping the routing index strict and AI-safe.

### 2.2 Object kinds

Each `objectKindEntry` describes a single contract family across the constellation. In v1 the spine covers four families:

- `moodContract`
- `eventContract`
- `regionContractCard`
- `seedContractCard`

Every entry has:

- `name` — canonical object kind name (e.g. `moodContract`).
- `description` — short human-facing summary.
- `allowedTiers` — which tiers this objectKind is legal in (`T1-core`, `T2-vault`, `T3-lab`).
- `routes` — array of `routeEntry` objects, one per `(tier, repo)` combination.

The `docHints.objectKindNotes` section in the instance annotates each objectKind with a `summary` and `usage` string that can be surfaced in generated docs and authoring tools.

### 2.3 Route entries

A `routeEntry` is the core unit of routing. It binds a triple `(objectKind, tier, repo)` to the schema and constraints that govern it.

Each route has:

- `id` — opaque identifier, usually of the form `objectKind.tier.repo` (e.g. `moodContract.T1-core.Horror.Place`), used in logs and CI.
- `tier` — one of `T1-core`, `T2-vault`, `T3-lab`.
- `repo` — canonical repo name (e.g. `Horror.Place`, `HorrorPlace-Atrocity-Seeds`).
- `schemaRef` — canonical schema URI/ID that artifacts must conform to (e.g. `schemas/contracts/event-contract-v1.json`).
- `aiAuthoringKind` — discriminator used in `AiAuthoringRequest.objectKind` (e.g. `eventContract`).
- `defaultTargetPath` — repo-relative directory root (e.g. `events/`, `schemas/moods/`, `cards/regions/`, `seeds/`).
- `registryRef` — path to the registry that must reference this artifact (e.g. `registry/registry-events.ndjson`).
- `invariants` — list of invariant codes that must be present and banded on the artifact (`CIC`, `MDI`, `AOS`, `RRM`, `FCF`, `SPR`, `RWF`, `DET`, `HVF`, `LSG`, `SHCI`).
- `metrics` — list of entertainment metrics expected as targets or bands (`UEC`, `EMD`, `STCI`, `CDL`, `ARR`).
- `constraints` — list of `mathConstraints.id` values that govern this route (e.g. `sprFromInvariants.v1`, `shciCoupling.v1`).
- `phase` — CHAT_DIRECTOR phase where this route is legal (e.g. `Phase0Schema`, `Phase2Contracts`).
- `additionalPropertiesRequired` — boolean flag stating that the bound contract schema must use `additionalProperties: false` at the top level.
- `docHints` — optional `summary` and `usage` text for this route.

The schema leaves uniqueness of `(tier, repo)` per objectKind as a doctrinal guarantee and CI responsibility. Tooling can use `id` plus `(name, tier, repo)` to detect duplicates.

### 2.4 Math constraints

The `mathConstraints` array declares reusable formulas and constraint relationships that can be linked from routes without embedding math directly into the routing table.

Each entry includes:

- `id` — identifier such as `sprFromInvariants.v1`.
- `version` — constraint version, for evolution over time.
- `kind` — one of `formula`, `inequality`, or `derivedRange`.
- `inputs` — invariants and metrics this constraint reads.
- `output` — invariant or metric this constraint constrains (e.g. `SPR`, `SHCI`, `DET`).
- `expressionRef` — path to the detailed formula spec (e.g. `schemas/formulas/sprFromInvariants.v1.json`).
- `spineRef` — pointer into the schema spine’s formula catalog.
- `appliesTo` — optional set of object kinds or route IDs the constraint is meant for.

This design keeps the routing spine structurally stable while allowing formula details to evolve in separate, versioned specs.

---

## 3. Concrete v1 Routes

This section summarizes the concrete routes present in `hpc-routing-spine-v1.json`. It is not exhaustive of all future objectKinds, but provides enough coverage to exercise the routing model.

### 3.1 moodContract

**Role**

`moodContract` is the design-level mood contract family. Tier-1 entries define canonical numeric bands for moods without any raw horror content; Tier-2 entries consume these contracts as part of implementation repos.

**Allowed tiers**

- `T1-core`
- `T2-vault`

**Routes**

1. **T1-core / Horror.Place**

- `id`: `moodContract.T1-core.Horror.Place`
- `repo`: `Horror.Place`
- `schemaRef`: `schemas/contracts/mood-contract-v1.json`
- `defaultTargetPath`: `schemas/moods/`
- `registryRef`: `registry/registry-moods.ndjson`
- `phase`: `Phase0Schema`
- `invariants`: `CIC`, `AOS`, `RRM`, `LSG`, `DET`, `SHCI`
- `metrics`: `UEC`, `EMD`, `STCI`, `CDL`, `ARR`
- `constraints`:
  - `sprFromInvariants.v1` — ensures `SPR` bands are consistent with core invariants.
  - `metricBandsByTier.mood.v1` — enforces per-tier UEC/ARR/CDL bands for mood contracts.
- `additionalPropertiesRequired`: `true`

This route is the schema authority for mood contracts and is expected to carry design-only content suitable for public Tier-1 repos.

2. **T2-vault / HorrorPlace-Codebase-of-Death**

- `id`: `moodContract.T2-vault.HorrorPlace-Codebase-of-Death`
- `repo`: `HorrorPlace-Codebase-of-Death`
- `schemaRef`: `schemas/contracts/mood-contract-v1.json`
- `defaultTargetPath`: `contracts/moods/`
- `registryRef`: `registry/registry-moods.ndjson`
- `phase`: `Phase2Contracts`
- `invariants` and `metrics`: same as T1 route.
- `constraints`: same as T1 route.
- `additionalPropertiesRequired`: `true`

This route is for vault consumers of mood contracts in implementation repos and binds the same schema and constraints into Tier-2.

### 3.2 eventContract

**Role**

`eventContract` defines region-bound story events, with full invariant coverage and SHCI coupling to regions and personas. It is primarily a Tier-2 vault family.

**Allowed tiers**

- `T2-vault`

**Routes**

1. **T2-vault / HorrorPlace-Atrocity-Seeds**

- `id`: `eventContract.T2-vault.HorrorPlace-Atrocity-Seeds`
- `repo`: `HorrorPlace-Atrocity-Seeds`
- `schemaRef`: `schemas/contracts/event-contract-v1.json`
- `defaultTargetPath`: `events/`
- `registryRef`: `registry/registry-events.ndjson`
- `phase`: `Phase2Contracts`
- `invariants`: full set — `CIC`, `MDI`, `AOS`, `RRM`, `FCF`, `SPR`, `RWF`, `DET`, `HVF`, `LSG`, `SHCI`
- `metrics`: `UEC`, `EMD`, `STCI`, `CDL`, `ARR`
- `constraints`:
  - `sprFromInvariants.v1`
  - `shciCoupling.v1`
- `additionalPropertiesRequired`: `true`

This route anchors all hero `eventContract` artifacts in Atrocity-Seeds and ensures they are SHCI-coupled in a way that respects attached region and persona bundles.

### 3.3 regionContractCard

**Role**

`regionContractCard` is the numeric design surface for hero regions. It is the canonical location where region-level invariant bands and entertainment metrics are declared.

**Allowed tiers**

- `T2-vault`

**Routes**

1. **T2-vault / HorrorPlace-Atrocity-Seeds**

- `id`: `regionContractCard.T2-vault.HorrorPlace-Atrocity-Seeds`
- `repo`: `HorrorPlace-Atrocity-Seeds`
- `schemaRef`: `schemas/contracts/regionContractCard.schema.json`
- `defaultTargetPath`: `cards/regions/`
- `registryRef`: `registry/registry-regions.ndjson`
- `phase`: `Phase2Contracts`
- `invariants`: all 11 invariants — `CIC`, `MDI`, `AOS`, `RRM`, `FCF`, `SPR`, `RWF`, `DET`, `HVF`, `LSG`, `SHCI`
- `metrics`: `UEC`, `EMD`, `STCI`, `CDL`, `ARR`
- `constraints`:
  - `sprFromInvariants.v1`
  - `regionDetCapsByTier.v1` — per-tier DET ceilings.
  - `metricBandsByTier.region.v1` — per-tier UEC/ARR bands for regions.
- `additionalPropertiesRequired`: `true`

This route is where tier-specific horror caps are enforced (e.g. DET upper bounds for public vs vault contexts) and provides a stable target for AI numeric design tools.

### 3.4 seedContractCard

**Role**

`seedContractCard` captures procedural story seeds that link regions, events, personas, and history. It enforces stage-aware UEC/ARR/CDL profiles across narrative stages.

**Allowed tiers**

- `T2-vault`

**Routes**

1. **T2-vault / HorrorPlace-Atrocity-Seeds**

- `id`: `seedContractCard.T2-vault.HorrorPlace-Atrocity-Seeds`
- `repo`: `HorrorPlace-Atrocity-Seeds`
- `schemaRef`: `schemas/contracts/seedContractCard.schema.json`
- `defaultTargetPath`: `seeds/`
- `registryRef`: `registry/registry-seeds.ndjson`
- `phase`: `Phase2Contracts`
- `invariants`: subset tailored to seeds — `CIC`, `MDI`, `AOS`, `RRM`, `FCF`, `SPR`, `LSG`, `DET`, `SHCI`
- `metrics`: `UEC`, `EMD`, `STCI`, `CDL`, `ARR`
- `constraints`:
  - `sequenceStageMetrics.v1` — ensures stage progression over `Outer/Threshold/Locus/Rupture/Fallout`.
  - `shciCoupling.v1`
  - `sprFromInvariants.v1`
- `additionalPropertiesRequired`: `true`

This route makes seeds a first-class contract family that is aware of both narrative stage and SHCI bounds.

---

## 4. How Routing Works for AI Systems

### 4.1 Basic resolution: (objectKind, tier) → route

External AI systems and CHAT_DIRECTOR can treat the routing spine as a pure data oracle. The typical resolution process is:

1. Load `schema-spine-index-v1.json`, `hpc-routing-spine-v1.json`, and the repo manifest.
2. Given an authoring intent (e.g. an `AiAuthoringRequest` or `ai-safe-authoring-contract` entry), read:
   - `objectKind` (e.g. `eventContract`).
   - `tier` (e.g. `T2-vault`).
3. Find the `objectKindEntry` where `name == objectKind`.
4. Filter `routes` for entries where:
   - `tier` matches the requested tier.
   - any additional manifest filters (e.g. target repo) are satisfied.
5. Assert that exactly one route matches; if not, treat this as a routing error.

From the chosen route, the AI system obtains:

- `repo` — the canonical `targetRepo`.
- `schemaRef` — the schema to validate against.
- `defaultTargetPath` — base directory for `destination`.
- `registryRef` — registry that must be updated.
- `invariants` and `metrics` — fields that must appear and be banded in the contract.
- `constraints` — formulas that must hold for the artifact to be valid.
- `phase` — which CHAT_DIRECTOR phase should be responsible for this objectKind.

This lets external systems prepare a fully specified file descriptor before emitting any content, and gives Wandering-entity guards a concrete routing surface to enforce.

### 4.2 Filling invariants and metrics

When generating artifacts, AI tools are expected to:

- Include bands for every invariant listed in `routeEntry.invariants`.
- Include target bands or values for every metric listed in `routeEntry.metrics`.
- Respect ranges defined in the invariants and metrics spines referenced from `schema-spine-index-v1.json`.
- Allow CI and CHAT_DIRECTOR to apply math constraints listed in `routeEntry.constraints` to ensure the combined invariant and metric profile is coherent (e.g. `SPR` derived from other invariants, SHCI bands compatible with region and persona combinations).

In practice, this means authoring envelopes or plans should:

- Limit numeric fields to the invariants and metrics enumerated for the chosen route.
- Declare which constraint versions they obey, when applicable.

### 4.3 Phase-aware routing

The `phase` attribute on each route is designed to align routing with CHAT_DIRECTOR’s multi-phase pipeline. For example:

- `moodContract` in `T1-core/Horror.Place` is tied to `Phase0Schema`, where design-only contracts are authored and frozen.
- `eventContract`, `regionContractCard`, and `seedContractCard` entries in Atrocity-Seeds are tied to `Phase2Contracts`, where rich contract content is authored with full invariant and metric bands.

CHAT_DIRECTOR can:

- Use the routing spine to check that a proposed authoring operation for this phase is legal.
- Reject authoring attempts for objectKinds whose routes do not list the current phase as allowed.
- Drive multi-stage workflows (e.g. first author `moodContract` in Phase0Schema, then use that as input when generating `eventContract` in Phase2Contracts).

---

## 5. Enforcing Uniqueness and CI Behavior

### 5.1 The uniqueness doctrine

The routing spine is designed around a simple doctrine:

> For each `objectKind`, the `(tier, repo)` pair must be unique.

In other words, there must not be two `routeEntry` objects for a given `objectKind` that share the same `(tier, repo)` combination with different `schemaRef` or constraints. This ensures that:

- An `(objectKind, tier)` pair will never be ambiguous about which repo it belongs to.
- A `(objectKind, tier, repo)` triple selects exactly one schema and constraint set.

The JSON Schema does not fully encode this as a structural uniqueness rule because JSON Schema has limited support for cross-field uniqueness, but it is treated as a hard CI invariant.

### 5.2 CI checks for the routing spine

CI for `HorrorPlace-Constellation-Contracts` should:

1. Validate `hpc-routing-spine-v1.json` against `hpc-routing-spine.schema.json` with strict settings (`additionalProperties: false`).
2. Run a custom uniqueness check that, for each `objectKindEntry`:
   - Builds a map keyed by `(tier, repo)`.
   - Fails the build if any key appears more than once.
3. Cross-check invariants and metrics:
   - Every code under `routeEntry.invariants` must exist in the invariants spine.
   - Every code under `routeEntry.metrics` must exist in the entertainment metrics spine.
4. Cross-check formulas:
   - Every constraint id in `routeEntry.constraints` must exist in `mathConstraints`.
   - Every `mathConstraints.spineRef` must resolve into `schema-spine-index-v1.json`.

These checks keep the routing spine aligned with global doctrine and prevent drift between schema, invariants, metrics, and routing data.

### 5.3 Repo-level CI gates

Tier-1 and Tier-2 repos can use the routing spine to enforce their own responsibilities:

- Each repo loads `hpc-routing-spine-v1.json` and its own repo manifest.
- For a given repo, CI checks:
  - Which routes point into this repo.
  - That artifacts in the repo belonging to each `objectKind` live under the `defaultTargetPath` for their route.
  - That there is exactly one `routeEntry` for each `objectKind + tier` that is meant to target this repo.

This prevents accidental duplication of responsibilities where two repos claim the same objectKind/tier combination and ensures that path structures stay consistent.

---

## 6. How CHAT_DIRECTOR and External AI Use the Spine

### 6.1 CHAT_DIRECTOR

Within CHAT_DIRECTOR, the routing spine is used in at least three places:

1. **Planning**

   When building an authoring plan, the director:

   - Reads `objectKind` and `tier` from the request or plan.
   - Calls a routing resolver over `hpc-routing-spine-v1.json` to obtain `{repo, schemaRef, defaultTargetPath, invariants, metrics, constraints, phase}`.
   - Asserts `phase` is compatible with the current pipeline stage.

   If no route or multiple routes are found, the director emits a routing error and does not proceed with authoring.

2. **Validation**

   When validating a planned artifact, the director can:

   - Confirm that `targetRepo` and `targetPath` match the resolved route.
   - Ensure that invariants and metrics in the artifact match those listed in the route.
   - Delegate math checks to constraint-specific validators keyed by the `constraints` list.

3. **AI-safe authoring contracts**

   For `ai-safe-authoring-contract` entries that list `objectKind`, `tier`, `targetRepo`, and `schemaRef`, CHAT_DIRECTOR can:

   - Re-derive the route from the spine.
   - Compare the contract’s triple to the resolved triple.
   - Reject plans whose routing data diverges from the spine, with a structured error pointing at the correct repo and schema.

### 6.2 External AI systems

External AI integrations — including RotCave’s Wandering-entity guards — can treat the routing spine as a public, self-describing guide:

1. Load the routing spine and schema spine.
2. For each requested artifact:
   - Choose an `objectKind` from the known list.
   - Choose a `tier` within `allowedTiers` for that objectKind.
   - Resolve the route using the same procedure as CHAT_DIRECTOR.
3. Use the resolved data to:
   - Set `targetRepo` and `destination` (prefixing with `defaultTargetPath`).
   - Set the `schemaRef` field in the authoring contract or request.
   - Populate invariants and metrics only from the lists permitted for that route.
   - Constrain numeric choices to the bands and formulas referenced by `constraints`.

By respecting the routing spine, external AI systems can generate artifacts that:

- Are correctly routed into the constellation.
- Obey the same schema, invariant, and metric contracts as internal tools.
- Can be safely validated and promoted by existing CI and governance layers.

---

## 7. Extension and Versioning

The routing spine is explicitly versioned via `routingSpineVersion`. As new objectKinds, tiers, repos, or formulas are added, the evolution strategy is:

- Add new `mathConstraints` entries with incremented versions (e.g. `sprFromInvariants.v2`) rather than mutating existing ones.
- Introduce new routes or objectKinds under a new routing spine version (e.g. `1.1.0`) while keeping older versions available for historical artifacts.
- Update CHAT_DIRECTOR and authoring contracts to reference the routing spine version they are compatible with.

Because the routing spine is purely structural and references formula specs by URI, existing artifacts remain valid as long as their schemas and constraint versions remain in the catalog. This allows the system to grow without breaking older contract families or authoring pipelines.

---

## 8. Next Steps

With `hpc-routing-spine.schema.json` and `hpc-routing-spine-v1.json` in place, the next natural steps are:

- Implement a small Rust or Lua routing resolver that wraps the JSON structures and exposes `resolve_route(objectKind, tier) → { repo, schemaRef, defaultTargetPath, invariants, metrics, constraints, phase }`.
- Wire this resolver into:
  - `hpc-chat-director` for planning and validation.
  - Wandering-entity guards and RotCave orchestration modules for proactive, spine-aware routing of AI-generated artifacts.
- Add CI jobs that enforce uniqueness of `(objectKind, tier, repo)` and cross-check invariants, metrics, and formulas against the existing spines.

Once those pieces are in place, the routing spine becomes the authoritative index for all contract routing decisions in the constellation.
