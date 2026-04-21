-- tools/lua/h_namespace_linter.lua
-- Simple linter that checks H.* namespace usage
-- against allowedHNamespaces from the repo manifest.

local json = require("dkjson")  -- or any JSON lib available in your toolchain
local lfs  = require("lfs")

local M = {}

local function read_file(path)
  local f, err = io.open(path, "r")
  if not f then
    return nil, ("cannot open file '%s': %s"):format(path, err)
  end
  local content = f:read("*a")
  f:close()
  return content
end

local function load_manifest(manifest_path)
  local raw, err = read_file(manifest_path)
  if not raw then
    return nil, err
  end
  local manifest, pos, jerr = json.decode(raw, 1, nil)
  if not manifest then
    return nil, ("invalid JSON in manifest '%s': %s at %d"):format(
      manifest_path,
      tostring(jerr),
      tonumber(pos or 0)
    )
  end
  return manifest
end

local function build_allowed_map(manifest)
  local allowed = {}
  local list = manifest.allowedHNamespaces or {}
  for _, ns in ipairs(list) do
    allowed[ns] = true
  end
  return allowed
end

local function is_lua_file(path)
  return path:match("%.lua$")
end

local function list_lua_files(root)
  local files = {}
  local function walk(dir)
    for entry in lfs.dir(dir) do
      if entry ~= "." and entry ~= ".." then
        local full = dir .. "/" .. entry
        local attr = lfs.attributes(full)
        if attr and attr.mode == "directory" then
          walk(full)
        elseif attr and attr.mode == "file" and is_lua_file(full) then
          table.insert(files, full)
        end
      end
    end
  end
  walk(root)
  return files
end

local function check_file(path, allowed)
  local content, err = read_file(path)
  if not content then
    return { { file = path, message = err } }
  end

  local violations = {}

  -- Simple pattern to catch H.<Name> identifiers:
  -- matches e.g. H.Selector, H.Node, H.Run, H.Audio, etc.
  for ns in content:gmatch("H%.([A-Za-z_][A-Za-z0-9_]*)") do
    local full = "H." .. ns
    if not allowed[full] then
      table.insert(violations, {
        file = path,
        namespace = full,
        message = ("Disallowed H namespace '%s' in %s"):format(full, path)
      })
    end
  end

  return violations
end

function M.run(repo_root, manifest_path)
  manifest_path = manifest_path or (repo_root .. "/repo-manifest.hpc.codebase-of-death.v1.json")

  local manifest, err = load_manifest(manifest_path)
  if not manifest then
    io.stderr:write("[HNamespaceLinter] ", err, "\n")
    return false
  end

  local allowed = build_allowed_map(manifest)
  local lua_files = list_lua_files(repo_root)

  local any_violation = false

  for _, path in ipairs(lua_files) do
    local violations = check_file(path, allowed)
    for _, v in ipairs(violations) do
      any_violation = true
      io.stderr:write(
        "[HNamespaceLinter] ",
        v.file,
        ": ",
        v.message,
        "\n"
      )
    end
  end

  if any_violation then
    return false
  end
  return true
end

return M
