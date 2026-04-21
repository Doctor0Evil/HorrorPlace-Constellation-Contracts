---
invariants-used: [CIC, MDI, AOS, RRM, FCF, SPR, RWF, DET, HVF, LSG, SHCI]
metrics-used: [UEC, EMD, STCI, CDL, ARR]
modules: [H.ImageHelper, H.Metrics, H.Director]
schema-refs:
  - image-helper-contract.v1.json
  - session-metrics-envelope-v1.json
---

# Image Helper Lua API v1

This document defines the canonical Lua surface for `H.ImageHelper`, which generates implication‑only visual descriptions for AI‑Chat and engine tools. It binds run context, invariants, metrics, and inventory into a pure data envelope that external image middleware can consume without accessing raw horror content.

All implementations must keep Tier‑1 repos free of actual image generation; `H.ImageHelper.describe` returns structured text and tags only.

---

## 1. Function signature

```lua
-- H.ImageHelper.describe(context, invariants, metrics, inventory)
-- Returns: { ok, data, error } envelope

local result = H.ImageHelper.describe(context, invariants, metrics, inventory)
```

### Parameters

| Name | Type | Description |
|------|------|-------------|
| `context` | table | Run and beat info (see §2). |
| `invariants` | table | Current invariant snapshot for node/tile. |
| `metrics` | table | Current entertainment metrics (targets or actuals). |
| `inventory` | table | Active interpretation tools/persona inventory for this run. |

### Return envelope

```lua
-- Success case
result.ok == true
result.data = {
  description = "implication-safe, text-only visual prompt",  -- string
  hiddenTags  = { "clue.water-rising", "symbol.ritual-glyph" }, -- array of strings
  metricDeltas = { UEC = 0.05, EMD = 0.1, CDL = 0.05 }  -- optional table
}

-- Error case
result.ok == false
result.error = {
  code = "INVENTORY_MISSING" | "INVARIANT_UNAVAILABLE",
  message = string,
  details = table | nil
}
```

---

## 2. Context table shape

The `context` parameter must include at minimum:

```lua
context = {
  runId             = "run-...",        -- required
  sessionId         = "sess-...",       -- required
  regionId          = "reg-...",        -- required
  tileId            = "tile-...",       -- optional
  nodeId            = "node-...",       -- optional
  eventStage        = "Outer" | "Locus" | "Rupture" | "Fallout",
  difficulty        = "Novice" | "Standard" | "Severe",
  directorPersonaId = "dir-...",        -- optional
  experienceContractId = "exp-...",     -- optional
  routerStateId     = "rtr-..."         -- optional
}
```

All fields are read‑only; implementations must not mutate `context`.

---

## 3. Invariants and metrics parameters

### 3.1 `invariants` table

Must contain canonical invariant keys with numeric values in [0.0, 1.0] (except DET which is [0.0, 10.0]):

```lua
invariants = {
  CIC  = 0.72,   -- Catastrophic Imprint Coefficient
  AOS  = 0.65,   -- Ambient Oscillation Score
  DET  = 5.2,    -- Dread Exposure Threshold (0-10)
  SHCI = 0.81,   -- Spectral-History Coupling Index
  LSG  = 0.77,   -- Liminal Stress Gradient
  HVF_mag = 0.69 -- Haunt Vector Field magnitude
  -- Other invariants optional
}
```

### 3.2 `metrics` table

Must contain entertainment metric keys with numeric values in [0.0, 1.0]:

```lua
metrics = {
  UEC  = 0.68,   -- Uncertainty Engagement Coefficient
  EMD  = 0.54,   -- Evidential Mystery Density
  STCI = 0.61,   -- Safe-Threat Contrast Index
  CDL  = 0.73,   -- Cognitive Dissonance Load
  ARR  = 0.82    -- Ambiguous Resolution Ratio
}
```

Values may be `nil` if not yet estimated; implementations must handle `nil` gracefully.

---

## 4. Inventory parameter

The `inventory` table describes active interpretation tools available to the player:

```lua
inventory = {
  tools = { "flashlight", "journal", "spectral_lens" },
  cluesDiscovered = { "clue.blood-trail", "symbol.ritual-glyph" },
  personaModifiers = { "clinical-tone", "ritual-aware" }
}
```

Implementations may use `inventory` to tailor `description` and `hiddenTags` to the player's current capabilities.

---

## 5. Output contract

### 5.1 `description` field

- Must be implication‑only: no explicit violence, gore, or graphic harm.
- Must respect `difficulty`: Novice runs must include at least one interpretable symbol per image.
- Must be shaped by `CDL` and `ARR` caps from the runContract: high‑CDL descriptions should avoid contradictory visual cues that would overwhelm novice players.
- May reference invariant‑derived aesthetics (e.g., "dim light with high spectral density") but never raw invariant values.

### 5.2 `hiddenTags` field

- Opaque clue identifiers used by CHATDIRECTOR and telemetry.
- Never rendered directly to the player.
- Must be drawn from a canonical tag registry (e.g., `schemas/clue-tags-v1.json`).

### 5.3 `metricDeltas` field (optional)

- Suggested updates to entertainment metrics if the player engages with the image.
- Used by `H.Metrics` to feed image‑driven clues back into the entertainment spine.
- Must be small, incremental values (e.g., `UEC +0.05`) to avoid metric spikes.

---

## 6. Implementation notes

- Tier‑1 repos (Horror.Place, Constellation‑Contracts) must contain only this spec and a no‑op or test implementation.
- Actual image generation happens in vault repos (Atrocity‑Seeds, Spectral‑Foundry) or external tools that consume `description` and `hiddenTags`.
- Godot/Unreal plugins forward run state to this Lua function and pass `description` to their image middleware; they never see raw invariants or metrics tables.

---

## 7. Telemetry integration

Every successful call to `H.ImageHelper.describe` should emit a lightweight telemetry event:

```json
{
  "event": "image-helper-describe-called",
  "sessionId": "...",
  "runId": "...",
  "nodeId": "...",
  "invariantsUsed": ["CIC", "LSG", "SHCI"],
  "metricsSnapshot": { "UEC": 0.68, "CDL": 0.73 },
  "hiddenTagsCount": 2,
  "timestamp": "2026-04-20T21:30:00Z"
}
```

This enables offline analysis of how image‑driven clues correlate with player engagement and metric trajectories.
