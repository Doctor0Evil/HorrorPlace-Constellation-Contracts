-- tooling/lua/hpc_manifest_client.lua

local json = require("dkjson")

local ManifestClient = {}
ManifestClient.__index = ManifestClient

local function read_file(path)
    local f, err = io.open(path, "r")
    if not f then
        return nil, err
    end
    local contents = f:read("*a")
    f:close()
    return contents
end

local function is_repo_manifest(name)
    return name:match("^repo%-manifest%.hpc%..+%.json$")
end

function ManifestClient.load(dir)
    local manifests_by_repo = {}

    local p = io.popen("ls " .. dir)
    if not p then
        return nil, "failed to list manifest directory"
    end
    for filename in p:lines() do
        if is_repo_manifest(filename) then
            local path = dir .. "/" .. filename
            local text, err = read_file(path)
            if text then
                local manifest, _, decode_err = json.decode(text)
                if not decode_err and manifest.repoName then
                    manifests_by_repo[manifest.repoName] = manifest
                end
            end
        end
    end
    p:close()

    local self = {
        manifests_by_repo = manifests_by_repo,
    }
    return setmetatable(self, ManifestClient)
end

function ManifestClient:get(repo_name)
    return self.manifests_by_repo[repo_name]
end

function ManifestClient:list_repos()
    local out = {}
    for name, m in pairs(self.manifests_by_repo) do
        table.insert(out, { repoName = name, tier = m.tier })
    end
    table.sort(out, function(a, b) return a.repoName < b.repoName end)
    return out
end

function ManifestClient:explain_route(object_kind, tier)
    local best = nil

    for repo_name, m in pairs(self.manifests_by_repo) do
        local allowed = false
        for _, k in ipairs(m.allowedObjectKinds or {}) do
            if k == object_kind then
                allowed = true
                break
            end
        end
        if not allowed then
            goto continue
        end

        if m.tier ~= tier then
            goto continue
        end

        local default_path = nil
        local defaults = m.defaultPaths or {}
        default_path = defaults[object_kind]

        best = {
            objectKind = object_kind,
            requestedTier = tier,
            repoName = repo_name,
            resolvedTier = m.tier,
            schemaWhitelist = m.schemaWhitelist or {},
            defaultPath = default_path,
            policies = m.policies or {},
            authoringHints = m.authoringHints or {},
        }
        break

        ::continue::
    end

    return best
end

return ManifestClient
