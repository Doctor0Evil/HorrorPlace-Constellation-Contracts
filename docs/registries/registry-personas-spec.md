# Registry Personas Specification

This document defines the structure, semantics, and usage rules for the `personas` registry in the HorrorPlace constellation. Personas are AI-driven characters, spectral entities, or narrative agents that interact with the player or environment.

## Schema Reference

All `personas` registry entries must conform to `schemas/registry/registry-personas.v1.json`, which extends `schemas/registry/registry-entry-base.v1.json` with persona-specific fields.

Canonical `$id`: `https://horrorplace.constellation/schemas/registry/registry-personas.v1.json`

## Required Fields (Beyond Base)

| Field | Type | Description |
|-------|------|-------------|
| `personaType` | enum | One of: `"npc"`, `"spectral-entity"`, `"environmental-agent"`, `"narrative-voice"`. |
| `historicalAnchor` | object | Structured data tying the persona to real or in-universe history: `{"eventRef":"EVT-ARAL-0001","role":"vanished-researcher","reliabilityWeight":0.8}`. |
| `behaviorProfile` | object | Map of behavior name → parameters (e.g., `{"stalk":{"minDistance":5,"maxDistance":20},"dialogue":{"fragmented":true}}`). |
| `invariantCoupling` | object | Map of invariant name → how the persona's behavior responds to it (e.g., `{"CIC":{"aggressionMultiplier":1.5}}`). |

## Optional Fields

| Field | Type | Description |
|-------|------|-------------|
| `spectralConsistencyIndex` | number [0–1] | SHCI (Spectral-Historical Consistency Index) for grounding entity behavior in historical source material. |
| `metricInfluence` | object | Map of metric name → expected impact (e.g., `{"EMD":{"+":[5,15]}}` means persona should raise EMD by 5–15 points). |
| `prismMetaRef` | string | Reference to a `prismMeta` document describing persona linkage and dependency graph. |
| `experimentalFlags` | array[string] | Tags for research-grade personas (e.g., `["neural-resonance-test","process-god-alpha"]`). Only valid in `lab` tier. |

## Persona Type Semantics

### `npc`
Non-player characters with dialogue, quests, or trade. Behavior is driven by `behaviorProfile` and constrained by `historicalAnchor` to maintain narrative coherence.

### `spectral-entity`
Apparitions, hauntings, or reality glitches. Requires `spectralConsistencyIndex` and vault-tier gating. Behavior is tuned by SPI (Spectral Plausibility Index) to maintain grounded dread.

### `environmental-agent`
Non-sentient but reactive elements: shifting architecture, whispering winds, anomalous weather. Driven by `invariantCoupling` to environmental invariants (CIC, AOS).

### `narrative-voice`
Disembodied narration, diary entries, or radio broadcasts. Used to modulate EMD (Evidential Mystery Density) and CDL (Cognitive Dissonance Load) without direct interaction.

## Historical Anchor Format

`historicalAnchor` ties a persona to the constellation's historical data layer:

```json
{
  "eventRef": "EVT-ARAL-0001",
  "role": "vanished-researcher",
  "reliabilityWeight": 0.8,
  "archivalSources": ["lab-report-1992","eyewitness-account-redacted"]
}
```

- `eventRef` must reference a valid event ID in the events registry.
- `reliabilityWeight` (RWF) rates source credibility; low-RWF personas are perfect for ambiguous, rumor-based encounters.
- `archivalSources` lists opaque references to source documents (never raw content).

## Behavior Profile Format

`behaviorProfile` defines runtime behavior parameters:

```json
{
  "stalk": {
    "minDistance": 5,
    "maxDistance": 20,
    "preferShadows": true
  },
  "dialogue": {
    "fragmented": true,
    "maxLinesPerEncounter": 3,
    "triggerConditions": {"playerSanity<": 40}
  }
}
```

- Behavior names are from an approved list in `docs/overview/design-principles.md`.
- Parameters are type-checked by the schema; the linter enforces allowed keys and value ranges.

## Invariant Coupling Format

`invariantCoupling` describes how a persona's behavior adapts to invariant values:

```json
{
  "CIC": {
    "aggressionMultiplier": 1.5,
    "manifestationChance": "linear-scale"
  },
  "AOS": {
    "audioCueIntensity": ">0.7"
  }
}
```

- Keys are invariant names from `invariants-spine.v1.json`.
- Values are behavior-response rules: multipliers, thresholds, or scaling functions.
- The linter validates that referenced invariants exist and response rules use approved syntax.

## Validation Rules

The registry linter (`hpc-lint-registry.py`) enforces:

1. `personaType` matches one of the allowed enums.
2. `historicalAnchor.eventRef` references a valid event ID.
3. `behaviorProfile` keys are from the approved behavior list; parameter values are type-correct.
4. `invariantCoupling` keys match invariant names; response rules use approved syntax.
5. If `spectralConsistencyIndex > 0.7`, then `tier` must be `"vault"` or `"lab"`.
6. If `experimentalFlags` is non-empty, then `tier` must be `"lab"`.

## Example Entry

```json
{"id":"PER-ARAL-0001","schemaref":"https://horrorplace.constellation/schemas/registry/registry-personas.v1.json","deadledgerref":"zkp:sha256:per001...","artifactid":"ipfs:bafyper001...","createdAt":"2026-01-15T03:00:00Z","status":"active","personaType":"spectral-entity","historicalAnchor":{"eventRef":"EVT-ARAL-0001","role":"vanished-researcher","reliabilityWeight":0.8,"archivalSources":["lab-report-1992","eyewitness-account-redacted"]},"behaviorProfile":{"stalk":{"minDistance":5,"maxDistance":20,"preferShadows":true},"manifest":{"audioCue":"distant-whisper","visualEffect":"heat-haze-silhouette"}},"invariantCoupling":{"CIC":{"aggressionMultiplier":1.5,"manifestationChance":"linear-scale"},"AOS":{"audioCueIntensity":">0.7"}},"spectralConsistencyIndex":0.82,"metricInfluence":{"EMD":{"+":[5,15]},"STCI":{"+":[0.1,0.3]}},"prismMetaRef":"prism:per-aral-0001"}
```

## Cross-Registry Linking

Personas are referenced by events and regions via ID. An event entry may list `personaIds: ["PER-ARAL-0001"]` to declare which agents can appear. The spine index enables tools to answer: "What personas are valid for a high-CIC, vault-tier event?"

## Runtime Integration

At runtime, the AI system loads the persona's `behaviorProfile` and `invariantCoupling` to configure:

- Pathfinding and stalking logic (e.g., `preferShadows: true` biases navmesh weights).
- Dialogue or audio cue selection (e.g., `fragmented: true` triggers procedural line assembly).
- Adaptive aggression or manifestation chance based on live invariant values.

The engine also reads `metricInfluence` to tune the drama manager: if a persona aims to raise EMD by 5–15 points, the system can prioritize encounters that support that goal.

## Related Documents

- `schemas/registry/registry-personas.v1.json`: Canonical schema for persona registry entries.
- `registry/formats/personas.example.ndjson`: Minimal valid example entries.
- `docs/schema-spine/invariants-and-metrics-spine.md`: Invariant definitions referenced by personas.
- `docs/tooling/prismMeta-and-agentProfiles.md`: How `prismMetaRef` enables bidirectional validation for AI agents.
