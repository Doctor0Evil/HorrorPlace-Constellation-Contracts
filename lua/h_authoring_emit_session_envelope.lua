-- lua/h_authoring_emit_session_envelope.lua
-- GitHub-safe helper for emitting ai-authoring-session-envelope-v1 NDJSON rows.
-- Depends on: H.Consent, H.TierRouter, H.Budget, H.Metrics (optional), H.Id (helper).

local H = require("hpcruntime")

local Authoring = {}

local function now_iso8601()
  -- Engine should provide a real clock; this is a placeholder hook.
  return H.Time.now_iso8601()
end

local function gen_id(prefix)
  return prefix .. "-" .. H.Id.nano()
end

-- params:
-- {
--   sessionId        = "sess-...",
--   authoringToolId  = "tool-...",
--   actorKind        = "humanDesigner|aiChat|batchTool|ciJob",
--   actorId          = "act-...",
--   targetRepo       = "...",
--   targetPath       = "schemas/...",
--   schemaRefTarget  = "consent-state-machine.v1",
--   objectKindTarget = "consentStateMachine",
--   intentSummary    = "short description",
--   invariantsTouched = { "CIC", "DET" },
--   metricsTouched    = { "UEC", "ARR" },
--   deadLedgerRefs    = { "DLN.bundle..." },
--   registryRefs      = { "reg-..." },
--   filesPlanned      = { { path=..., schemaRef=..., objectKind=..., tier=..., mode="create" }, ... },
--   filesWritten      = { { path=..., schemaRef=..., objectKind=..., tier=..., mode="create", status="success" }, ... },
--   ciPassed          = true,
--   status            = "completed|abortedByUser|abortedByPolicy|ciFailed|noChanges",
--   warnings          = { "..." },
--   errors            = { "..." },
--   prismMeta         = { ... } -- standard prism meta block
-- }
function Authoring.emitSessionEnvelope(params)
  local sessionId = params.sessionId
  if not sessionId then
    return nil, { code = "authoring.missingSessionId", message = "sessionId is required" }
  end

  -- 1. Consent state (consentTier, policyTier, routerStateId, fieldId / explicitness policy).
  local consentRes = H.Consent.currentState(sessionId)
  if not consentRes or not consentRes.ok then
    return nil, { code = "authoring.consentError", message = "Failed to resolve consent state", details = consentRes and consentRes.error or nil }
  end
  local consent = consentRes.data

  -- 2. Tier router state (effective tier / field profile).
  local routerRes = H.TierRouter.resolve({
    sessionId       = sessionId,
    tier            = consent.policyTier,
    consentStateRef = consent.consentProfileId,
    repoAccess      = nil
  })
  if not routerRes or not routerRes.ok then
    return nil, { code = "authoring.routerError", message = "Failed to resolve router state", details = routerRes and routerRes.error or nil }
  end
  local routerState = routerRes.data

  -- 3. Budget snapshot (optional, but useful context).
  local budgetSnapRes = H.Budget.snapshot(sessionId)
  local budgetSnap = budgetSnapRes and budgetSnapRes.data or nil

  -- 4. Build policyContext using explicitness / fields APIs if present.
  local fieldId = routerState.fieldId or "Field-A"
  local expceilId = routerState.explicitnessPolicyId or "expceil-default"

  local policyContext = {
    consentTier          = consent.consentTier,
    policyTier           = consent.policyTier,
    routerStateId        = consent.routerStateId,
    fieldId              = fieldId,
    explicitnessCeilingId = expceilId
  }

  -- 5. Assemble envelope object.
  local envelope = {
    id                 = gen_id("authenv"),
    schemaRef          = "ai-authoring-session-envelope.v1",
    objectKind         = "aiAuthoringSessionEnvelope",
    version            = "v1.0.0",
    timestamp          = now_iso8601(),
    authoringToolId    = params.authoringToolId or "tool-unknown",
    sessionId          = sessionId,
    actorKind          = params.actorKind or "aiChat",
    actorId            = params.actorId or "act-unknown",
    targetRepo         = params.targetRepo,
    targetPath         = params.targetPath,
    tier               = routerState.tier or "Tier1Public",
    schemaRefTarget    = params.schemaRefTarget,
    objectKindTarget   = params.objectKindTarget,
    intentSummary      = params.intentSummary or "",
    invariantsTouched  = params.invariantsTouched or {},
    metricsTouched     = params.metricsTouched or {},
    deadLedgerRefs     = params.deadLedgerRefs or {},
    registryRefs       = params.registryRefs or {},
    filesPlanned       = params.filesPlanned or {},
    filesWritten       = params.filesWritten or {},
    policyContext      = policyContext,
    result             = {
      status     = params.status or "completed",
      ciPassed   = params.ciPassed == true,
      filesCount = #(params.filesWritten or {}),
      warnings   = params.warnings or {},
      errors     = params.errors or {}
    },
    prismMeta          = params.prismMeta or {}
  }

  -- 6. Optional: attach a shallow budget summary into prismMeta for analysis.
  if budgetSnap then
    envelope.prismMeta.budgetSnapshot = {
      band          = budgetSnap.band,
      budgetMax     = budgetSnap.budgetMax,
      budgetRemaining = budgetSnap.budgetRemaining
    }
  end

  -- 7. Emit as NDJSON via a shared telemetry sink.
  local ok, err = H.Telemetry.emitNdjson("ai-authoring-session-envelope-v1", envelope)
  if not ok then
    return nil, { code = "authoring.emitFailed", message = "Failed to emit authoring session envelope", details = err }
  end

  return envelope, nil
end

return Authoring
