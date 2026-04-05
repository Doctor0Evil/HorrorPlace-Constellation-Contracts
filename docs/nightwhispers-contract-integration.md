# NightWhispers Contract Integration Specification v1

This document defines how NightWhispers legend contracts integrate with the Horror$Place VM-constellation's invariant spine, Seed contracts, and persona system. It is the authoritative guide for engine developers, AI authoring tools, and pipeline engineers.

---

## 1. Doctrine Alignment

NightWhispers legend contracts are **Seed-like controllers**, not content stores. They:

- Reference Atrocity-Seeds sequence seeds via `seed_ids`; they do not contain raw horror content.
- Bind to persona contracts via `persona_hooks`; they do not define NPC behavior directly.
- Declare metric targets (`UEC`, `EMD`, `STCI`, `CDL`, `ARR`); they do not hardcode narrative outcomes.
- Respect invariant constraints (`SPR_min`, `DET_max`, `CIC_band`, `SHCI_min`); they query the canonical history layer before manifesting.

All legend contracts validate against `schema://HorrorPlace-Constellation-Contracts/nightwhispers-legend-contract-v1.json`.

---

## 2. Contract Structure Overview

### 2.1 Core Fields

| Field | Type | Purpose |
|-------|------|---------|
| `legend_id` | `string` | Stable identifier for the legend; used in NightWhispers engine and registries |
| `schemaref` | `string` | Canonical schema URI; must match `nightwhispers-legend-contract-v1.json` |
| `seed_ids` | `array[string]` | References to Atrocity-Seeds sequence seeds that this legend can manifest |
| `persona_hooks` | `array[string]` | git@ URIs to persona contracts that can trigger or resolve this legend |
| `metric_targets` | `object` | Target bands for entertainment metrics when legend is active |
| `status_transitions` | `object` | Deterministic FSM defining legend state changes based on player actions |
| `hazard_delta_rules` | `object` | Rules for adjusting local hazard levels; bounded by `cap` |
| `invariant_constraints` | `object` | Filters ensuring legend only manifests in compatible invariant contexts |
| `safetytier` | `enum` | Safety classification for entitlement gating |
| `intensityband` | `int [0-10]` | Thematic intensity; values ≥ 8 require `deadledgerref` |
| `deadledgerref` | `object|null` | Dead-Ledger attestation reference for high-intensity legends |

### 2.2 Status Transition FSM

The `status_transitions` object defines a deterministic finite state machine:

```json
{
  "states": ["Asleep", "Awakening", "Active", "Fulfilled", "Broken"],
  "transitions": [
    {
      "from": "Asleep",
      "action_tag": "spread_rumor",
      "intensity_threshold": 0.3,
      "to": "Awakening",
      "hazard_delta": 0.15
    },
    {
      "from": "Awakening",
      "action_tag": "spread_rumor",
      "intensity_threshold": 0.7,
      "to": "Active",
      "hazard_delta": 0.25
    },
    {
      "from": "Active",
      "action_tag": "perform_ritual_success",
      "intensity_threshold": 0.5,
      "to": "Fulfilled",
      "hazard_delta": -0.3
    },
    {
      "from": "Active",
      "action_tag": "disprove_legend",
      "intensity_threshold": 0.4,
      "to": "Broken",
      "hazard_delta": -0.2
    }
  ]
}
```

Key rules:

- `intensity_threshold` is clamped to `[0,1]`; values outside this range are rejected by CI.
- `hazard_delta` adjustments are applied cumulatively but bounded by `hazard_delta_rules.cap`.
- Unknown `action_tag` values result in no state change (fail-safe default).

---

## 3. Invariant Query Protocol

Before any legend manifestation, the NightWhispers engine must query the canonical history layer via the `H.` API:

```lua
-- Example Lua pseudocode
local legend = H.legend_contract("CandleEyedWidow")
local zone = H.zone_view("LanternAlley")

-- Check invariant constraints
if zone.invariants.SPR < legend.invariant_constraints.SPR_min then
  return nil  -- Legend cannot manifest in this zone
end

if zone.invariants.DET > legend.invariant_constraints.DET_max then
  return nil  -- Player exposure cap exceeded; block escalation
end

-- Filter compatible seeds
local compatible_seeds = {}
for _, seed_id in ipairs(legend.seed_ids) do
  local seed = H.seed_contract(seed_id)
  if seed.invariants.compatible_with(zone.invariants) then
    table.insert(compatible_seeds, seed)
  end
end

if #compatible_seeds == 0 then
  return nil  -- No compatible manifestation available
end

-- Select best seed by metric alignment
local best_seed = select_by_metric_alignment(compatible_seeds, legend.metric_targets)
return build_manifestation_plan(best_seed, legend, zone)
```

This protocol ensures:

- Legends never manifest in invariant-incompatible contexts.
- Player safety (DET caps) is enforced at the contract level.
- Manifestation selection is driven by metric targets, not arbitrary choice.

---

## 4. Persona Hook Resolution

Persona hooks link legends to behavioral contracts in `HorrorPlace-Constellation-Contracts`:

```json
"persona_hooks": [
  "git@github.com:Doctor0Evil/HorrorPlace-Constellation-Contracts.git#contracts/persona_contract_v1.json#CandleEyedWidow",
  "git@github.com:Doctor0Evil/HorrorPlace-Constellation-Contracts.git#contracts/persona_contract_v1.json#RagpickerQueen"
]
```

Resolution flow:

1. NightWhispers engine requests persona contract via `H.persona(persona_ref)`.
2. Orchestrator fetches contract from `HorrorPlace-Constellation-Contracts`.
3. Persona's `metricInfluence` and `invariantCoupling` fields are merged with legend's `metric_targets`.
4. Resulting behavior vector is cached per-session; updates propagate via contract versioning.

Personas may:

- Trigger legend state transitions when certain interaction patterns occur.
- Modify `hazard_delta` based on their `personality_base` vectors.
- Provide BCI calibration hints via `bci_calibration` fields.

---

## 5. Metric Target Enforcement

The `metric_targets` object defines acceptable bands for entertainment metrics when the legend is active:

```json
"metric_targets": {
  "UEC": { "min": 0.6, "max": 0.9 },
  "EMD": { "min": 0.5, "max": 0.8 },
  "STCI": { "min": 0.3, "max": 0.7 },
  "CDL": { "min": 0.4, "max": 0.85 },
  "ARR": { "min": 0.7, "max": 0.95 }
}
```

Enforcement rules:

- The Director system monitors live metric values during legend activity.
- If any metric drifts outside its target band for > 3 consecutive ticks, the system may:
  - Adjust rumor decay rates.
  - Trigger compensatory minor events.
  - Request persona behavioral adjustments.
- Persistent violations trigger a legend state review (e.g., forced transition to `Broken` if metrics indicate player distress).

---

## 6. High-Intensity Legend Gating

Legends with `intensityband >= 8` must include a `deadledgerref`:

```json
"intensityband": 9,
"deadledgerref": {
  "proof_envelope_id": "dln.envelope.legend.candle_eyed_widow.v1",
  "verifier_ref": "dln.verifier.core.v1",
  "circuit_type": "charter_agreement_and_age_gate",
  "required_proofs": ["age_over_18", "charter_agreement_v1", "neurorights_ack"]
}
```

Gating workflow:

1. At runtime, engine requests legend contract.
2. If `intensityband >= 8`, engine queries Dead-Ledger via `H.deadledger_verify(deadledgerref)`.
3. Dead-Ledger validates ZKP proofs against player entitlements.
4. Only if verification succeeds does the legend become available for manifestation.

This ensures high-intensity content is only accessible to entitled, consenting players.

---

## 7. CI Validation Requirements

All legend contracts must pass the following CI checks:

1. **Schema validation**: Must validate against `nightwhispers-legend-contract-v1.json`.
2. **Invariant range checks**: All `invariant_constraints` values must be within canonical ranges.
3. **Metric band validation**: `metric_targets` min/max pairs must satisfy `min <= max`.
4. **FSM consistency**: All `transitions` must reference valid `states`; no unreachable states allowed.
5. **Dead-Ledger linkage**: If `intensityband >= 8`, `deadledgerref` must be present and well-formed.
6. **Seed reference resolution**: All `seed_ids` must correspond to valid entries in Atrocity-Seeds registries.
7. **Persona hook resolution**: All `persona_hooks` must resolve to valid persona contracts.

CI failures block merge; authors must correct violations before resubmission.

---

## 8. Registry Integration

Legend contracts are discovered via registries in `Horror.Place`:

```json
// Horror.Place/registry/legends.json (example entry)
{
  "legend_id": "CandleEyedWidow",
  "schemaref": "schema://HorrorPlace-Constellation-Contracts/nightwhispers-legend-contract-v1.json",
  "path": "git@github.com:Doctor0Evil/HorrorPlace-Constellation-Contracts.git#contracts/nightwhispers-legend-contract-v1.json#CandleEyedWidow",
  "hash": "sha256:abc123...",
  "tier": "mature",
  "intensityband": 7,
  "tags": ["widow", "children", "backwards_midnight_walk"]
}
```

Key points:

- Registries contain only metadata and references; never raw contract content.
- `hash` is computed by CI over canonicalized JSON; engines verify integrity before use.
- `tier` and `intensityband` enable quick filtering without full contract fetch.

---

## 9. Migration Notes for Existing NightWhispers Code

1. Replace static legend JSON loads with `H.legend_contract(legend_id)` calls.
2. Remove any hardcoded invariant values; always resolve via `invariant_constraints`.
3. Update FFI signatures to return contract references, not raw data.
4. Add `schemaref` fields to all emitted legend artifacts for CI validation.
5. Ensure `deadledgerref` is present for any legend with `intensityband >= 8`.

---

*This document is implication-only and schema-bound. It contains no raw horror content, narrative prose, or explicit descriptions of harm. All references to trauma, history, or spectral phenomena are mediated through invariant bundles and Seed contracts.*
