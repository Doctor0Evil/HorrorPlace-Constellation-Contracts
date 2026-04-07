-- horror_modules.lua
local H = {}

H.Module4 = {}

function H.Module4.recordGovernanceEvent(sessionContext, params)
    -- validate minimal keys
    if not sessionContext or not sessionContext.sessionId then
        return { ok = false, data = nil, error = "missing_session_id" }
    end

    -- module-specific logic (simplified)
    local event = {
        moduleId = "4",
        version = "1.0.0",
        sessionId = sessionContext.sessionId,
        regionId = sessionContext.regionId,
        seedId = sessionContext.seedId,
        descriptorId = sessionContext.descriptorId,
        consentTier = sessionContext.consentTier,
        routerStateId = params.routerStateId,
        selectorPatternId = params.selectorPatternId,
        sessionMetricsEnvelopeId = sessionContext.sessionMetricsEnvelopeId,
        data = {
            eventType = params.eventType,
            policyId = params.policyId,
            phase = params.phase,
            rwf = params.rwf,
        }
    }

    -- telemetry hook (Module 2)
    Telemetry.logModuleEvent("4", event)

    return { ok = true, data = event, error = nil }
end

return H
