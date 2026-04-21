---
invariants-used: [CIC, MDI, AOS, RRM, FCF, SPR, RWF, DET, HVF, LSG, SHCI]
metrics-used: [UEC, EMD, STCI, CDL, ARR]
modules: [H.Selector, H.Node, H.Director, H.ImageHelper, H.Metrics]
schema-refs:
  - h-context-envelope-v1.json
---

# Uniform Context Table for H. Modules v1

This document defines the canonical `context` table shape that all public H. decision functions must accept as their first parameter. The goal is to avoid parameter explosion, enable consistent telemetry, and simplify cross‑module wiring.

All new H. APIs must adopt this pattern; existing functions should be migrated in backward‑compatible phases.

---

## 1. Baseline context shape

```lua
local ctx = {
  -- Session & run identity (required)
  sessionId         = "sess-abc123",      -- opaque session identifier
  runId             = "run-xyz789",       -- run contract identifier

  -- Spatial context (required for node/region operations)
  regionId          = "reg-mansion-v1",   -- region contract ID
  tileId            = "tile-lobby-01",    -- optional tile/seed ID
  nodeId            = "node-stairwell-02",-- optional dungeon node ID

  -- Pattern & persona context (optional)
  patternId         = "hsp-liminal-hooks-v1", -- selector pattern ID
  directorPersonaId = "dir-cold-overseer-01", -- active director persona
  experienceContractId = "exp-guided-ritual-01", -- experience contract

  -- Stage & difficulty (optional but recommended)
  stage             = "Outer",            -- event stage: Outer/Locus/Rupture/Fallout
  difficulty        = "Standard",         -- from runContract: Novice/Standard/Severe

  -- Router & entitlement state (optional)
  routerStateId     = "rtr-mature-v1",    -- age-tier router state
  deadLedgerDecisionToken = "dlt-a1b2c3d4", -- optional ZKP entitlement token

  -- Module-specific extensions (namespaced to avoid collisions)
  _selector = { hints = { preferLiminal = true } },
  _director = { stylePreference = "clinical" },
  _imageHelper = { inventory = { tools = { "flashlight" } } }
}
```

### Field constraints

| Field | Required | Type | Notes |
|-------|----------|------|-------|
| `sessionId` | Yes | string | Must match consent/budget state. |
| `runId` | Yes | string | Binds to runContract for metric targets. |
| `regionId` | Yes | string | Region contract identifier. |
| `tileId` | No | string | Tile/seed identifier for invariant sampling. |
| `nodeId` | No | string | Dungeon node identifier for graph traversal. |
| `patternId` | No | string | History-selector pattern ID. |
| `directorPersonaId` | No | string | Active director persona contract ID. |
| `experienceContractId` | No | string | Experience contract for metric envelopes. |
| `stage` | No | enum | Event stage for pacing logic. |
| `difficulty` | No | enum | Player difficulty for content gating. |
| `routerStateId` | No | string | Age-tier router state for entitlement. |
| `deadLedgerDecisionToken` | No | string | ZKP token for attested content access. |

---

## 2. Module adoption patterns

### 2.1 H.Selector

```lua
-- Before (legacy)
H.Selector.selectPattern(sessionId, regionId, context)

-- After (uniform)
H.Selector.selectPattern(ctx, selectorHints)
-- Reads ctx.sessionId, ctx.regionId, ctx.patternId, ctx.directorPersonaId internally
```

### 2.2 H.Node

```lua
-- Before (legacy)
H.Node.describe(regionId, nodeId, sessionId)

-- After (uniform)
H.Node.describe(ctx)
-- Reads ctx.regionId, ctx.nodeId, ctx.sessionId internally
```

### 2.3 H.Director

```lua
-- Before (legacy)
H.Director.applySafetyDecision(sessionId, safetyDecision, budgetSnapshot)

-- After (uniform)
H.Director.applySafetyDecision(ctx, safetyDecision, budgetSnapshot)
-- Reads ctx.directorPersonaId, ctx.comfortPolicyRef, ctx.deadLedgerDecisionToken internally
```

### 2.4 H.ImageHelper

```lua
-- Uniform from start
H.ImageHelper.describe(ctx, invariants, metrics, inventory)
```

---

## 3. CI enforcement rules

Codebase‑of‑Death CI must enforce:

1. **Signature discipline**: All public H. functions must accept `ctx` as their first parameter (plus module‑specific extras). Low‑level helpers not exported in the H. surface are exempt.

2. **Field validation**: Functions that require specific context fields (e.g., `H.Node.describe` needs `regionId` and `nodeId`) must assert their presence and return structured errors if missing.

3. **Namespacing**: Module‑specific extensions must use underscore‑prefixed keys (e.g., `_selector`, `_director`) to avoid collisions.

4. **Telemetry correlation**: Every H. function that emits telemetry must include `ctx.sessionId`, `ctx.runId`, and `ctx.regionId` in its events for offline analysis.

---

## 4. Extension protocol

When a new concern requires additional context data:

1. Add the field to this baseline shape (e.g., `deadLedgerDecisionToken` for entitlement).
2. Update module implementations to read the field where needed.
3. Bump the schema version in `h-context-envelope-v1.json` and document the change in this file.

This keeps signature changes localized to the context definition, not scattered across every H. function.

---

## 5. Example: Dead‑Ledger token propagation

```lua
-- Step 1: Request entitlement
local dlResult = Policy.DeadLedger.requestBundleAccess(deadLedgerRef, {
  sessionId = ctx.sessionId,
  scope = "dungeonRunContract"
})

-- Step 2: Attach token to context
if dlResult.ok then
  ctx.deadLedgerDecisionToken = dlResult.decisionToken
end

-- Step 3: Pass context to downstream modules
local selectorResult = H.Selector.selectPattern(ctx, hints)
local directorResult = H.Director.applySafetyDecision(ctx, safetyDecision, budget)

-- Modules internally check ctx.deadLedgerDecisionToken when operating on attested bundles
```

This pattern ensures entitlement decisions are explicit, machine‑checked, and auditable via telemetry.
