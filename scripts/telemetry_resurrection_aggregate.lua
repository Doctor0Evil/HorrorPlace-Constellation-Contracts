local json = require("dkjson")

local function read_runs(path)
  local runs = {}
  local f = io.open(path, "r")
  if not f then
    error("failed to open " .. tostring(path))
  end
  for line in f:lines() do
    if line ~= "" then
      local obj, pos, err = json.decode(line, 1, nil)
      if obj then
        table.insert(runs, obj)
      end
    end
  end
  f:close()
  return runs
end

local function key_for(run)
  local eid = run.eventId or ""
  return run.seedId .. "|" .. run.resurrectionKind .. "|" .. eid
end

local function aggregate(runs)
  local groups = {}
  for _, run in ipairs(runs) do
    local k = key_for(run)
    local g = groups[k]
    if not g then
      g = {
        seedId = run.seedId,
        eventId = run.eventId,
        resurrectionKind = run.resurrectionKind,
        attempts = 0,
        allowed = 0,
        blocked = 0,
        scores = {},
        detOverflows = 0,
        cdlOverflows = 0,
        arrTooLow = 0,
        sumUECdelta = 0.0,
        sumEMDdelta = 0.0,
        sumSTCIdelta = 0.0,
        sumCDLdelta = 0.0,
        sumARR = 0.0,
        firstSeen = run.timestamps and run.timestamps.recordedAt,
        lastSeen = run.timestamps and run.timestamps.recordedAt
      }
      groups[k] = g
    end

    g.attempts = g.attempts + 1
    if run.gate and run.gate.allowed then
      g.allowed = g.allowed + 1
    else
      g.blocked = g.blocked + 1
    end

    if run.quality and run.quality.localScore then
      table.insert(g.scores, run.quality.localScore)
    end

    local mb, ma = run.metricsBefore, run.metricsAfter
    if mb and ma then
      g.sumUECdelta  = g.sumUECdelta  + (ma.UEC  - mb.UEC)
      g.sumEMDdelta  = g.sumEMDdelta  + (ma.EMD  - mb.EMD)
      g.sumSTCIdelta = g.sumSTCIdelta + (ma.STCI - mb.STCI)
      g.sumCDLdelta  = g.sumCDLdelta  + (ma.CDL  - mb.CDL)
      g.sumARR       = g.sumARR       + ma.ARR
    end

    local detCap = 0.85
    local cdlCap = 0.75
    local arrMin = 0.6
    if ma and ma.DET and ma.DET > detCap then
      g.detOverflows = g.detOverflows + 1
    end
    if ma and ma.CDL and ma.CDL > cdlCap then
      g.cdlOverflows = g.cdlOverflows + 1
    end
    if ma and ma.ARR and ma.ARR < arrMin then
      g.arrTooLow = g.arrTooLow + 1
    end

    local t = run.timestamps and run.timestamps.recordedAt
    if t then
      if not g.firstSeen or t < g.firstSeen then
        g.firstSeen = t
      end
      if not g.lastSeen or t > g.lastSeen then
        g.lastSeen = t
      end
    end
  end

  local out = {}
  for _, g in pairs(groups) do
    table.sort(g.scores)
    local n = #g.scores
    local avg = 0.0
    for _, s in ipairs(g.scores) do
      avg = avg + s
    end
    if n > 0 then
      avg = avg / n
    end
    local function percentile(p)
      if n == 0 then return 0.0 end
      local idx = math.floor(p * (n - 1) + 1)
      return g.scores[idx]
    end

    local avgUEC  = (g.attempts > 0) and (g.sumUECdelta  / g.attempts) or 0.0
    local avgEMD  = (g.attempts > 0) and (g.sumEMDdelta  / g.attempts) or 0.0
    local avgSTCI = (g.attempts > 0) and (g.sumSTCIdelta / g.attempts) or 0.0
    local avgCDL  = (g.attempts > 0) and (g.sumCDLdelta  / g.attempts) or 0.0
    local avgARR  = (g.attempts > 0) and (g.sumARR       / g.attempts) or 0.0

    local overflowPenalty = g.detOverflows + g.cdlOverflows + g.arrTooLow
    local rwfScore = math.max(0.0, math.min(1.0, avg - 0.05 * overflowPenalty))

    local status
    if rwfScore >= 0.8 and overflowPenalty == 0 then
      status = "stable"
    elseif rwfScore >= 0.5 then
      status = "provisional"
    elseif overflowPenalty > 0 then
      status = "quarantine"
    else
      status = "experimental"
    end

    table.insert(out, {
      id = string.format("RESQ-%s-%s", g.seedId, g.resurrectionKind),
      seedId = g.seedId,
      eventId = g.eventId,
      resurrectionKind = g.resurrectionKind,
      counters = {
        attempts = g.attempts,
        allowed  = g.allowed,
        blocked  = g.blocked
      },
      scores = {
        avgLocal = avg,
        p25Local = percentile(0.25),
        p75Local = percentile(0.75)
      },
      metricsRealized = {
        avgUECdelta  = avgUEC,
        avgEMDdelta  = avgEMD,
        avgSTCIdelta = avgSTCI,
        avgCDLdelta  = avgCDL,
        avgARR       = avgARR
      },
      safety = {
        detOverflows = g.detOverflows,
        cdlOverflows = g.cdlOverflows,
        arrTooLow    = g.arrTooLow
      },
      rwfHint = {
        rwfScore = rwfScore,
        status   = status
      },
      timestamps = {
        firstSeen = g.firstSeen,
        lastSeen  = g.lastSeen
      }
    })
  end

  return out
end

local function main()
  local path = arg[1] or "-"
  local runs
  if path == "-" then
    local tmp = {}
    for line in io.lines() do
      if line ~= "" then
        local obj = json.decode(line)
        if obj then table.insert(tmp, obj) end
      end
    end
    runs = tmp
  else
    runs = read_runs(path)
  end

  local aggregated = aggregate(runs)
  for _, row in ipairs(aggregated) do
    io.write(json.encode(row), "\n")
  end
end

main()
