-- tooling/lua/hpc_contractcards.lua
--
-- Lightweight contract card band-check helper.
--
-- Responsibilities:
-- - Load schema-spine-index-v1.json
-- - Load a contract card JSON file
-- - Given objectKind and tier (from envelope or contract), look up
--   canonical invariant and metric bands from the spine
-- - Compare invariantBindings and metricTargets against those bands
-- - Return / print a concise summary per invariant/metric:
--     "within", "edge", or "outside" band
--
-- Dependencies:
--   - json: a minimal JSON library providing json.decode(string) -> table

local json = require("json")  -- adapt to your JSON lib name

local HpcContractCards = {}

-- Configuration: relative path to spine index (can be overridden)
HpcContractCards.spine_path = "schemas/core/schema-spine-index-v1.json"

----------------------------------------------------------------------
-- Internal helpers
----------------------------------------------------------------------

local function read_file(path)
  local f, err = io.open(path, "r")
  if not f then
    return nil, ("unable to open file '%s': %s"):format(path, tostring(err))
  end
  local content = f:read("*a")
  f:close()
  return content, nil
end

local function load_json_file(path)
  local raw, err = read_file(path)
  if not raw then
    return nil, err
  end
  local ok, decoded = pcall(function()
    return json.decode(raw)
  end)
  if not ok then
    return nil, ("failed to decode JSON from '%s': %s"):format(path, tostring(decoded))
  end
  return decoded, nil
end

local function abs(x)
  if x < 0 then
    return -x
  end
  return x
end

----------------------------------------------------------------------
-- Spine loading and lookup
----------------------------------------------------------------------

local function load_spine(spine_path)
  local spine, err = load_json_file(spine_path)
  if not spine then
    return nil, err
  end
  return spine, nil
end

local function find_schema_entry_for_object_kind(spine, object_kind)
  if type(spine) ~= "table" or type(spine.schemas) ~= "table" then
    return nil
  end
  for _, entry in ipairs(spine.schemas) do
    if entry.object_kind == object_kind or entry.objectKind == object_kind then
      return entry
    end
  end
  return nil
end

local function build_band_index(catalog, contract_family)
  local index = {}
  if type(catalog) ~= "table" then
    return index
  end
  for _, entry in ipairs(catalog) do
    local key = entry.key
    if type(key) == "string" then
      local family = entry.contract_family or entry.contractFamily
      if not contract_family or not family or family == contract_family then
        index[key] = index[key] or {}
        table.insert(index[key], {
          tier = entry.tier,
          min = entry.min,
          max = entry.max,
        })
      end
    end
  end
  return index
end

local function pick_band(bands, tier)
  if type(bands) ~= "table" then
    return nil
  end
  if not tier then
    return bands[1]
  end
  for _, b in ipairs(bands) do
    if b.tier == tier then
      return b
    end
  end
  return bands[1]
end

----------------------------------------------------------------------
-- Band classification
----------------------------------------------------------------------

local function classify_value(value, band)
  if type(value) ~= "number" or type(band) ~= "table" then
    return "unknown"
  end

  local min_v = band.min
  local max_v = band.max
  if type(min_v) ~= "number" and type(max_v) ~= "number" then
    return "unknown"
  end

  local lower = min_v or value
  local upper = max_v or value

  if value < lower or value > upper then
    return "outside"
  end

  local span = upper - lower
  if span <= 0 then
    return "within"
  end

  local dist_to_lower = abs(value - lower)
  local dist_to_upper = abs(upper - value)
  local edge_threshold = span * 0.1

  if dist_to_lower <= edge_threshold or dist_to_upper <= edge_threshold then
    return "edge"
  end

  return "within"
end

----------------------------------------------------------------------
-- Public API
----------------------------------------------------------------------

function HpcContractCards.load_spine(spine_path)
  return load_spine(spine_path or HpcContractCards.spine_path)
end

function HpcContractCards.load_contract(path)
  return load_json_file(path)
end

function HpcContractCards.check_contract(spine, contract, object_kind_override, tier_override)
  if type(spine) ~= "table" then
    return nil, "invalid spine data"
  end
  if type(contract) ~= "table" then
    return nil, "invalid contract data"
  end

  local object_kind = object_kind_override or contract.objectKind or contract.object_kind
  if type(object_kind) ~= "string" then
    return nil, "objectKind missing from contract and not provided"
  end

  local tier = tier_override or contract.tier
  if type(tier) ~= "string" then
    tier = nil
  end

  local schema_entry = find_schema_entry_for_object_kind(spine, object_kind)
  local contract_family = schema_entry and (schema_entry.contract_family or schema_entry.contractFamily)

  local inv_catalog = spine.invariants_catalog or spine.invariantsCatalog
  local met_catalog = spine.metrics_catalog or spine.metricsCatalog

  local inv_index = build_band_index(inv_catalog, contract_family)
  local met_index = build_band_index(met_catalog, contract_family)

  local invariant_bindings = contract.invariantBindings or {}
  local metric_targets = contract.metricTargets or {}

  local invariant_results = {}
  local metric_results = {}

  for key, value in pairs(invariant_bindings) do
    local bands = inv_index[key]
    local band = bands and pick_band(bands, tier) or nil
    local status = classify_value(value, band)
    table.insert(invariant_results, {
      key = key,
      value = value,
      min = band and band.min or nil,
      max = band and band.max or nil,
      status = status,
    })
  end

  for key, value in pairs(metric_targets) do
    local bands = met_index[key]
    local band = bands and pick_band(bands, tier) or nil

    if type(value) == "table" and #value == 2 then
      local low = value[1]
      local high = value[2]
      local status_low = classify_value(low, band)
      local status_high = classify_value(high, band)
      local combined
      if status_low == "outside" or status_high == "outside" then
        combined = "outside"
      elseif status_low == "edge" or status_high == "edge" then
        combined = "edge"
      else
        combined = "within"
      end
      table.insert(metric_results, {
        key = key,
        value = { low, high },
        min = band and band.min or nil,
        max = band and band.max or nil,
        status = combined,
      })
    else
      local status = classify_value(value, band)
      table.insert(metric_results, {
        key = key,
        value = value,
        min = band and band.min or nil,
        max = band and band.max or nil,
        status = status,
      })
    end
  end

  return {
    objectKind = object_kind,
    tier = tier,
    contractFamily = contract_family,
    invariants = invariant_results,
    metrics = metric_results,
  }, nil
end

function HpcContractCards.print_summary(summary)
  if type(summary) ~= "table" then
    io.write("no summary available\n")
    return
  end

  io.write(("Contract band check summary for objectKind=%s tier=%s\n")
    :format(tostring(summary.objectKind), tostring(summary.tier)))
  if summary.contractFamily then
    io.write(("  contractFamily: %s\n"):format(summary.contractFamily))
  end

  io.write("\nInvariants:\n")
  for _, item in ipairs(summary.invariants or {}) do
    io.write(("- %s: value=%s, band=[%s,%s], status=%s\n")
      :format(
        tostring(item.key),
        tostring(item.value),
        item.min ~= nil and tostring(item.min) or "-",
        item.max ~= nil and tostring(item.max) or "-",
        tostring(item.status)
      ))
  end

  io.write("\nMetrics:\n")
  for _, item in ipairs(summary.metrics or {}) do
    local val_str
    if type(item.value) == "table" and #item.value == 2 then
      val_str = ("[%s,%s]"):format(tostring(item.value[1]), tostring(item.value[2]))
    else
      val_str = tostring(item.value)
    end
    io.write(("- %s: value=%s, band=[%s,%s], status=%s\n")
      :format(
        tostring(item.key),
        val_str,
        item.min ~= nil and tostring(item.min) or "-",
        item.max ~= nil and tostring(item.max) or "-",
        tostring(item.status)
      ))
  end
end

function HpcContractCards.main(argv)
  argv = argv or {}

  local spine_path = HpcContractCards.spine_path
  local contract_path
  local object_kind
  local tier

  local i = 1
  while i <= #argv do
    local a = argv[i]
    if a == "--spine" then
      i = i + 1
      spine_path = argv[i]
    elseif a == "--kind" then
      i = i + 1
      object_kind = argv[i]
    elseif a == "--tier" then
      i = i + 1
      tier = argv[i]
    else
      contract_path = a
    end
    i = i + 1
  end

  if not contract_path then
    io.stderr:write("usage: lua hpc_contractcards.lua [--spine path] [--kind objectKind] [--tier tier] <contract.json>\n")
    return 1
  end

  local spine, err = HpcContractCards.load_spine(spine_path)
  if not spine then
    io.stderr:write(("error: %s\n"):format(err))
    return 1
  end

  local contract, err2 = HpcContractCards.load_contract(contract_path)
  if not contract then
    io.stderr:write(("error: %s\n"):format(err2))
    return 1
  end

  local summary, err3 = HpcContractCards.check_contract(spine, contract, object_kind, tier)
  if not summary then
    io.stderr:write(("error: %s\n"):format(err3))
    return 1
  end

  HpcContractCards.print_summary(summary)
  return 0
end

if debug.getinfo(1, "S").short_src == arg[0] then
  os.exit(HpcContractCards.main(arg))
end

return HpcContractCards
