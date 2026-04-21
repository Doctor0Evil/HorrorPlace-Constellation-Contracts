-- lua/runtime/hmetrics.lua

local H = H or {}
H.Metrics = H.Metrics or {}

local Metrics = {}
local Telemetry = require("horror.telemetry")  -- thin adapter you provide

local function clamp(x, minv, maxv)
  if x < minv then return minv end
  if x > maxv then return maxv end
  return x
end

local function safe_div(num, den)
  if not den or den == 0 then
    return nil
  end
  return num / den
end

-- Internal: compute simple aggregates over a scalar series.
local function summarize_series(values)
  local count = #values
  if count == 0 then
    return {
      count = 0,
      mean = nil,
      min = nil,
      max = nil,
      first = nil,
      last = nil,
    }
  end

  local sum = 0
  local minv = values[1]
  local maxv = values[1]
  for i = 1, count do
    local v = values[i]
    sum = sum + v
    if v < minv then minv = v end
    if v > maxv then maxv = v end
  end

  return {
    count = count,
    mean = sum / count,
    min = minv,
    max = maxv,
    first = values[1],
    last = values[count],
  }
end

-- Internal: compute simple peak statistics for DET.
local function summarize_det(values, threshold)
  local peaks = 0
  local above = false
  for i = 1, #values do
    local v = values[i]
    if v >= threshold then
      if not above then
        peaks = peaks + 1
        above = true
      end
    else
      above = false
    end
  end
  return {
    peakCount = peaks,
    threshold = threshold,
  }
end

-- Public: compute derived session metrics for UEC, EMD, STCI, CDL, ARR, DET.
-- Returns the standard ok/data/error envelope.
--
-- data.derivedMetrics has the shape the session route envelope expects, e.g.:
--   {
--     uec = { mean = ..., min = ..., max = ..., first = ..., last = ..., count = ... },
--     emd = { ... },
--     stci = { ... },
--     cdl = { ... },
--     arr = { ... },
--     det = {
--       series = { mean = ..., min = ..., max = ..., first = ..., last = ..., count = ... },
--       peaks  = { peakCount = ..., threshold = ... },
--       area   = { approximate = ..., stepCount = ... },
--     },
--     length = { turns = N },
--   }
function Metrics.computeDerivedMetrics(sessionId)
  if not sessionId or sessionId == "" then
    return {
      ok = false,
      data = nil,
      error = {
        code = "INVALID_SESSION_ID",
        message = "sessionId is required",
        details = nil,
      },
    }
  end

  local ok, route, err = Telemetry.loadSessionRoute(sessionId)
  if not ok then
    return {
      ok = false,
      data = nil,
      error = {
        code = err and err.code or "ROUTE_NOT_FOUND",
        message = err and err.message or "No session route found for sessionId",
        details = err and err.details or { sessionId = sessionId },
      },
    }
  end

  local steps = route.steps or {}
  local n = #steps
  if n == 0 then
    return {
      ok = true,
      data = {
        derivedMetrics = {
          uec = summarize_series({}),
          emd = summarize_series({}),
          stci = summarize_series({}),
          cdl = summarize_series({}),
          arr = summarize_series({}),
          det = {
            series = summarize_series({}),
            peaks = {
              peakCount = 0,
              threshold = nil,
            },
            area = {
              approximate = 0,
              stepCount = 0,
            },
          },
          length = { turns = 0 },
        },
      },
      error = nil,
    }
  end

  local uec_series = {}
  local emd_series = {}
  local stci_series = {}
  local cdl_series = {}
  local arr_series = {}
  local det_series = {}

  for i = 1, n do
    local m = steps[i].metrics or {}
    if type(m.UEC) == "number" then table.insert(uec_series, m.UEC) end
    if type(m.EMD) == "number" then table.insert(emd_series, m.EMD) end
    if type(m.STCI) == "number" then table.insert(stci_series, m.STCI) end
    if type(m.CDL) == "number" then table.insert(cdl_series, m.CDL) end
    if type(m.ARR) == "number" then table.insert(arr_series, m.ARR) end
    if type(m.DET) == "number" then table.insert(det_series, m.DET) end
  end

  local uec_summary = summarize_series(uec_series)
  local emd_summary = summarize_series(emd_series)
  local stci_summary = summarize_series(stci_series)
  local cdl_summary = summarize_series(cdl_series)
  local arr_summary = summarize_series(arr_series)
  local det_summary = summarize_series(det_series)

  -- Approximate DET area under curve using simple rectangle method over steps.
  local det_area = 0
  for i = 1, #det_series do
    det_area = det_area + det_series[i]
  end

  local det_peaks = summarize_det(det_series, 0.7)  -- you can tune threshold per tier

  local derived = {
    uec = uec_summary,
    emd = emd_summary,
    stci = stci_summary,
    cdl = cdl_summary,
    arr = arr_summary,
    det = {
      series = det_summary,
      peaks = det_peaks,
      area = {
        approximate = det_area,
        stepCount = #det_series,
      },
    },
    length = {
      turns = n,
    },
  }

  return {
    ok = true,
    data = {
      derivedMetrics = derived,
    },
    error = nil,
  }
end

H.Metrics.computeDerivedMetrics = Metrics.computeDerivedMetrics

return H.Metrics
