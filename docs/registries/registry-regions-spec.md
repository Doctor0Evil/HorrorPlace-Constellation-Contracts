# Registry Regions Specification

This document defines the structure, semantics, and usage rules for the `regions` registry in the HorrorPlace constellation. Regions are geographic or conceptual tiles that serve as the primary spatial unit for horror content placement.

## Schema Reference

All `regions` registry entries must conform to `schemas/registry/registry-regions.v1.json`, which extends `schemas/registry/registry-entry-base.v1.json` with region-specific fields.

Canonical `$id`: `https://horrorplace.constellation/schemas/registry/registry-regions.v1.json`

## Required Fields (Beyond Base)

| Field | Type | Description |
|-------|------|-------------|
| `regionType` | enum | One of: `"geographic"`, `"conceptual"`, `"liminal"`, `"industrial"`, `"natural"`. |
| `coordinates` | object | Bounding box or centroid: `{"lat":45.123,"lon":60.456,"radiusKm":5.0}` or `{"gridX":12,"gridY":8,"gridSize":"1km"}`. |
| `invariantProfile` | object | Map of invariant name → canonical value or range for this region (e.g., `{"CIC":0.88,"AOS":7.5,"DET":{"min":3,"max":9}}`). |
| `biomeTags` | array[string] | Lowercase, hyphenated tags describing environment (e.g., `["salt-flat","abandoned-industrial","post-soviet"]`). |

## Optional Fields

| Field | Type | Description |
|-------|------|-------------|
| `historicalFootprint` | object | Structured data about real or in-universe history: `{"disasters":["aral-sea-collapse"],"events":["bio-weapon-trial-1992"],"myths":["vanishing-convoy"]}`. |
| `liminalStressGradient` | number [0–1] | LSG (Liminal Stress Gradient) value for threshold-based event placement. |
| `spectralProbabilityIndex` | number [0–1] | SPI (Spectral Plausibility Index) for grounding apparitions in historical plausibility. |
| `ritualResidueMap` | array[string] | List of `artifactid` references to RRM (Ritual Residue Map) layers for occult mechanics. |
| `folkloricConvergenceFactor` | number [0–1] | FCF (Folkloric Convergence Factor) for narrative pressure and boss-zone designation. |
| `prismMetaRef` | string | Reference to a `prismMeta` document describing region linkage and dependency graph. |

## Region Type Semantics

### `geographic`
Real-world or realistic fictional locations with defined coordinates and biome data. Used for grounded horror tied to actual historical trauma (e.g., Aral Sea, Chernobyl Exclusion Zone).

### `conceptual`
Abstract or metaphysical spaces: "the space between memories," "the echo of a vanished town." No fixed coordinates; placement is narrative-driven. Requires high `folkloricConvergenceFactor`.

### `liminal`
Threshold spaces: doorways, causeways, forest edges. Defined by high `liminalStressGradient`. Ideal for stalking encounters and audio crossfades.

### `industrial` / `natural`
Subtypes of `geographic` with specialized `biomeTags`. `industrial` regions favor `environmental-hazard` events; `natural` regions favor `spectral-manifestation` tied to folklore.

## Invariant Profile Format

`invariantProfile` binds a region to the canonical invariants defined in `invariants-spine.v1.json`:

```json
{
  "CIC": 0.88,
  "AOS": 7.5,
  "DET": {"min": 3, "max": 9},
  "SHCI": {">=": 0.6}
}
```

- Scalar values fix the invariant for the region.
- Range objects (`min`/`max` or comparison operators) allow runtime variation within bounds.
- All values must fall within canonical ranges; the linter enforces this.

## Validation Rules

The registry linter (`hpc-lint-registry.py`) enforces:

1. `coordinates` uses either lat/lon/radius or grid-based system consistently.
2. `invariantProfile` keys match invariant names in `invariants-spine.v1.json`.
3. All `invariantProfile` values fall within canonical ranges.
4. If `spectralProbabilityIndex > 0.7`, then `tier` must be `"vault"` or `"lab"`.
5. `biomeTags` are lowercase, hyphenated, and from the approved list in `docs/overview/design-principles.md`.

## Example Entry

```json
{"id":"REG-ARAL-0001","schemaref":"https://horrorplace.constellation/schemas/registry/registry-regions.v1.json","deadledgerref":"zkp:sha256:reg001...","artifactid":"ipfs:bafyreg001...","createdAt":"2026-01-15T03:00:00Z","status":"active","regionType":"geographic","coordinates":{"lat":45.123,"lon":60.456,"radiusKm":5.0},"invariantProfile":{"CIC":0.92,"AOS":8.1,"DET":{"min":4,"max":9}},"biomeTags":["salt-flat","abandoned-industrial","post-soviet","biohazard"],"historicalFootprint":{"disasters":["aral-sea-collapse"],"events":["bio-weapon-trial-1992"],"myths":["vanishing-convoy"]},"liminalStressGradient":0.65,"spectralProbabilityIndex":0.88,"ritualResidueMap":["ipfs:bafyrrm001..."],"folkloricConvergenceFactor":0.79,"prismMetaRef":"prism:reg-aral-0001"}
```

## Cross-Registry Linking

Regions are referenced by events, styles, and personas via ID. The spine index maintains a reverse map: given a region ID, tools can list all events valid in that region, all styles applicable, etc. This enables AI agents to propose coherent, location-aware content without guessing relationships.

## Telemetry and Runtime Integration

At runtime, the engine loads the region's `invariantProfile` and `biomeTags` to configure:

- Environmental audio mixes (e.g., wind + distant machinery for `industrial` + `salt-flat`).
- AI behavior weights (e.g., higher aggression in high-CIC regions).
- Procedural asset selection (e.g., rusted machinery decals in `abandoned-industrial` zones).

Telemetry emitted during play (UEC, EMD, etc.) is tagged with the region ID, enabling post-hoc analysis: "Do high-AOS regions actually produce higher player tension?"

## Related Documents

- `schemas/registry/registry-regions.v1.json`: Canonical schema for region registry entries.
- `registry/formats/regions.example.ndjson`: Minimal valid example entries.
- `docs/schema-spine/invariants-and-metrics-spine.md`: Invariant definitions referenced by regions.
- `docs/integration/engine-agnostic-integration.md`: How engines consume region data at runtime.
