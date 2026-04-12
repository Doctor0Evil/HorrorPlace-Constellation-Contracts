# Region Registry Specification (registry-regions.v1)

This document defines the Tier‑1 Region registry format for the HorrorPlace constellation and explains how `registry-regions.v1.json` is used to validate NDJSON lines, anchor Regions as primary graph nodes, and join them to Events, Seeds, and governance/intensity profiles.

The goal is to give authors, CI, and orchestrators a single reference for what a “Region registry line” means and how it participates in the wider routing spine and constellation graph.

---

## 1. Purpose and scope

The Region registry is the authoritative index of Region contract cards visible to AI tools, orchestrators, and runtime selectors.

It answers four questions:

- Which Region contracts exist and where are their contract files on disk?
- What routing triple (objectKind, tier, target repo) and safety envelope applies to each Region?
- How do Regions act as anchors for Events, Seeds, personas, and styles?
- How do Regions expose invariant and intensity bands used by governance, Dead‑Ledger, and PCG?

This spec covers:

- The intended JSON Schema at `schemas/registry/registry-regions.v1.json`.
- NDJSON authoring and validation rules.
- Join semantics with Event, Seed, style, and invariants/metrics spines.
- CI expectations for Constellation‑Contracts and downstream repos.

It does not define the Region contract card schema itself; that is handled by Region contract schemas in `schemas/core` or `schemas/contracts`.

---

## 2. Schema overview

The Region registry schema follows the same structural pattern as the Event registry:

- Top‑level object with:
  - `schema`: fixed URI identifying this schema.
  - `version`: semantic version of the registry document.
  - `regions`: array of Region registry entries.

Each `regions[]` item is a `registryEntry` that can also be used as a standalone NDJSON line. CI and tooling validate each line against `#/definitions/registryEntry`.

### 2.1 Top‑level fields

- `schema`  
  Fixed string: `https://horror.place/schemas/registry/registry-regions.v1.json` (or equivalent final URI).  
  Validators use this to select the correct schema version.

- `version`  
  Semantic version of the registry document, for example `1.0.0` or `1.1.0-rc1`.  
  CI can use this to enforce ordered migrations.

- `regions`  
  Array of `registryEntry` objects. In NDJSON mode, each line corresponds to one such entry.

### 2.2 Per‑region registry entry

Each `registryEntry` describes exactly one Region contract card and its routing/yield properties.

Core identity and routing:

- `regionId`  
  Stable identifier for the Region, used as the primary join key across the constellation.  
  Examples: `facility.sublevel-b1`, `town.square.north`, `corridor.loop-a.v1`.  
  Constrained to lowercase letters, digits, dots, underscores, and hyphens to remain NDJSON and filename friendly.

- `path`  
  Repository‑relative path to the Region contract JSON file.  
  Example: `contracts/regions/facility.sublevel-b1.v1.json`.  
  Orchestrators combine this with repo manifests to locate the contract.

- `hash`  
  Content hash of the Region contract, usually SHA‑256 hex (32–64 characters).  
  Used for reproducibility, Dead‑Ledger proofs, and registry sanity checks.

- `objectKind`  
  Spine‑defined object kind, typically `regionContractCard` (or a compatible variant).  
  Routing code uses this together with `tier` to pick the target repo and path family.

- `tier`  
  Governance tier for the Region: `Tier1Public`, `Tier2Internal`, or `Tier3Vault`.  
  Determines which repos and orchestrators are allowed to host and surface the Region.

Safety, invariants, and intensity:

- `safetyTier`  
  Safety classification aligned with Charter and engine policies: `SafetyTier1`, `SafetyTier2`, or `SafetyTier3`.  
  This is orthogonal to `tier` and governs default exposure caps for content under the Region.

- `intensityBand`  
  Integer band in the 0–10 domain describing the coarse horror intensity bound for the Region.  
  Seeds and Events inside this Region are expected to sit within or below this band.

- `deadLedgerRef` (optional)  
  Dead‑Ledger reference string (or `null`) that attests publication‑time compliance for high‑intensity or sensitive Regions.  
  CI policy usually requires a non‑null `deadLedgerRef` when `intensityBand` or other invariant bands exceed configured public thresholds.

- `consentTier` (optional but recommended)  
  Minimal consent/entitlement tier required to expose this Region in live builds (1–3).  
  Runtime selectors intersect this with session consent and Dead‑Ledger proofs.

- `invariantProfileId` (optional)  
  Identifier for an invariant profile describing canonical CIC/AOS/DET/HVF/LSG/SHCI bands for this Region.  
  Typically references invariants spine or Black‑Archivum bundles.

- `detBand` (optional)  
  Object with `{ "min": number, "max": number }` in the 0.0–1.0 domain describing the Region‑level Dread Exposure band.  
  Seeds and Events must not push session DET beyond this Region’s caps without governance overrides.

- `metricProfileId` (optional)  
  Identifier for a metrics profile describing intended UEC/EMD/STCI/CDL/ARR envelopes for experiences anchored in this Region.

Topology, class, and joins:

- `regionClass` (optional)  
  Coarse class label for the Region, e.g., `corridor`, `atrium`, `sanctuary`, `threshold`, `locus`.  
  PCG and selectors can use this to select appropriate Seeds and Events.

- `parentRegionId` (optional)  
  ID of a parent Region when Regions are nested (e.g., a corridor inside a facility or wing).  
  Used to inherit invariant/metric bands and for hierarchical graph checks.

- `childRegionIds` (optional)  
  Array of child Region IDs, when Region hierarchies are explicitly declared.  
  This is optional; many graphs will derive child links from `parentRegionId` instead.

- `eventIds` (optional)  
  Optional list of Event IDs typically present or eligible within this Region.  
  This is the reverse direction of `regionIds` in the Event registry and can be used to check bidirectional consistency.

- `seedIds` (optional)  
  Optional list of Seed IDs that are anchored to this Region.  
  Atrocity‑Seeds and PCG modules often treat this list as a primary anchor for map generation and seeding.

- `styleProfileId` (optional)  
  Identifier of a style or atmosphere profile associated with this Region (e.g., audio landscape, grading, spectral style sets).

Lifecycle and metadata:

- `status`  
  Lifecycle state of the Region registry record: `active`, `deprecated`, `experimental`, or `draft`.  
  Tools and orchestrators may treat non‑`active` entries as non‑deployable or lab‑only.

- `tags` (optional)  
  Free‑form tags for search and routing, e.g., `hub`, `sanctuary`, `corridor`, `lab-only`, `ritual-site`.

- `notes` (optional)  
  Short human‑readable note for operators and designers; ignored by CI.

---

## 3. NDJSON authoring and validation

The registry supports two concrete encodings, mirroring Events:

1. **JSON document**  
   A full JSON object with top‑level `schema`, `version`, and `regions` array.  
   Used for bulk updates and manual inspection.

2. **NDJSON stream**  
   Each line is a standalone `registryEntry` object.  
   This is the preferred format for telemetry‑style pipelines and incremental updates.

### 3.1 JSON mode

In JSON mode, all entries are nested under:

```json
{
  "schema": "https://horror.place/schemas/registry/registry-regions.v1.json",
  "version": "1.0.0",
  "regions": [
    { "...": "registryEntry #1" },
    { "...": "registryEntry #2" }
  ]
}
```

CI validates:

- The top‑level object against `registry-regions.v1.json`.
- Each `regions[]` item against `#/definitions/registryEntry`.
- Global invariants such as uniqueness of `regionId` across the array.

### 3.2 NDJSON mode

In NDJSON mode, a file such as `registries/regions.ndjson` (exact path policy is controlled by manifests) contains one JSON object per line:

```json
{"regionId":"facility.sublevel-b1","path":"contracts/regions/facility.sublevel-b1.v1.json","hash":"...","objectKind":"regionContractCard","tier":"Tier2Internal","safetyTier":"SafetyTier2","intensityBand":6,"deadLedgerRef":"deadledger://...","consentTier":2,"regionClass":"sublevel","status":"active"}
{"regionId":"facility.main-corridors","path":"contracts/regions/facility.main-corridors.v1.json","hash":"...","objectKind":"regionContractCard","tier":"Tier1Public","safetyTier":"SafetyTier1","intensityBand":3,"deadLedgerRef":null,"consentTier":1,"regionClass":"corridor","status":"active"}
```

Validation rules:

- Each line must parse as a JSON object and validate against `#/definitions/registryEntry`.
- No top‑level `schema` or `version` fields are present in NDJSON mode; those are carried by the schema and tooling context.
- `regionId` values must be unique across the entire NDJSON file, not just per line.

CI workflows in Constellation‑Contracts typically:

- Load the schema from `schemas/registry/registry-regions.v1.json`.
- Validate:
  - JSON registries at `registries/regions.json` or similar, if present.
  - NDJSON registries at `registries/regions.ndjson`, if configured.
- Enforce additional out‑of‑schema invariants:
  - Uniqueness of `regionId`.
  - No `deprecated` Regions referenced as parents of `active` Regions without explicit deprecation policy.

---

## 4. Join semantics and graph structure

The Region registry defines foundational graph nodes that other registries and contract cards attach to.

### 4.1 Regions as primary anchors

Regions are the primary spatial/invariant anchors for:

- Events (`registry-events` uses `regionIds`).
- Seeds (Atrocity‑Seeds contracts usually carry a `regionId`).
- Personas and spectral entities (Spectral‑Foundry and Codebase‑of‑Death often reference Regions).
- Style and atmosphere profiles.

Authoring and CI must treat Region registry entries as the root of these relationships.

### 4.2 Regions → Events

The Event registry’s `regionIds` field references `regionId` values from the Region registry.

- **Authoring rules**:
  - When a Region is known to host specific Events, authors may list those Event IDs in the Region’s `eventIds` array.
  - This is optional but recommended for critical encounters and rituals.

- **CI checks**:
  - For each Event entry with `regionIds`, verify that the referenced Regions exist.
  - Optionally check bidirectional consistency:
    - If a Region lists `eventIds`, each listed Event should exist and include that Region in its `regionIds` (or be explicitly marked as global/systemic).

### 4.3 Regions → Seeds

Seeds in Atrocity‑Seeds typically specify a `regionId` indicating where they can fire or be instantiated.

- **Authoring rules**:
  - The Region registry may list `seedIds` as a convenience and for graph introspection.
  - Seed contract cards must reference `regionId` directly; the Region registry is an index, not the only source of truth.

- **CI checks**:
  - Ensure that Seeds referencing a `regionId` only do so when that Region exists and is not `deprecated` (unless explicitly allowed by policy).
  - Optional lints:
    - If a Region lists a Seed ID in `seedIds`, verify that the Seed’s `regionId` matches.
    - Enforce that Seeds’ `intensityBand` and DET bands sit inside the Region’s `intensityBand` and `detBand`.

### 4.4 Regions → invariants and metrics

Regions are the natural place to anchor invariant bands and metric envelopes. The `invariantProfileId`, `metricProfileId`, `detBand`, and `intensityBand` fields connect Region registry entries to:

- Invariants spine: CIC, AOS, RRM, FCF, SPR, RWF, DET, HVF, LSG, SHCI.
- Metrics spine: UEC, EMD, STCI, CDL, ARR.

Authoring and CI expectations:

- Region‑level bands are parents in the narrowing discipline; Seeds, Events, and rituals must define bands that are subsets of their Region’s bands.
- CI should enforce:
  - `detBand.max` does not exceed global DET caps for the Region’s `tier` and `safetyTier`.
  - Child objects (Seeds, Events) do not specify `intensityBand` or DET caps above their Region’s.

### 4.5 Regions → Dead‑Ledger and consent

`deadLedgerRef`, `tier`, `safetyTier`, `intensityBand`, `detBand`, and `consentTier` together connect Region registry entries to governance and proof layers.

- **Authoring rules**:
  - Public, low‑intensity Regions may omit `deadLedgerRef` and `detBand`.
  - High‑intensity or research Regions must:
    - Carry a non‑null `deadLedgerRef` referencing a Dead‑Ledger proof envelope.
    - Declare `consentTier` consistent with the proof envelope and session consent.

- **CI checks**:
  - If `intensityBand` or `detBand.max` exceeds configured thresholds for public content, require `deadLedgerRef` and `consentTier`.
  - Enforce that child Events and Seeds with higher bands do not appear under Regions lacking sufficient `deadLedgerRef` and consent.

---

## 5. CI expectations

Constellation‑Contracts CI must treat the Region registry as a foundational surface.

Typical CI pipeline responsibilities:

1. **Schema validation**  
   - Validate all Region registries (JSON and/or NDJSON) against `registry-regions.v1.json`.

2. **Uniqueness and routing**  
   - Ensure:
     - `regionId` is unique across a registry file.
     - `objectKind` is compatible with the routing spine and repo manifests.
     - `tier` is allowed for the target repo.

3. **Hierarchy sanity**  
   - When `parentRegionId` is present:
     - Verify that the parent exists.
     - Optionally enforce no cycles in Region parent/child relationships.
   - When `childRegionIds` is present:
     - Verify that each child exists and has this Region as its `parentRegionId` (or is compatible by policy).

4. **Join hygiene**  
   - Ensure:
     - Any `eventIds` are valid Event IDs in the Event registry.
     - Any `seedIds` are valid Seed IDs in Seed registries.

5. **Governance and Dead‑Ledger**  
   - Enforce:
     - Dead‑Ledger requirement for high‑intensity Regions.
     - `safetyTier` and `consentTier` consistency with safety policies.
     - `detBand` and `intensityBand` within allowed ranges for the Region’s `tier`.

6. **Lifecycle hygiene**  
   - Block:
     - New Seeds or Events binding to `deprecated` Regions unless explicitly allowed.
   - Optionally warn:
     - When `experimental` Regions appear in Tier‑1 public registries.

---

## 6. Authoring guidelines

To keep Region registries predictable and machine‑friendly:

- **Use stable, hierarchical IDs**  
  Encode structure in `regionId` using dotted segments where appropriate, e.g., `facility.sublevel-b1`, `facility.sublevel-b1.corridor-west`.

- **Keep `path` and `hash` in sync**  
  Update `hash` whenever the underlying Region contract file changes. CI should reject mismatches.

- **Define Region classes explicitly**  
  Use `regionClass` to clarify the Region’s role (`hub`, `corridor`, `sanctuary`, `locus`, etc.) so PCG and selectors can make better choices.

- **Respect narrowing discipline**  
  Choose Region‑level bands (`intensityBand`, `detBand`, invariant/metric profiles) with the expectation that Seeds, Events, and rituals will define stricter subsets, not broader ones.

- **Use `tags` and `notes` for humans**  
  Keep `tags` short and consistent (`hub`, `ritual-site`, `lab-only`), and reserve `notes` for brief operator/design hints.

---

## 7. Relationship to other registry specs

The Region registry spec is designed to be parallel to:

- **Event registry spec (`registry-events-spec.md`)**  
  Regions and Events cross‑reference each other via `regionIds` and `eventIds`, and share intensity/governance semantics.

- **Style and Persona registries**  
  Regions act as anchors for style and persona profiles in related registries, and share the same `tier`, `safetyTier`, and band semantics.

Keeping these documents aligned ensures that:

- Join keys (`regionId`, `eventId`, `seedId`, `styleId`) are consistently defined.
- Bands and governance fields (`intensityBand`, `detBand`, `safetyTier`, `consentTier`, `deadLedgerRef`) behave identically across registries.
- AI‑assisted authoring, CI, and runtime selectors can use shared logic for validation and routing.

---

Future updates to the Region registry schema should be versioned explicitly (e.g., `registry-regions.v2`) and accompanied by migration plans and CI checks, so existing Region entries remain stable while new fields or invariants are introduced.
