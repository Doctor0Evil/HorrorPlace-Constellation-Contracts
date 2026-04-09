-- wandering_guard.lua
local H = require("horror_invariants")

local Guard = {}

function Guard.propose_path(ai_request)
  local res = H.validate_next_step(ai_request)
  if not res.ok then
    return { status = "rejected", reason = res.error_code }
  end

  return {
    status      = "accepted",
    targetRepo  = ai_request.targetRepo,
    targetPath  = res.resolution.targetPath,
    schemaref   = res.resolution.schemaref,
  }
end

return Guard
