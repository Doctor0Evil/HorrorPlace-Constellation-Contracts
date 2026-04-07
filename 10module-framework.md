## Module 1 – Consent Profile Comfort

Schemas (`schemas/consent/`):

- `consent-state-machine.v1.json`  
- `consent-explicitness-caps.v1.json`  
- `consent-session-metrics.v1.json`  

Cross‑module / shared consent–explicitness policy (from Shared Infrastructure): [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/1da8d1e2-78d2-4933-8be4-c249627de9e7/this-research-focuses-on-advan-Gn_aMO_jQvqfg7wXixdLTw.md)

- `fields-zone-profile-v1.json` (under `schemas/policy/`)  
- `explicitness-ceiling-policy-v1.json` (under `schemas/policy/`)  
- `consent-wave-policy-v1.json` (under `schemas/policy/`)  
- `explicitness-session-envelope-v1.json` (under `schemas/telemetry/`)  
- `consent-wave-telemetry-v1.json` (under `schemas/telemetry/`)  
- `explicitness-runtime-state-v1.json` (under `schemas/runtime/`)  

Docs (`docs/policy/`, `docs/telemetry/`, `docs/runtime/`):

- `fields-zone-profile-explainer-v1.md`  
- `explicitness-ceiling-spec-v1.md`  
- `consent-wave-spec-v1.md`  
- `explicitness-and-consent-telemetry-design-v1.md`  
- `explicitness-lua-api-v1.md`  

***

## Module 2 – Input Pattern Banter Classifier

Schemas (`schemas/classifier/`): [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/1da8d1e2-78d2-4933-8be4-c249627de9e7/this-research-focuses-on-advan-Gn_aMO_jQvqfg7wXixdLTw.md)

- `input-label-taxonomy.v1.json`  
- `input-banter-policy.v1.json`  
- `banter-adaptation-policy.v1.json`  

Docs (`docs/classifier/`):

- `banter-invariant-integration-v1.md`  

***

## Module 3 – Horror Intensity Budget

Schemas (`schemas/budget/`): [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/1da8d1e2-78d2-4933-8be4-c249627de9e7/this-research-focuses-on-advan-Gn_aMO_jQvqfg7wXixdLTw.md)

- `horror-intensity-budget-policy.v1.json`  
- `horror-intensity-budget-metrics.v1.json`  
- `horror-intensity-overbudget-policy.v1.json`  

Docs (`docs/budget/`):

- `budget-lua-api-v1.md`  
- `budget-cross-module-v1.md`  

***

## Module 4 – Content Boundary Policy Engine

Schemas (`schemas/safety/`): [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/1da8d1e2-78d2-4933-8be4-c249627de9e7/this-research-focuses-on-advan-Gn_aMO_jQvqfg7wXixdLTw.md)

- `chat-safety-invariants-v1.json`  
- `content-boundary-priority-rules-v1.json`  
- `content-boundary-ruleset-v1.json`  

Docs (`docs/safety/`):

- `content-boundary-engine-wiring-v1.md`  
- `github-vault-governance-v1.md`  

***

## Module 5 – Age‑Gated Horror Tier Router

Schemas (`schemas/runtime/` and `schemas/security/`): [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/1da8d1e2-78d2-4933-8be4-c249627de9e7/this-research-focuses-on-advan-Gn_aMO_jQvqfg7wXixdLTw.md)

- `age-tier-routing-table.v1.json` (under `schemas/runtime/`)  
- `age-tier-router-policy.v1.json` (under `schemas/runtime/`)  
- `repo-access-capability.v1.json` (under `schemas/security/`)  
- `repo-access-proof-envelope.v1.json` (under `schemas/security/`)  
- `age-tier-routing-metrics.v1.json` (under `schemas/telemetry/`)  

***

## Module 6 – Telemetry Entertainment Metrics

Schemas (`schemas/telemetry/` and `schemas/policy/`): [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/1da8d1e2-78d2-4933-8be4-c249627de9e7/this-research-focuses-on-advan-Gn_aMO_jQvqfg7wXixdLTw.md)

- `entertainment-metrics.v1.json`  
- `session-segmentation-v1.json`  
- `session-metrics-envelope-v1.json`  
- `metrics-ethics-guardrails-v1.json` (under `schemas/policy/`)  

Docs (`docs/telemetry/`):

- `session-segmentation-design-v1.md`  
- `metrics-aggregation-design-v1.md`  
- `metrics-vs-ethics-rules-v1.md`  
- `telemetry-metrics-lua-api-v1.md`  

***

## Module 7 – History‑Aware Content Selector

Schemas (`schemas/selector/`, `schemas/policy/`, `schemas/telemetry/`): [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/1da8d1e2-78d2-4933-8be4-c249627de9e7/this-research-focuses-on-advan-Gn_aMO_jQvqfg7wXixdLTw.md)

- `history-selector-rule-v1.json`  
- `history-selector-pattern-v1.json`  
- `history-selector-conflict-policy-v1.json` (under `schemas/policy/`)  
- `history-selector-pattern-governance-v1.json` (under `schemas/policy/`)  
- `history-selector-decision-event-v1.json` (under `schemas/telemetry/`)  

Docs (`docs/selector/`, `docs/policy/`, `docs/telemetry/`, `docs/runtime/`):

- `history-selector-dsl-v1.md`  
- `history-selector-conflict-handling-v1.md`  
- `history-selector-logging-v1.md`  
- `history-selector-lua-api-v1.md`  
- `history-selector-telemetry-analysis-v1.md`  

***

## Module 8 – AI‑Chat Template Redaction

Schemas (`schemas/runtime/`, `schemas/tooling/`, `schemas/telemetry/`): [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/1da8d1e2-78d2-4933-8be4-c249627de9e7/this-research-focuses-on-advan-Gn_aMO_jQvqfg7wXixdLTw.md)

- `ai-chat-template-contract.v1.json` (under `schemas/runtime/`)  
- `redaction-profile.v1.json` (under `schemas/runtime/`)  
- `ai-chat-template-routing.v1.json` (under `schemas/runtime/`)  
- `ai-chat-template-eval-config.v1.json` (under `schemas/tooling/`)  
- `ai-chat-template-metrics.v1.json` (under `schemas/telemetry/`)  

Docs (`docs/runtime/`, `docs/tooling/`, `docs/telemetry/`):

- `ai-chat-template-lua-api-v1.md` (API surface for `H.Templates.*`)  
- `ai-chat-template-leakcheck.md` (paired with CI workflow)  

CI (in `HorrorPlace-Orchestrator`): [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/1da8d1e2-78d2-4933-8be4-c249627de9e7/this-research-focuses-on-advan-Gn_aMO_jQvqfg7wXixdLTw.md)

- `.github/workflows/ai-chat-template-leakcheck.yml`  

***

## Module 9 – DeadLedger / ALN Governance & Registry

Schemas (`schemas/registry/`, `schemas/governance/`): [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/1da8d1e2-78d2-4933-8be4-c249627de9e7/this-research-focuses-on-advan-Gn_aMO_jQvqfg7wXixdLTw.md)

- `descriptor-policy-v1.json` (under `schemas/registry/`)  
- `public-registry-policy.json` (under `schemas/registry/`)  
- `core-registry-entry-v1.json` (under `schemas/registry/`)  

- `promotion-proposal-v1.json` (under `schemas/governance/`)  
- `voting-rule-v1.json` (under `schemas/governance/`)  
- `build-flag-binding-v1.json` (under `schemas/governance/`)  

Capability / provenance (shared with Module 10): [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/1da8d1e2-78d2-4933-8be4-c249627de9e7/this-research-focuses-on-advan-Gn_aMO_jQvqfg7wXixdLTw.md)

- `capability-token-v1.json` (under `schemas/policy/`)  
- `deadledgerref-v1.json` (under `schemas/policy/`)  

Dead‑Ledger repo (external, but referenced):

- `promotion-lock-event-v1.json` (in `HorrorPlace-Dead-Ledger-Network/schemas/governance/`)  

Docs (`docs/governance/`): [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/1da8d1e2-78d2-4933-8be4-c249627de9e7/this-research-focuses-on-advan-Gn_aMO_jQvqfg7wXixdLTw.md)

- `ten-modules-index-v1.md`  
- `ten-module-governance-framework-v1.md`  
- `tier-aware-governance-flow-v1.md`  

***

## Module 10 – PCG Seed / Region Generator

Schemas (`schemas/pcg/`, `schemas/contracts/`, `schemas/telemetry/`, `schemas/policy/`): [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/70a41ea2-e93f-4334-a304-d9585fd52806/CHAT_DIRECTOR-v1-Implementation-Research-Module-Specifications.docx)

Core contracts:

- `pcg-seed-contract-v1.json` (under `schemas/pcg/`)  
- `pcg-seed-generation-policy-v1.json` (under `schemas/pcg/`)  
- `pcg-seed-validation-v1.json` (under `schemas/pcg/`)  
- `pcg-seed-telemetry-v1.json` (under `schemas/pcg/`)  

Region/seed contract cards:

- `region-contract-card-v1.json` (under `schemas/contracts/`)  
- `seed-contract-card-v1.json` (under `schemas/contracts/`)  
- `event-contract-v1.json` (under `schemas/contracts/`)  

Shared registry/policy (also listed under Module 9): [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/1da8d1e2-78d2-4933-8be4-c249627de9e7/this-research-focuses-on-advan-Gn_aMO_jQvqfg7wXixdLTw.md)

- `core-registry-entry-v1.json` (under `schemas/registry/`)  
- `capability-token-v1.json` (under `schemas/policy/`)  
- `deadledgerref-v1.json` (under `schemas/policy/`)  

Telemetry / logging:

- `pcg-seed-telemetry-v1.json` (as above, under `schemas/pcg/`)  

Docs (`docs/runtime/`, `docs/pcg/`):

- `pcg-lua-api-v1.md`  

External vault repo (`HorrorPlace-Atrocity-Seeds`):

- `registry-regions.ndjson`  
- `registry-seeds.ndjson`  
- `registry-events.ndjson`  

***

## Shared Spine & Infrastructure (used by all modules)

Schemas (`schemas/` root and shared folders): [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/70a41ea2-e93f-4334-a304-d9585fd52806/CHAT_DIRECTOR-v1-Implementation-Research-Module-Specifications.docx)

- `schema-spine-index-v1.json`  
- `invariants-spine.v1.json`  
- `entertainment-metrics-spine.v1.json`  

- `ai-authoring-request-v1.json` (under `schemas/runtime/`)  
- `ai-authoring-response-v1.json` (under `schemas/runtime/`)  

- `schema-spine-index-v1.json` (core spine index)  

Directory‑level layout (already agreed): [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/1da8d1e2-78d2-4933-8be4-c249627de9e7/this-research-focuses-on-advan-Gn_aMO_jQvqfg7wXixdLTw.md)

- `schemas/consent/`  
- `schemas/classifier/`  
- `schemas/budget/`  
- `schemas/safety/`  
- `schemas/runtime/`  
- `schemas/telemetry/`  
- `schemas/selector/`  
- `schemas/registry/`  
- `schemas/governance/`  
- `schemas/pcg/`  
- `schemas/security/`  
- `docs/`  
