-- hpc_registry_client.lua
-- Engine-agnostic helper for loading NDJSON registries and resolving entries by ID.
-- Expects host environment to provide `io.open`, `string.gmatch`, and `json.decode`.
-- Compatible with Lua 5.3+.

local M = {}

--- Load and parse an NDJSON registry file into an indexed table
-- @param filepath string Path to .ndjson or .ndjson.example file
-- @return table indexed_registry { [id] = entry_data }, table raw_entries
function M.load_registry(filepath)
  local indexed = {}
  local raw = {}
  
  local file, err = io.open(filepath, "r")
  if not file then
    print("[WARN] hpc_registry: Could not open " .. tostring(filepath) .. ": " .. tostring(err))
    return indexed, raw
  end

  for line in file:lines() do
    line = line:match("^%s*(.-)%s*$") -- trim
    if #line > 0 then
      local ok, data = pcall(json.decode, line)
      if ok and type(data) == "table" and data.id then
        indexed[data.id] = data
        table.insert(raw, data)
      end
    end
  end
  file:close()
  return indexed, raw
end

--- Resolve a registry ID to its data
-- @param registry_index table Output from load_registry
-- @param target_id string ID to resolve
-- @return table|nil entry_data
function M.resolve_id(registry_index, target_id)
  return registry_index[target_id] or nil
end

--- Filter registry entries by tier or tag
-- @param raw_entries table Array of parsed entries
-- @param filter_key string Field to filter (e.g., "tier", "tags")
-- @param filter_val string|table Value to match
-- @return table filtered_entries
function M.filter_entries(raw_entries, filter_key, filter_val)
  local out = {}
  for _, entry in ipairs(raw_entries) do
    local val = entry[filter_key]
    if type(filter_val) == "table" and type(val) == "table" then
      -- Intersection check for arrays/tags
      for _, v in ipairs(filter_val) do
        for _, e in ipairs(val) do
          if v == e then table.insert(out, entry); goto continue end
        end
      end
    elseif val == filter_val then
      table.insert(out, entry)
    end
    ::continue::
  end
  return out
end

return M
