# Consent Lua API v1

This document specifies the canonical Lua API surface for querying and enforcing the consent state machine (`consent-state-machine-v1.json`) and telemetry bindings (`consent-session-metrics-v1.json`) at runtime. All functions follow the `H.Consent.*` namespace pattern and return structured `{ ok, data, error }` envelopes for deterministic error handling.

**Target invariants**: CIC, MDI, AOS, DET, SHCI  
**Target metrics**: UEC, EMD, STCI, CDL, ARR  
**Tiers**: minor, adult-basic, adult-horror, research  
**Dead-Ledger surfaces**: bundle_attestation, agent_attestation

---

## 1. Core API Functions

### `H.Consent.currentState(sessionId) -> result`

Returns the effective consent state for a given session.

**Parameters**:
- `sessionId` (string): Opaque UUID for the session.

**Returns**:
```lua
{
  ok = true,
  data = {
    consent_tier = "adult-basic",      -- enum: minor|adult-basic|adult-horror|research
    policy_tier = "standard",          -- enum: standard|mature|research
    age_tier = "adult",                -- enum: minor|adult
    invariant_caps = {                 -- active caps from consent-state-machine-v1.json
      CIC = 0.6, MDI = 0.7, AOS = 0.7, DET = 6.0, SHCI = 0.5
    },
    explicitness_budget = {            -- current budget state
      max = 0.8, used = 0.3, remaining = 0.5
    },
    router_state_id = "rtr-a1b2c3d4",  -- optional reference to Age-Gated Tier Router decision
    consent_profile_id = "cp-xyz789"   -- optional reference to consent profile object
  },
  error = nil
}
```

**Error cases**:
- `session_not_found`: No active session with given `sessionId`.
- `consent_unverified`: Session lacks required proof attestation for requested tier.

---

### `H.Consent.canTransition(sessionId, targetTier) -> result`

Checks whether a session may transition to a different consent tier.

**Parameters**:
- `sessionId` (string): Opaque UUID for the session.
- `targetTier` (string): Desired target tier (`minor`, `adult-basic`, `adult-horror`, `research`).

**Returns**:
```lua
{
  ok = true,
  data = {
    allowed = true,
    requires_proofs = { "age_verification", "explicit_opt_in" },
    cooldown_remaining_minutes = 0,
    deadledger_refs_required = { "dln.envelope.age.v1" }  -- if targetTier >= adult-horror
  },
  error = nil
}
```

**Error cases**:
- `invalid_target_tier`: `targetTier` not recognized.
- `proof_missing`: Required proof not present in session context.
- `cooldown_active`: Transition blocked by cooldown window.

---

### `H.Consent.requiredProofs(sessionId, targetTier) -> result`

Returns the set of proofs required to achieve a target consent tier.

**Parameters**:
- `sessionId` (string): Opaque UUID for the session.
- `targetTier` (string): Desired target tier.

**Returns**:
```lua
{
  ok = true,
  data = {
    proofs = {
      { type = "age_verification", provider = "did_kyc", status = "verified" },
      { type = "explicit_opt_in", timestamp = "2026-04-01T12:00:00Z", status = "pending" }
    },
    deadledger_envelope_id = "dln.envelope.consent.upgrade.v1"  -- if applicable
  },
  error = nil
}
```

**Error cases**:
- `session_not_found`: No active session with given `sessionId`.
- `invalid_target_tier`: `targetTier` not recognized.

---

### `H.Consent.bindSession(sessionId, consentProfileId) -> result`

Binds a session to a consent profile, initializing state and budget.

**Parameters**:
- `sessionId` (string): Opaque UUID for the session.
- `consentProfileId` (string): Reference to a consent profile object.

**Returns**:
```lua
{
  ok = true,
  data = {
    session_id = "sess-uuid-123",
    consent_tier = "adult-basic",
    budget_initialized = true,
    invariant_caps_applied = { CIC = 0.6, DET = 6.0 }
  },
  error = nil
}
```

**Error cases**:
- `profile_not_found`: Consent profile ID does not exist.
- `tier_mismatch`: Profile tier incompatible with session age gate.

---

### `H.Consent.consumeBudget(sessionId, modality, amount) -> result`

Decrements the explicitness budget for a given content modality.

**Parameters**:
- `sessionId` (string): Opaque UUID for the session.
- `modality` (string): One of `"violence"`, `"gore"`, `"sexual"`, `"psychological"`.
- `amount` (number): Amount to consume (0.0–1.0).

**Returns**:
```lua
{
  ok = true,
  data = {
    budget_remaining = 0.45,
    over_budget = false,
    modality_breakdown = {
      violence = { used = 0.2, capped = false },
      gore = { used = 0.1, capped = false }
    }
  },
  error = nil
}
```

**Error cases**:
- `budget_exhausted`: Requested amount exceeds remaining budget.
- `modality_forbidden`: Modality not allowed for current consent tier.
- `session_not_found`: No active session with given `sessionId`.

---

### `H.Consent.checkGuardrails(sessionId) -> result`

Evaluates ethics guardrails and returns any violations.

**Parameters**:
- `sessionId` (string): Opaque UUID for the session.

**Returns**:
```lua
{
  ok = true,
  data = {
    guardrails_ok = true,
    violations = {},
    recommendations = {}
  },
  error = nil
}
```

**Example violation**:
```lua
violations = {
  {
    rule_id = "ethics.no_prolonged_high_cic",
    condition = "CIC > 0.8 for duration >= 300s",
    current_value = { CIC = 0.85, duration_seconds = 320 },
    suggested_action = "force_cool_down"
  }
}
```

**Error cases**:
- `session_not_found`: No active session with given `sessionId`.

---

### `H.Consent.logDistress(sessionId, signalType, severity, context) -> result`

Records a distress signal for later analysis (PII-free).

**Parameters**:
- `sessionId` (string): Opaque UUID for the session.
- `signalType` (string): One of `"user_abort"`, `"comfort_drop"`, `"explicitness_spike"`, `"guardrail_trigger"`.
- `severity` (string): `"low"`, `"medium"`, `"high"`.
- `context` (string, optional): Opaque tag (max 256 chars, no PII).

**Returns**:
```lua
{
  ok = true,
  data = { logged = true },
  error = nil
}
```

**Error cases**:
- `session_not_found`: No active session with given `sessionId`.

---

## 2. Telemetry Integration

### `H.Consent.emitSessionMetrics(sessionId) -> result`

Emits a `consent_session_metrics` record conforming to `consent-session-metrics-v1.json`.

**Parameters**:
- `sessionId` (string): Opaque UUID for the session.

**Returns**:
```lua
{
  ok = true,
  data = {
    envelope_emitted = true,
    record_id = "csm-uuid-456",
    ndjson_path = "telemetry/consent/2026-04-01.ndjson"
  },
  error = nil
}
```

**Envelope structure** (emitted as NDJSON):
```json
{
  "consent_session_metrics": {
    "session_id": "sess-uuid-123",
    "user_hash": "a1b2c3...64hex",
    "consent_tier": "adult-basic",
    "policy_tier": "standard",
    "age_tier": "adult",
    "session_start": "2026-04-01T10:00:00Z",
    "session_end": "2026-04-01T10:45:00Z",
    "router_state_id": "rtr-a1b2c3d4",
    "consent_profile_id": "cp-xyz789",
    "metrics_aggregates": {
      "UEC": { "min": 0.6, "max": 0.85, "mean": 0.72 },
      "EMD": { "min": 0.5, "max": 0.8, "mean": 0.65 },
      "STCI": { "min": 0.4, "max": 0.7, "mean": 0.55 },
      "CDL": { "min": 0.3, "max": 0.6, "mean": 0.45 },
      "ARR": { "min": 0.7, "max": 0.9, "mean": 0.8 }
    },
    "explicitness_budget": {
      "budget_max": 0.8,
      "budget_used": 0.35,
      "budget_remaining": 0.45,
      "modality_breakdown": {
        "violence": { "used": 0.2, "capped": false },
        "gore": { "used": 0.1, "capped": false },
        "sexual": { "used": 0.0, "capped": false },
        "psychological": { "used": 0.05, "capped": false }
      }
    },
    "invariant_caps_applied": {
      "CIC": { "max": 0.6 },
      "MDI": { "max": 0.7 },
      "AOS": { "max": 0.7 },
      "DET": { "max": 6.0 },
      "SHCI": { "max": 0.5 }
    },
    "distress_signals": [
      {
        "signal_type": "comfort_drop",
        "timestamp": "2026-04-01T10:30:00Z",
        "severity": "low",
        "context": "peak_segment"
      }
    ],
    "segment_summaries": [
      {
        "segment_id": "pre_horror",
        "start_offset": 0,
        "end_offset": 300,
        "metrics": {
          "UEC": 0.65,
          "EMD": 0.6,
          "STCI": 0.5,
          "CDL": 0.4,
          "ARR": 0.75
        }
      }
    ],
    "governance_events": [
      {
        "event_type": "consent_upgrade",
        "timestamp": "2026-04-01T10:05:00Z",
        "deadledger_ref": "dln.envelope.consent.upgrade.v1"
      }
    ]
  }
}
```

---

## 3. Error Envelope Standard

All functions return a structured envelope:

```lua
{
  ok = boolean,      -- true if operation succeeded
  data = table,      -- result payload (nil if error)
  error = {          -- nil if ok == true
    code = string,   -- machine-readable error code
    message = string,-- human-readable description
    details = table  -- optional contextual data
  }
}
```

**Common error codes**:
- `session_not_found`
- `consent_unverified`
- `proof_missing`
- `cooldown_active`
- `budget_exhausted`
- `modality_forbidden`
- `guardrail_violation`
- `deadledger_verification_failed`

---

## 4. Integration Notes

### With Age-Gated Tier Router

- `H.Consent.currentState` uses `router_state_id` to correlate with router decisions.
- Consent transitions respect router-enforced tier caps: `effectiveTier = min(ageGateTier, consentTier, capabilityTierMax)`.

### With Intensity Budget Module

- `H.Consent.consumeBudget` and `H.Consent.checkGuardrails` share budget state with `H.Budget.*` functions.
- Invariant caps from consent state override regional caps when stricter.

### With Content Boundary Engine

- `H.Consent.currentState` provides `invariant_caps` and `explicitness_budget` to inform boundary decisions.
- Distress signals logged via `H.Consent.logDistress` may trigger the boundary engine to soften or refuse content.

### With Dead-Ledger Network

- Functions requiring proofs (`canTransition`, `requiredProofs`) return `deadledger_refs_required` for ZKP verification.
- Emitted telemetry includes `deadledger_ref` fields for governance-auditable events.

---

## 5. CI and Validation

All Lua code using this API must:

- Validate return envelopes against the `{ ok, data, error }` pattern.
- Never log raw `user_hash` or `session_id` values; use opaque references only.
- Ensure telemetry emission conforms to `consent-session-metrics-v1.json` via schema validation in CI.

A reusable GitHub Actions workflow `consent-lua-api-validate.reusable.yml` in HorrorPlace-Constellation-Contracts should enforce:

- Type safety for `H.Consent.*` calls via static analysis.
- Schema validation for emitted NDJSON records.
- Prohibition of PII-bearing fields in logged data.

---

## 6. Example Usage Pattern

```lua
-- At session start
local bind = H.Consent.bindSession(session_id, consent_profile_id)
if not bind.ok then
  return handle_error(bind.error)
end

-- Before generating content
local state = H.Consent.currentState(session_id)
if not state.ok then
  return handle_error(state.error)
end

-- Check if requested content fits budget
local budget = H.Consent.consumeBudget(session_id, "psychological", 0.05)
if not budget.ok or budget.data.over_budget then
  -- Downshift to implied-only or refuse
  return H.Templates.apply("implied_horror_template", candidate_output)
end

-- After content generation, log metrics
local emit = H.Consent.emitSessionMetrics(session_id)
if not emit.ok then
  handle_error(emit.error)
end
```

This pattern keeps consent state, budget enforcement, and telemetry emission synchronized across the session lifecycle.
