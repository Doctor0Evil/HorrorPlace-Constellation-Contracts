-- tooling/lua/hpc_spine_client.lua

local json = require("dkjson")  -- or your preferred JSON lib

local SpineClient = {}
SpineClient.__index = SpineClient

local function read_file(path)
    local f, err = io.open(path, "r")
    if not f then
        return nil, ("failed to open file: " .. tostring(err))
    end
    local contents = f:read("*a")
    f:close()
    return contents
end

function SpineClient.load(root)
    -- root is typically the repo root, e.g. ".." from tooling scripts
    local base = root .. "/schemas/core/"
    local spine_text, err1 = read_file(base .. "schema-spine-index-v1.json")
    if not spine_text then
        return nil, err1
    end
    local invariants_text, err2 = read_file(base .. "invariants-spine.v1.json")
    if not invariants_text then
        return nil, err2
    end
    local entertainment_text, err3 = read_file(base .. "entertainment-metrics-spine.v1.json")
    if not entertainment_text then
        return nil, err3
    end

    local spine, _, spine_err = json.decode(spine_text)
    if spine_err then
        return nil, ("failed to decode schema spine: " .. tostring(spine_err))
    end
    local invariants, _, inv_err = json.decode(invariants_text)
    if inv_err then
        return nil, ("failed to decode invariants spine: " .. tostring(inv_err))
    end
    local entertainment, _, ent_err = json.decode(entertainment_text)
    if ent_err then
        return nil, ("failed to decode entertainment spine: " .. tostring(ent_err))
    end

    local self = {
        schema_spine = spine,
        invariants_spine = invariants,
        entertainment_spine = entertainment,
        invariants_by_abbr = {},
        metrics_by_abbr = {},
    }

    for _, inv in ipairs(invariants.invariants or {}) do
        if inv.abbreviation then
            self.invariants_by_abbr[inv.abbreviation] = inv
        end
    end
    for _, metric in ipairs(entertainment.metrics or {}) do
        if metric.abbreviation then
            self.metrics_by_abbr[metric.abbreviation] = metric
        end
    end

    return setmetatable(self, SpineClient)
end

function SpineClient:describe_invariant(abbr)
    return self.invariants_by_abbr[abbr]
end

function SpineClient:describe_metric(abbr)
    return self.metrics_by_abbr[abbr]
end

function SpineClient:list_invariants()
    local out = {}
    for abbr, inv in pairs(self.invariants_by_abbr) do
        table.insert(out, { abbr = abbr, name = inv.name, description = inv.description })
    end
    table.sort(out, function(a, b) return a.abbr < b.abbr end)
    return out
end

function SpineClient:list_metrics()
    local out = {}
    for abbr, metric in pairs(self.metrics_by_abbr) do
        table.insert(out, { abbr = abbr, name = metric.name, description = metric.description })
    end
    table.sort(out, function(a, b) return a.abbr < b.abbr end)
    return out
end

function SpineClient:default_bands(object_kind, tier)
    -- Reads safeDefaults object if present; AI can call this before generating numeric bands.
    local defaults = self.schema_spine.safeDefaults or {}
    local key = object_kind .. "@" .. tier
    return defaults[key]
end

return SpineClient
