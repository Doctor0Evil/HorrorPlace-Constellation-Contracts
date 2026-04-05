-- engine/surprise/systemic_timing_scheduled_jump.lua

local H = require("engine.horror_invariants")
local Metrics = require("engine.telemetry_metrics")
local Events = require("engine.events")

local Jump = {}

function Jump.maybe_fire_or_deflect(region_id, player_id, dt)
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
    Events.trigger("surprise.jump_scare", { region_id = region_id, player_id = player_id })
    Events.log("surprise.schedjump_fired", { region_id = region_id, player_id = player_id, det = det, uec = uec, arr = arr })
    return true
  else
    Events.log("surprise.schedjump_deflected", { region_id = region_id, player_id = player_id, det = det, uec = uec, arr = arr })
    return false
  end
end

return Jump
