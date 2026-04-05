---
title: The VM-Constellation of Horror$Place – Systemic Timing Surprise Mechanics v1
version: 1.0.0
doctype: spec-v1
schemaref:
  - schema.HorrorPlace-Constellation-Contracts.surpriseMechanicContract.v1.json
  - schema.HorrorPlace-Constellation-Contracts.behaviorNodeContract.v1.json
  - schema.HorrorPlace-Constellation-Contracts.alnBindingContract.v1.json
  - schema.HorrorPlace-Constellation-Contracts.personaContract.v1.json
invariantsused:
  - CIC
  - MDI
  - AOS
  - DET
  - HVF
  - LSG
  - SHCI
metricsused:
  - UEC
  - EMD
  - STCI
  - CDL
  - ARR
tiers:
  - public
  - vault
  - lab
aiauthoringcontract: horrorplace-constellation-ai-authoring-contract-v1
---

# The VM-Constellation of Horror$Place – Systemic Timing Surprise Mechanics v1

## 1. Scope and Intent

This spec-sheet defines three Systemic Timing surprise mechanics as contract-ready patterns, plus cross-repo workflow strategies and a persona roster for tuning and seed-generation. The goal is to make timing-based surprise behavior a first-class, contract-driven surface in the VM-Constellation, explicitly bound to invariants (CIC/MDI/AOS/DET…) and entertainment metrics (UEC/EMD/STCI/CDL/ARR).

The mechanics described here target the `SystemicTiming` category of `surpriseMechanicContract.v1` and assume a canonical Lua runtime layer (`H.*`, `Telemetry.*`, `Events.*`) plus ALN bindings for behavior nodes.

---

## 2. Systemic Timing Mechanics (Category 2)

This section defines three Systemic Timing mechanics: Near-Trigger Abortion, Scheduled-but-Mutable Jump, and Delayed Payoff. Each has:

- A contract-level JSON fragment suitable for inclusion in `surpriseMechanicContract` instances.  
- A Lua stub sketch showing how invariants and metrics are consulted.  
- An ALN node stub for runtime binding.

### 2.1 SURP.SYSTM.NEARTRIG.ABORT.v1 – Near-Trigger Abortion

**Intent**

A near-event that arms under certain invariant conditions but frequently aborts just before execution when uncertainty and mystery are already high. This raises UEC/EMD/ARR while keeping DET under control and avoiding over-exposure.

**Contract fragment (mechanic-level config)**

```json
{
  "id": "SURP.SYSTM.NEARTRIG.ABORT.v1",
  "schemaref": "schema.HorrorPlace-Constellation-Contracts.surpriseMechanicContract.v1.json",
  "category": "SystemicTiming",
  "tier": "vault",
  "intensityBand": "moderate",
  "invariantPreconditions": {
    "CIC": { "min": 0.4, "max": 0.9 },
    "MDI": { "min": 0.3, "max": 0.8 },
    "AOS": { "min": 0.3, "max": 0.9 },
    "DET": { "min": 2.0, "max": 7.0 }
  },
  "metricIntent": {
    "UEC": [0.6, 0.9],
    "EMD": [0.6, 0.9],
    "STCI": [0.5, 0.8],
    "CDL": [0.4, 0.7],
    "ARR": [0.7, 1.0]
  },
  "detCaps": { "min": 0.0, "max": 7.0 },
  "lsgCaps": { "min": 0.2, "max": 0.9 },
  "shciMode": "BoundToRegion",
  "arrProfile": {
    "targetARR": { "min": 0.7, "max": 1.0 },
    "maxResolutionsPerSession": 2
  },
  "categoryHints": {
    "SystemicTiming": {
      "detTrajectory": "rampUp",
      "maxPeaksPerSession": 2
    }
  }
}
```

**Lua stub (sketch)**

```lua
-- engine/surprise/systemic_timing_neartrigger_abort.lua

local H = require("engine.horror_invariants")
local Metrics = require("engine.telemetry_metrics")
local Events = require("engine.events")

local NearTriggerAbort = {}

function NearTriggerAbort.try_arm_or_abort(region_id, player_id, dt)
  local det = H.DET(region_id, player_id)
  local cic = H.CIC(region_id)
  local mdi = H.MDI(region_id)
  local aos = H.AOS(region_id)

  if det < 2.0 or det > 7.0 then
    return false
  end
  if cic < 0.4 or cic > 0.9 or mdi < 0.3 or mdi > 0.8 or aos < 0.3 or aos > 0.9 then
    return false
  end

  local uec_before = Metrics.getUEC(region_id, player_id)
  local emd_before = Metrics.getEMD(region_id, player_id)

  local should_abort = (uec_before > 0.7 and emd_before > 0.7)

  if should_abort then
    Events.log("surprise.neartrigger_aborted", {
      region_id = region_id,
      player_id = player_id,
      det = det,
      cic = cic,
      mdi = mdi,
      aos = aos,
      uec = uec_before,
      emd = emd_before
    })
    return false
  else
    Events.schedule("surprise.neartrigger_arm", {
      region_id = region_id,
      player_id = player_id
    })
    return true
  end
end

return NearTriggerAbort
```

**ALN node stub**

```json
{
  "id": "ALN.Node.SystemicTiming.NearTriggerAbort.v1",
  "type": "LuaCall",
  "luaSymbol": "Lua.Engine.SystemicTiming.NearTriggerAbort",
  "parameters": [
    { "name": "region_id", "type": "string" },
    { "name": "player_id", "type": "string" },
    { "name": "dt", "type": "float" }
  ],
  "metricEffects": {
    "UEC": "increase",
    "EMD": "increase",
    "ARR": "increase"
  }
}
```

---

### 2.2 SURP.SYSTM.SCHEDJUMP.MUTABLE.v1 – Scheduled-but-Mutable Jump

**Intent**

A scheduled jump scare that can be deflected or re-timed based on current UEC and ARR, so the engine avoids firing when the player is already over-stressed or when ambiguity should be preserved.

**Contract fragment**

```json
{
  "id": "SURP.SYSTM.SCHEDJUMP.MUTABLE.v1",
  "schemaref": "schema.HorrorPlace-Constellation-Contracts.surpriseMechanicContract.v1.json",
  "category": "SystemicTiming",
  "tier": "vault",
  "intensityBand": "severe",
  "invariantPreconditions": {
    "CIC": { "min": 0.6, "max": 1.0 },
    "AOS": { "min": 0.5, "max": 1.0 },
    "DET": { "min": 3.0, "max": 9.0 }
  },
  "metricIntent": {
    "UEC": [0.5, 0.8],
    "EMD": [0.4, 0.7],
    "STCI": [0.6, 0.9],
    "CDL": [0.3, 0.6],
    "ARR": [0.3, 0.6]
  },
  "detCaps": { "min": 0.0, "max": 9.0 },
  "lsgCaps": { "min": 0.3, "max": 1.0 },
  "shciMode": "BoundToEventChain",
  "arrProfile": {
    "targetARR": { "min": 0.3, "max": 0.6 },
    "maxResolutionsPerSession": 1
  },
  "categoryHints": {
    "SystemicTiming": {
      "detTrajectory": "pulse",
      "maxPeaksPerSession": 1
    }
  }
}
```

**Lua stub**

```lua
-- engine/surprise/systemic_timing_scheduled_jump.lua

local H = require("engine.horror_invariants")
local Metrics = require("engine.telemetry_metrics")
local Events = require("engine.events")

local ScheduledJump = {}

function ScheduledJump.maybe_fire_or_deflect(region_id, player_id, dt)
  local det = H.DET(region_id, player_id)
  if det < 3.0 or det > 9.0 then
    return false
  end

  local uec = Metrics.getUEC(region_id, player_id)
  local arr = Metrics.getARR(region_id, player_id)

  local fire_prob = 0.5
  if uec < 0.4 then fire_prob = fire_prob + 0.2 end
  if arr < 0.3 then fire_prob = fire_prob - 0.2 end

  local r = math.random()
  if r < fire_prob then
    Events.trigger("surprise.jump_scare", {
      region_id = region_id,
      player_id = player_id
    })
    Events.log("surprise.schedjump_fired", {
      region_id = region_id,
      player_id = player_id,
      det = det,
      uec = uec,
      arr = arr
    })
    return true
  else
    Events.log("surprise.schedjump_deflected", {
      region_id = region_id,
      player_id = player_id,
      det = det,
      uec = uec,
      arr = arr
    })
    return false
  end
end

return ScheduledJump
```

**ALN node stub**

```json
{
  "id": "ALN.Node.SystemicTiming.ScheduledJumpMutable.v1",
  "type": "LuaCall",
  "luaSymbol": "Lua.Engine.SystemicTiming.ScheduledJumpMutable",
  "parameters": [
    { "name": "region_id", "type": "string" },
    { "name": "player_id", "type": "string" },
    { "name": "dt", "type": "float" }
  ]
}
```

---

### 2.3 SURP.SYSTM.DELAYEDPAYOFF.v1 – Delayed Payoff

**Intent**

Actions in one region schedule surprise payoffs in another region after a delay, binding cross-region CIC/MDI/DET context to later UEC/EMD/ARR effects.

**Contract fragment**

```json
{
  "id": "SURP.SYSTM.DELAYEDPAYOFF.v1",
  "schemaref": "schema.HorrorPlace-Constellation-Contracts.surpriseMechanicContract.v1.json",
  "category": "SystemicTiming",
  "tier": "lab",
  "intensityBand": "moderate",
  "invariantPreconditions": {
    "CIC": { "min": 0.3, "max": 0.8 },
    "MDI": { "min": 0.4, "max": 0.9 },
    "DET": { "min": 1.0, "max": 6.0 }
  },
  "metricIntent": {
    "UEC": [0.6, 0.9],
    "EMD": [0.7, 1.0],
    "STCI": [0.4, 0.7],
    "CDL": [0.5, 0.8],
    "ARR": [0.6, 0.9]
  },
  "detCaps": { "min": 0.0, "max": 6.0 },
  "lsgCaps": { "min": 0.0, "max": 0.8 },
  "shciMode": "LooseEcho",
  "arrProfile": {
    "targetARR": { "min": 0.6, "max": 0.9 },
    "maxResolutionsPerSession": 3
  },
  "categoryHints": {
    "SystemicTiming": {
      "detTrajectory": "rampUp",
      "maxPeaksPerSession": 3
    }
  }
}
```

**Lua stub**

```lua
-- engine/surprise/systemic_timing_delayed_payoff.lua

local H = require("engine.horror_invariants")
local Metrics = require("engine.telemetry_metrics")
local Events = require("engine.events")

local DelayedPayoff = {}
local pending = {}

function DelayedPayoff.schedule(region_src, region_dst, player_id, delay_seconds)
  local cic = H.CIC(region_src)
  local mdi = H.MDI(region_src)
  local det = H.DET(region_src, player_id)

  if cic < 0.3 or cic > 0.8 then return false end
  if mdi < 0.4 or mdi > 0.9 then return false end
  if det < 1.0 or det > 6.0 then return false end

  local t_now = os.time()
  table.insert(pending, {
    region_src = region_src,
    region_dst = region_dst,
    player_id = player_id,
    due_at = t_now + delay_seconds
  })

  Events.log("surprise.delayedpayoff_scheduled", {
    region_src = region_src,
    region_dst = region_dst,
    player_id = player_id,
    det = det
  })
  return true
end

function DelayedPayoff.tick(now)
  for i = #pending, 1, -1 do
    local item = pending[i]
    if now >= item.due_at then
      local uec = Metrics.getUEC(item.region_dst, item.player_id)
      local emd = Metrics.getEMD(item.region_dst, item.player_id)

      Events.trigger("surprise.delayedpayoff_trigger", {
        region_id = item.region_dst,
        player_id = item.player_id,
        uec = uec,
        emd = emd
      })

      Events.log("surprise.delayedpayoff_fired", {
        region_src = item.region_src,
        region_dst = item.region_dst,
        player_id = item.player_id,
        uec = uec,
        emd = emd
      })
      table.remove(pending, i)
    end
  end
end

return DelayedPayoff
```

**ALN nodes**

- `ALN.Node.SystemicTiming.DelayedPayoffSchedule.v1` → schedules payoff.  
- `ALN.Node.SystemicTiming.DelayedPayoffTick.v1` → called from a global heartbeat to process pending payoffs.

---

## 3. Cross-Repo Workflow and CI Ideas

This section captures CI and workflow patterns that increase “computational reasoning” across all 12+ repos by aligning personas, mechanics, and telemetry.

### 3.1 Persona-Aware Spine Scanner

Extend the existing schema spine scanner to also extract persona contracts:

- Input: persona NDJSON/JSON contracts from Constellation-Contracts and persona-aware vaults.  
- Output: `docs/persona-spine-index.json` mapping each persona to:  
  - The invariants/metrics they may read and write.  
  - The surprise mechanic categories they may influence.

CI can then warn or fail when a new mechanic or region would grant a persona unauthorized access to invariants or metrics.

### 3.2 Cross-Repo “What-If” Lints

Define a job that:

- Loads surpriseMechanic contracts, persona contracts, and hero region contracts.  
- Performs static reasoning over invariant and metric ranges to simulate combined effects (persona + mechanic + region).  
- Fails if any combination can exceed DET caps or metric envelopes defined in policy envelopes.

This provides constellation-level safety checks without running a full engine.

### 3.3 YAML-Driven Persona Bundles

Create meta-bundle YAML files, for example:

```yaml
personas:
  - id: persona.archivist.v1
    tiers: [ "public", "vault" ]
    repos:
      - Horror.Place
      - HorrorPlace-Atrocity-Seeds
  - id: persona.process_gods.v1
    tiers: [ "lab", "research" ]
    repos:
      - Process-Gods-Research
      - HorrorPlace-Dead-Ledger-Network
```

CI uses these bundles to ensure:

- Each listed persona has a contract in the referenced repos.  
- No new persona is introduced without updating the bundle, keeping a coherent persona index for the constellation.

### 3.4 Telemetry Schema Introspection

Introduce telemetry schema introspection that:

- Parses all telemetry schemas across repos.  
- Produces `docs/telemetry-metric-coverage.json` summarizing which metrics are logged where.  
- Highlights gaps (e.g., seeds using EMD bands without any telemetry logging EMD) as warnings or failures.

This supports data-driven tuning and persona evolution.

---

## 4. Persona Roster for Tuning and Seed Generation

This section enumerates a compact persona roster for developer and publisher tuning, to be encoded as persona contracts and used for seed generation.

| Persona ID                       | Role / Flavor                                  | Primary levers                          |
|----------------------------------|-----------------------------------------------|-----------------------------------------|
| persona.archivist.v1            | Curatorial historian of trauma                | EMD, ARR, CDL; reads CIC/AOS/SHCI       |
| persona.process_gods.v1         | System daemons / “code spirits”               | STCI, CDL; reads DET, HVF, LSG          |
| persona.witness.v1              | Player-proxy narrator                         | UEC, ARR; reads AOS, MDI                |
| persona.custodian.v1            | Safety warden / charter enforcer              | DET caps, intensity bands; CIC, SHCI    |
| persona.cartographer.v1         | Liminal mapmaker                               | HVF, LSG; nudges UEC, EMD               |
| persona.spectral_mediator.v1    | Broker between entities and regions           | ARR, CDL; reads SHCI, RWF               |
| persona.operator.v1             | Runtime overseer (orchestrator voice)         | STCI, DET; reads all invariants/metrics |
| persona.echo_child.v1           | Childlike echo of past players                | UEC, EMD; uses time-in-region telemetry |
| persona.machine_canyon.v1       | Industrial/machine horror aesthetic driver    | CIC, HVF; tunes STCI, CDL               |
| persona.dread_forger.v1         | Atmosphere/mood persona (ties to DreadForge)  | EMD, STCI; reads AOS, CIC, RRM          |
| persona.redacted_chronicler.v1  | Redaction and censorship persona              | ARR, CDL; clamps via style contracts    |
| persona.taphonomist.v1          | Decay and remains specialist                  | RRM, CIC; influences EMD, UEC           |
| persona.threshold_keeper.v1     | Portal/doorway guardian                       | LSG, HVF; gates DET and ARR             |
| persona.black_archivist.v1      | Deep vault historian (Black-Archivum)         | CIC/MDI bundle shaping; affects EMD     |
| persona.ledger_warden.v1        | Dead-Ledger guardian                          | intensity bands, tiers; no runtime write|
| persona.neural_resonator.v1     | BCI-only persona for lab testing              | small UEC/ARR corrections from physiology |
| persona.seance_director.v1      | Multi-player session orchestrator             | STCI, ARR; coordinates cross-player events |
| persona.cold_reader.v1          | Behavior predictor entity                     | UEC, CDL; uses playstyle classification |
| persona.noise_weaver.v1         | Hellscape Mixer proxy                         | audio-only EMD, STCI control            |
| persona.grit_tester.v1          | Internal QA “stress test” persona             | pushes DET/ARR to boundary within caps  |

Each persona should have:

- A `personaContract` describing allowed invariant/metric reads/writes and permitted surprise categories.  
- Style bindings for text/voice, governed by style contracts and implication rules.  
- CI rules that block seeds or mechanics granting personas unauthorized control over invariants or metrics.

---

## 5. Integration Notes

- These mechanics and personas are research-ready and intended to be narrowed into concrete contract cards per region and policy envelope.  
- Repos consuming these specs (Horror.Place, Atrocity-Seeds, Dead-Ledger, Orchestrator, Process-Gods-Research) should reference this document via `schemaref` or doc IDs in their own specs and workflows.  
- Future work can extend this spec with additional Systemic Timing mechanics and full behaviorNode / alnBinding schemas once the underlying contract schemas are stable.
