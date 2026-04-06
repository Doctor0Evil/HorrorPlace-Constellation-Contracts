-- tooling/lua/hpc_discovery.lua

local json = require("dkjson")
local SpineClient = require("hpc_spine_client")
local ManifestClient = require("hpc_manifest_client")

local Discovery = {}
Discovery.__index = Discovery

function Discovery.load(root)
    local spine, spine_err = SpineClient.load(root)
    if not spine then
        return nil, spine_err
    end
    local manifests, man_err = ManifestClient.load(root .. "/manifests")
    if not manifests then
        return nil, man_err
    end
    local self = {
        spine = spine,
        manifests = manifests,
    }
    return setmetatable(self, Discovery)
end

function Discovery:capability_catalog()
    local invariants = self.spine:list_invariants()
    local metrics = self.spine:list_metrics()
    local repos = self.manifests:list_repos()

    return {
        invariants = invariants,
        metrics = metrics,
        repos = repos,
    }
end

function Discovery:route_profile(object_kind, tier)
    local route = self.manifests:explain_route(object_kind, tier)
    if not route then
        return nil
    end

    return {
        route = route,
        defaults = self.spine:default_bands(object_kind, tier),
    }
end

function Discovery:print_catalog(pretty)
    local cat = self:capability_catalog()
    local text = json.encode(cat, { indent = pretty and 2 or nil })
    print(text)
end

return Discovery
