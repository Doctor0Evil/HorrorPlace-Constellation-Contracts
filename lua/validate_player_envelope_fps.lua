-- validate_player_envelope_fps.lua
-- Executable validator for H.PlayerEnvelope.FPS.v1 envelopes.
-- This module encodes derived safety inequalities that cannot be expressed
-- directly in JSON Schema. It is intended for use in CI and offline checks.

local M = {}

local function sum_sanity_weights(w)
  return (w.wCIC or 0.0)
       + (w.wDET or 0.0)
       + (w.wLSG or 0.0)
       + (w.wHVF or 0.0)
       + (w.wUEC or 0.0)
       + (w.wEMD or 0.0)
       + (w.wCDL or 0.0)
       + (w.wARR or 0.0)
end

local function assert_number(name, v)
  if type(v) ~= "number" then
    error(string.format("ERR_PLAYERENV_FIELD_TYPE: %s must be number", name))
  end
end

local function assert_bool(name, v)
  if type(v) ~= "boolean" then
    error(string.format("ERR_PLAYERENV_FIELD_TYPE: %s must be boolean", name))
  end
end

local function assert_table(name, v)
  if type(v) ~= "table" then
    error(string.format("ERR_PLAYERENV_FIELD_TYPE: %s must be object", name))
  end
end

local function require_field(tbl, name)
  local v = tbl[name]
  if v == nil then
    error(string.format("ERR_PLAYERENV_FIELD_MISSING: %s", name))
  end
  return v
end

--- Validate high-level structure and derived safety inequalities.
-- @param env table Parsed H.PlayerEnvelope.FPS.v1 JSON.
-- @param caps table Tier caps for this tier (usually derived from spine safeDefaults).
-- @return true on success or raises error on violation.
function M.validate_player_envelope(env, caps)
  assert_table("env", env)
  assert_table("caps", caps)

  local tier = require_field(env, "tier")
  local tick = require_field(env, "tickSeconds")
  local T    = require_field(env, "sessionHorizonSeconds")

  assert_number("tickSeconds", tick)
  assert_number("sessionHorizonSeconds", T)
  if tick <= 0.0 then
    error("ERR_PLAYERENV_TICK_NONPOSITIVE")
  end
  if T <= 0.0 then
    error("ERR_PLAYERENV_SESSION_HORIZON_NONPOSITIVE")
  end

  local safeCaps = require_field(env, "safeCaps")
  assert_table("safeCaps", safeCaps)

  local vMax       = require_field(safeCaps, "vMax")
  local staminaMin = require_field(safeCaps, "staminaMin")
  local sanityMin  = require_field(safeCaps, "sanityMin")
  local exposureMax= require_field(safeCaps, "exposureMax")
  local opacityMax = require_field(safeCaps, "opacityMax")
  local batteryMin = require_field(safeCaps, "batteryMin")
  local flashDuty  = require_field(safeCaps, "flashOnDutyMax")

  assert_number("safeCaps.vMax", vMax)
  assert_number("safeCaps.staminaMin", staminaMin)
  assert_number("safeCaps.sanityMin", sanityMin)
  assert_number("safeCaps.exposureMax", exposureMax)
  assert_number("safeCaps.opacityMax", opacityMax)
  assert_number("safeCaps.batteryMin", batteryMin)
  assert_number("safeCaps.flashOnDutyMax", flashDuty)

  if flashDuty < 0.0 or flashDuty > 1.0 then
    error("ERR_PLAYERENV_FLASH_DUTY_RANGE")
  end

  -- Stamina envelope
  local stamina = require_field(env, "stamina")
  assert_table("stamina", stamina)

  local lambdaDrain  = require_field(stamina, "lambdaDrain")
  local betaEnv      = require_field(stamina, "betaEnv")
  local lambdaRest   = require_field(stamina, "lambdaRest")
  local sigmaStamina = require_field(stamina, "sigmaStamina")
  local staminaFloor = require_field(stamina, "minFloor")

  assert_number("stamina.lambdaDrain", lambdaDrain)
  assert_number("stamina.betaEnv",     betaEnv)
  assert_number("stamina.lambdaRest",  lambdaRest)
  assert_number("stamina.sigmaStamina", sigmaStamina)
  assert_number("stamina.minFloor",    staminaFloor)

  if staminaFloor < staminaMin then
    error("ERR_PLAYERENV_STAMINA_FLOOR_BELOW_TIER")
  end

  do
    local env_factor = 1.0 + 3.0 * math.max(betaEnv, 0.0)
    local per_second = lambdaDrain * env_factor + math.max(sigmaStamina, 0.0)
    local max_drop   = T * per_second
    local band       = 1.0 - staminaFloor
    if max_drop > band + 1e-6 then
      error("ERR_PLAYERENV_STAMINA_EXCURSION")
    end
  end

  -- Sanity envelope
  local sanity = require_field(env, "sanity")
  assert_table("sanity", sanity)

  local lambdaDecay   = require_field(sanity, "lambdaDecay")
  local lambdaRecover = require_field(sanity, "lambdaRecover")
  local sigmaSanity   = require_field(sanity, "sigmaSanity")
  local sanityFloor   = require_field(sanity, "minFloor")
  local weights       = require_field(sanity, "weights")
  local gammaArousal  = require_field(sanity, "gammaArousal")
  local gammaOverload = require_field(sanity, "gammaOverload")

  assert_number("sanity.lambdaDecay",   lambdaDecay)
  assert_number("sanity.lambdaRecover", lambdaRecover)
  assert_number("sanity.sigmaSanity",   sigmaSanity)
  assert_number("sanity.minFloor",      sanityFloor)
  assert_table("sanity.weights",        weights)
  assert_number("sanity.gammaArousal",  gammaArousal)
  assert_number("sanity.gammaOverload", gammaOverload)

  if sanityFloor < sanityMin then
    error("ERR_PLAYERENV_SANITY_FLOOR_BELOW_TIER")
  end

  local W = sum_sanity_weights(weights)
  local gammaMax = math.max(gammaArousal, 0.0) + math.max(gammaOverload, 0.0)
  local per_second_san = lambdaDecay * W * (1.0 + gammaMax) + math.max(sigmaSanity, 0.0)
  local max_san_drop   = T * per_second_san
  local san_band       = 1.0 - sanityFloor
  if max_san_drop > san_band + 1e-6 then
    error("ERR_PLAYERENV_SANITY_EXCURSION")
  end

  -- Opacity weights sum constraint
  local opacity = require_field(env, "opacity")
  assert_table("opacity", opacity)

  local theta1 = require_field(opacity, "theta1")
  local theta2 = require_field(opacity, "theta2")
  local theta3 = require_field(opacity, "theta3")
  local sigmaO = require_field(opacity, "sigmaO")
  local lambdaO= require_field(opacity, "lambdaO")

  assert_number("opacity.theta1", theta1)
  assert_number("opacity.theta2", theta2)
  assert_number("opacity.theta3", theta3)
  assert_number("opacity.sigmaO", sigmaO)
  assert_number("opacity.lambdaO", lambdaO)

  local theta_sum = theta1 + theta2 + theta3
  if theta_sum > 1.0 + 1e-6 then
    error("ERR_PLAYERENV_OPACITY_THETA_SUM_GT_ONE")
  end

  if opacityMax < 1.0 and opacityMax > 0.0 then
    -- nothing extra here yet; reserved for tighter opacity excursion checks
  end

  -- Exposure envelope (R)
  local exposure = require_field(env, "exposure")
  assert_table("exposure", exposure)

  local lambdaR = require_field(exposure, "lambdaR")
  local sigmaR  = require_field(exposure, "sigmaR")

  assert_number("exposure.lambdaR", lambdaR)
  assert_number("exposure.sigmaR",  sigmaR)

  -- Battery envelope
  local battery = require_field(env, "battery")
  assert_table("battery", battery)

  local lambdaBD  = require_field(battery, "lambdaDrain")
  local lambdaBR  = require_field(battery, "lambdaRecover")
  local phiCIC    = require_field(battery, "phiCIC")
  local sigmaBat  = require_field(battery, "sigmaBattery")
  local minReserve= require_field(battery, "minReserve")

  assert_number("battery.lambdaDrain",  lambdaBD)
  assert_number("battery.lambdaRecover",lambdaBR)
  assert_number("battery.phiCIC",       phiCIC)
  assert_number("battery.sigmaBattery", sigmaBat)
  assert_number("battery.minReserve",   minReserve)

  if minReserve < batteryMin then
    error("ERR_PLAYERENV_BATTERY_RESERVE_BELOW_TIER")
  end

  do
    local max_slope = lambdaBD * (1.0 + math.max(phiCIC, 0.0)) + math.max(sigmaBat, 0.0)
    local duty      = math.max(math.min(flashDuty, 1.0), 0.0)
    local max_drop  = duty * T * max_slope
    local bat_band  = 1.0 - minReserve
    if max_drop > bat_band + 1e-6 then
      error("ERR_PLAYERENV_BATTERY_EXCURSION")
    end
  end

  -- Noise configuration sanity
  local noise = require_field(env, "noise")
  assert_table("noise", noise)
  local enabled = require_field(noise, "enabled")
  assert_bool("noise.enabled", enabled)

  return true
end

return M
