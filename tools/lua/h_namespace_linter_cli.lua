-- tools/lua/h_namespace_linter_cli.lua
local linter = require("tools.lua.h_namespace_linter")

local repo_root = arg[1] or "."
local manifest_path = arg[2]

local ok = linter.run(repo_root, manifest_path)
if not ok then
  os.exit(1)
end
