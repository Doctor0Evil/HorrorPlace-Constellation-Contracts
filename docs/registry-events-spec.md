# Event Registry Specification (registry-events.v1)

This document defines the Tier‑1 Event registry format for the HorrorPlace constellation and explains how `registry-events.v1.json` is used to validate NDJSON lines, join Events to Regions and Seeds, and wire Dead‑Ledger and intensity/metric profiles.

The intent is to give authors, CI, and orchestrators a single, stable reference for what an “Event registry line” means and how it participates in the wider routing spine and constellation graph.

---

## 1. Purpose and scope

The Event registry is the authoritative index of Event contract cards visible to AI tools, orchestrators, and runtime selectors.

It answers four questions:

- Which Event contracts exist and where are their contract files on disk?
- What routing triple (objectKind, tier, target repo) and safety envelope applies to each Event?
- How do Events attach to Regions, Seeds, and styles, so the constellation graph can be validated and navigated?
- When are Dead‑Ledger proofs and intensity / DET bands required for a given Event?

This spec covers:

- The JSON Schema at `schemas/registry/registry-events.v1.json`.
- NDJSON authoring and validation rules.
- Join semantics with region, seed, style, and Dead‑Ledger surfaces.
- CI expectations for Constellation‑Contracts and downstream repos.

It does not define the Event contract card schema itself; that is handled by the Event contract schema in `schemas/core` or `schemas/contracts`.

---

## 2. Schema overview

The registry schema is defined in `schemas/registry/registry-events.v1.json` and follows the same pattern as other Tier‑1 registries:

- Top‑level object with:
  - `schema`: fixed URI identifying this schema.
  - `version`: semantic version of the registry document.
  - `events`: array of Event registry entries.

Each `events[]` item is a `registryEntry` that can also be used as a standalone NDJSON line. CI and tooling validate each line against `#/definitions/registryEntry`.

### 2.1 Top‑level fields

- `schema`  
  Fixed string: `https://horror.place/schemas/registry/registry-events.v1.json`.  
  Validators use this to select the correct schema version.

- `version`  
  Semantic version of the registry document, for example `1.0.0` or `1.1.0-rc1`.  
  CI can use this to enforce ordered migrations.

- `events`  
  Array of `registryEntry` objects. In NDJSON mode, each line corresponds to one such entry.

### 2.2 Per‑event registry entry

Each `registryEntry` describes exactly one Event contract card and its routing/yield properties.

Core identity and routing:

- `eventId`  
  Stable identifier for the Event, used as the join key across the constellation.  
  Examples: `elevators-haunted-basement.v1`, `ritual-locusrupture-01`.  
  Constrained to lowercase letters, digits, dots, underscores, and hyphens to remain NDJSON and filename friendly.

- `path`  
  Repository‑relative path to the Event contract JSON file.  
  Example: `contracts/events/elevators-haunted-basement.v1.json`.  
  Orchestrators combine this with repo manifests to locate the contract.

- `hash`  
  Content hash of the Event contract, usually SHA‑256 hex (32–64 characters).  
  Used for reproducibility, Dead‑Ledger proofs, and registry sanity checks.

- `objectKind`  
  Spine‑defined object kind, typically `eventContractCard` (or a compatible variant).  
  Routing code uses this together with `tier` to pick the target repo and path family.

- `tier`  
  Governance tier for the Event: `Tier1Public`, `Tier2Internal`, or `Tier3Vault`.  
  Determines which repos and orchestrators are allowed to host and surface the Event.

Safety and intensity:

- `safetyTier`  
  Safety classification aligned with Charter and engine policies: `SafetyTier1`, `SafetyTier2`, or `SafetyTier3`.  
  This is orthogonal to `tier` and is used by selectors and Dead‑Ledger to enforce exposure bands.

- `intensityBand`  
  Integer band in the 0–10 domain describing the coarse horror intensity of the Event (aligned with entertainment‑metrics spine bands).  
  Higher bands may require Dead‑Ledger attestations.

- `deadLedgerRef` (optional)  
  Dead‑Ledger reference string (or `null`) that attests publication‑time compliance for high‑intensity Events.  
  CI policy usually requires a non‑null `deadLedgerRef` when `intensityBand` exceeds a configured threshold, or when `safetyTier` indicates constrained content.

Consent and join fields:

- `consentTier` (optional but recommended)  
  Minimal consent/entitlement tier required to expose the Event in runtime (1–3).  
  Downstream runtime selectors intersect this with session consent and Dead‑Ledger proofs.

- `regionIds` (optional)  
  Array of region IDs (from `registry-regions.v1.json`) where this Event is valid.  
  This is the primary join key for `region → events` relationships.  
  In Atrocity‑Seeds and related repos, region contract cards and PCG seeds cross‑reference these IDs.

- `seedIds` (optional)  
  Array of Seed IDs (typically defined in Atrocity‑Seeds) that can trigger or host this Event.  
  This enables `seed → events` joins and CI checks ensuring that seeds only bind to Events within compatible intensity and safety bands.

- `styleIds` (optional)  
  Array of style contract IDs (e.g., audio landscape, atmospheric grading, gore assembly styles) referenced by the Event.  
  Used by style governance and GXI caps to ensure that Events only consume allowed styles for their `safetyTier`, `intensityBand`, and `consentTier`.

Metric and intensity profiles:

- `metricProfileId` (optional)  
  Identifier for an entertainment‑metrics profile describing intended UEC/EMD/STCI/CDL/ARR envelopes for this Event.  
  This should match a profile defined in metrics or ritual/experience envelope schemas.

- `detBand` (optional)  
  Object with `{ "min": number, "max": number }` in the 0.0–1.0 domain describing the Event‑level Dread Exposure band.  
  Used by CI, Dead‑Ledger policies, and run‑time gates to enforce DET budgets.

- `intensityProfileId` (optional)  
  Identifier for a cross‑stage audiovisual intensity envelope profile reused across multiple Events.  
  Typically points at a profile defined in ritual or experience‑envelope contracts.

Lifecycle and metadata:

- `status`  
  Lifecycle state of the Event registry record: `active`, `deprecated`, `experimental`, or `draft`.  
  Tools and orchestrators may treat non‑`active` entries as non‑deployable or lab‑only.

- `tags` (optional)  
  Free‑form tags for search and routing, e.g., `corridor`, `boss`, `systemic`, `ritual-bound`, `lab-only`.

- `notes` (optional)  
  Short human‑readable note for operators and designers; ignored by CI.

---

## 3. NDJSON authoring and validation

The registry supports two concrete encodings:

1. **JSON document**  
   A full JSON object with top‑level `schema`, `version`, and `events` array.  
   Used for bulk updates and manual inspection.

2. **NDJSON stream**  
   Each line is a standalone `registryEntry` object.  
   This is the preferred format for telemetry‑style pipelines and incremental updates.

### 3.1 JSON mode

In JSON mode, all entries are nested under:

```json
{
  "schema": "https://horror.place/schemas/registry/registry-events.v1.json",
  "version": "1.0.0",
  "events": [
    { "...": "registryEntry #1" },
    { "...": "registryEntry #2" }
  ]
}
```

CI validates:

- The top‑level object against `registry-events.v1.json`.
- Each `events[]` item against `#/definitions/registryEntry`.
- Global invariants such as uniqueness of `eventId` across the array.

### 3.2 NDJSON mode

In NDJSON mode, a file such as `registries/events.ndjson` (exact path policy is controlled by manifests) contains one JSON object per line:

```json
{"eventId":"elevators-haunted-basement.v1","path":"contracts/events/elevators-haunted-basement.v1.json","hash":"...","objectKind":"eventContractCard","tier":"Tier2Internal","safetyTier":"SafetyTier2","intensityBand":7,"deadLedgerRef":"deadledger://...","consentTier":2,"regionIds":["facility.sublevel-b1"],"seedIds":["seed-elevator-fall-01"],"styleIds":["atmo-industrial-drone","sfx-cable-strain"],"status":"active"}
{"eventId":"corridor-flicker-loop.v1","path":"contracts/events/corridor-flicker-loop.v1.json","hash":"...","objectKind":"eventContractCard","tier":"Tier1Public","safetyTier":"SafetyTier1","intensityBand":3,"deadLedgerRef":null,"consentTier":1,"regionIds":["facility.main-corridors"],"styleIds":["atmo-low-hum","vfx-light-flicker-soft"],"status":"active"}
```

Validation rules:

- Each line must parse as a JSON object and validate against `#/definitions/registryEntry`.
- No top‑level `schema` or `version` fields are present in NDJSON mode; those are carried by the schema and tooling context.
- `eventId` values must be unique across the entire NDJSON file, not just per line.

CI workflows in Constellation‑Contracts typically:

- Load the schema from `schemas/registry/registry-events.v1.json`.
- Validate:
  - JSON registries at `registries/events.json` or similar, if present.
  - NDJSON registries at `registries/events.ndjson`, if configured.
- Enforce additional out‑of‑schema invariants:
  - Uniqueness of `eventId`.
  - No `deprecated` Events referenced from public Region or Seed registries (unless explicitly allowed by policy).

---

## 4. Join semantics and graph links

The Event registry is one layer of the constellation graph. It must join cleanly with Regions, Seeds, styles, and governance/proof surfaces.

### 4.1 Regions → Events

`regionIds` in `registry-events` lines must reference valid `regionId` values from the Regions registry (e.g., `registry-regions.v1.json`).

- **Authoring rules**:
  - Authors should only list region IDs that exist and are appropriate for the Event’s invariant bands and intensity profile.
  - For purely system‑wide Events (e.g., global pacing pulses), `regionIds` may be omitted or left empty, and selectors will rely on other bindings (e.g., Seeds, rituals).

- **CI checks**:
  - For each `event.registryEntry.regionIds[i]`, verify that a corresponding region registry entry exists.
  - Optionally check that the Event’s `intensityBand` and `detBand` fall within allowed ranges for each referenced Region’s bands (if Regions expose such bands).

### 4.2 Seeds → Events

`seedIds` reference Seeds defined in Atrocity‑Seeds (or similar vault repos). They let PCG and runtime treat Events as effects or overlays attached to Seeds.

- **Authoring rules**:
  - When a Seed contract card names an Event (e.g., as a candidate manifestation), that Event should in turn list the Seed ID in `seedIds`.
  - Use this field to make bidirectional relationships explicit for graph linters.

- **CI checks**:
  - Optionally enforce that any Seed naming an Event must appear in that Event’s `seedIds`, and vice versa.
  - Enforce safety compatibility: Seed safety tier and `intensityBand` must not exceed the Event’s or Region’s configured bounds.

### 4.3 Events → styles

`styleIds` reference style contracts, typically defined using style schemas in Constellation‑Contracts and implemented in Atrocity‑Seeds, Spectral‑Foundry, or Obscura‑Nexus.

- **Authoring rules**:
  - Authors should use style IDs instead of descriptive text when describing audio, VFX, or gore implications.
  - For high‑band Events, style IDs and their GXI/intensity profiles must remain within the Event’s `safetyTier`, `intensityBand`, `consentTier`, and any Dead‑Ledger proof envelopes.

- **CI checks**:
  - For each `styleId`, verify that a style registry entry exists.
  - Ensure that style GXI/intensity bands do not exceed the Event’s allowed bands.
  - When Dead‑Ledger gore governance is active, verify that styles requiring higher proof bands are only used when `deadLedgerRef` and `consentTier` permit them.

### 4.4 Events → Dead‑Ledger + consent

`deadLedgerRef`, `tier`, `safetyTier`, `intensityBand`, `detBand`, and `consentTier` together connect Event registry entries to governance and proof layers.

- **Authoring rules**:
  - Events at low `intensityBand` and `SafetyTier1` can omit `deadLedgerRef` and optionally `detBand`.
  - Events at higher bands must declare:
    - A non‑null `deadLedgerRef` pointing at a Dead‑Ledger proof envelope.
    - A `consentTier` consistent with the proof envelope and session consent.
    - A `detBand` that respects global DET caps.

- **CI checks**:
  - If `intensityBand` ≥ configured threshold (e.g., 7), require `deadLedgerRef` to be non‑null and syntactically valid.
  - Ensure `consentTier` ≤ session‑level consent tiers allowed by manifests and policies.
  - Enforce that `detBand.max` does not exceed global DET caps for the Event’s `tier` and `safetyTier`.

---

## 5. CI expectations

Constellation‑Contracts CI must treat the Event registry as a first‑class surface for correctness and governance.

Typical CI pipeline responsibilities:

1. **Schema validation**  
   - Validate all Event registries (JSON and/or NDJSON) against `registry-events.v1.json`.

2. **Uniqueness and routing**  
   - Ensure:
     - `eventId` is unique across a registry file.
     - `objectKind` is compatible with the routing spine and repo manifests.
     - `tier` is allowed for the target repo.

3. **Join integrity**  
   - Verify:
     - `regionIds` exist in Region registries.
     - `seedIds` exist in Seed registries (where available).
     - `styleIds` exist in Style registries.

4. **Governance and Dead‑Ledger**  
   - Enforce:
     - Dead‑Ledger requirement for high‑intensity Events (non‑null `deadLedgerRef`).
     - `safetyTier` and `consentTier` consistency with safety policies.
     - `detBand` within allowed DET ranges.

5. **Lifecycle hygiene**  
   - Block:
     - `deprecated` Events from being referenced in new Regions or Seeds (unless explicitly allowed by policy).
   - Optionally warn on:
     - `experimental` Events linked into Tier‑1 public Regions.

---

## 6. Authoring guidelines

To keep Event registries predictable and machine‑friendly:

- **Use stable, descriptive IDs**  
  Choose `eventId` values that encode both domain and version, e.g., `facility-elevator-drop.v1`, `ritual-locusrupture-01.v1`.

- **Keep `path` and `hash` in sync**  
  Update `hash` whenever the underlying Event contract file changes. CI should reject mismatches.

- **Prefer explicit joins**  
  When an Event is tightly coupled to specific Regions, Seeds, or styles, record those relationships in `regionIds`, `seedIds`, and `styleIds`. This makes graph linting and tooling safer.

- **Respect governance bands**  
  Before assigning high `intensityBand` values, make sure the Event contract, Regions, Seeds, and Telemetry/Dead‑Ledger policies agree on DET caps, GXI bands, and consent tiers.

- **Use `tags` and `notes` for humans**  
  Keep `tags` short and consistent (`systemic`, `boss`, `corridor`, `lab-only`), and reserve `notes` for brief operator/design hints.

---

## 7. Future extensions

Potential extensions for future versions of the Event registry schema include:

- Adding explicit `graphNodeKind` and `graphEdges` fields to mirror a constellation‑graph index directly in the registry.
- Adding telemetry profile IDs to connect Events to specific NDJSON telemetry families and expected metric envelopes.
- Adding `ritualIds` to make Event–Ritual joins explicit where Seed‑Ritual contracts drive pacing.

Any such extensions should be added in a new schema version (e.g., `registry-events.v2`), with clear migration guidance and CI checks to prevent drift.

---

This specification should be kept in lockstep with `schemas/registry/registry-events.v1.json`, Region/Seed/Style registry specs, and Dead‑Ledger governance docs so that AI‑assisted authoring, CI, and runtime selectors can rely on a single, coherent contract for Event registries.
