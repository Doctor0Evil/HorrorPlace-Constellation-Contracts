-- engine/surprise/systemic_timing_delayed_payoff.lua

local H = require("engine.horror_invariants")
local Metrics = require("engine.telemetry_metrics")
local Events = require("engine.events")
local Payoff = {}

local pending = {}

function Payoff.schedule(region_src, region_dst, player_id, delay_seconds)
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

function Payoff.tick(now)
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

return Payoff
