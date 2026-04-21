# CHATDIRECTOR Selector Context & History Selector Integration v1

## 1. Overview

This document refines how CHATDIRECTOR interacts with the history‑aware selector and director subsystems for dungeon and horror runs, with a focus on anonymous sessions, selector patterns, director metric constraints, and telemetry events. It specifies concrete data shapes and behaviors suitable for implementation in Lua, Godot bindings, and NDJSON telemetry. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)

***

## 2. Selector Context Population for Anonymous Sessions

When the user is anonymous, CHATDIRECTOR still must emit a fully shaped `selectorContext` per call into `H.Selector.selectPattern` and related helpers. Anonymity changes how user identity is handled, but not the presence or structure of selector context fields. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)

### 2.1 Required fields

For every session, including anonymous:

- `descriptorId`  
- `consentTier`  
- `routerStateId` [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)

### 2.2 Population rules

**descriptorId**

- `descriptorId` refers to the **active selector descriptor** in use for this session, not to any account or human identity. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)
- CHATDIRECTOR must ensure that at session start, or when a new experience is created, a descriptor is chosen or instantiated:
  - Either from the registry (e.g., `desc-aral-dungeon-selector.v1`), or  
  - As a temporary session‑scoped descriptor (e.g., `desc-sess-<short-uuid>`). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)
- That value is then written into `selectorContext.descriptorId` on all subsequent calls to `H.Selector` for this session. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)

**consentTier**

- Consent is always tracked per session. For anonymous users, CHATDIRECTOR:
  - Calls `H.Consent.bindSession(sessionId, consentProfileId)`, using a non‑PII profile such as `cp-anon-standard` or a region‑specific default. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)
  - Calls `H.Consent.currentState(sessionId)` to obtain `consentTier`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)
- `selectorContext.consentTier` is set to that value (e.g., `minor`, `adult-basic`, `adult-horror`, `research`). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)

**routerStateId**

- The router state is resolved by the age‑tier router using consent and policy tables, and produces a `routerStateId` such as `rtr-xyz123`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)
- CHATDIRECTOR must cache this ID per session and set `selectorContext.routerStateId = <that value>` for each selector call. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)

### 2.3 Anonymity guarantees

- No PII or long‑lived identity is embedded in `descriptorId`, `consentTier`, or `routerStateId`; they are all **session‑scoped or policy‑scoped identifiers**. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)
- Telemetry and selectors can still join on these IDs, so behavior remains fully auditable even for anonymous users. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)

***

## 3. `historySelectorPattern` Spec for Liminal Stairwells and Sanity Ticks

This section defines a canonical JSON structure for a `historySelectorPattern` instance that prefers `liminal:stairwell` tags and mechanic hooks containing `sanity_tick`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)

### 3.1 Pattern structure

```json
{
  "id": "hsp-dungeon-liminal-stairwell-sanity.v1",
  "schemaRef": "history-selector-pattern.v1",
  "objectKind": "historySelectorPattern",
  "version": "v1.0.0",

  "patternType": "DUNGEON_NODE",
  "label": "Liminal stairwell with sanity tick",
  "description": "Prefers stairwell liminal nodes that can apply low-level sanity tick mechanics.",

  "liminalTagPreferences": {
    "preferred": ["liminal:stairwell"],
    "discouraged": [],
    "weights": {
      "preferredBonus": 0.8,
      "perMatchingTag": 0.4,
      "discouragedPenalty": -0.5,
      "perDiscouragedTag": -0.3
    }
  },

  "mechanicHookPreferences": {
    "preferred": ["sanity_tick"],
    "discouraged": [],
    "weights": {
      "preferredBonus": 0.6,
      "perMatchingHook": 0.3,
      "discouragedPenalty": -0.4,
      "perDiscouragedHook": -0.2
    }
  },

  "historyCoupling": {
    "minSHCI": 0.5,
    "preferredBand": [0.6, 1.0],
    "weights": {
      "insideBandBonus": 0.5,
      "outsideBandPenalty": -0.3
    }
  },

  "metricIntent": {
    "ARR": {
      "min": 0.6,
      "max": 0.9
    },
    "DET": {
      "min": 0.4,
      "max": 0.7
    }
  },

  "prismMeta": {
    "author": "tofill",
    "governanceRef": "tofill",
    "deadLedgerRef": "tofill-optional"
  }
}
```

### 3.2 Runtime expectations

- `H.Node.score_next_from_history` must:
  - Read `liminalTagPreferences` and `mechanicHookPreferences` and adjust scores based on node tags (`node.liminalTags`) and mechanic hooks (`node.mechanicHooks`). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)
  - Combine these with `historyCoupling` (SHCI fit) to produce a final node score. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)
- `metricIntent` is used as a soft guidance for entertainment metrics (e.g., keep ARR and DET within the provided bands when repeatedly choosing nodes under this pattern). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)

***

## 4. DET Bands in `directorPersonaContract`

The `directorPersonaContract` schema includes `invariantBands` that define the persona’s permissible ranges for core invariants and metrics. For DET, this spec clarifies granularity. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)

### 4.1 Persona-level DET band

At the persona contract level, `invariantBands.DET` is a **single band**:

```json
"invariantBands": {
  "DET": { "min": 2.0, "max": 6.0 },
  "CIC": { "min": 0.2, "max": 0.8 },
  "SHCI": { "min": 0.3, "max": 0.7 }
}
```

- This band expresses the overall dread exposure range the persona is allowed to operate in before consent and comfort policies are applied. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)
- It must be intersected with consent caps and comfort policy bands at runtime. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)

### 4.2 Per-event-type DET shaping in comfort policy

Per‑event‑type DET bands (e.g., ambient, event, climax) belong in `director-comfort-policy`, not in the persona contract schema. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)

Example within a comfort policy:

```json
"detBands": {
  "ambient": { "min": 1.0, "max": 3.0 },
  "event":   { "min": 2.0, "max": 5.0 },
  "climax":  { "min": 4.0, "max": 6.0 }
}
```

- `H.Director.loadPersona` must compute effective DET bands by intersecting:
  - persona band,  
  - consent caps,  
  - comfort policy `detBands` for the current event type. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)

***

## 5. Behavior of `H.Director.constrainMetrics` on Out-of-Band Values

`H.Director.constrainMetrics` is responsible for clamping `intendedMetrics` into the strictest band implied by persona, consent, and comfort policy. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)

### 5.1 Band intersection

For each metric (e.g., DET):

1. Load persona band from `directorPersonaContract.invariantBands`.  
2. Load consent caps from `H.Consent.currentState(sessionId).invariantCaps`.  
3. Load comfort policy bands if present (e.g., per event type).  
4. Compute the **effective band** as the intersection of these ranges. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)

If any intersection is empty (e.g., misconfigured contracts), the engine should treat it as an error path and clamp to the lowest safe band (typically the most restrictive consent cap) while logging a configuration error.

### 5.2 Clamping rule

Given `intendedMetrics[metric] = v_intended` and effective band `[v_min, v_max]`:

- Compute:

  \[
  v_{\text{effective}} = \max(v_{\min}, \min(v_{\max}, v_{\text{intended}}))
  \]

- Even if `v_intended` exceeds both persona band and consent cap, `v_effective` must **not** exceed the consent cap. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)

### 5.3 Return envelope

`H.Director.constrainMetrics` returns:

```lua
{
  ok = true,
  data = {
    effectiveMetrics = {
      DET = v_effective,
      -- other metrics as requested
    },
    bands = {
      DET = { min = v_min, max = v_max }
      -- per-metric bands
    },
    details = {
      clamped = true,          -- if any metric was clamped
      clampedMetrics = { "DET" }
    }
  },
  error = nil
}
```

- `clamped` and `clampedMetrics` are advisory but strongly recommended; they help personas decide when to `soften` or `downshift` subsequent beats. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)
- Under no circumstance may `effectiveMetrics` exceed consent caps or comfort policy maxima. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)

***

## 6. `H.Selector.recordDecision` Selector Decision Structure

`H.Selector.recordDecision(sessionId, selectorDecision, safetyDecision, budgetEventId)` emits a telemetry event describing how a selector pattern chose its next beat. This section specifies the expected structure of the `selectorDecision` table. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)

### 6.1 Required fields

At minimum:

```lua
local selectorDecision = {
  selectorPatternId = "hsp-dungeon-liminal-stairwell-sanity.v1",
  ruleCatalogSnapshot = {
    {
      ruleId = "SEL-R001",
      phase = "safetyprefilter",
      action = "EXCLUDE",
      applied = false,
      scoreDelta = 0.0,
      excluded = false
    },
    {
      ruleId = "SEL-R006",
      phase = "patternmatch",
      action = "SCOREMODIFY",
      applied = true,
      scoreDelta = 0.8,
      excluded = false
    }
    -- … through SEL-R009 as applicable
  }
}
```

- `selectorPatternId` – the ID of the pattern used for this decision. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)
- `ruleCatalogSnapshot` – an array of rule applications in the scoring pipeline, at least including `ruleId` and `phase` for each rule considered. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)

### 6.2 Recommended optional fields

For richer telemetry and debugging:

```lua
selectorDecision.candidates = {
  {
    candidateId = "node-stairwell-02",
    score = 2.35,
    selected = true,
    reasons = {
      liminalTags = { "liminal:stairwell" },
      mechanicHooks = { "sanity_tick" },
      shci = 0.84
    }
  },
  {
    candidateId = "node-maintenance-03",
    score = 1.10,
    selected = false,
    reasons = {
      liminalTags = {},
      mechanicHooks = { "light_drop" },
      shci = 0.72
    }
  }
}

selectorDecision.conflictsResolved = {
  -- e.g., tiebreaks, conflict policies activated
  -- { type = "tiebreak", ruleId = "SEL-R009", strategy = "freshness" }
}

selectorDecision.invariantsSnapshot = {
  CIC = 0.4,
  AOS = 0.3,
  DET = 4.0,
  SHCI = 0.8
}

selectorDecision.metricsSnapshot = {
  UEC = 0.62,
  EMD = 0.41,
  STCI = 0.55,
  CDL = 0.47,
  ARR = 0.74
}
```

### 6.3 Telemetry linkage

`H.Selector.recordDecision` must:

- Emit the `selectorDecision` payload as a `history-selector-decision` NDJSON record, conforming to `history-selector-decision-event-v1.json` once finalized. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)
- Link the record to:
  - `sessionId`,  
  - `budgetEventId`,  
  - any consent or safety envelope IDs supplied by `safetyDecision` and budget modules. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/6e73e024-f705-41be-9f81-7b91e38d04af/compiled-report-of-todos-missi-lHKqTuY4Q8O.bRJJ6b1a4w.md)

This allows cross‑module analysis of how patterns, safety decisions, and budget events interacted for each step in a run.
