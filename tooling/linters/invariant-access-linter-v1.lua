-- tooling/linters/invariant-access-linter-v1.lua
-- Purpose: Static analysis linter that enforces
--          "no behavior without QueryHistoryLayer → SetBehaviorFromInvariants"
--          by rejecting direct access to history/invariant tables.
-- Usage: Run via pre-commit hook or CI job in Codebase-of-Death.

local linter = {}

-- Configuration: modules that are allowed to access raw history/invariant tables
local TRUSTED_MODULES = {
  "h_invariants_core",      -- Core invariant sampling implementation
  "h_history_layer",        -- History query implementation
  "h_contract_loader",      -- Contract registry access
}

-- Patterns that indicate forbidden direct access
local FORBIDDEN_PATTERNS = {
  "history%[",              -- history[...]
  "region%.invariants",     -- region.invariants
  "tile%.invariant",        -- tile.invariant
  "raw_invariants",         -- raw_invariants variable
  "direct_invariant_fetch", -- direct_invariant_fetch function
}

-- Allowed accessor functions (whitelist)
local ALLOWED_ACCESSORS = {
  "H.Invariants.sample",
  "H.requireInvariants",
  "H.Contract.load",
  "H.Node.describe",
  "H.Selector.selectPattern",
  "H.Director.loadPersona",
}

function linter.check_file(filepath, content)
  local errors = {}
  local lines = {}
  for line in content:gmatch("[^\r\n]+") do
    table.insert(lines, line)
  end

  -- Skip trusted modules
  local moduleName = filepath:match("([^/]+)%.lua$")
  if moduleName and vim.tbl_contains(TRUSTED_MODULES, moduleName) then
    return errors
  end

  for lineno, line in ipairs(lines) do
    -- Check for forbidden patterns
    for _, pattern in ipairs(FORBIDDEN_PATTERNS) do
      if line:match(pattern) then
        table.insert(errors, {
          file = filepath,
          line = lineno,
          code = "DIRECT_INVARIANT_ACCESS",
          message = string.format("Direct access to invariant/history table detected: '%s'. Use H.requireInvariants or H.Invariants.sample instead.", pattern),
          suggestion = "Replace with H.requireInvariants(regionId, tileId, ctx) before behavior logic."
        })
      end
    end

    -- Check that invariant-dependent functions call requireInvariants
    if line:match("function.*choosePattern") or 
       line:match("function.*choose_next") or
       line:match("function.*applySafetyDecision") then
      -- Simple heuristic: look for requireInvariants call within next N lines
      local found_accessor = false
      for i = lineno, math.min(lineno + 20, #lines) do
        for _, accessor in ipairs(ALLOWED_ACCESSORS) do
          if lines[i]:match(accessor) then
            found_accessor = true
            break
          end
        end
        if found_accessor then break end
      end
      if not found_accessor then
        table.insert(errors, {
          file = filepath,
          line = lineno,
          code = "MISSING_INVARIANT_REQUIRE",
          message = string.format("Function '%s' may depend on invariants but does not call H.requireInvariants or approved accessor.", line:match("function%s+([^%(]+)")),
          suggestion = "Add local req = H.requireInvariants(regionId, tileId, ctx) at function start."
        })
      end
    end
  end

  return errors
end

function linter.run(root_dir)
  local all_errors = {}
  for filepath in vim.fn.globpath(root_dir, "**/*.lua", true, true) do
    local content = vim.fn.readfile(filepath)
    if content then
      local content_str = table.concat(content, "\n")
      local errors = linter.check_file(filepath, content_str)
      vim.list_extend(all_errors, errors)
    end
  end
  return all_errors
end

return linter
