-- hpcregistryclient.lua
--
-- Lightweight NDJSON registry client for Lua environments.
--
-- Responsibilities:
--   - Load registry NDJSON files into memory.
--   - Index entries by id.
--   - Provide lookup helpers (resolve_id, filter_by_tier, filter_by_tag).
--   - Provide simple cross-registry reference validation helpers, so
--     Lua-based tools can confirm that IDs referenced by one registry
--     exist in another (e.g., events -> regions / personas).
--
-- Dependencies:
--   - json: decode function for JSON objects per NDJSON line.

local HpcRegistryClient = {}

----------------------------------------------------------------------
-- Internal helpers
----------------------------------------------------------------------

local function read_ndjson(path)
  local f, err = io.open(path, "r")
  if not f then
    return nil, ("unable to open NDJSON file '%s': %s"):format(path, tostring(err))
  end

  local records = {}
  local line_no = 0

  for line in f:lines() do
    line_no = line_no + 1
    local trimmed = line:match("^%s*(.-)%s*$")
    if trimmed ~= "" then
      local ok, obj = pcall(function()
        return json.decode(trimmed)
      end)
      if not ok then
        f:close()
        return nil, ("failed to decode JSON on line %d of '%s': %s"):format(
          line_no,
          path,
          tostring(obj)
        )
      end
      table.insert(records, obj)
    end
  end

  f:close()
  return records, nil
end

local function index_by_id(records)
  local index = {}
  for _, rec in ipairs(records) do
    local id = rec.id
    if type(id) == "string" then
      index[id] = rec
    end
  end
  return index
end

local function has_tag(rec, tag)
  if type(rec) ~= "table" then
    return false
  end
  local tags = rec.tags or rec.Tags
  if type(tags) ~= "table" then
    return false
  end
  for _, t in ipairs(tags) do
    if t == tag then
      return true
    end
  end
  return false
end

----------------------------------------------------------------------
-- Registry handle
----------------------------------------------------------------------

local Registry = {}
Registry.__index = Registry

function Registry:resolve_id(id)
  return self.index[id]
end

function Registry:filter_by_tier(tier)
  local out = {}
  for _, rec in pairs(self.index) do
    if rec.tier == tier then
      table.insert(out, rec)
    end
  end
  return out
end

function Registry:filter_by_tag(tag)
  local out = {}
  for _, rec in pairs(self.index) do
    if has_tag(rec, tag) then
      table.insert(out, rec)
    end
  end
  return out
end

function Registry:all()
  local out = {}
  for _, rec in pairs(self.index) do
    table.insert(out, rec)
  end
  return out
end

----------------------------------------------------------------------
-- Public API: loading registries
----------------------------------------------------------------------

function HpcRegistryClient.load_registry(path)
  local records, err = read_ndjson(path)
  if not records then
    return nil, err
  end

  local idx = index_by_id(records)

  local handle = {
    path = path,
    records = records,
    index = idx,
  }
  setmetatable(handle, Registry)
  return handle, nil
end

function HpcRegistryClient.resolve_id(registry, id)
  if not registry or not registry.index then
    return nil
  end
  return registry:resolve_id(id)
end

function HpcRegistryClient.filter_by_tier(registry, tier)
  if not registry or not registry.index then
    return {}
  end
  return registry:filter_by_tier(tier)
end

function HpcRegistryClient.filter_by_tag(registry, tag)
  if not registry or not registry.index then
    return {}
  end
  return registry:filter_by_tag(tag)
end

----------------------------------------------------------------------
-- Cross-registry reference validation
----------------------------------------------------------------------

local function collect_references(rec)
  local refs = {}

  local fields_scalar = {
    "regionId",
    "personaId",
    "styleId",
    "seedId",
    "policyId",
  }

  for _, field in ipairs(fields_scalar) do
    local v = rec[field]
    if type(v) == "string" then
      table.insert(refs, { field = field, id = v })
    end
  end

  local list_fields = {
    "referencedIds",
    "relatedIds",
  }

  for _, field in ipairs(list_fields) do
    local arr = rec[field]
    if type(arr) == "table" then
      for _, v in ipairs(arr) do
        if type(v) == "string" then
          table.insert(refs, { field = field, id = v })
        end
      end
    end
  end

  return refs
end

function HpcRegistryClient.validate_references(source_registry, target_registry, opts)
  opts = opts or {}
  local source_name = opts.source_name or source_registry.path or "source"
  local target_name = opts.target_name or target_registry.path or "target"

  local diagnostics = {}

  if not source_registry or not source_registry.index then
    table.insert(diagnostics, {
      severity = "error",
      code = "INVALID_SOURCE_REGISTRY",
      message = "source_registry is missing or not loaded via HpcRegistryClient.load_registry",
      context = { source_name = source_name },
    })
    return diagnostics
  end

  if not target_registry or not target_registry.index then
    table.insert(diagnostics, {
      severity = "error",
      code = "INVALID_TARGET_REGISTRY",
      message = "target_registry is missing or not loaded via HpcRegistryClient.load_registry",
      context = { target_name = target_name },
    })
    return diagnostics
  end

  for _, rec in ipairs(source_registry.records) do
    local rec_id = rec.id or "(no-id)"
    local refs = collect_references(rec)

    for _, ref in ipairs(refs) do
      local target = target_registry.index[ref.id]
      if not target then
        table.insert(diagnostics, {
          severity = "error",
          code = "MISSING_REFERENCE",
          message = ("Record '%s' in %s references '%s' in field '%s' but it does not exist in %s")
            :format(tostring(rec_id), source_name, tostring(ref.id), ref.field, target_name),
          context = {
            source_id = rec_id,
            source_field = ref.field,
            referenced_id = ref.id,
            source_registry = source_name,
            target_registry = target_name,
          },
        })
      end
    end
  end

  return diagnostics
end

function HpcRegistryClient.print_diagnostics(diags)
  for _, d in ipairs(diags or {}) do
    local msg = ("[%s] %s: %s\n"):format(
      string.upper(d.severity or "info"),
      d.code or "UNKNOWN",
      d.message or ""
    )
    io.write(msg)
  end
end

return HpcRegistryClient
