-- hpc_contract_cards.lua
-- Engine-agnostic helper for loading, querying, and validating contract cards.
-- Expects the host environment to provide a `json.decode(str)` function.
-- Compatible with Lua 5.3+.

local M = {}

--- Validate basic contract card structure
-- @param contract table Parsed contract card
-- @return boolean valid, string|nil error_message
function M.validate_structure(contract)
  if type(contract) ~= "table" then return false, "Contract must be a table" end
  if not contract.id or not contract.schemaVersion then
    return false, "Missing required fields: id, schemaVersion"
  end
  if contract.tier ~= "public" and contract.tier ~= "vault" and contract.tier ~= "lab" then
    return false, "Invalid tier: " .. tostring(contract.tier)
  end
  return true, nil
end

--- Extract invariant bindings safely
-- @param contract table
-- @return table invariant_map
function M.get_invariants(contract)
  return contract.invariantBindings or contract.seedBundle and contract.seedBundle.invariantWeights or {}
end

--- Check tier gating requirements
-- @param contract table
-- @return boolean compliant, string|nil reason
function M.check_tier_gating(contract)
  if contract.tier == "public" then return true end
  if not contract.deadledgerref or contract.deadledgerref == "" then
    return false, "Vault/Lab contracts require deadledgerref"
  end
  return true
end

--- Resolve prismMeta linkage
-- @param contract table
-- @return table|nil linkage or nil
function M.get_linkage(contract)
  if contract.prismMeta and contract.prismMeta.linkage then
    return contract.prismMeta.linkage
  end
  return nil
end

return M
