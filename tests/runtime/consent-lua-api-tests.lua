-- consent-lua-api-tests.lua
-- Test harness for H.Consent.* API surface
-- Target: HorrorPlace-Constellation-Contracts
-- Validates envelope patterns, schema compliance, and consent state machine behavior

local json = require("cjson")
local utest = require("luaunit")

-- Mock H.Consent module for testing (replaced by engine in production)
local H = { Consent = {} }

-- Canonical error codes from consent-lua-api-v1.md
local ERROR_CODES = {
  session_not_found = "session_not_found",
  consent_unverified = "consent_unverified",
  proof_missing = "proof_missing",
  cooldown_active = "cooldown_active",
  budget_exhausted = "budget_exhausted",
  modality_forbidden = "modality_forbidden",
  guardrail_violation = "guardrail_violation",
  deadledger_verification_failed = "deadledger_verification_failed"
}

-- Mock session store for isolated tests
local mock_sessions = {}

-- Helper: assert envelope structure { ok, data, error }
local function assert_envelope(result, expected_ok, expected_error_code)
  utest.assertIsTable(result)
  utest.assertIsBoolean(result.ok)
  if expected_ok ~= nil then
    utest.assertEquals(result.ok, expected_ok)
  end
  if result.ok then
    utest.assertIsNil(result.error)
    utest.assertIsNotNil(result.data)
  else
    utest.assertIsNotNil(result.error)
    utest.assertIsString(result.error.code)
    utest.assertIsString(result.error.message)
    if expected_error_code then
      utest.assertEquals(result.error.code, expected_error_code)
    end
  end
  return result
end

-- Helper: validate metrics_aggregates structure against entertainment_metrics_v1 bands
local function assert_metrics_bands(metrics)
  utest.assertIsTable(metrics)
  for _, metric in ipairs({ "UEC", "EMD", "STCI", "CDL", "ARR" }) do
    utest.assertIsNotNil(metrics[metric])
    utest.assertIsNumber(metrics[metric].min)
    utest.assertIsNumber(metrics[metric].max)
    utest.assertIsNumber(metrics[metric].mean)
    utest.assertGreaterOrEquals(metrics[metric].min, 0)
    utest.assertLessOrEquals(metrics[metric].max, 1)
    utest.assertGreaterOrEquals(metrics[metric].mean, metrics[metric].min)
    utest.assertLessOrEquals(metrics[metric].mean, metrics[metric].max)
  end
end

-- Helper: validate invariant caps structure against consent-state-machine-v1
local function assert_invariant_caps(caps)
  utest.assertIsTable(caps)
  for _, inv in ipairs({ "CIC", "MDI", "AOS", "SHCI" }) do
    if caps[inv] then
      utest.assertIsNumber(caps[inv].max)
      utest.assertGreaterOrEquals(caps[inv].max, 0)
      utest.assertLessOrEquals(caps[inv].max, 1)
    end
  end
  if caps.DET then
    utest.assertIsNumber(caps.DET.max)
    utest.assertGreaterOrEquals(caps.DET.max, 0)
    utest.assertLessOrEquals(caps.DET.max, 10)
  end
end

-------------------------------------------------------------------------------
-- Test Suite: H.Consent.currentState
-------------------------------------------------------------------------------
function TestConsentCurrentState:test_currentState_valid_session()
  local session_id = "sess-test-001"
  mock_sessions[session_id] = {
    consent_tier = "adult-basic",
    policy_tier = "standard",
    age_tier = "adult",
    invariant_caps = { CIC = 0.6, MDI = 0.7, AOS = 0.7, DET = 6.0, SHCI = 0.5 },
    explicitness_budget = { max = 0.8, used = 0.3, remaining = 0.5 },
    router_state_id = "rtr-a1b2c3d4",
    consent_profile_id = "cp-xyz789"
  }

  -- Mock implementation for test
  H.Consent.currentState = function(sid)
    if not mock_sessions[sid] then
      return { ok = false, data = nil, error = { code = ERROR_CODES.session_not_found, message = "Session not found" } }
    end
    return { ok = true, data = mock_sessions[sid], error = nil }
  end

  local result = H.Consent.currentState(session_id)
  assert_envelope(result, true)
  utest.assertEquals(result.data.consent_tier, "adult-basic")
  assert_invariant_caps(result.data.invariant_caps)
end

function TestConsentCurrentState:test_currentState_missing_session()
  H.Consent.currentState = function(sid)
    return { ok = false, data = nil, error = { code = ERROR_CODES.session_not_found, message = "Session not found" } }
  end

  local result = H.Consent.currentState("sess-nonexistent")
  assert_envelope(result, false, ERROR_CODES.session_not_found)
end

-------------------------------------------------------------------------------
-- Test Suite: H.Consent.canTransition
-------------------------------------------------------------------------------
function TestConsentCanTransition:test_canTransition_allowed()
  local session_id = "sess-test-002"
  mock_sessions[session_id] = { consent_tier = "adult-basic", age_tier = "adult" }

  H.Consent.canTransition = function(sid, target)
    if target == "adult-horror" then
      return {
        ok = true,
        data = {
          allowed = true,
          requires_proofs = { "age_verification", "explicit_opt_in" },
          cooldown_remaining_minutes = 0,
          deadledger_refs_required = { "dln.envelope.age.v1" }
        },
        error = nil
      }
    end
    return { ok = false, data = nil, error = { code = "invalid_target_tier", message = "Unknown tier" } }
  end

  local result = H.Consent.canTransition(session_id, "adult-horror")
  assert_envelope(result, true)
  utest.assertIsTrue(result.data.allowed)
  utest.assertIsTable(result.data.requires_proofs)
  utest.assertIsTable(result.data.deadledger_refs_required)
end

function TestConsentCanTransition:test_canTransition_cooldown_active()
  H.Consent.canTransition = function(sid, target)
    return {
      ok = false,
      data = nil,
      error = {
        code = ERROR_CODES.cooldown_active,
        message = "Transition blocked by cooldown window",
        details = { cooldown_remaining_minutes = 120 }
      }
    }
  end

  local result = H.Consent.canTransition("sess-test", "adult-horror")
  assert_envelope(result, false, ERROR_CODES.cooldown_active)
  utest.assertIsTable(result.error.details)
end

-------------------------------------------------------------------------------
-- Test Suite: H.Consent.consumeBudget
-------------------------------------------------------------------------------
function TestConsentConsumeBudget:test_consumeBudget_success()
  local session_id = "sess-test-003"
  mock_sessions[session_id] = {
    explicitness_budget = { max = 0.8, used = 0.3, remaining = 0.5 },
    modality_breakdown = { violence = { used = 0.2, capped = false }, gore = { used = 0.1, capped = false } }
  }

  H.Consent.consumeBudget = function(sid, modality, amount)
    if amount > mock_sessions[sid].explicitness_budget.remaining then
      return { ok = false, data = nil, error = { code = ERROR_CODES.budget_exhausted, message = "Insufficient budget" } }
    end
    mock_sessions[sid].explicitness_budget.used = mock_sessions[sid].explicitness_budget.used + amount
    mock_sessions[sid].explicitness_budget.remaining = mock_sessions[sid].explicitness_budget.remaining - amount
    return {
      ok = true,
      data = {
        budget_remaining = mock_sessions[sid].explicitness_budget.remaining,
        over_budget = false,
        modality_breakdown = mock_sessions[sid].modality_breakdown
      },
      error = nil
    }
  end

  local result = H.Consent.consumeBudget(session_id, "psychological", 0.05)
  assert_envelope(result, true)
  utest.assertEquals(result.data.budget_remaining, 0.45)
  utest.assertIsFalse(result.data.over_budget)
end

function TestConsentConsumeBudget:test_consumeBudget_modality_forbidden()
  H.Consent.consumeBudget = function(sid, modality, amount)
    if modality == "sexual" then
      return { ok = false, data = nil, error = { code = ERROR_CODES.modality_forbidden, message = "Modality not allowed for current consent tier" } }
    end
    -- fallback success for other modalities
    return { ok = true, data = { budget_remaining = 0.5, over_budget = false }, error = nil }
  end

  local result = H.Consent.consumeBudget("sess-test", "sexual", 0.1)
  assert_envelope(result, false, ERROR_CODES.modality_forbidden)
end

-------------------------------------------------------------------------------
-- Test Suite: H.Consent.checkGuardrails
-------------------------------------------------------------------------------
function TestConsentCheckGuardrails:test_checkGuardrails_no_violations()
  H.Consent.checkGuardrails = function(sid)
    return {
      ok = true,
      data = {
        guardrails_ok = true,
        violations = {},
        recommendations = {}
      },
      error = nil
    }
  end

  local result = H.Consent.checkGuardrails("sess-test")
  assert_envelope(result, true)
  utest.assertIsTrue(result.data.guardrails_ok)
  utest.assertEquals(#result.data.violations, 0)
end

function TestConsentCheckGuardrails:test_checkGuardrails_violation_detected()
  H.Consent.checkGuardrails = function(sid)
    return {
      ok = true,
      data = {
        guardrails_ok = false,
        violations = {
          {
            rule_id = "ethics.no_prolonged_high_cic",
            condition = "CIC > 0.8 for duration >= 300s",
            current_value = { CIC = 0.85, duration_seconds = 320 },
            suggested_action = "force_cool_down"
          }
        },
        recommendations = { "insert_cool_down", "reduce_explicitness" }
      },
      error = nil
    }
  end

  local result = H.Consent.checkGuardrails("sess-test")
  assert_envelope(result, true)
  utest.assertIsFalse(result.data.guardrails_ok)
  utest.assertEquals(#result.data.violations, 1)
  utest.assertEquals(result.data.violations[1].suggested_action, "force_cool_down")
end

-------------------------------------------------------------------------------
-- Test Suite: H.Consent.emitSessionMetrics (schema validation)
-------------------------------------------------------------------------------
function TestConsentEmitSessionMetrics:test_emitSessionMetrics_valid_envelope()
  -- Mock NDJSON emission (in production, writes to telemetry stream)
  local emitted_records = {}
  H.Consent.emitSessionMetrics = function(sid)
    local record = {
      consent_session_metrics = {
        session_id = sid,
        user_hash = "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2",
        consent_tier = "adult-basic",
        policy_tier = "standard",
        age_tier = "adult",
        session_start = "2026-04-01T10:00:00Z",
        session_end = "2026-04-01T10:45:00Z",
        router_state_id = "rtr-a1b2c3d4",
        consent_profile_id = "cp-xyz789",
        metrics_aggregates = {
          UEC = { min = 0.6, max = 0.85, mean = 0.72 },
          EMD = { min = 0.5, max = 0.8, mean = 0.65 },
          STCI = { min = 0.4, max = 0.7, mean = 0.55 },
          CDL = { min = 0.3, max = 0.6, mean = 0.45 },
          ARR = { min = 0.7, max = 0.9, mean = 0.8 }
        },
        explicitness_budget = {
          budget_max = 0.8,
          budget_used = 0.35,
          budget_remaining = 0.45,
          modality_breakdown = {
            violence = { used = 0.2, capped = false },
            gore = { used = 0.1, capped = false },
            sexual = { used = 0.0, capped = false },
            psychological = { used = 0.05, capped = false }
          }
        },
        invariant_caps_applied = {
          CIC = { max = 0.6 },
          MDI = { max = 0.7 },
          AOS = { max = 0.7 },
          DET = { max = 6.0 },
          SHCI = { max = 0.5 }
        }
      }
    }
    table.insert(emitted_records, record)
    return { ok = true, data = { envelope_emitted = true, record_id = "csm-uuid-456" }, error = nil }
  end

  local result = H.Consent.emitSessionMetrics("sess-test-004")
  assert_envelope(result, true)

  -- Validate emitted record structure matches consent-session-metrics-v1 schema expectations
  local record = emitted_records[1]
  utest.assertIsNotNil(record.consent_session_metrics)
  local metrics = record.consent_session_metrics
  utest.assertMatches(metrics.user_hash, "^[a-f0-9]{64}$")
  utest.assertMatches(metrics.session_id, "^sess%-test%-")
  assert_metrics_bands(metrics.metrics_aggregates)
  assert_invariant_caps(metrics.invariant_caps_applied)
  utest.assertEquals(metrics.explicitness_budget.budget_remaining, 0.45)
end

-------------------------------------------------------------------------------
-- Test Suite: Error envelope consistency
-------------------------------------------------------------------------------
function TestErrorEnvelope:test_all_errors_follow_standard_shape()
  local error_cases = {
    { code = ERROR_CODES.session_not_found, message = "Session not found" },
    { code = ERROR_CODES.proof_missing, message = "Required proof not present", details = { proof_type = "age_verification" } },
    { code = ERROR_CODES.deadledger_verification_failed, message = "ZKP verification failed", details = { envelope_id = "dln.envelope.test.v1" } }
  }

  for _, err in ipairs(error_cases) do
    local result = { ok = false, data = nil, error = err }
    assert_envelope(result, false)
    utest.assertMatches(result.error.code, "^[a-z_]+$")
    utest.assertGreaterOrEquals(#result.error.message, 10)
  end
end

-------------------------------------------------------------------------------
-- Test Suite: Dead-Ledger reference enforcement for high tiers
-------------------------------------------------------------------------------
function TestDeadLedgerEnforcement:test_high_tier_transition_requires_deadledger_ref()
  -- Simulate transition to adult-horror or research tier
  local function requires_deadledger_ref(target_tier)
    return target_tier == "adult-horror" or target_tier == "research"
  end

  utest.assertIsTrue(requires_deadledger_ref("adult-horror"))
  utest.assertIsTrue(requires_deadledger_ref("research"))
  utest.assertIsFalse(requires_deadledger_ref("adult-basic"))
  utest.assertIsFalse(requires_deadledger_ref("minor"))
end

-------------------------------------------------------------------------------
-- Test Suite: PII safety in telemetry
-------------------------------------------------------------------------------
function TestPIISafety:test_no_pii_in_emitted_metrics()
  local prohibited_patterns = {
    "user_name", "email", "phone", "address", "ssn", "credit_card",
    "raw_bci_signal", "neural_trace", "brain_wave_raw",
    "password", "secret", "api_key", "token"
  }

  -- Mock a telemetry record that should pass PII scan
  local safe_record = {
    consent_session_metrics = {
      session_id = "sess-uuid-123",
      user_hash = "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2",
      consent_tier = "adult-basic"
      -- ... other fields
    }
  }

  -- Verify no prohibited keys appear at top level or nested
  local function scan_for_pii(obj, path)
    path = path or "root"
    if type(obj) == "table" then
      for k, v in pairs(obj) do
        local key_path = path .. "." .. tostring(k)
        for _, pattern in ipairs(prohibited_patterns) do
          if string.match(tostring(k):lower(), pattern:lower()) then
            error("PII pattern '" .. pattern .. "' found at " .. key_path)
          end
        end
        scan_for_pii(v, key_path)
      end
    end
  end

  -- Should not raise
  local ok, err = pcall(scan_for_pii, safe_record)
  utest.assertIsTrue(ok, "PII scan should pass for safe record: " .. tostring(err))
end

-------------------------------------------------------------------------------
-- Test runner
-------------------------------------------------------------------------------
function TestConsentAPI:test_all()
  -- LuaUnit auto-discovers test_* methods; this is a placeholder for explicit sequencing if needed
  print("Consent Lua API test suite completed.")
end

-- Run tests if executed directly
if arg and arg[0] and string.match(arg[0], "consent%-lua%-api%-tests") then
  os.exit(utest.LuaUnit.run())
end

return {
  H = H,
  ERROR_CODES = ERROR_CODES,
  assert_envelope = assert_envelope,
  assert_metrics_bands = assert_metrics_bands,
  assert_invariant_caps = assert_invariant_caps
}
