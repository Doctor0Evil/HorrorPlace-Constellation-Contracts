-- tooling/lua/hpc_contractcards.lua
--
-- Usage (from repo root):
--   lua tooling/lua/hpc_contractcards.lua \
--       --root . \
--       --kind moodContract \
--       --tier Tier1Public \
--       contracts/mood/example_mood.json
--
-- Prints a band-check summary comparing invariantBindings/metricTargets
-- against the spine-defined ranges for the given objectKind + tier.

local json = require("dkjson")
local SpineClient = require("hpc_spine_client")

local M = {}

local function read_file(path)
    local f, err = io.open(path, "r")
    if not f then
        return nil, ("failed to open file " .. path .. ": " .. tostring(err))
    end
    local contents = f:read("*a")
    f:close()
    return contents
end

local function parse_args(argv)
    local opts = {
        root = ".",
        kind = "moodContract",
        tier = "Tier1Public",
        path = nil,
    }

    local i = 1
    while i <= #argv do
        local a = argv[i]
        if a == "--root" then
            i = i + 1
            opts.root = argv[i]
        elseif a == "--kind" then
            i = i + 1
            opts.kind = argv[i]
        elseif a == "--tier" then
            i = i + 1
            opts.tier = argv[i]
        else
            opts.path = a
        end
        i = i + 1
    end

    if not opts.path then
        return nil, "missing contract JSON path"
    end
    return opts
end

local function extract_invariants(doc)
    local out = {}
    local bindings = doc.invariantBindings
    if type(bindings) ~= "table" then
        return out
    end
    for name, spec in pairs(bindings) do
        if type(spec) == "table" and spec.value ~= nil then
            out[name] = tonumber(spec.value)
        end
    end
    return out
end

local function extract_metrics(doc)
    local out = {}
    local targets = doc.metricTargets
    if type(targets) ~= "table" then
        return out
    end
    for name, spec in pairs(targets) do
        if type(spec) == "table" and spec.target ~= nil then
            out[name] = tonumber(spec.target)
        end
    end
    return out
end

local function band_for_invariant(spine, object_kind, tier, abbr)
    local inv = spine:describe_invariant(abbr)
    if not inv then
        return nil
    end
    -- spine may carry tierOverrides; this helper just returns canonical range
    local min = inv.min or (inv.range and inv.range[1])
    local max = inv.max or (inv.range and inv.range[2])
    return min, max
end

local function band_for_metric(spine, object_kind, tier, abbr)
    local metric = spine:describe_metric(abbr)
    if not metric then
        return nil
    end
    local band = metric.targetBand or metric.targetband
    if type(band) == "table" then
        return band[1], band[2]
    end
    return nil
end

local function classify(value, min, max)
    if value == nil or min == nil or max == nil then
        return "unknown"
    end
    if value < min then
        return "below"
    elseif value > max then
        return "above"
    else
        return "within"
    end
end

local function summarize_bands(spine, opts, doc)
    local invariants = extract_invariants(doc)
    local metrics = extract_metrics(doc)

    local report = {
        objectKind = opts.kind,
        tier = opts.tier,
        invariants = {},
        metrics = {},
    }

    for name, value in pairs(invariants) do
        local min, max = band_for_invariant(spine, opts.kind, opts.tier, name)
        local status = classify(value, min, max)
        table.insert(report.invariants, {
            name = name,
            value = value,
            min = min,
            max = max,
            status = status,
        })
    end

    for name, value in pairs(metrics) do
        local min, max = band_for_metric(spine, opts.kind, opts.tier, name)
        local status = classify(value, min, max)
        table.insert(report.metrics, {
            name = name,
            value = value,
            min = min,
            max = max,
            status = status,
        })
    end

    table.sort(report.invariants, function(a, b) return a.name < b.name end)
    table.sort(report.metrics, function(a, b) return a.name < b.name end)

    return report
end

function M.main(argv)
    local opts, err = parse_args(argv)
    if not opts then
        io.stderr:write("error: " .. err .. "\n")
        io.stderr:write("usage: lua hpc_contractcards.lua --root <path> --kind <objectKind> --tier <tier> <contract.json>\n")
        return 1
    end

    local spine, spine_err = SpineClient.load(opts.root)
    if not spine then
        io.stderr:write("error loading spine: " .. tostring(spine_err) .. "\n")
        return 1
    end

    local text, read_err = read_file(opts.path)
    if not text then
        io.stderr:write("error reading contract: " .. tostring(read_err) .. "\n")
        return 1
    end

    local doc, _, decode_err = json.decode(text)
    if decode_err then
        io.stderr:write("error decoding JSON: " .. tostring(decode_err) .. "\n")
        return 1
    end

    local report = summarize_bands(spine, opts, doc)
    local out = json.encode(report, { indent = 2 })
    print(out)
    return 0
end

if debug.getinfo(1, "S").short_src == arg[0] then
    os.exit(M.main({ select(1, table.unpack(arg, 1, #arg)) }))
end

return M
