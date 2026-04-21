---
invariants-used: [CIC, MDI, AOS, RRM, FCF, SPR, RWF, DET, HVF, LSG, SHCI]
metrics-used: [UEC, EMD, STCI, CDL, ARR]
modules: [H.Metrics, H.Director, H.Selector]
schema-refs:
  - entertainment-metrics.v1.json
  - director-persona-contract.v1.json
---

# Metrics Snapshot Envelope v1

This document defines the canonical return shape for `H.Metrics.snapshot(sessionId)`, used by AI‑Chat, selector modules, and engine runtime to inspect current entertainment metric state without accessing raw telemetry streams.

## Envelope Shape

All calls return the standard `ok / data / error` envelope:

```lua
local result = H.Metrics.snapshot(sessionId)

-- Success case
result.ok == true
result.data = {
  metrics = {
    UEC  = number | nil,  -- current estimate, nil if unknown
    EMD  = number | nil,
    STCI = number | nil,
    CDL  = number | nil,
    ARR  = number | nil,
  },
  bands = {
    UEC  = { min = number, max = number },  -- effective band from persona/run
    EMD  = { min = number, max = number },
    STCI = { min = number, max = number },
    CDL  = { min = number, max = number },
    ARR  = { min = number, max = number },
  },
  source = {
    directorPersonaId = string | nil,  -- bound persona, if any
    aiChatProfileId   = string | nil,
    runContractId     = string | nil,
  }
}

-- Error case
result.ok == false
result.error = {
  code = "SESSION_NOT_FOUND" | "METRICS_UNAVAILABLE",
  message = string,
  details = table | nil
}
```

## Semantics

- **Nil means unknown**: A metric value of `nil` indicates no estimate is yet available; it is not equivalent to `0.0`. Callers must explicitly check for `nil` before numeric comparisons.
- **Bands are authoritative**: The `bands` table reflects the intersection of persona targets, consent caps, budget constraints, and ethics guardrails. Selector and director logic should clamp intended values into these bands via `H.Director.constrainMetrics`.
- **Provenance is optional**: The `source` block helps telemetry correlate metric snapshots with the contracts that produced them. It may be omitted in minimal engine builds.

## Usage Patterns

### Selector modules
```lua
local snap = H.Metrics.snapshot(sessionId)
if snap.ok and snap.data.metrics.UEC then
  if snap.data.metrics.UEC < snap.data.bands.UEC.min then
    -- Prefer patterns that raise UEC toward band center
  end
end
```

### AI‑Chat narration
```lua
local snap = H.Metrics.snapshot(sessionId)
if snap.ok then
  local hints = {}
  if snap.data.metrics.CDL and snap.data.metrics.CDL > 0.7 then
    table.insert(hints, "cognitive load is elevated")
  end
  -- Use hints to modulate explanation style
end
```

### Node describers
```lua
local snap = H.Metrics.snapshot(sessionId)
local nodeDesc = H.Node.describe(regionId, nodeId, { sessionId = sessionId })
if nodeDesc.ok and snap.ok then
  -- Combine node invariants with session metrics for richer narration
end
```

## Error Handling

Callers should treat `SESSION_NOT_FOUND` as a hard failure and fall back to non‑metric‑dependent logic. `METRICS_UNAVAILABLE` indicates transient unavailability; callers may retry on next turn or proceed with `nil` values.

## Telemetry

Every successful call to `H.Metrics.snapshot` should emit a lightweight telemetry event:

```json
{
  "event": "metrics-snapshot-queried",
  "sessionId": "...",
  "timestamp": "...",
  "metricsPresent": ["UEC", "ARR"],
  "bandsSource": "directorPersonaContract"
}
```

This enables post‑hoc analysis of how metric availability correlates with selector decisions and user experience outcomes.
