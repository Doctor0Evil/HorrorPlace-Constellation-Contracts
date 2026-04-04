# From Schema to Spine: Persona Contracts in the Constellation

This document describes how personas are modeled as NDJSON contracts, how `personaType` governs required fields and behavior, and how personas bind to events, regions, styles, invariants, and entertainment metrics across the VM‑constellation.

It is designed as a human‑readable companion to:

- `schemas/contracts/persona-contract-v1.json`
- `registry/registry-personas.v1.json`
- Invariant and metric spines (`invariants-spine.v1.json`, `entertainment-metrics-spine.v1.json`)

The goal is to make every persona a machine‑governed agent whose behavior, history, and experiential impact are fully derivable from contracts, not from ad‑hoc scripts.

---

## 1. Persona Taxonomy and `personaType`

Every persona contract declares a `personaType` field that determines:

- Which extra fields are required.
- How invariant coupling and metrics are interpreted.
- How engines and AI tools are allowed to use the persona at runtime.

The canonical `personaType` enum:

- `npc`
- `spectral-entity`
- `environmental-agent`
- `narrative-voice`

Each value represents a different point on the spectrum of agency and interaction, from grounded humanoids to disembodied narration and sentient environments.

### 1.1 `npc`

`npc` personas model conventional non‑player characters: conversational entities, quest givers, operators, and merchants.

Contract expectations:

- Primary function: dialogue, stateful interactions, resource exchange.
- Required persona‑specific sections:
  - `behaviorProfile.dialogue` – dialogue pacing, branching parameters, topic keys.
  - Optional `behaviorProfile.trade` – inventory surfaces, price models, availability conditions.
- Coupling:
  - Typically sensitive to `AOS` and `RWF` invariants:
    - High `AOS` → fragmented, unreliable or redacted information.
    - `RWF` sets how trustworthy the persona’s knowledge should be.
- Metrics:
  - `metricInfluence` often nudges:
    - `EMD` upwards (more evidence and hints).
    - `CDL` mildly upwards (ambiguity, half‑truths).
    - Keeps `ARR` and `STCI` within safe conversational bands.

### 1.2 `spectral-entity`

`spectral-entity` covers apparitions, hauntings, glitches, and other spectral presences.

Contract expectations:

- Primary function: stalking, manifestation, environmental interference.
- Required persona‑specific fields:
  - `spectralConsistencyIndex` – the SHCI‑like scalar that ties behavior to local history.
  - `behaviorProfile.manifest`, `behaviorProfile.stalk`, and similar keys.
- Coupling:
  - Bound tightly to `CIC`, `AOS`, and derived spectral indices:
    - High `CIC` → higher aggression and manifestation likelihood.
    - High `AOS` → more ambiguous, distorted forms and behaviors.
- Metrics:
  - `metricInfluence` typically targets:
    - `UEC` (unplanned encounters).
    - `STCI` (suspense vs. chaos balance).
    - Optionally `ARR` for acute fear spikes.

Because of their intensity and entanglement with historical trauma, spectral entities are usually gated to stronger safety tiers (`vault`, `lab`) and require valid `deadledgerref` entries and enforcement in downstream repos.

### 1.3 `environmental-agent`

`environmental-agent` personas represent non‑humanoid agency in the world: architecture, weather, localized anomalies, or systemic “process gods.”

Contract expectations:

- Primary function: environmental reaction and emergent threats.
- Required persona‑specific behavior:
  - `behaviorProfile.deformGeometry`, `behaviorProfile.anomalousWeather`, or similar keys that influence space, physics, and ambience.
- Coupling:
  - Bound to invariants like `CIC`, `LSG`, `DET`, `HVF`:
    - High `LSG` + high `DET` → trigger liminal distortions and spatial anomalies.
- Metrics:
  - `metricInfluence` tuned for:
    - `ARR` (ambient anxiety and dread).
    - Secondary `CDL` (cognitive strain from space behaving “wrong”).

These personas let the environment itself become an agent without defining a humanoid antagonist, and they are a core tool for systemic horror and emergent tension.

### 1.4 `narrative-voice`

`narrative-voice` personas represent meta‑narrative channels: radio, logs, diary fragments, internal monologues, and system voices.

Contract expectations:

- Primary function: lore delivery, mystery construction, psychological pressure.
- Required persona‑specific behavior:
  - `behaviorProfile.deliverLore`, with controls for:
    - Fragmentation.
    - Frequency or pacing.
    - Trigger conditions (sanity ranges, invariant gates).
- Coupling:
  - Strongly linked to `AOS`, `RWF`, and selected invariant bundles:
    - High `AOS` → more contradictions and redactions.
    - `RWF` tunes perceived reliability of delivered information.
- Metrics:
  - `metricInfluence` typically focuses on:
    - `EMD` – evidence and lore density.
    - `CDL` – cognitive dissonance and interpretive load.
    - Often low or neutral `UEC` and `ARR` to avoid constant jump‑scare pressure.

---

### 1.5 Persona Type Summary Table

| Persona Type           | Primary Function                         | Key Contract Fields                                          | Typical Behaviors                                   | Example Archetype     |
| ---------------------- | ---------------------------------------- | ------------------------------------------------------------ | --------------------------------------------------- | --------------------- |
| `npc`                  | Conversation, quests, transactions       | `behaviorProfile.dialogue`, optional `behaviorProfile.trade` | Dialogue branching, trading, stateful operators     | Mysterious Operator   |
| `spectral-entity`      | Haunting, manifestation, stalking        | `spectralConsistencyIndex`, `behaviorProfile.manifest`       | Appearing, vanishing, chasing, ambient interference | Vanished Researcher   |
| `environmental-agent`  | Environmental reaction, emergent threats | `behaviorProfile.deformGeometry`, `reactToInvariants`        | Shifting corridors, anomalous weather, gravity drift| Process‑God           |
| `narrative-voice`      | Lore, mystery, psychological pressure    | `behaviorProfile.deliverLore`, `reliabilityWeight`           | Broadcasts, logs, internal monologue, system voices | Chief Archivist       |

---

## 2. Persona Contract Shape (NDJSON)

Each persona is represented as one line of JSON in an NDJSON registry file, plus an associated contract file (if you choose to separate registry from full contracts). This section describes the single‑file persona contract pattern, suitable for `registry-personas.v1.json` when acting as a compact NDJSON registry.

### 2.1 Required base fields

All persona entries must define:

- `id` – globally unique persona ID, using `<TYPE>-<REGION>-<SEQUENCE>` convention, for example:
  - `PER-ARAL-0001`
  - `PER-MARSH-0012`
- `schemaref` – URI for the persona contract schema (e.g., `https://horror.place/constellation/schemas/contracts/persona-contract-v1.json`).
- `deadledgerref` – opaque reference to the cryptographic proof or governance ledger record for this persona.
- `artifactid` – opaque reference to the implementation artifact (behavior tree, animation, script, agent bundle).
- `createdAt` – ISO 8601 timestamp (UTC).
- `status` – lifecycle enum: `"active"`, `"deprecated"`, `"archived"`, `"draft"`.

These base fields allow registry tooling to:

- Validate structure via JSON Schema.
- Resolve entitlement and provenance via Dead‑Ledger‑style mechanisms.
- Locate the actual implementation asset, without exposing raw horror content.

### 2.2 Persona‑specific core objects

Every persona adds the following persona‑specific sections:

- `personaType` – one of `npc`, `spectral-entity`, `environmental-agent`, `narrative-voice`.
- `historicalAnchor` – anchors the persona to the history layer:
  - `eventRef` – ID of a canonical event, such as `EVT-ARAL-0001`.
  - `role` – short descriptor such as `chief-librarian`, `survivor`, `process-god-of-the-corridor`.
  - `reliabilityWeight` – numeric RWF (0–1) expressing the credibility of the underlying sources.
  - `archivalSources` – array of opaque references to supporting documents, logs, or myths.
- `behaviorProfile` – map of behavior keys to configuration objects:
  - Example keys: `"dialogue"`, `"stalk"`, `"manifest"`, `"deliverLore"`, `"deformGeometry"`.
  - Each value is a typed configuration object validated by the schema.
- `invariantCoupling` – map of invariant names to response rules:
  - For example, `CIC`, `AOS`, `LSG`, `DET`.
  - Values describe multipliers, thresholds, and scaling functions.
- `metricInfluence` – map of entertainment metric names to target delta ranges:
  - Metrics: `UEC`, `EMD`, `STCI`, `CDL`, `ARR`.
  - Values are usually `{"+":[min, max]}` ranges.
- Optional `prismMetaRef` or embedded `prismMeta` – links to or contains prism metadata used by AI authoring, provenance tracking, and CI workflows.

### 2.3 Example NDJSON entries

The examples below are minimal, public‑safe entries that can live in a `personas.example.ndjson` file. All opaque IDs are placeholders.

#### 2.3.1 Archivist (`narrative-voice`)

```json
{"id":"PER-ARAL-0001","schemaref":"https://horror.place/constellation/schemas/contracts/persona-contract-v1.json","deadledgerref":"zkp:sha256:per-aral-archivist...","artifactid":"ipfs:bafy...archivist-behavior-tree","createdAt":"2026-01-15T03:00:00Z","status":"active","personaType":"narrative-voice","historicalAnchor":{"eventRef":"EVT-ARAL-0001","role":"chief-librarian","reliabilityWeight":0.6,"archivalSources":["project-xiv-lab-log-001","aral-sea-digital-archive"]},"behaviorProfile":{"deliverLore":{"fragmented":true,"maxLinesPerSession":5,"triggerConditions":{"playerSanity>=":40}}},"invariantCoupling":{"AOS":{"audioCueIntensity":">0.7"}},"metricInfluence":{"EMD":{"+":},"CDL":{"+":}},"prismMetaRef":"prism:per-aral-0001"}
```

#### 2.3.2 Witness (`spectral-entity`)

```json
{"id":"PER-ARAL-0002","schemaref":"https://horror.place/constellation/schemas/contracts/persona-contract-v1.json","deadledgerref":"zkp:sha256:per-aral-witness...","artifactid":"ipfs:bafy...ghost-animation-asset","createdAt":"2026-01-15T03:00:00Z","status":"active","personaType":"spectral-entity","historicalAnchor":{"eventRef":"EVT-ARAL-0001","role":"survivor","reliabilityWeight":0.9,"archivalSources":["eyewitness-account-redacted-007"]},"behaviorProfile":{"stalk":{"minDistance":3,"maxDistance":15,"preferShadows":true,"triggerConditions":{"playerSanity<":50}},"manifest":{"audioCue":"distorted-whisper","visualEffect":"heat-haze-silhouette"}},"invariantCoupling":{"CIC":{"aggressionMultiplier":1.2,"manifestationChance":"linear-scale"}},"spectralConsistencyIndex":0.85,"metricInfluence":{"UEC":{"+":},"STCI":{"+":[0.2,0.5]}},"prismMetaRef":"prism:per-aral-0002"}
```

#### 2.3.3 Process‑God (`environmental-agent`)

```json
{"id":"PER-ARAL-0003","schemaref":"https://horror.place/constellation/schemas/contracts/persona-contract-v1.json","deadledgerref":"zkp:sha256:per-aral-processgod...","artifactid":"ipfs:bafy...deformable-mesh-script","createdAt":"2026-01-15T03:00:00Z","status":"active","personaType":"environmental-agent","historicalAnchor":{"eventRef":"EVT-ARAL-0001","role":"process-god-of-the-corridor","reliabilityWeight":1.0,"archivalSources":["vanishing-convoy-myth-text"]},"behaviorProfile":{"deformGeometry":{"deformationRadius":2.0,"intensityCurve":"exponential"}},"invariantCoupling":{"LSG":{"intensityMultiplier":2.0},"DET":{"triggerThreshold":">=7"}},"metricInfluence":{"ARR":{"+":},"CDL":{"+":}},"prismMetaRef":"prism:per-aral-0003"}[1]
```

These examples demonstrate:

- Stable IDs.
- Clean separation of schema reference, ledger reference, and artifact reference.
- Persona‑type‑specific behavior sections and invariant hooks.
- Metric targets that express design intent.

---

## 3. Invariant Coupling and SHCI

Personas do not choose behaviors in a vacuum. They read from the history layer and invariant spine, and their contracts encode how their behavior must track those values.

Key invariants used by personas include:

- `CIC` – Catastrophic Imprint Coefficient (historical trauma intensity).
- `MDI` – Mythic Density Index.
- `AOS` – Archival Opacity Score.
- `RRM` – Ritual Residue Map (strength).
- `FCF` – Folkloric Convergence Factor.
- `SPR` – Spectral Plausibility Rating (derived).
- `RWF` – Reliability Weighting Factor (source credibility).
- `DET` – Dread Exposure Threshold.
- `HVF` – Haunt Vector Field (magnitude/direction).
- `LSG` – Liminal Stress Gradient.
- `SHCI` – Spectral‑History Coupling Index (tightness of binding between entity and local history).

### 3.1 Coupling schema expectations

The `invariantCoupling` object:

- Keys: invariant IDs from the invariant spine.
- Values: objects describing response rules, such as:
  - Multipliers (`"aggressionMultiplier": 1.5`).
  - Thresholds (`"triggerThreshold": ">=0.8"`).
  - Scaling functions (`"intensity": "linear-scale"`).

Behavioral engines must:

1. Query the history/invariant layer via a canonical API (e.g., `H.CIC(region_id, tile_id)`).
2. Apply the response rules from `invariantCoupling`.
3. Set behavioral parameters accordingly.

Designers should keep `SHCI` high when a persona is tightly constrained by local history. This supports:

- Clear mapping from historical trauma to manifestation.
- Safety tiering and entitlement (high‑SHCI entities likely live in vaults or labs).

---

## 4. Metric Influence and experiential targets

`metricInfluence` turns personas into pacing instruments. It defines how each persona is intended to push or pull entertainment metrics when the encounter “lands.”

Core metrics:

- `UEC` – Uncertainty Engagement Coefficient.
- `EMD` – Evidential Mystery Density.
- `STCI` – Safe‑Threat Contrast Index.
- `CDL` – Cognitive Dissonance Load.
- `ARR` – Ambiguous Resolution Ratio.

### 4.1 Metric influence structure

Each entry in `metricInfluence` is:

- Key: metric name.
- Value: object describing a target delta band.

Common pattern:

```json
"metricInfluence": {
  "EMD": {"+":},
  "CDL": {"+":}
}
```

This means:

- On a successful interaction with this persona:
  - `EMD` should increase by 10–25 units.
  - `CDL` should increase by 2–5 units.

The drama manager or director uses these target bands to:

- Select personas that move session metrics towards desired ranges.
- Enforce caps based on style, tier, or session configuration.

### 4.2 Archetype profiles

Some typical profiles:

- Archivist (`narrative-voice`):
  - High positive `EMD` and `CDL`.
  - Low `UEC`, moderate `ARR`.

- Witness (`spectral-entity`):
  - Positive `UEC` (unplanned encounters).
  - Positive `STCI` (from suspense to chaos in bursts).

- Process‑God (`environmental-agent`):
  - Strong positive `ARR` (anxiety).
  - Mild to moderate `CDL` (space and physics contradictions).

Because metrics are part of the core spine, persona contracts can be evaluated the same way across engines and repos, and telemetry can be fed back into revising personas over time.

---

## 5. Integration with Events, Regions, and Styles

Personas are not isolated entries. They are wired into events, regions, and styles entirely by ID and invariant compatibility. This makes the constellation navigable as a graph rather than by filesystem paths.

### 5.1 Event bindings

Events reference personas directly:

- Event contracts may include a `personaIds` array listing allowed persona IDs for that scenario.

At runtime:

1. The drama manager chooses an event.
2. It reads the event’s `personaIds`.
3. For each candidate persona ID, it resolves the persona entry via the persona registry.
4. It checks invariant compatibility (region, tile, safety tier).
5. It instantiates only valid persona implementations.

This guarantees that:

- Scenarios are tightly bound to their cast.
- Vault‑only personas never leak into public events.

### 5.2 Region compatibility

Regions define:

- `invariantProfile` – local ranges for CIC, AOS, LSG, and other invariants.
- `tier` – public, vault, or lab, plus platform capabilities.

Persona compatibility is determined by:

- Intersection of region invariant ranges and persona `invariantCoupling` expectations.
- Matching safety tiers (persona tier must be allowed in the region’s tier).
- Optional additional filters, such as biome tags.

Tools can compute:

- For each persona ID, which regions are valid habitats.
- For each region ID, which personas are admissible.

### 5.3 Style and aesthetic routing

Styles define:

- Audio, visual, and narrative presentation envelopes.
- Metric “levers” and invariant‑based gating rules.
- Implication vs explicitness constraints.

Personas connect to styles via:

- Behavior profile keys that reference style‑defined cues, such as:
  - `audioCue`: `"distorted-whisper"`.
  - `visualEffect`: `"heat-haze-silhouette"`.
- Explicit style references via IDs in persona or region contracts.

This separation allows:

- Multiple style implementations for the same persona contract.
- Consistent enforcement of implication‑based horror across styles and tiers.

---

## 6. Lua / Engine Integration Pattern

While this repository is contract‑only, downstream engines and tools are expected to integrate personas using a narrow, history‑aware API.

A minimal Lua interaction pattern:

```lua
-- horror_invariants.lua (engine side, consumes invariant spine)
local H = {}

function H.CIC(region_id, tile_id)
  -- Lookup CIC from invariant bundle
end

function H.AOS(region_id, tile_id)
  -- Lookup AOS
end

function H.LSG(region_id, tile_id)
  -- Lookup LSG
end

return H
```

```lua
-- hpc_persona_runtime.lua (engine side, consumes persona contracts)
local H = require("horror_invariants")

local PersonaRuntime = {}

function PersonaRuntime.apply_invariant_coupling(persona, region_id, tile_id, runtime_state)
  local coupling = persona.invariantCoupling or {}

  for invariant, rules in pairs(coupling) do
    local value
    if invariant == "CIC" then
      value = H.CIC(region_id, tile_id)
    elseif invariant == "AOS" then
      value = H.AOS(region_id, tile_id)
    elseif invariant == "LSG" then
      value = H.LSG(region_id, tile_id)
    end

    -- Example: threshold rule
    if rules.triggerThreshold and value then
      local threshold = tonumber(string.match(rules.triggerThreshold, ">=([%d%.]+)"))
      if threshold and value >= threshold then
        runtime_state.triggers[invariant] = true
      end
    end

    -- Example: multiplier rule
    if rules.aggressionMultiplier and runtime_state.aggression then
      runtime_state.aggression = runtime_state.aggression * rules.aggressionMultiplier
    end
  end
end

return PersonaRuntime
```

This pattern ensures that:

- All persona behavior consults invariants before making decisions.
- Runtime state is derived from data, not handwritten conditionals.
- Additional layers (styles, events, regions) can read the same invariants and metrics to maintain coherence.

---

## 7. Registry and CI Expectations

To keep persona contracts coherent and audit‑ready, CI tooling should:

- Validate each NDJSON line against `persona-contract-v1.json`.
- Enforce ID format (`PER-<REGION>-<SEQUENCE>`).
- Enforce that:
  - `historicalAnchor.eventRef` exists in the events registry.
  - Referenced invariants in `invariantCoupling` exist in the invariant spine.
  - Referenced metrics in `metricInfluence` exist in the metrics spine.
- Enforce tier rules:
  - Persona tier is compatible with `deadledgerref` requirements.
  - Spectral entities with high `spectralConsistencyIndex` are not placed in public tiers.
- Optionally validate `prismMeta` or `prismMetaRef`:
  - Ensure every persona generated by AI tools has provenance, dependencies, and validation metadata.

By treating personas as first‑class NDJSON contracts wired into the schema spine, the constellation can safely evolve a population of agents that are historically anchored, invariant‑driven, and metrics‑aligned, across engines and repositories.
