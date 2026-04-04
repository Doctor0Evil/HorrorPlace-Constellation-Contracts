# Registry Events Specification

This document defines the structure, semantics, and usage rules for the `events` registry in the HorrorPlace constellation. Events are interactive scenarios, encounters, or narrative beats that occur within a region or across multiple regions.

## Schema Reference

All `events` registry entries must conform to `schemas/registry/registry-events.v1.json`, which extends `schemas/registry/registry-entry-base.v1.json` with event-specific fields.

Canonical `$id`: `https://horrorplace.constellation/schemas/registry/registry-events.v1.json`

## Required Fields (Beyond Base)

| Field | Type | Description |
|-------|------|-------------|
| `eventType` | enum | One of: `"ambient"`, `"encounter"`, `"narrative-beat"`, `"environmental-hazard"`, `"spectral-manifestation"`. |
| `regionIds` | array[string] | List of region IDs where this event can occur. Must reference valid `REG-*` IDs. |
| `triggerConditions` | object | Conditions that activate the event (e.g., `{"timeOfDay":"night","playerSanity<":30}`). |
| `invariantRequirements` | object | Map of invariant name → minimum/maximum value required for event to be valid (e.g., `{"CIC":{">=":0.7},"DET":{"<=":8}}`). |

## Optional Fields

| Field | Type | Description |
|-------|------|-------------|
| `spectralAnchor` | boolean | If `true`, this event may manifest a spectral presence tied to historical trauma. Default: `false`. |
| `mythicDensityThreshold` | number [0–1] | Minimum MDI (Mythic Density Index) required for this event to feel plausible. |
| `archivalOpacityBonus` | number [0–1] | Amount to increase AOS (Archival Opacity Score) if this event resolves ambiguously. |
| `telemetryHooks` | array[string] | List of metric names this event emits telemetry for (e.g., `["UEC","EMD","CDL"]`). |
| `prismMetaRef` | string | Reference to a `prismMeta` document describing event linkage and dependency graph. |

## Event Type Semantics

### `ambient`
Low-intensity, background events that enhance atmosphere without direct player interaction. Examples: distant chanting, flickering lights, subtle environmental shifts. Used to raise EMD (Evidential Mystery Density) without triggering combat or major narrative branches.

### `encounter`
Direct interaction with an NPC, creature, or environmental threat. May involve combat, dialogue, or stealth. Must declare `invariantRequirements` to ensure historical plausibility (e.g., high-CIC regions get more traumatic encounters).

### `narrative-beat`
Story-advancing events that reveal lore, unlock quests, or shift player understanding. Often tied to `spectralAnchor: true` and high `archivalOpacityBonus` to reward investigation.

### `environmental-hazard`
Non-sentient threats: radiation pockets, structural collapse, toxic fog. Driven by CIC (Catastrophic Imprint Coefficient) and real-world disaster data.

### `spectral-manifestation`
Full apparition or reality glitch events. Requires `spectralAnchor: true`, high `mythicDensityThreshold`, and vault-tier gating. Behavior is tuned by SPI (Spectral Plausibility Index) to maintain grounded dread.

## Trigger Condition Format

`triggerConditions` is a JSON object with simple predicate syntax:

```json
{
  "timeOfDay": "night",
  "playerSanity<": 30,
  "regionTags": ["biohazard","abandoned"],
  "previousEvents": ["EVT-ARAL-0001"]
}
```

Supported operators: `=`, `<`, `>`, `<=`, `>=`, `in` (for arrays), `hasTag` (for region tags). All conditions are ANDed; OR logic requires separate event entries.

## Validation Rules

The registry linter (`hpc-lint-registry.py`) enforces:

1. `regionIds` references exist in `regions.example.ndjson` (or production `regions.ndjson`).
2. `invariantRequirements` values fall within canonical ranges defined in `invariants-spine.v1.json`.
3. If `spectralAnchor: true`, then `tier` must be `"vault"` or `"lab"` and `deadledgerref` must be present.
4. `eventType` matches one of the allowed enums.
5. `triggerConditions` uses only supported operators and field names.

## Example Entry

```json
{"id":"EVT-ARAL-0001","schemaref":"https://horrorplace.constellation/schemas/registry/registry-events.v1.json","deadledgerref":"zkp:sha256:evt001...","artifactid":"ipfs:bafyevt001...","createdAt":"2026-01-15T03:00:00Z","status":"active","eventType":"spectral-manifestation","regionIds":["REG-ARAL-0001"],"triggerConditions":{"timeOfDay":"night","playerSanity<":40,"regionTags":["biohazard"]},"invariantRequirements":{"CIC":{">=":0.85},"AOS":{">=":7.0}},"spectralAnchor":true,"mythicDensityThreshold":0.7,"archivalOpacityBonus":0.2,"telemetryHooks":["UEC","EMD","STCI"],"prismMetaRef":"prism:evt-aral-0001"}
```

## Cross-Registry Linking

Events reference regions via `regionIds`. Conversely, region entries may list `eventIds` to declare which events are valid in that region. Tools use the spine index to resolve these bidirectional links without hardcoding paths.

## Telemetry Integration

Events declare `telemetryHooks` to specify which metrics they affect. At runtime, the engine emits telemetry envelopes (`session-metrics-envelope.v1.json`) tagged with the event ID. The spine index uses this to correlate event design with player experience data (UEC, EMD, etc.) for iterative refinement.

## Related Documents

- `schemas/registry/registry-events.v1.json`: Canonical schema for event registry entries.
- `registry/formats/events.example.ndjson`: Minimal valid example entries.
- `docs/schema-spine/invariants-and-metrics-spine.md`: Invariant and metric definitions referenced by events.
- `docs/tooling/prismMeta-and-agentProfiles.md`: How `prismMetaRef` enables bidirectional validation.
