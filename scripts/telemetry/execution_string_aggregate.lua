-- scripts/telemetry/execution_string_aggregate.lua
--
-- NDJSON aggregator for execution-string-run.v1 telemetry.
-- Usage:
--   lua scripts/telemetry/execution_string_aggregate.lua --input path/to/log.ndjson > execution-string-aggregate.ndjson
--
-- Input:  one execution-string-run.v1 JSON object per line.
-- Output: one aggregate JSON object per (executionId, executionKind) group per line.

local json = require("dkjson")

local function parse_args(argv)
  local cfg = { input = nil }
  local i = 1
  while i <= #argv do
    local a = argv[i]
    if a == "--input" or a == "-i" then
      i = i + 1
      cfg.input = argv[i]
    elseif a == "--help" or a == "-h" then
      print("Usage: lua execution_string_aggregate.lua --input path/to/log.ndjson > aggregate.ndjson")
      os.exit(0)
    else
      io.stderr:write("Unknown argument: " .. tostring(a) .. "\n")
      os.exit(2)
    end
    i = i + 1
  end
  if not cfg.input or cfg.input == "-" then
    cfg.input = "-"
  end
  return cfg
end

local function open_input(path)
  if path == "-" then
    return io.stdin
  end
  local f, err = io.open(path, "r")
  if not f then
    io.stderr:write("Failed to open input: " .. tostring(err) .. "\n")
    os.exit(2)
  end
  return f
end

local function nearest_rank_percentile(values, p)
  if #values == 0 then
    return 0.0
  end
  table.sort(values)
  local n = #values
  local idx = math.floor(p * (n - 1) + 1)
  if idx < 1 then idx = 1 end
  if idx > n then idx = n end
  return values[idx]
end

local function key_for(run)
  local eid = run.executionId or "unknown"
  local kind = run.executionKind or "unknown"
  return eid .. "|" .. kind
end

local function accumulate(groups, run)
  local k = key_for(run)
  local g = groups[k]
  if not g then
    g = {
      executionId = run.executionId or "unknown",
      executionKind = run.executionKind or "unknown",
      tier = run.tier or "unknown",
      attempts = 0,
      allowed = 0,
      downgraded = 0,
      detOverflows = 0,
      cdlOverflows = 0,
      arrTooLow = 0,
      localScores = {},
      deltaUECs = {},
      deltaARRs = {},
      deltaDETs = {}
    }
    groups[k] = g
  end

  g.attempts = g.attempts + 1
  if run.gate and run.gate.allowed then
    g.allowed = g.allowed + 1
  end
  if run.gate and run.gate.downgraded then
    g.downgraded = g.downgraded + 1
  end

  if run.safety then
    if run.safety.detOverflow then
      g.detOverflows = g.detOverflows + 1
    end
    if run.safety.cdlOverflow then
      g.cdlOverflows = g.cdlOverflows + 1
    end
    if run.safety.arrTooLow then
      g.arrTooLow = g.arrTooLow + 1
    end
  end

  if run.quality then
    local ls = tonumber(run.quality.localScore)
    if ls then
      table.insert(g.localScores, ls)
    end
    local dUEC = tonumber(run.quality.deltaUEC)
    if dUEC then
      table.insert(g.deltaUECs, dUEC)
    end
    local dARR = tonumber(run.quality.deltaARR)
    if dARR then
      table.insert(g.deltaARRs, dARR)
    end
    local dDET = tonumber(run.quality.deltaDET)
    if dDET then
      table.insert(g.deltaDETs, dDET)
    end
  end
end

local function compute_rwf(group)
  if group.attempts == 0 then
    return 0.0, "experimental"
  end

  local base = 1.0
  local penalty_det = 0.05 * group.detOverflows
  local penalty_cdl = 0.05 * group.cdlOverflows
  local penalty_arr = 0.05 * group.arrTooLow

  local rwf = base - (penalty_det + penalty_cdl + penalty_arr)
  if rwf < 0.0 then rwf = 0.0 end
  if rwf > 1.0 then rwf = 1.0 end

  local status
  if rwf >= 0.8 then
    status = "stable"
  elseif rwf >= 0.6 then
    status = "provisional"
  elseif rwf >= 0.4 then
    status = "experimental"
  else
    status = "quarantine"
  end

  return rwf, status
end

local function aggregate(groups)
  local out = {}

  for _, g in pairs(groups) do
    local avgLocal = 0.0
    local p25Local = 0.0
    local p75Local = 0.0
    if #g.localScores > 0 then
      local sum = 0.0
      for _, v in ipairs(g.localScores) do
        sum = sum + v
      end
      avgLocal = sum / #g.localScores
      p25Local = nearest_rank_percentile(g.localScores, 0.25)
      p75Local = nearest_rank_percentile(g.localScores, 0.75)
    end

    local avgDeltaUEC = 0.0
    if #g.deltaUECs > 0 then
      local s = 0.0
      for _, v in ipairs(g.deltaUECs) do
        s = s + v
      end
      avgDeltaUEC = s / #g.deltaUECs
    end

    local avgDeltaARR = 0.0
    if #g.deltaARRs > 0 then
      local s = 0.0
      for _, v in ipairs(g.deltaARRs) do
        s = s + v
      end
      avgDeltaARR = s / #g.deltaARRs
    end

    local avgDeltaDET = 0.0
    if #g.deltaDETs > 0 then
      local s = 0.0
      for _, v in ipairs(g.deltaDETs) do
        s = s + v
      end
      avgDeltaDET = s / #g.deltaDETs
    end

    local rwfScore, status = compute_rwf(g)

    local record = {
      version = "1.0.0",
      schemaRef = "execution-string-aggregate.v1",
      recordedAt = os.date("!%Y-%m-%dT%H:%M:%SZ"),
      executionId = g.executionId,
      executionKind = g.executionKind,
      tier = g.tier,
      counters = {
        attempts = g.attempts,
        allowed = g.allowed,
        downgraded = g.downgraded
      },
      safety = {
        detOverflows = g.detOverflows,
        cdlOverflows = g.cdlOverflows,
        arrTooLow = g.arrTooLow
      },
      scores = {
        avgLocal = avgLocal,
        p25Local = p25Local,
        p75Local = p75Local
      },
      deltas = {
        avgDeltaUEC = avgDeltaUEC,
        avgDeltaARR = avgDeltaARR,
        avgDeltaDET = avgDeltaDET
      },
      rwfHint = {
        rwfScore = rwfScore,
        status = status
      }
    }

    table.insert(out, record)
  end

  table.sort(out, function(a, b)
    if a.executionId == b.executionId then
      return a.executionKind < b.executionKind
    end
    return a.executionId < b.executionId
  end)

  return out
end

local function main()
  local cfg = parse_args(arg)
  local fh = open_input(cfg.input)
  local groups = {}

  for line in fh:lines() do
    if line ~= "" then
      local obj, pos, err = json.decode(line, 1, nil)
      if not obj then
        io.stderr:write("Failed to decode JSON at pos " .. tostring(pos) .. ": " .. tostring(err) .. "\n")
        os.exit(1)
      end
      if obj.schemaRef == "execution-string-run.v1" then
        accumulate(groups, obj)
      else
        -- ignore other records; this script is dedicated to execution_string runs
      end
    end
  end

  if fh ~= io.stdin then
    fh:close()
  end

  local aggregates = aggregate(groups)
  for _, rec in ipairs(aggregates) do
    local txt = json.encode(rec)
    io.write(txt)
    io.write("\n")
  end
end

main()
