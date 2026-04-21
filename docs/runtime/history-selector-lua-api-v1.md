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
  - H.Selector
  - H.Node
schema-refs:
  - history-selector-rule-v1.json
  - history-selector-pattern-v1.json
  - history-selector-decision-event-v1.json
---

# History Selector Lua API v1

This document defines the Lua surface for the History-Aware Content Selector (Module 7) and its node-level helpers. It extends the unified `H.` runtime API described in `docs/runtime/horror-engine-lua-api-v1.md` and is intended for both engine code and AI-Chat tools.

The selector API is responsible for turning geo-historical invariants and entertainment metrics into concrete pattern and node choices. It never invents content: it only binds existing contracts and invariants.

---

## 1. Namespaces and responsibilities

Two primary namespaces are exposed at runtime:

- `H.Selector`  
  Pattern-level selector that works at the region / beat level. It reads selector rules, selector patterns, consent state, budget, and entertainment metrics to choose a `selectorPatternId` and record decisions.

- `H.Node`  
  Node-level helper for dungeon or graph-based experiences. It reads node contracts and invariants and applies a concrete `historySelectorPattern` to rank and choose the next node when "drilling into" a design session or runtime walk.

All functions in this document return the standard `ok / data / error` envelope used by the unified `H.` API.

```lua
-- Standard result envelope
local result = H.Selector.selectPattern(sessionId, regionId, context)

-- Envelope shape:
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

## 2. H.Selector – pattern-level API

The pattern-level API is defined at a high level in `horror-engine-lua-api-v1.md`. This section restates the two core calls that matter to node selection.

### 2.1 `H.Selector.selectPattern(sessionId, regionId, context)`

Selects a history-aware pattern for the next beat in a session.

**Parameters**

- `sessionId : string`  
  Opaque session identifier. Must match telemetry envelopes and consent / budget state.

- `regionId : string`  
  Region / POI identifier for the current segment. Shared with region and seed contracts.

- `context : table`  
  Selector context, typically including:
  - `descriptorId : string` – ID of the selector descriptor using this rule set.  
  - `consentTier : string` – `minor`, `adult-basic`, `adult-horror`, `research`.  
  - `routerStateId : string` – age-tier router state ID.  
  - `budgetSnapshot : table` – from `H.Budget.snapshot`.  
  - `banterPlan : table` – from `H.Banter.planResponse`.  
  - Optional `patternHints` such as desired mood, experience preset, or director persona ID.

**Successful result**

```lua
local result = H.Selector.selectPattern(sessionId, regionId, context)

-- result.ok == true
-- result.data = {
--   selectorPatternId = "pat-dungeon-liminal-hooks-01",
--   templateId        = "tmpl-dungeon-guided-walkthrough",
--   invariants        = {
--     CIC  = number,
--     AOS  = number,
--     DET  = number,
--     SHCI = number
--   },
--   metrics           = {
--     UEC  = number,
--     EMD  = number,
--     STCI = number,
--     CDL  = number,
--     ARR  = number
--   },
--   deadLedgerRef     = "DLN.bundle.region.example.v1.0.0"
-- }
```

A successful result provides a `selectorPatternId` that may later be passed to `H.Node.score_next_from_history` / `H.Node.choose_next` for dungeon-style flows.

**Error result**

Typical error codes:

- `SESSION_NOT_FOUND` – session ID unknown to consent / budget modules.  
- `REGION_NOT_FOUND` – region ID not present in registry or invariants.  
- `NO_ELIGIBLE_PATTERN` – all patterns excluded by selector rules or consent / budget caps.  

### 2.2 `H.Selector.recordDecision(sessionId, selectorDecision, safetyDecision, budgetEventId)`

Records a selector decision and links it to safety and budget events. Uses the `history-selector-decision-event-v1.json` schema.

**Parameters**

- `sessionId : string` – as above.  
- `selectorDecision : table` – includes `selectorPatternId`, `ruleCatalogSnapshot`, and any conflicts resolved.  
- `safetyDecision : table | nil` – optional content boundary outcome for the same beat.  
- `budgetEventId : string | nil` – ID of the associated budget consumption event.

**Successful result**

```lua
local result = H.Selector.recordDecision(sessionId, selectorDecision, safetyDecision, budgetEventId)

-- result.ok == true
-- result.data = {
--   eventId = "hsel-decision-xyz123"
-- }
```

---

## 3. H.Node – dungeon node helpers

The `H.Node` helper is a small, runtime-facing Lua module (typically in `engine/library/horror_nodes.lua`) that makes selector patterns useful for dungeon or graph-based flows. It operates strictly on top of contracts and invariants:

- Reads `dungeon-node-contract.v1` cards via a contract loader.  
- Reads invariants via `H.CIC`, `H.AOS`, `H.SPR`, `H.SHCI`, `H.LSG`.  
- Applies a `historySelectorPattern` to rank neighbor nodes and choose a next step.

Engine code and AI-Chat should treat `H.Node` as the only way to describe or traverse dungeon nodes in a selector-aware way.

### 3.1 `H.Node.describe(regionId, nodeId, sessionId?)`

Returns a compact description of a node, suitable for AI-Chat narration and debugging.

**Parameters**

- `regionId : string`  
  Region in which this node’s contract is registered.

- `nodeId : string`  
  Node identifier matching the dungeon node contract.

- `sessionId : string | nil`  
  Optional; if provided, `H.Node.describe` may include current entertainment metrics for the session in this region.

**Successful result**

```lua
local result = H.Node.describe(regionId, nodeId, sessionId)

-- result.ok == true
-- result.data = {
--   id            = "node-entrance-01",
--   name          = "Collapsed Service Corridor",
--   role          = "liminal", -- e.g., connector, hub, high_risk, secret
--   liminalTags   = { "liminal:service_corridor", "liminal:threshold" },
--   mechanicHooks = { "sanity_tick", "audio_glitch" },
--   topology      = {
--     connections = {
--       { targetNodeId = "node-stairwell-02", kind = "standard" },
--       { targetNodeId = "node-maintenance-03", kind = "side_branch" }
--     }
--   },
--   invariants = {
--     CIC  = number,
--     AOS  = number,
--     SPR  = number,
--     SHCI = number,
--     LSG  = number
--   },
--   metrics = {
--     UEC  = number,
--     EMD  = number,
--     STCI = number,
--     CDL  = number,
--     ARR  = number
--   },
--   feel = "heavy_imprint",      -- or "uneasy", "relatively_calm"
--   hints = {
--     liminal   = "This node sits on a liminal structure: liminal:service_corridor, liminal:threshold",
--     mechanics = "Mechanic hooks available here: sanity_tick, audio_glitch"
--   }
-- }
```

**Error result**

- `NODE_NOT_FOUND` – no contract for the given node ID in this region.  

**Typical usage**

- AI-Chat uses `H.Node.describe` to introduce the current node in natural language.  
- Engine debug UIs call it to show why a node feels “off” (e.g., high CIC / AOS or strong LSG spikes).

### 3.2 `H.Node.score_next_from_history(sessionId, regionId, currentNodeId, patternId)`

Scores neighbor nodes of `currentNodeId` using a concrete `historySelectorPattern` (e.g., one that prefers liminal structures and specific mechanic hooks).

This call does **not** choose a node; it returns a ranked list with reasons.

**Parameters**

- `sessionId : string`  
  Session for which this traversal is occurring (used for metrics and logging).

- `regionId : string`  
  Region containing the current node and its neighbors.

- `currentNodeId : string`  
  Node whose `topology.connections` will be treated as candidate edges.

- `patternId : string`  
  ID of a pattern defined in `history-selector-pattern-v1.json` (for example, `hsp-dungeon-liminal-hooks.v1`).

**Successful result**

```lua
local result = H.Node.score_next_from_history(sessionId, regionId, currentNodeId, patternId)

-- result.ok == true
-- result.data = {
--   currentNodeId = "node-entrance-01",
--   patternId     = "hsp-dungeon-liminal-hooks.v1",
--   ranked        = {
--     {
--       nodeId = "node-stairwell-02",
--       score  = 2.35,
--       reasons = {
--         liminalTags   = { "liminal:stairwell" },
--         mechanicHooks = { "sanity_tick", "memory_overlay" },
--         shci          = 0.84
--       }
--     },
--     {
--       nodeId = "node-maintenance-03",
--       score  = 1.10,
--       reasons = {
--         liminalTags   = {},
--         mechanicHooks = { "light_drop" },
--         shci          = 0.72
--       }
--     }
--   }
-- }
```

The scoring logic is defined entirely by the pattern’s `mechanicHookPreferences`, `liminalTagPreferences`, and `historyCoupling` blocks, interpreted against each neighbor’s contract and local invariants.

**Error result**

- `PATTERN_NOT_FOUND` – unknown `patternId` in pattern registry.  
- `NODE_NO_CONNECTIONS` – `currentNodeId` has no `topology.connections` in its contract.  

**Typical usage**

- Engine tools call this during design-time to visualize how a pattern would steer traversal.  
- AI-Chat uses the ranked list to explain why certain options are “more haunted” or better aligned with the current run’s intent.

### 3.3 `H.Node.choose_next(sessionId, regionId, currentNodeId, patternId)`

Chooses the “best” next node according to the same scoring model as `H.Node.score_next_from_history`, and returns both the `nextNodeId` and a short, explanation string suitable for narration.

**Parameters**

Same as `H.Node.score_next_from_history`.

**Successful result**

```lua
local result = H.Node.choose_next(sessionId, regionId, currentNodeId, patternId)

-- result.ok == true
-- result.data = {
--   nextNodeId  = "node-stairwell-02",
--   patternId   = "hsp-dungeon-liminal-hooks.v1",
--   explanation = "Next node chosen because of liminal structure (liminal:stairwell); mechanic hooks (sanity_tick, memory_overlay); history coupling SHCI=0.84.",
--   debug       = {
--     ranked = {
--       -- same structure as score_next_from_history().data.ranked
--     }
--   }
-- }
```

**Error result**

- `NO_NEXT_NODE` – no valid neighbor nodes after filtering / scoring.  

**Typical usage**

- AI-Chat guided runs:  
  1. Call `H.Selector.selectPattern` to pick a pattern for the next beat.  
  2. Call `H.Node.choose_next` with the returned `selectorPatternId`.  
  3. Call `H.Node.describe` on the returned `nextNodeId` and narrate from that description.  
  4. Log the decision via `H.Selector.recordDecision` and appropriate telemetry events.

---

## 4. Engine and AI-Chat expectations

Engines and AI-Chat tools using these APIs should follow a consistent sequence to keep selector logic aligned with consent, budget, and safety modules. [file:9]  

### 4.1 Runtime loop (per turn / step)

1. Query consent and budget:

   - `H.Consent.currentState(sessionId)`  
   - `H.Budget.snapshot(sessionId)`

2. Plan selector pattern:

   - `H.Banter.planResponse(sessionId, labels, context)`  
   - `H.Selector.selectPattern(sessionId, regionId, selectorContext)`

3. Choose node (for dungeon-style runs):

   - `H.Node.choose_next(sessionId, regionId, currentNodeId, selectorPatternId)`  
   - `H.Node.describe(regionId, nextNodeId, sessionId)` for narration.

4. Evaluate and record:

   - `H.Safety.evaluateTurn(sessionId, candidateOutput, context)`  
   - `H.Budget.consume(sessionId, modality, amount, context)`  
   - `H.Selector.recordDecision(sessionId, selectorDecision, safetyDecision, budgetEventId)`

### 4.2 AI-Chat usage

AI-Chat agents must not infer history or node structure directly from prose. Instead, they should:

- Use `H.Node.describe` as the authoritative description of the current node.  
- Use `H.Node.choose_next` as the sole arbiter of which node to visit next under a given pattern.  
- Use the `explanation` field from `H.Node.choose_next` to justify transitions in-character, keeping horror logic aligned with actual invariants (CIC, AOS, SHCI, LSG) and pattern preferences (liminal tags, mechanic hooks).

---

## 5. Implementation notes and next steps

This API stub assumes:

- `H.Selector` is backed by `history-selector-rule-v1.json` and `history-selector-pattern-v1.json`.  
- `H.Node` is implemented as `engine/library/horror_nodes.lua` and uses a contract loader (`Contracts.node(regionId, nodeId)`) plus `H.CIC`, `H.AOS`, `H.SPR`, `H.SHCI`, and `H.LSG` for invariant snapshots. [file:19]  

Next incremental slices that fit this doc:

- A small `dungeon-node-contract-v1.json` schema clarifying `liminalTags`, `mechanicHooks`, and `topology.connections`.  
- A selector‑telemetry explainer (`docs/runtime/history-selector-telemetry-analysis-v1.md`) showing how node choices and pattern IDs appear in NDJSON and how to analyze their effect on UEC / EMD / ARR. [file:9]
