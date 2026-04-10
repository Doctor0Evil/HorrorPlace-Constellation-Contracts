# Research Documentation: Contract Enforcement & Lua/Rust Mechanic Validation in Horror$Place

**Target Repository:** `HorrorPlace-Constellation-Contracts`  
**Target Path:** `docs/research/contract-enforcement-lua-rust-validation-v1.md`  
**Schema Reference:** `schema://HorrorPlace-Constellation-Contracts/research_doc_v1.json`  
**Tier:** `T1_public`  
**Invariants Used:** `[CIC, DET, UEC, ARR, SHCI, RWF]`  
**Metrics Used:** `[UEC, ARR, CDL, STCI, EMD]`

---

## 1. Inventory of Existing Research Objects (Category: Contract Enforcement & Lua/Rust Validation)

The following research objects are already defined across the Horror$Place VM-constellation and directly support the validation of systemic timing mechanics via invariant-driven contracts.

### 1.1. Schema & Contract Objects

| Object ID | Repository | Path | Role in Validation |
|-----------|-----------|------|-----------------|
| `invariants_v1.json` | `Horror.Place` | `schemas/invariants_v1.json` | Canonical definitions of CIC, DET, UEC, ARR with ranges and preconditions; used by CI to reject out-of-bounds configs. |
| `entertainment_metrics_v1.json` | `Horror.Place` | `schemas/entertainment_metrics_v1.json` | Defines UEC, ARR, CDL, STCI, EMD bands; telemetry schemas reference these for before/after logging. |
| `eventcontract_v1.json` | `Horror.Place` | `schemas/eventcontract_v1.json` | Binds events to invariant/metric targets; Atrocity-Seeds mirrors this for seed validation. |
| `regioncontract_v1.json` | `Horror.Place` | `schemas/regioncontract_v1.json` | Defines region-level invariant bands (CIC, LSG, SHCI) that gate lethal mechanics. |
| `policyEnvelope_v1.json` | `HorrorPlace-Constellation-Contracts` | `schemas/policyEnvelope_v1.json` | Formalizes player-envelope inequalities (DET caps, ARR floors) used for runtime gating. |
| `ai_file_envelope_v1.json` | `HorrorPlace-Constellation-Contracts` | `schemas/ai_file_envelope_v1.json` | Ensures AI-generated artifacts declare target repo/path/schema before CI validation. |
| `execution-string-run.v1.json` | `HorrorPlace-Constellation-Contracts` | `schemas/telemetry/execution-string-run.v1.json` | Logs individual death events with pre/post invariant/metric snapshots for empirical analysis. |
| `systemic-timing-run.v1.json` | `HorrorPlace-Constellation-Contracts` | `schemas/telemetry/systemic-timing-run.v1.json` | Logs systemic timing decisions (Coyote Time, Rule of Three, Adaptive Damage) with helper outputs. |

### 1.2. Implementation Pattern Objects

| Object ID | Repository | Path | Role in Validation |
|-----------|-----------|------|-----------------|
| `kill_zone.rs` | `HorrorPlace-Codebase-of-Death` | `crates/death_engine/src/systems/kill_zone.rs` | Rust/Bevy ECS system that emits `CinematicDeathPending` without committing health changes. |
| `cinematic_death.rs` | `HorrorPlace-Codebase-of-Death` | `crates/death_engine/src/systems/cinematic_death.rs` | Handles coyote-time windows and final death commitment; reads `KillZoneTuning` resource. |
| `horror_invariants_bridge.lua` | `HorrorPlace-Codebase-of-Death` | `lua/horror_invariants_bridge.lua` | Lua layer that queries `H.DET`, `H.ARR`, etc., and pushes tuned parameters to Rust. |
| `h_kill_cinematic.lua` | `HorrorPlace-Codebase-of-Death` | `engine/runtime/h_kill_cinematic.lua` | Implements Rule of Three and lethal→near-lethal downgrade logic using invariant-aware helpers. |
| `systemic_timing.lua` | `HorrorPlace-Codebase-of-Death` | `engine/systemic_timing.lua` | Canonical Lua helpers: `CoyoteTime_maybe_grant`, `RuleOfThree_maybe_downgrade`, `AdaptiveDamage_scale`. |

### 1.3. Telemetry & Analysis Objects

| Object ID | Repository | Path | Role in Validation |
|-----------|-----------|------|-----------------|
| `telemetry_metrics.lua` | `HorrorPlace-Codebase-of-Death` | `engine/telemetry_metrics.lua` | Exposes `Metrics.getUEC`, `Metrics.getARR` for Lua helpers; ensures metric reads are auditable. |
| `execution_string_aggregate.lua` | `HorrorPlace-Codebase-of-Death` | `scripts/telemetry/execution_string_aggregate.lua` | Aggregates `execution-string-run.v1` logs to compute per-pattern DET/ARR deltas and skip rates. |
| `systemic_timing_analysis.ipynb` | `HorrorPlace-Process-Gods-Research` | `notebooks/systemic_timing_analysis_v1.ipynb` | Jupyter notebook for correlating timing-helper decisions with UEC/ARR outcomes across playtests. |

---

## 2. Proof Structures & Verification Hooks

The Horror$Place architecture embeds multiple layers of formal and empirical verification to ensure contract compliance.

### 2.1. Static Proofs (Compile-Time / CI)

| Proof Type | Mechanism | What It Guarantees |
|------------|-----------|-------------------|
| **Schema Validation** | JSON Schema validators in CI (`validateinvariants.rs`, `jsonschema` Python lib) | All JSON/NDJSON artifacts conform to canonical field names, types, and ranges; no `additionalProperties`. |
| **Interval Inclusion** | Invariant enforcement tables in `spine.rs` | Any region/seed config must satisfy `parent_band ⊇ child_band` for CIC, DET, ARR, etc.; prevents "escalation loopholes". |
| **Envelope Inequality Checks** | `validate_player_envelope.rs` | Player state variables (stamina, sanity, DET exposure) never violate hard caps over a session; runtime gates are mathematically sound. |
| **Dead-Ledger Attestation** | ZKP verifier registry in `HorrorPlace-Dead-Ledger-Network` | High-intensity seeds (`intensityband ≥ 8`) have cryptographic proof of age-gating and charter compliance before being surfaced. |

### 2.2. Dynamic Proofs (Runtime)

| Proof Type | Mechanism | What It Guarantees |
|------------|-----------|-------------------|
| **Borrow-Checker Safety** | Rust ECS + `CinematicDeathPending` marker pattern | No dangling pointers or race conditions when multiple enemies trigger lethal zones; only one final death commit occurs. |
| **Metric-Aware Gating** | Lua helpers reading `H.DET`, `H.ARR` before deciding lethal vs. near-lethal | Systemic timing decisions are driven by player state, not hardcoded thresholds; prevents "cheap death" frustration. |
| **Telemetry-Closed Loop** | `execution-string-run.v1` + aggregation notebooks | Empirical evidence that each death cinematic produces intended DET/ARR deltas; enables data-driven tuning. |
| **Sanctuary Spawning** | Director persona scheduling relief checkpoints when DET/UEC drift exceeds bands | Player vulnerability resets after high-stress segments; prevents endless instant-fail gauntlets. |

### 2.3. Implied Formal Verification Paths

While not yet fully implemented, the architecture supports these future proof efforts:

1. **Monotonicity Proofs for Timing Formulas**  
   Encode `p_downgrade(F)` and `p_coyote(DET, ARR)` in a proof assistant (Lean/Coq) to verify:
   - Higher DET → higher probability of mercy/downgrade.
   - Outputs always bounded in `[0, 1]` or `[min_multiplier, max_multiplier]`.

2. **Non-Interference Guarantees**  
   Prove that systemic timing helpers cannot inadvertently violate DET caps or ARR floors, even under adversarial input sequences.

3. **Telemetry Soundness**  
   Formalize that `execution-string-run.v1` logs are sufficient to reconstruct the causal chain from invariant state → mechanic decision → metric outcome.

---

## 3. Next Research Objectives & Mathematical Formulas

The following objectives advance the validation of Lua/Rust systemic timing patterns, with concrete formulas ready for implementation and empirical testing.

### 3.1. Objective: Formalize Timing Formulas as Tunable Contracts

Replace hardcoded thresholds in Lua helpers with simple, analyzable functions of DET, ARR, UEC.

#### 3.1.1. Coyote Time Granting Probability

$$
p_{\text{coyote}} = \sigma\left(w_1 \cdot \frac{\text{DET} - \text{DET}_{\text{min}}}{\text{DET}_{\text{range}}} + w_2 \cdot \left(1 - \frac{\text{ARR} - \text{ARR}_{\text{min}}}{\text{ARR}_{\text{range}}}\right) - T\right)
$$

- $\sigma(x) = \frac{1}{1 + e^{-x}}$ (sigmoid for smooth probability mapping)
- $\text{DET}_{\text{range}} = \text{DET}_{\text{max}} - \text{DET}_{\text{min}}$ (from spine bands)
- $w_1, w_2, T$: tunable weights/threshold stored in mechanic contracts
- **Guarantee**: $p_{\text{coyote}} \in [0, 1]$; monotone increasing in DET, decreasing in ARR

#### 3.1.2. Rule of Three Downgrade Probability

$$
p_{\text{downgrade}} = \sigma\left(-a_1 \cdot \frac{\text{DET}}{\text{DET}_{\text{cap}}} + a_2 \cdot \text{UEC} - a_3 \cdot \left(1 - \frac{\text{ARR}}{\text{ARR}_{\text{floor}}}\right) + a_4 \cdot \frac{\text{deathCount}}{\text{maxAttempts}} - B\right)
$$

- $\text{DET}_{\text{cap}}$, $\text{ARR}_{\text{floor}}$: hard bounds from player envelope
- $a_1 \dots a_4, B$: coefficients in mechanic contract
- **Guarantee**: Downgrade more likely when DET high, ARR low, deathCount high

#### 3.1.3. Adaptive Damage Scaling Multiplier

$$
m = \text{clip}\left(m_{\text{base}} \cdot \left(1 + \alpha \cdot \frac{\text{DET} - \text{DET}_{\text{base}}}{\text{DET}_{\text{range}}}\right) \cdot \left(1 - \beta \cdot \frac{\text{ARR} - \text{ARR}_{\text{base}}}{\text{ARR}_{\text{range}}}\right), m_{\text{min}}, m_{\text{max}}\right)
$$

- $\text{clip}(x, a, b) = \max(a, \min(b, x))$
- $m_{\text{base}}, \alpha, \beta, m_{\text{min}}, m_{\text{max}}$: contract parameters
- **Guarantee**: Damage scales smoothly with DET/ARR; never exceeds contract bounds

### 3.2. Objective: Implement Formula-Driven Lua Helpers

Refactor `H.Systemic.*` helpers to consume contract parameters and apply the above formulas.

**Target Repository:** `HorrorPlace-Codebase-of-Death`  
**Target Path:** `engine/systemic_timing.lua`

```lua
-- engine/systemic_timing.lua (formula-driven stub)
local H = require "engine.horror_invariants"
local Metrics = require "engine.telemetry_metrics"

local Systemic = {}

-- Sigmoid helper
local function sigmoid(x)
    return 1 / (1 + math.exp(-x))
end

-- Coyote Time granting probability (Formula 3.1.1)
function Systemic.CoyoteTime_probability(region_id, player_id, contract)
    local det = H.DET(region_id, player_id)
    local arr = Metrics.getARR(region_id, player_id)
    
    local det_norm = (det - contract.det_min) / (contract.det_max - contract.det_min)
    local arr_norm = (arr - contract.arr_min) / (contract.arr_max - contract.arr_min)
    
    local x = contract.w1 * det_norm + contract.w2 * (1 - arr_norm) - contract.threshold
    return sigmoid(x)
end

-- Rule of Three downgrade probability (Formula 3.1.2)
function Systemic.RuleOfThree_downgrade_probability(region_id, player_id, contract, death_count)
    local det = H.DET(region_id, player_id)
    local arr = Metrics.getARR(region_id, player_id)
    local uec = Metrics.getUEC(region_id, player_id)
    
    local det_norm = det / contract.det_cap
    local arr_norm = arr / contract.arr_floor
    
    local x = -contract.a1 * det_norm 
            + contract.a2 * uec 
            - contract.a3 * (1 - arr_norm) 
            + contract.a4 * (death_count / contract.max_attempts) 
            - contract.bias
    return sigmoid(x)
end

return Systemic
```

### 3.3. Objective: Empirical Validation via Telemetry Aggregation

Use `systemic-timing-run.v1` logs to measure formula effectiveness.

**Target Repository:** `HorrorPlace-Process-Gods-Research`  
**Target Path:** `notebooks/systemic_timing_formula_validation_v1.ipynb`

```python
# Pseudocode for aggregation analysis
import pandas as pd
import numpy as np

# Load systemic-timing-run.v1 NDJSON
df = pd.read_json("telemetry/systemic-timing-run.ndjson", lines=True)

# Group by helper_kind and variant_id
for (helper, variant), group in df.groupby(["helperKind", "variantId"]):
    # Compute mean delta metrics
    delta_uec = group["metricsAfter.UEC"] - group["metricsBefore.UEC"]
    delta_arr = group["metricsAfter.ARR"] - group["metricsBefore.ARR"]
    
    # Correlate with formula parameters
    params = extract_params(variant)  # e.g., w1, w2, T for CoyoteTime
    print(f"{helper}/{variant}: ΔUEC={delta_uec.mean():.3f}, ΔARR={delta_arr.mean():.3f}")
    
    # Flag variants that improve UEC without collapsing ARR
    if delta_uec.mean() > 0.05 and delta_arr.mean() > -0.1:
        print(f"  ✓ Candidate for promotion to default")
```

### 3.4. Objective: Formal Verification of Formula Properties

**Target Repository:** `HorrorPlace-Process-Gods-Research`  
**Target Path:** `formal/timing_formulas_monotonicity.lean`

```lean
-- Lean 4 sketch: monotonicity proof for p_coyote
import Mathlib.Analysis.SpecialFunctions.Exp

def sigmoid (x : ℝ) : ℝ := 1 / (1 + Real.exp (-x))

theorem sigmoid_monotone : Monotone sigmoid := by
  -- Proof omitted: derivative positive everywhere
  sorry

def p_coyote (det arr : ℝ) (w1 w2 T det_min det_max arr_min arr_max : ℝ) : ℝ :=
  let det_norm := (det - det_min) / (det_max - det_min)
  let arr_norm := (arr - arr_min) / (arr_max - arr_min)
  sigmoid (w1 * det_norm + w2 * (1 - arr_norm) - T)

theorem p_coyote_det_monotone (h_w1 : 0 ≤ w1) (h_ranges : det_min < det_max ∧ arr_min < arr_max) :
  Monotone (fun det => p_coyote det arr w1 w2 T det_min det_max arr_min arr_max) := by
  -- Follows from sigmoid_monotone and linearity of det_norm
  sorry
```

---

## 4. Concrete Next-Step Artifacts (GitHub-Ready)

### 4.1. Schema Extension: Timing Formula Parameters

**Target Repository:** `HorrorPlace-Constellation-Contracts`  
**Target Path:** `schemas/mechanic_contract_timing_params_v1.json`

```json
{
  "$id": "schema://HorrorPlace-Constellation-Contracts/mechanic_contract_timing_params_v1.json",
  "title": "Mechanic Contract Timing Parameters v1",
  "type": "object",
  "properties": {
    "coyoteTime": {
      "type": "object",
      "properties": {
        "w1": { "type": "number", "minimum": 0, "maximum": 10 },
        "w2": { "type": "number", "minimum": 0, "maximum": 10 },
        "threshold": { "type": "number" },
        "det_min": { "type": "number", "minimum": 0, "maximum": 10 },
        "det_max": { "type": "number", "minimum": 0, "maximum": 10 },
        "arr_min": { "type": "number", "minimum": 0, "maximum": 1 },
        "arr_max": { "type": "number", "minimum": 0, "maximum": 1 }
      },
      "required": ["w1", "w2", "threshold", "det_min", "det_max", "arr_min", "arr_max"]
    },
    "ruleOfThree": {
      "type": "object",
      "properties": {
        "a1": { "type": "number", "minimum": 0 },
        "a2": { "type": "number", "minimum": 0 },
        "a3": { "type": "number", "minimum": 0 },
        "a4": { "type": "number", "minimum": 0 },
        "bias": { "type": "number" },
        "det_cap": { "type": "number", "minimum": 0, "maximum": 10 },
        "arr_floor": { "type": "number", "minimum": 0, "maximum": 1 },
        "max_attempts": { "type": "integer", "minimum": 1 }
      },
      "required": ["a1", "a2", "a3", "a4", "bias", "det_cap", "arr_floor", "max_attempts"]
    },
    "adaptiveDamage": {
      "type": "object",
      "properties": {
        "m_base": { "type": "number", "minimum": 0.1 },
        "alpha": { "type": "number", "minimum": -1, "maximum": 1 },
        "beta": { "type": "number", "minimum": -1, "maximum": 1 },
        "m_min": { "type": "number", "minimum": 0 },
        "m_max": { "type": "number", "minimum": 0 },
        "det_base": { "type": "number", "minimum": 0, "maximum": 10 },
        "det_range": { "type": "number", "minimum": 0 },
        "arr_base": { "type": "number", "minimum": 0, "maximum": 1 },
        "arr_range": { "type": "number", "minimum": 0 }
      },
      "required": ["m_base", "alpha", "beta", "m_min", "m_max", "det_base", "det_range", "arr_base", "arr_range"]
    }
  },
  "required": ["coyoteTime", "ruleOfThree", "adaptiveDamage"]
}
```

### 4.2. CI Hook: Validate Formula Parameters

**Target Repository:** `HorrorPlace-Constellation-Contracts`  
**Target Path:** `.github/workflows/validate-timing-params.reusable.yml`

```yaml
name: Validate Timing Formula Parameters (Reusable)

on:
  workflow_call:
    inputs:
      mechanic_contract_glob:
        description: "Glob pattern for mechanic contract JSON files"
        required: true
        type: string

jobs:
  validate-timing-params:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout caller repo
        uses: actions/checkout@v4

      - name: Set up Python
        uses: actions/setup-python@v5
        with:
          python-version: "3.11"

      - name: Install jsonschema
        run: pip install jsonschema

      - name: Download timing params schema
        uses: actions/checkout@v4
        with:
          repository: Doctor0Evil/HorrorPlace-Constellation-Contracts
          path: constellation-contracts

      - name: Validate mechanic contracts
        run: |
          SCHEMA="constellation-contracts/schemas/mechanic_contract_timing_params_v1.json"
          for f in $(git ls-files "${{ inputs.mechanic_contract_glob }}"); do
            python -c "
import json, sys
from jsonschema import validate
with open('$SCHEMA') as s, open('$f') as c:
    validate(instance=json.load(c)['systemicTiming'], schema=json.load(s))
print(f'OK: $f timing params valid')
"
          done
```

---

## 5. Summary & Progression Path

| Phase | Objective | Deliverable | Success Metric |
|-------|-----------|-------------|---------------|
| **Phase 1** | Formalize timing formulas as contract parameters | `mechanic_contract_timing_params_v1.json` + Lua stubs | CI validates all mechanic contracts against new schema |
| **Phase 2** | Implement formula-driven Lua helpers | `engine/systemic_timing.lua` refactored | Helpers produce bounded, monotone outputs; telemetry logs decisions |
| **Phase 3** | Empirical validation via telemetry aggregation | `systemic_timing_formula_validation_v1.ipynb` | Identify parameter sets that improve UEC without collapsing ARR |
| **Phase 4** | Formal verification of formula properties | `timing_formulas_monotonicity.lean` + extracted code | Machine-checked proofs that helpers cannot violate invariant bounds |

This research trajectory ensures that Horror$Place's systemic timing mechanics evolve from ad-hoc thresholds to mathematically grounded, empirically validated, and formally verified contracts—preserving the invariant-driven spine while enabling data-driven refinement of player experience.

<div align="center">⁂</div>
