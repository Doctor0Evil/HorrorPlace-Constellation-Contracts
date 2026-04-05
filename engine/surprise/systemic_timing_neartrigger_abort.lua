-- engine/surprise/systemic_timing_neartrigger_abort.lua

local H = require("engine.horror_invariants")
local Metrics = require("engine.telemetry_metrics")
local Events = require("engine.events")

local M = {}

function M.try_arm_or_abort(region_id, player_id, dt)
  local det = H.DET(region_id, player_id)
  local cic = H.CIC(region_id)
  local mdi = H.MDI(region_id)
  local aos = H.AOS(region_id)

  if det < 2.0 or det > 7.0 then
    return false
  end
  if cic < 0.4 or mdi < 0.3 or aos < 0.3 then
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

return M
