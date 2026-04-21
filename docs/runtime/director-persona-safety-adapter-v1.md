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
  - H.Director
  - H.Budget
  - H.Safety
  - H.Consent
schema-refs:
  - director-persona-contract.v1.json
  - director-comfort-policy.v1.json
  - consent-state-machine.v1.json
  - horror-intensity-budget-policy.v1.json
  - content-boundary-priority-rules-v1.json
  - entertainment-metrics.v1.json
---

# Director Persona & Safety Adapter API v1

This document defines the Lua surface for director personas (e.g., "Cold Overseer") and their safety adapter. It binds director persona contracts, consent, budget, and content boundary decisions into a single runtime interface that AI-Chat and engines can use without touching raw policy tables.

Director personas never generate horror content directly. They orchestrate which contracts are active and how safety decisions appear in-world.

---

## 1. Concepts and contracts

### 1.1 Director persona contracts

A director persona is a runtime contract (objectKind `directorPersonaContract`) that declares:

- **Identification**
  - `id` – stable persona ID.
  - `directorKind` – e.g., `coldOverseer`, `ritualStatistician`, `silentArchivistDirector`.
  - `toneLabel` – short label for AI-Chat style (e.g., `clinical`, `ritual`, `archivist`).
  - `allowedTier` – which content tiers this persona may operate in.

- **Invariant bands**
  - `invariantBands` – min/max bands for key invariants (e.g., CIC, AOS, DET, SHCI), consistent with spine and tier ceilings.

- **Metric targets**
  - `metricTargets` – bands for UEC, EMD, STCI, CDL, ARR that define the persona’s intended feel.

- **Comfort policy**
  - `comfortPolicyRef` – reference to a comfort policy that constrains per-session DET, cooldown ratios, and allowed CDL span.

These contracts are validated by the same invariant and metric machinery as other cards and must sit inside spine-defined bands. [file:9]  

### 1.2 Safety adapter responsibilities

The safety adapter functions in `H.Director` are responsible for:

- Reading consent and budget state.
- Interpreting content boundary decisions.
- Mapping internal decisions onto a small set of **strategy verbs** that persona code must respect:
  - `allow`
  - `soften`
  - `imply`
  - `refuse`
  - `downshift` (optional hint to lower intensity within the same beat)

AI-Chat and engine persona scripts use these verbs instead of branching on raw flags from modules 1 (Consent), 3 (Budget), and 4 (Safety). [file:9]  

---

## 2. Namespace overview

The `H.Director` namespace exposes three primary functions:

- `H.Director.loadPersona(sessionContext, directorPersonaId)`  
  Load a director persona contract and compute effective invariant/metric envelopes for this session.

- `H.Director.constrainMetrics(sessionId, intendedMetrics)`  
  Clamp intended UEC/EMD/STCI/CDL/ARR values to persona, consent, and comfort policy bands.

- `H.Director.applySafetyDecision(sessionId, safetyDecision, budgetSnapshot)`  
  Translate boundary and budget outcomes into strategy verbs for persona behavior.

All functions use the standard `ok / data / error` envelope used by other `H.` namespaces.

```lua
-- Standard envelope
local result = H.Director.loadPersona(sessionContext, directorPersonaId)

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

---

## 3. H.Director.loadPersona

### 3.1 Signature

```lua
--- Load director persona for a session and compute effective envelopes.
-- @param sessionContext table
--   {
--     sessionId        = "sess-...",
--     consentTier      = "minor" | "adult-basic" | "adult-horror" | "research",
--     regionId         = "reg-...",
--     aiChatProfileId  = "acp-...",  -- optional link to AI-chat horror profile
--     tier             = "standard" | "mature" | "research"
--   }
-- @param directorPersonaId string
-- @return result table (standard envelope)
function H.Director.loadPersona(sessionContext, directorPersonaId) end
```

### 3.2 Behavior

`H.Director.loadPersona` must:

1. Load the `directorPersonaContract` by ID and verify that:
   - The session tier is allowed for this persona.
   - The session consent tier is compatible with persona invariant and metric bands.

2. Apply comfort policy referenced by `comfortPolicyRef`:
   - Compute `maxDetPerSession`.
   - Compute `requiredCooldownRatio` (low-DET vs high-DET segments).
   - Compute allowed CDL span for this session.

3. Intersect persona bands with:
   - Consent caps from Module 1.
   - Budget and safety caps from Modules 3 and 4, if available.

### 3.3 Successful result

```lua
local result = H.Director.loadPersona(sessionContext, directorPersonaId)

-- result.ok == true
-- result.data = {
--   directorPersonaId = "dir-cold-overseer-01",
--   directorKind      = "coldOverseer",
--   toneLabel         = "clinical",
--   allowedTier       = "mature",
--   invariantEnvelope = {
--     CIC  = { min = 0.5, max = 0.8 },
--     AOS  = { min = 0.4, max = 0.9 },
--     DET  = { min = 3.0, max = 6.0 },
--     SHCI = { min = 0.7, max = 1.0 }
--   },
--   metricEnvelope = {
--     UEC  = { min = 0.4, max = 0.7 },
--     EMD  = { min = 0.1, max = 0.3 },
--     STCI = { min = 0.4, max = 0.7 },
--     CDL  = { min = 0.5, max = 0.7 },
--     ARR  = { min = 0.5, max = 0.7 }
--   },
--   comfortPolicy = {
--     maxDetPerSession    = 6.0,
--     requiredCooldown    = { lowDetMinutes = 2, highDetMinutes = 1 },
--     allowedCdlSpan      = { min = 0.5, max = 0.7 }
--   }
-- }
```

### 3.4 Error cases

- `DIRECTOR_NOT_FOUND` – unknown director persona ID.  
- `TIER_NOT_ALLOWED` – session tier not permitted by persona.  
- `CONSENT_INCOMPATIBLE` – persona bands cannot be reconciled with consent tier.  

Persona and AI-Chat code should treat any error as a hard failure and fall back to a safe, non-horror persona or refuse the run.

---

## 4. H.Director.constrainMetrics

### 4.1 Signature

```lua
--- Clamp intended metrics to persona, consent, and comfort policy bands.
-- @param sessionId string
-- @param intendedMetrics table
--   {
--     UEC  = number | nil,
--     EMD  = number | nil,
--     STCI = number | nil,
--     CDL  = number | nil,
--     ARR  = number | nil
--   }
-- @return result table (standard envelope)
function H.Director.constrainMetrics(sessionId, intendedMetrics) end
```

### 4.2 Behavior

`H.Director.constrainMetrics` must:

1. Look up the active director persona envelope bound to `sessionId`.  
2. Pull current consent caps, budget bands, and ethics guardrails.  
3. Compute the effective band per metric as an intersection of:
   - Persona metricTargets band.
   - Consent / budget caps.
   - Ethics guardrail caps.

4. Clamp each `intendedMetrics` value into the effective band, leaving `nil` entries unchanged.

### 4.3 Successful result

```lua
local result = H.Director.constrainMetrics(sessionId, {
  UEC = 0.8,
  CDL = 0.3
})

-- result.ok == true
-- result.data = {
--   effectiveMetrics = {
--     UEC = 0.7,  -- clamped to persona/consent band
--     EMD = nil,
--     STCI = nil,
--     CDL = 0.5,  -- clamped up to minimum allowed CDL
--     ARR = nil
--   },
--   bands = {
--     UEC = { min = 0.4, max = 0.7 },
--     CDL = { min = 0.5, max = 0.7 }
--   }
-- }
```

Persona code and AI-Chat should treat `effectiveMetrics` as authoritative for template, pattern, or event selection.

### 4.4 Error cases

- `DIRECTOR_NOT_BOUND` – no persona has been loaded for this session.  
- `SESSION_NOT_FOUND` – unknown session ID.  

---

## 5. H.Director.applySafetyDecision

### 5.1 Signature

```lua
--- Map boundary and budget outcomes to persona strategy verbs.
-- @param sessionId string
-- @param safetyDecision table
--   {
--     decision          = "allow" | "soften" | "imply" | "refuse",
--     redactionProfile  = "rp-..." | nil,
--     guardrailsOk      = boolean,
--     violations        = table | nil,
--     boundaryBand      = string | nil  -- e.g., "near-soft-cap"
--   }
-- @param budgetSnapshot table
--   -- From H.Budget.snapshot(sessionId)
-- @return result table (standard envelope)
function H.Director.applySafetyDecision(sessionId, safetyDecision, budgetSnapshot) end
```

### 5.2 Behavior

`H.Director.applySafetyDecision` must:

1. Inspect the `safetyDecision` (Module 4) and `budgetSnapshot` (Module 3). [file:9]  
2. Combine them with the director’s comfort policy to determine a **strategy verb** and optional **style hint**:

   Example mapping:

   - If boundary `decision = "allow"` and budget below soft cap → `strategy = "allow"`.  
   - If `decision = "soften"` or `boundaryBand = "near-soft-cap"` → `strategy = "soften"` or `strategy = "downshift"`.  
   - If `decision = "imply"` → `strategy = "imply"`.  
   - If `decision = "refuse"` or comfort policy exceeded → `strategy = "refuse"`.

3. Optionally set a `styleHint` for AI-Chat, such as `gentle`, `clinical`, `ominous`, while staying inside persona tone.

### 5.3 Successful result

```lua
local result = H.Director.applySafetyDecision(sessionId, safetyDecision, budgetSnapshot)

-- result.ok == true
-- result.data = {
--   strategy  = "soften",  -- allow | soften | imply | refuse | downshift
--   styleHint = "ritual",  -- optional hint for persona phrasing
--   reason    = "boundary-soft-cap-and-budget-near-limit"
-- }
```

Persona code must only branch on `strategy` and optional `styleHint`. It must not inspect raw `violations` or budget fields directly.

### 5.4 Error cases

- `DIRECTOR_NOT_BOUND` – director persona not loaded for this session.  
- `INVALID_SAFETY_PAYLOAD` – missing or malformed safety decision data.  

---

## 6. Recommended call order for AI-Chat runs

A typical AI-Chat horror session using a director persona should follow this sequence:

1. **Session start**
   - Bind consent and budget modules:
     - `H.Consent.bindSession(sessionId, consentProfileId)`
     - `H.Budget.initSession(sessionId, consentState)`
   - Load director persona:
     - `H.Director.loadPersona(sessionContext, directorPersonaId)`

2. **Per turn planning**
   - Get consent state and budget snapshot:
     - `H.Consent.currentState(sessionId)`
     - `H.Budget.snapshot(sessionId)`
   - Compute or refine intended metrics for this turn (e.g., from experience presets).
   - Clamp metrics:
     - `H.Director.constrainMetrics(sessionId, intendedMetrics)`
   - Choose patterns / templates consistent with `effectiveMetrics`.

3. **Candidate output + safety**
   - Build candidate output via AI-Chat templates and persona tone.
   - Evaluate boundary:
     - `H.Safety.evaluateTurn(sessionId, candidateOutput, context)`
   - Apply director strategy:
     - `H.Director.applySafetyDecision(sessionId, safetyDecision, budgetSnapshot)`
   - Render final in-world response based on `strategy` and `styleHint`.

4. **Logging**
   - Consume budget:
     - `H.Budget.consume(sessionId, modality, amount, context)`
   - Log selector and director decisions via existing telemetry and director-specific fields.

---

## 7. Implementation notes

- The actual loading of contracts (director persona and comfort policy) is handled by internal vault code; this spec only defines the public Lua surface.  
- All invariants and metrics used here must be read through the unified H. runtime (e.g., `H.metrics`, `H.CIC`, selectors), not through raw vault tables.  
- CI should enforce that persona scripts only touch director, consent, budget, and safety state via these functions.
