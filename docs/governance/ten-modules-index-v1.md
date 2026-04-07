# Ten Modules Index – Horror.Place Governance Map

This document defines ten core governance modules for the Horror.Place constellation and points to their schemas, APIs, and telemetry surfaces. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/78c783b1-2fb1-4507-8cf5-4dd215599bdf/schema-aware-copilot-feature-o-G0anHDVHTb..muNF6D0Olw.md)

***

## 1. AI‑Chat Template & Redaction Module

**Purpose**  
Govern all AI‑Chat prompts and responses via schema‑validated templates and redaction profiles, preventing raw content leaks while allowing implied adult horror. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

**Core Schemas**  
- `schemas/runtime/ai-chat-template-contract.v1.json`  
- `schemas/runtime/redaction-profile.v1.json`  
- `schemas/runtime/ai-chat-template-routing.v1.json`  
- `schemas/tooling/ai-chat-template-eval-config.v1.json` [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

**Lua / API Surfaces**  
- `H.Templates.select(sessionContext, intent)`  
- `H.Templates.apply(templateId, modelOutput)`  
- `H.Templates.logRedaction(sessionId, redactionEvents)` [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/78c783b1-2fb1-4507-8cf5-4dd215599bdf/schema-aware-copilot-feature-o-G0anHDVHTb..muNF6D0Olw.md)

**Telemetry / CI**  
- `.github/workflows/ai-chat-template-leakcheck.yml`  
- `schemas/telemetry/ai-chat-template-metrics.v1.json` [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

***

## 2. Telemetry & Entertainment Metrics Module

**Purpose**  
Collect UEC, EMD, STCI, CDL, ARR and related signals, segment sessions, and enforce ethical guardrails on intensity and ambiguity. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/78c783b1-2fb1-4507-8cf5-4dd215599bdf/schema-aware-copilot-feature-o-G0anHDVHTb..muNF6D0Olw.md)

**Core Schemas**  
- `schemas/telemetry/entertainmentmetricsv1.json`  
- `schemas/telemetry/session-segmentation-v1.json`  
- `schemas/policy/metrics-ethics-guardrails-v1.json`  
- `schemas/telemetry/session-metrics-envelope-v1.json` [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

**Lua / API Surfaces**  
- `H.Metrics.recordEvent(sessionId, event)`  
- `H.Metrics.updateFromSignals(sessionId, dt, signals)`  
- `H.Metrics.currentBands(sessionId)`  
- `H.Metrics.segmentTracker(sessionId)`  
- `H.Metrics.checkGuardrails(sessionId)` [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/78c783b1-2fb1-4507-8cf5-4dd215599bdf/schema-aware-copilot-feature-o-G0anHDVHTb..muNF6D0Olw.md)

**Telemetry / CI**  
- Metrics range‑check workflow (reusable): `.github/workflows/metrics-rangecheck.reusable.yml`  
- NDJSON logs for session envelopes. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

***

## 3. History‑Aware Content Selector Module

**Purpose**  
Query CIC, MDI, AOS, RRM, FCF, SPR, RWF, DET, HVF, LSG, SHCI to choose motifs and explicitness levels that respect local history. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/78c783b1-2fb1-4507-8cf5-4dd215599bdf/schema-aware-copilot-feature-o-G0anHDVHTb..muNF6D0Olw.md)

**Core Schemas**  
- `schemas/selector/history-selector-rule-v1.json`  
- `schemas/selector/history-selector-pattern-v1.json`  
- `schemas/policy/history-selector-conflict-policy-v1.json`  
- `schemas/telemetry/history-selector-decision-event-v1.json`  
- `schemas/policy/history-selector-pattern-governance-v1.json` [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

**Lua / API Surfaces**  
- `H.Selector.choosePattern(regionId, tileId, userIntent, sessionContext)`  
- `H.Selector.logDecision(sessionId, selectorResult)` [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/78c783b1-2fb1-4507-8cf5-4dd215599bdf/schema-aware-copilot-feature-o-G0anHDVHTb..muNF6D0Olw.md)

**Telemetry / CI**  
- NDJSON decision logs (pattern usage, conflicts, resolutions).  
- CI checks verifying no prose in selector events. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

***

## 4. Age‑Gated Tier Router Module

**Purpose**  
Route users and AI‑Chat flows across tiers (public, vault, lab) based on age, consent, jurisdiction, and ALN/DeadLedger entitlements. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

**Core Schemas**  
- `schemas/policy/tier-routing-profile-v1.json`  
- `schemas/policy/user-tier-and-consent-v1.json`  
- `schemas/policy/aln-entitlement-bundle-ref-v1.json` [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

**Lua / API Surfaces**  
- `H.Tier.resolveUserTier(userContext)`  
- `H.Tier.routeRequest(userTier, intent, region)` [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/78c783b1-2fb1-4507-8cf5-4dd215599bdf/schema-aware-copilot-feature-o-G0anHDVHTb..muNF6D0Olw.md)

**Telemetry / CI**  
- Logs of tier decisions (opaque, ID‑only).  
- CI rules ensuring templates, selectors, and metrics respect tier caps. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

***

## 5. Intensity Budget & Scheduling Module

**Purpose**  
Assign and enforce an “intensity budget” over time (DET, STCI, CDL exposure) per session or campaign, coordinating with metrics and selector decisions. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/78c783b1-2fb1-4507-8cf5-4dd215599bdf/schema-aware-copilot-feature-o-G0anHDVHTb..muNF6D0Olw.md)

**Core Schemas**  
- `schemas/policy/intensity-budget-profile-v1.json`  
- `schemas/telemetry/intensity-budget-usage-v1.json` [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

**Lua / API Surfaces**  
- `H.Budget.initSession(sessionId, profileId)`  
- `H.Budget.requestSpike(sessionId, spikeParams)`  
- `H.Budget.canSpike(sessionId)`  
- `H.Budget.consumeBudget(sessionId, spikeDecision)` [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/78c783b1-2fb1-4507-8cf5-4dd215599bdf/schema-aware-copilot-feature-o-G0anHDVHTb..muNF6D0Olw.md)

**Telemetry / CI**  
- Budget usage logs joined with metrics envelopes.  
- CI checks that seeds/regions don’t exceed policy budgets by design. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

***

## 6. Content Boundary & Policy Engine Module

**Purpose**  
Centralize doctrinal content boundaries (no explicit gore, no raw trauma payloads, etc.) and enforce them across templates, selector, and AI‑Chat. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/78c783b1-2fb1-4507-8cf5-4dd215599bdf/schema-aware-copilot-feature-o-G0anHDVHTb..muNF6D0Olw.md)

**Core Schemas**  
- `schemas/policy/content-boundary-ruleset-v1.json`  
- `schemas/policy/forbidden-pattern-set-v1.json` [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

**Lua / API Surfaces**  
- `H.Policy.evaluateCandidate(output, context)`  
- `H.Policy.enforce(output, context)` [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/78c783b1-2fb1-4507-8cf5-4dd215599bdf/schema-aware-copilot-feature-o-G0anHDVHTb..muNF6D0Olw.md)

**Telemetry / CI**  
- Shared forbidden pattern libraries used by leak tests and runtime.  
- Logs of boundary decisions for analysis. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

***

## 7. Consent Profile & Comfort Module

**Purpose**  
Model individual player consent and comfort bands, aligning module behavior (selector, metrics, templates) to user‑specific limits. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

**Core Schemas**  
- `schemas/policy/consent-profile-v1.json`  
- `schemas/telemetry/consent-and-comfort-events-v1.json` [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

**Lua / API Surfaces**  
- `H.Consent.resolveProfile(userContext)`  
- `H.Consent.applyToIntent(userIntent, consentProfile)` [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/78c783b1-2fb1-4507-8cf5-4dd215599bdf/schema-aware-copilot-feature-o-G0anHDVHTb..muNF6D0Olw.md)

**Telemetry / CI**  
- Logs of comfort/opt‑out events, mapped to patterns and segments.  
- Guardrails preventing modules from violating declared comfort bands. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

***

## 8. Editor Copilot & ContractCard Module

**Purpose**  
Use contractCards as the shared numeric surface between designers, AI Copilot, CI, and DeadLedger (invariants targets + metrics bands). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/78c783b1-2fb1-4507-8cf5-4dd215599bdf/schema-aware-copilot-feature-o-G0anHDVHTb..muNF6D0Olw.md)

**Core Schemas**  
- `coreschemas/contractcardv1.json`  
- `coreschemas/signaturev1.json` (with embedded `contractCard` field) [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/78c783b1-2fb1-4507-8cf5-4dd215599bdf/schema-aware-copilot-feature-o-G0anHDVHTb..muNF6D0Olw.md)

**Lua / API Surfaces**  
- Editor‑side schema validation (tooling, not runtime).  
- `H.Contract.validateCard(card)`  
- `H.Contract.deriveSignature(card, policyProfile)` [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/78c783b1-2fb1-4507-8cf5-4dd215599bdf/schema-aware-copilot-feature-o-G0anHDVHTb..muNF6D0Olw.md)

**Telemetry / CI**  
- `schemaspinescanner` + Subset Rule checks between `contractCard` and `signature` envelopes.  
- Logs of prompt → contractCard → shipped values for research. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/78c783b1-2fb1-4507-8cf5-4dd215599bdf/schema-aware-copilot-feature-o-G0anHDVHTb..muNF6D0Olw.md)

***

## 9. DeadLedger / ALN Governance Module

**Purpose**  
Provide cryptographic provenance, entitlements, and capability envelopes that constrain what other modules can do. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

**Core Schemas**  
- (In DeadLedger repo, referenced here): `zkp-proof-v1.json`, `verifier-registry-v1.json`  
- In Constellation‑Contracts: `schemas/policy/capability-token-v1.json`, `schemas/policy/deadledgerref-v1.json` [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

**Lua / API Surfaces**  
- `H.ALN.checkCapability(agentId, capabilityId)`  
- `H.ALN.verifyDeadLedgerRef(deadledgerref)` [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/78c783b1-2fb1-4507-8cf5-4dd215599bdf/schema-aware-copilot-feature-o-G0anHDVHTb..muNF6D0Olw.md)

**Telemetry / CI**  
- Mandatory deadledgerref checks in CI for higher tiers.  
- Logs linking contractCards, signatures, and ledger entries. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

***

## 10. PCG Seed & Region Generator Module

**Purpose**  
Generate regions, seeds, and encounters as history‑bound artifacts that respect invariants, metrics, and all upstream policy modules. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/78c783b1-2fb1-4507-8cf5-4dd215599bdf/schema-aware-copilot-feature-o-G0anHDVHTb..muNF6D0Olw.md)

**Core Schemas**  
- `schemas/contracts/region-contract-card-v1.json`  
- `schemas/contracts/seed-contract-card-v1.json`  
- `schemas/contracts/event-contract-v1.json`  
- NDJSON registries: `registry-regions.ndjson`, `registry-seeds.ndjson`, `registry-events.ndjson` [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

**Lua / API Surfaces**  
- `H.PCG.generateSeed(contractCard, historyContext)`  
- `H.PCG.generateRegion(contractCard, historyContext)` [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/78c783b1-2fb1-4507-8cf5-4dd215599bdf/schema-aware-copilot-feature-o-G0anHDVHTb..muNF6D0Olw.md)

**Telemetry / CI**  
- CI enforcing schema validity, invariants/metrics ranges, and ethics guardrails for generated seeds/regions.  
- Telemetry linking seeds to session metrics and selector outcomes. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

***

## Shared Infrastructure

All ten modules rely on:

- **Schema Spine**  
  - `schemas/invariantsv1.json` (CIC, MDI, AOS, RRM, FCF, SPR, RWF, DET, HVF, LSG, SHCI)  
  - `schemas/entertainmentmetricsv1.json` (UEC, EMD, STCI, CDL, ARR) [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/78c783b1-2fb1-4507-8cf5-4dd215599bdf/schema-aware-copilot-feature-o-G0anHDVHTb..muNF6D0Olw.md)

- **Registries**  
  - NDJSON registries with `id`, `schemaref`, `tier`, `deadledgerref`.  

- **CI / Pre‑Commit Pack**  
  - Schema validation, registry linting, metrics range checks, leak‑tests, guardrail checks, and selector logging validation. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/0c0bd18c-03a9-4266-a38f-a653516c7e43/this-research-focuses-on-creat-bAyCRooEQbm7o.KNSJl3VA.md)

This index should sit at the top of HorrorPlace‑Constellation‑Contracts and be updated whenever a module gains or changes schemas/APIs, keeping AI‑Chat and engine agents aligned with the same governance map. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/78c783b1-2fb1-4507-8cf5-4dd215599bdf/schema-aware-copilot-feature-o-G0anHDVHTb..muNF6D0Olw.md)
