## Target repository and document shape

- Target repo: `HorrorPlace-Constellation-Contracts`  
- Target doc (future): `docs/vm-constellation/systemic-timing-mechanics-and-telemetry-v1.md`  
- Scope: spec‑v1, no new core schemas, one lab‑only NDJSON schema name for analysis.

***

## 1. Lua API: H.Systemic helpers

All helpers must be thin facades over invariants (`H.*`), metrics (`Metrics.*`), and events (`Events.*`). They never touch raw tables or hard‑coded bands; thresholds live in mechanic contracts and mood/region archetypes.

### 1.1 Module surface

```lua
-- engine/systemic_timing.lua
local H       = require "engine.horror_invariants"
local Metrics = require "engine.telemetry_metrics"
local Events  = require "engine.events"

local Systemic = {}

-- Coyote Time: grace window around lethal thresholds
function Systemic.CoyoteTime_maybe_grant(region_id, player_id, mech_contract, dt)
    -- returns:
    -- { granted = bool,
    --   window_seconds = number,
    --   reason = "det_high" | "arr_low" | "uec_soft" | "none" }
end

-- Rule of Three: repetition‑aware gating and downgrade
function Systemic.RuleOfThree_maybe_downgrade(region_id, player_id, mech_contract, death_index)
    -- death_index is 1‑based death / failure count within this segment
    -- returns:
    -- { downgrade = bool,
    --   suppression = bool,   -- e.g. fully suppress this attempt
    --   reason = "threshold_hit" | "segment_exhausted" | "none" }
end

-- Adaptive Damage: DET/ARR/UEC‑aware damage / punishment scaling
function Systemic.AdaptiveDamage_scale(region_id, player_id, mech_contract, base_damage)
    -- returns:
    -- { scaled_damage = number,
    --   clamp_reason = "det_cap" | "arr_floor" | "uec_soft" | "none" }
end

return Systemic
```

### 1.2 Mechanic‑level contract hints

These helpers expect to be driven by per‑mechanic config embedded in `surpriseMechanicContract.v1` (or sibling) under a `systemicTiming` or `systemicCaps` object. All fields are numeric bands; CI enforces ranges via existing spine rules.

Conceptual fragment (not a new schema, just a config pattern):

```json
{
  "id": "SURP.SYSTM.EXAMPLE.MECH.v1",
  "category": "SystemicTiming",
  "systemicTiming": {
    "coyoteTime": {
      "detWindow": { "min": 2.0, "max": 7.0 },
      "arrFloor": 0.55,
      "uecTargetBand": { "min": 0.55, "max": 0.85 },
      "maxWindowSeconds": 0.40
    },
    "ruleOfThree": {
      "maxFullExecutions": 2,
      "maxPunishAttempts": 3,
      "detSoftCap": 7.0,
      "arrSoftFloor": 0.50
    },
    "adaptiveDamage": {
      "detBands": [
        { "maxDet": 3.0, "multiplier": 1.15 },
        { "maxDet": 6.0, "multiplier": 1.00 },
        { "maxDet": 9.0, "multiplier": 0.75 }
      ],
      "arrBands": [
        { "minArr": 0.80, "multiplier": 0.85 },
        { "minArr": 0.60, "multiplier": 1.00 },
        { "minArr": 0.00, "multiplier": 1.10 }
      ],
      "maxDamageMultiplier": 1.25,
      "minDamageMultiplier": 0.50
    }
  }
}
```

### 1.3 Helper behavior sketches

#### H.Systemic.CoyoteTime_maybe_grant

Inputs:

- `region_id`, `player_id`  
- `mech_contract.systemicTiming.coyoteTime`  
- Per‑frame `dt` (for accumulating / decaying a running “stress window” if desired)  

Runtime logic (pure function, apart from telemetry/events):

1. Read invariants and metrics:

   - `det = H.DET(region_id, player_id)`  
   - `uec = Metrics.getUEC(region_id, player_id)`  
   - `arr = Metrics.getARR(region_id, player_id)`

2. Check contract bands:

   - If `det` is outside `detWindow` → `{granted=false, window_seconds=0, reason="none"}`.  
   - If `arr < arrFloor` → bias toward granting a window.  
   - If `uec > uecTargetBand.max` → prefer relief, not extra tension.

3. Compute target window:

   - Define a normalized stress factor `s_det` in `[0,1]` from DET within window.  
   - Define a retention factor `s_arr` in `[0,1]` from ARR (more grace when ARR is low).  
   - Window suggestion:  
     \[
     w = \text{maxWindowSeconds} \cdot (0.5 \cdot s_\text{det} + 0.5 \cdot (1 - s_\text{arr}))
     \]
   - Clamp `w` to `[0, maxWindowSeconds]`.

4. Decide grant:

   - If `w > 0.01` and current mechanic state has not already granted a window this attempt, return `granted=true`, `window_seconds=w`, `reason` set from whichever term dominated (e.g., `det_high` if `s_det` > `1 - s_arr`).

Kill / hazard scripts call this helper when a lethal condition is about to be enforced; if `granted`, they substitute “near‑miss” or a recoverable state instead of instant death.

#### H.Systemic.RuleOfThree_maybe_downgrade

Inputs:

- `region_id`, `player_id`  
- `mech_contract.systemicTiming.ruleOfThree`  
- `death_index` within the current segment  

Logic:

1. If `death_index > maxPunishAttempts`: always return `{downgrade=true, suppression=true, reason="segment_exhausted"}` – Director should force a checkpoint or skip this pattern.  
2. Read DET/ARR:

   - `det = H.DET(region_id, player_id)`  
   - `arr = Metrics.getARR(region_id, player_id)`

3. If `death_index > maxFullExecutions` and (`det >= detSoftCap` or `arr <= arrSoftFloor`), return `{downgrade=true, suppression=false, reason="threshold_hit"}` → e.g., turn an execution into grab + knockdown.  
4. Otherwise `{downgrade=false, suppression=false, reason="none"}`.

Enemy/encounter logic must treat this as authoritative: no local counters should ignore a systemic downgrade/suppression signal.

#### H.Systemic.AdaptiveDamage_scale

Inputs:

- `region_id`, `player_id`  
- `mech_contract.systemicTiming.adaptiveDamage`  
- `base_damage` (or base “punishment score” for non‑HP outcomes)  

Logic:

1. Read DET, ARR, UEC:

   - `det = H.DET(region_id, player_id)`  
   - `arr = Metrics.getARR(region_id, player_id)`  
   - Optionally `uec = Metrics.getUEC(region_id, player_id)` to bias toward relief when UEC is high but DET has plateaued.

2. Select band multipliers:

   - From `detBands`, pick the first band with `det <= maxDet`; if none, use the last.  
   - From `arrBands`, pick the first band with `arr >= minArr`; if none, use the last.

3. Combine:

   - `m_det * m_arr = m_raw` (optionally blended with a soft UEC‑based factor).  
   - Clamp `m_raw` into `[minDamageMultiplier, maxDamageMultiplier]`.  
   - `scaled_damage = base_damage * m_clamped`.

4. Decide clamp_reason:

   - If clamped at min/max, set `clamp_reason` accordingly (`"det_cap"` or `"arr_floor"` depending on which contributed more).  
   - Else `"none"`.

All callers must apply `scaled_damage` and forward `clamp_reason` into telemetry.

***

## 2. NDJSON schema: systemic-timing-run.v1 (lab‑tier)

This is a lab‑only telemetry schema (Neural‑Resonance‑Lab / Redacted‑Chronicles) for ranking systemic timing variants. It does not change public DSLs.

### 2.1 Conceptual schema summary

Each line is a JSON object representing one systemic timing decision window (e.g., a death/fail occurrence, a Coyote Time check, or a damage scaling event) with before/after metrics and helper outputs.

Key fields (conceptual, not full JSON Schema expansion here):

- Identity  
  - `runId`: string (session / experiment run)  
  - `playerId`: string (pseudonymous)  
  - `regionId`: string  
  - `tileId`: string  
  - `timestamp`: ISO8601 or numeric ms since start  

- Mechanic context  
  - `mechanicId`: string (e.g., `SURP.SYSTM.NEARTRIG.ABORT.v1`)  
  - `mechanicCategory`: `"SystemicTiming"` | other  
  - `helperKind`: `"CoyoteTime"` | `"RuleOfThree"` | `"AdaptiveDamage"`  
  - `deathIndex`: integer (if applicable)  
  - `baseDamage`: number (if applicable)  

- Invariants snapshot (normalized to 0–1 or canonical spine ranges)  
  - `invariants`:  
    - `CIC`, `MDI`, `AOS`, `DET`, `HVF`, `LSG`, `SHCI`, `RWF` (subset sufficient for systemic timing)  

- Entertainment metrics before / after window  
  - `metricsBefore`:  
    - `UEC`, `ARR`, `EMD`, `STCI`, `CDL`  
  - `metricsAfter`:  
    - `UEC`, `ARR`, `EMD`, `STCI`, `CDL`  

- Helper decision outputs  
  - For Coyote Time:  
    - `coyoteTimeGranted`: boolean  
    - `coyoteTimeWindowSeconds`: number  
    - `coyoteTimeReason`: string enum  
  - For Rule of Three:  
    - `ruleOfThreeDowngrade`: boolean  
    - `ruleOfThreeSuppression`: boolean  
    - `ruleOfThreeReason`: string enum  
  - For Adaptive Damage:  
    - `adaptiveDamageMultiplier`: number  
    - `adaptiveDamageScaled`: number  
    - `adaptiveDamageClampReason`: string enum  

- Outcome annotations  
  - `outcomeKind`: `"instant_death"` | `"near_lethal_grab"` | `"escaped_with_coyote"` | `"checkpoint_spawned"` | `"other"`  
  - `abandonmentFlag`: boolean (session ended within N seconds of this decision)  

- Mapping / experiment tags  
  - `familyCode`: string (hex mapping family id if these helpers are being mutated / tested)  
  - `variantId`: string (parameter variant label)  
  - `policyId`: string (BCI / intensity policy in effect, if relevant)  

### 2.2 Logging hooks

Each helper should emit a single systemic timing frame via a small telemetry module, e.g.:

```lua
-- engine/systemic_timing_telemetry.lua
local SysTelem = {}

function SysTelem.log_frame(payload)
    -- payload: table shaped like systemic-timing-run.v1 record
    -- Implementation:
    -- - adds runId/session info
    -- - validates basic ranges in lab builds
    -- - appends NDJSON line to configured sink
end

return SysTelem
```

Call sites:

- Kill / fail handlers call `SysTelem.log_frame` after consulting `H.Systemic.CoyoteTime_maybe_grant` and `H.Systemic.RuleOfThree_maybe_downgrade`, filling in `metricsBefore` and `outcomeKind`.  
- Damage application sites call it after applying `H.Systemic.AdaptiveDamage_scale`, filling `baseDamage`, `adaptiveDamageMultiplier`, `adaptiveDamageScaled`, and `metricsAfter`.  

Kotlin analysis tools can then group by `helperKind`, `mechanicId`, `familyCode`, and region‑class (via invariants) to compute:

- Average `deltaUEC`, `deltaARR`, `deltaEMD` per helper configuration.  
- Abandonment rates vs. presence/absence of Coyote Time and downgrades.  
- Which parameter sets yield “safe uplift” (UEC/ARR up, CDL/STCI in band) rather than simple frustration.
