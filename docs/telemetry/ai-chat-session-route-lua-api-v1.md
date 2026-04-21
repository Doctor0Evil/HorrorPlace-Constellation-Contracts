---
invariants-used:
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
metrics-used:
  - UEC
  - EMD
  - STCI
  - CDL
  - ARR
modules:
  - H.Metrics
  - H.Selector
  - H.Budget
  - H.Consent
  - H.Safety
schema-refs:
  - session-metrics-envelope-v1.json
  - ai-chat-session-route-v1.json
---

# AI-Chat Session Route & Metrics Lua API v1

This document defines a small Lua surface for emitting AI-Chat session routes and metric curves into NDJSON telemetry. It is an extension of the Telemetry (Module 6) and AI-Chat entertainment values research: every governed AI-Chat horror session MUST produce a route envelope that can be replayed, audited, and analyzed without exposing raw narrative text.

The API is intentionally narrow: it lets engines and AI-Chat tools create, append to, and finalize a "session route" object backed by `ai-chat-session-route-v1.json`, using metrics from `H.Metrics` and contract IDs from selector / run contracts.

---

## 1. Concepts and schemas

A *session route* is a telemetry-only object that answers three questions:

1. Which AI-Chat horror profile and director persona were active for this session?
2. Which contract IDs (mood, event, run, pattern) were used, and in what order and stage?
3. How did entertainment metrics (UEC, EMD, STCI, CDL, ARR) evolve across those steps?

The corresponding JSON Schema lives in the Telemetry module as:

- `schemas/telemetry/ai-chat-session-route-v1.json` – session-level route and metrics envelope.

It links to:

- `schemas/telemetry/session-metrics-envelope-v1.json` – shared metrics fields and comfort outcomes.
- Contract registries (`registry-events.ndjson`, `registry-regions.ndjson`, etc.) via contract IDs.

AI-Chat and runtime code **never** write JSON by hand; they call the Lua helpers in this document, which construct route tables that can be serialized to NDJSON by engine-specific sinks.

---

## 2. Namespaces and responsibilities

The route API is exposed as a small `H.Route` namespace:

- `H.Route.start(sessionContext, profileContext)`  
  Creates a new route object for an AI-Chat horror session and returns a route ID.

- `H.Route.appendStep(sessionId, routeId, stepContext)`  
  Appends a contract step with metrics snapshot to the route.

- `H.Route.finalize(sessionId, routeId, outcomeContext)`  
  Seals the route, computes derived summaries (comfort outcome, curves), and returns a ready-to-emit telemetry object.

All functions return the standard `ok / data / error` envelope used by the unified `H.` runtime API.

```lua
-- Standard envelope example
local result = H.Route.start(sessionContext, profileContext)

-- result = {
--   ok = boolean,
--   data = table | nil,
--   error = {
--     code = string,
--     message = string,
--     details = table | nil
--   } | nil
-- }
```

The implementation of `H.Route` is engine-specific (e.g., in `engine/telemetry/ai_chat_route.lua`), but the shape and field names described here are normative.

---

## 3. H.Route.start

### 3.1 Purpose

`H.Route.start` is called once at the beginning of a governed AI-Chat horror session, after consent and routing are bound but before the first horror beat is delivered. It binds the route to the AI-Chat horror profile, the director persona (if any), and the initial metrics context.

### 3.2 Parameters

```lua
local result = H.Route.start(sessionContext, profileContext)
```

- `sessionContext : table`  
  Required fields:

  ```lua
  sessionContext = {
    sessionId          = "sess-...",
    aiChatMode         = "GuidedRitual",     -- small enum
    regionId           = "reg-...",
    runContractId      = "run-...",          -- optional for non-run sessions
    consentTier        = "adult-basic",      -- matches Consent module enums
    routerStateId      = "rtr-...",          -- from Age-Gated Tier Router
    directorPersonaId  = "dir-cold-overseer" -- optional
  }
  ```

- `profileContext : table`  
  Binds to the AI-Chat horror profile and initial metrics:

  ```lua
  profileContext = {
    aiChatHorrorProfileId = "achp-guided-ritual-01",
    intendedInvariants    = {
      CIC  = { min = 0.3, max = 0.7 },
      DET  = { min = 2.0, max = 5.5 },
      SHCI = { min = 0.4, max = 0.9 }
    },
    intendedMetrics       = {
      UEC  = { min = 0.5, max = 0.8 },
      EMD  = { min = 0.2, max = 0.6 },
      STCI = { min = 0.5, max = 0.8 },
      CDL  = { min = 0.3, max = 0.7 },
      ARR  = { min = 0.6, max = 0.9 }
    }
  }
  ```

The `intendedInvariants` and `intendedMetrics` fields are usually taken from the aiChatHorrorProfile contract and ai-safe authoring contract, not invented at runtime.

### 3.3 Successful result

```lua
-- On success
result.ok   == true
result.data = {
  routeId   = "rt-sess-xyz-001",
  sessionId = "sess-xyz"
}
```

The returned `routeId` is opaque and must be used in subsequent `H.Route.appendStep` and `H.Route.finalize` calls for this session.

### 3.4 Error conditions

Typical error codes:

- `SESSION_NOT_BOUND` – consent / routing not yet initialized for this `sessionId`.
- `PROFILE_MISSING` – `aiChatHorrorProfileId` is missing or invalid.
- `PROFILE_INCOMPATIBLE` – intended bands conflict with session tier or director persona constraints.

---

## 4. H.Route.appendStep

### 4.1 Purpose

`H.Route.appendStep` appends a single step to the route: a reference to the contract(s) used for the turn, the phase/stage label, and a snapshot of metrics at that point. It is called once per narrative beat or major turn, after selector and budget decisions for that beat are known.

### 4.2 Parameters

```lua
local result = H.Route.appendStep(sessionId, routeId, stepContext)
```

- `sessionId : string`  
  Must match the `sessionId` passed to `H.Route.start`.

- `routeId : string`  
  Route identifier returned by `H.Route.start`.

- `stepContext : table`  
  Required fields:

  ```lua
  stepContext = {
    stepIndex        = 1,             -- 0-based or 1-based, but consistent
    timestampIso8601 = "2026-04-07T00:00:00Z",

    -- Contract references
    moodContractId   = "mood-outer-threshold-01", -- optional
    eventContractId  = "event-ritual-door-03",   -- optional
    runContractId    = "run-guided-ritual-01",   -- optional (if not already bound)
    selectorPatternId= "pat-dungeon-liminal-01", -- optional

    -- Stage / structure hints
    stage            = "OuterThreshold", -- enum: OuterThreshold, Locus, Rupture, Fallout, Epilogue
    branchLabel      = "accepted-invitation",    -- optional designer label

    -- Metrics snapshot (from H.Metrics)
    metrics = {
      UEC  = 0.62,
      EMD  = 0.31,
      STCI = 0.55,
      CDL  = 0.40,
      ARR  = 0.78
    },

    -- Optional invariant snapshot for analysis
    invariantsSnapshot = {
      CIC  = 0.58,
      AOS  = 0.44,
      DET  = 3.8,
      SHCI = 0.76
    },

    -- Optional safety / budget linkage
    budgetBand          = "near-soft-cap", -- from H.Budget.snapshot / consume
    boundaryDecision    = "soften",        -- allow | soften | imply | refuse
    redactionProfileId  = "rp-default-soften"
  }
  ```

The metrics block should usually be populated by calling `H.Metrics.snapshot(sessionId)` or an equivalent helper, rather than computed manually.

### 4.3 Successful result

```lua
result.ok   == true
result.data = {
  routeId   = "rt-sess-xyz-001",
  stepIndex = 1
}
```

### 4.4 Error conditions

- `ROUTE_NOT_FOUND` – `routeId` unknown for this `sessionId`.
- `STEP_INDEX_DUPLICATE` – a step with the same `stepIndex` already exists.
- `CONTRACT_REF_INVALID` – referenced contract IDs fail registry lookup.

---

## 5. H.Route.finalize

### 5.1 Purpose

`H.Route.finalize` seals the route and prepares a telemetry object conforming to `ai-chat-session-route-v1.json`. It computes any derived metrics (comfort outcome, summary bands), ensures all mandatory fields are present, and returns the final route object for writing to NDJSON.

It should be called once at the end of the session, after all steps have been appended.

### 5.2 Parameters

```lua
local result = H.Route.finalize(sessionId, routeId, outcomeContext)
```

- `sessionId : string`  
  As above.

- `routeId : string`  
  Route identifier returned by `H.Route.start`.

- `outcomeContext : table`  
  Optional fields to override or supplement computed summaries:

  ```lua
  outcomeContext = {
    -- Session-level comfort outcome, as defined in Telemetry module
    comfortOutcome = "within-band", -- within-band | too-intense | too-flat

    -- Optional user-initiated termination reason
    terminationReason = "user-opt-out", -- user-opt-out | natural-end | safety-stop

    -- Optional final metrics / band summaries
    finalMetrics = {
      UEC  = 0.65,
      EMD  = 0.38,
      STCI = 0.57,
      CDL  = 0.42,
      ARR  = 0.80
    }
  }
  ```

If `comfortOutcome` is omitted, `H.Route.finalize` is expected to derive it from the accumulated metrics and consent / budget signals, using the Telemetry module’s comfort-outcome logic.

### 5.3 Successful result

```lua
result.ok == true

-- result.data is a Lua table that matches ai-chat-session-route-v1.json:
-- {
--   schemaRef: "ai-chat-session-route-v1",
--   sessionId: "sess-xyz",
--   aiChatHorrorProfileId: "achp-guided-ritual-01",
--   directorPersonaId: "dir-cold-overseer",
--   mode: "GuidedRitual",
--   regionId: "reg-...",
--   runContractId: "run-guided-ritual-01",
--   steps: [
--     {
--       stepIndex: 1,
--       timestamp: "...",
--       moodContractId: "mood-...",
--       eventContractId: "event-...",
--       stage: "OuterThreshold",
--       metrics: { UEC: 0.62, ... },
--       invariantsSnapshot: { CIC: 0.58, ... },
--       budgetBand: "near-soft-cap",
--       boundaryDecision: "soften"
--     },
--     ...
--   ],
--   summary: {
--     comfortOutcome: "within-band",
--     metricsOverTime: {
--       UEC: [0.52, 0.62, 0.60],
--       EMD: [...],
--       STCI: [...],
--       CDL: [...],
--       ARR: [...]
--     }
--   }
-- }
```

Implementations may choose not to expand `metricsOverTime` if the NDJSON strategy prefers a separate metrics stream, but the schema should allow at least a minimal summary consistent with the session-metrics envelope.

### 5.4 Error conditions

- `ROUTE_NOT_FOUND` – `routeId` unknown or already finalized.
- `ROUTE_EMPTY` – no steps were appended before finalize.
- `METRICS_MISSING` – steps lack metrics fields required by schema.

---

## 6. Recommended call order

### 6.1 Runtime (engine) loop

For a governed AI-Chat horror run, the engine or host should:

1. Bind consent and budget:

   - `H.Consent.bindSession(sessionId, consentProfileId)`
   - `H.Budget.initSession(sessionId, consentState)`

2. Resolve AI-Chat horror profile and director persona:

   - Load `aiChatHorrorProfile` contract for this session.
   - Optionally load `directorPersonaContract`.
   - Call `H.Route.start(sessionContext, profileContext)`.

3. For each narrative beat / turn:

   - Plan beat: `H.Consent.currentState`, `H.Budget.snapshot`, `H.Banter.planResponse`, `H.Selector.selectPattern`.
   - Generate candidate output and evaluate via `H.Safety.evaluateTurn`.
   - Commit budget: `H.Budget.consume`.
   - Append route step: `H.Route.appendStep(sessionId, routeId, stepContext)`.

4. At session end:

   - Call `H.Route.finalize(sessionId, routeId, outcomeContext)` and hand the resulting table to the NDJSON writer.
   - Optionally emit `session-metrics-envelope-v1` via existing Telemetry APIs.

### 6.2 AI-Chat and coding agents

AI-Chat and coding agents should treat route emission as a hard requirement, not an optional extra:

- Before narrating horror events, ensure `H.Route.start` has been called and a `routeId` is present in the session context.
- After each significant branch or contract switch, call `H.Route.appendStep` with the current contract IDs, stage, and metrics from `H.Metrics`.
- When users opt out or sessions reach a narrative end, trigger `H.Route.finalize` with an appropriate `terminationReason`.

This keeps entertainment metrics (UEC, EMD, STCI, CDL, ARR) and contract usage aligned with session telemetry, enabling replayable, analyzable runs without exposing raw horror content.
