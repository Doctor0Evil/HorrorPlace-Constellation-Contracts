---
invariants_used: [CIC, MDI, AOS, RRM, FCF, SPR, RWF, DET, HVF, LSG, SHCI, UEC, EMD, STCI, CDL, ARR, SVI, HVI, SHI, GXI, NSI]
metrics_used: [UEC, EMD, STCI, CDL, ARR]
tiers: [standard, mature, research]
deadledger_surface: [zkpproof_schema, verifiers_registry]
---

# Horror.Place — Ten-Module Governance Framework  
Canonical Module Cards, Research Objects, Gap Analysis & Cross-Module Alignment

**Target-repo:** HorrorPlace-Constellation-Contracts  
**Path:** docs/governance/ten-module-governance-framework-v1.md  
**Version:** 1.0 — April 2026  
**Visibility:** Tier 1 (GitHub-safe, no raw horror content)  
**Author:** Doctor0Evil  
**Date:** 6 April 2026  

---

## 1. Executive Overview

Horror.Place is a non-profit, horror-content AI-Chat platform built on a VM-constellation architecture. The platform operates under a ten-module governance stack designed to enable high-quality immersive horror experiences while maintaining safety guarantees, regulatory compliance, and auditable decision-making. This document is the canonical reference for all ten governance modules, their schemas, APIs, research objects, cross-dependencies, and implementation roadmap.

### 1.1 Architectural Principles

The platform is built on four primary principles.

**Contract-first doctrine.** JSON Schemas define data shapes; registries and domain-specific languages (DSLs) encode behaviors; Lua and engine code consume contracts. No module may operate outside its contract envelope.

**Lua as primary logic layer.** All governance logic is exposed as a canonical `H.*` API surface (for example `H.Consent`, `H.Budget`, `H.Boundary`). This keeps behavior deterministic and auditable across repos.

**Tier separation.** Content is split into three visibility tiers:  
- Tier 1 (Public / GitHub-safe): schemas, contracts, public docs; no raw horror content.  
- Tier 2 (Private vault): age-gated horror logic, archives, seeds, style contracts.  
- Tier 3 (Research / Lab): high-risk research (e.g. BCI, spectral seeds) governed by separate ethics charters.

**13-repo constellation.** Governance is spread across thirteen repositories, each with tier and role assignments, described in Section 2.

### 1.2 Key Formulas

Two formulas govern dynamic intensity and consent mechanics.

**Dynamic allowed explicitness ceiling**

\[
E_{\max} = R \times (0.4 + 0.4C + 0.2H)
\]

Here \(R\) is a regulatory or regional cap, \(C\) is a consent factor, and \(H\) is a trust/history factor. This ceiling constrains effective explicitness \(E\) for a given session.

**Consent wave function**

\[
W(t) = \alpha C + \beta H(t) - \gamma D(t)
\]

Here \(C\) is baseline consent, \(H(t)\) models account maturity over time, \(D(t)\) tracks recent distress, and \(\alpha, \beta, \gamma\) are governance-tuned coefficients. Thresholds on \(W(t)\) determine which field (A–D) is active at a given moment.

### 1.3 Hard Prohibitions

The following prohibitions are absolute and tier-independent.

- No sexual violence: SVI must be 0 at all times.  
- No real-person targeting in horror scenarios.  
- No content targeting minors as victims, participants, or targets.  
- No glorification of real-world atrocities involving identified living victims.

### 1.4 Machine-Checkable Invariants

All invariants are quantized on \([0, 1]\) except where noted and are enforced via schemas and CI.

**Category A — Geo-Historical Invariants**

| Abbrev | Name                            | Range | Description                                              |
|--------|---------------------------------|-------|----------------------------------------------------------|
| CIC    | Catastrophic Imprint Coefficient | [0,1] | Historical catastrophe imprint of a location             |
| MDI    | Mythic Density Index            | [0,1] | Mythological narrative concentration                     |
| AOS    | Archival Opacity Score          | [0,1] | Incompleteness or obscurity of records                   |
| RRM    | Ritual Residue Map              | [0,1] | Presence of ritualistic historical practice              |
| FCF    | Folkloric Convergence Factor    | [0,1] | Overlap of independent folkloric traditions              |
| SPR    | Spectral Plausibility Rating    | [0,1] | Verisimilitude of supernatural claims                    |
| RWF    | Reliability Weighting Factor    | [0,1] | Reliability of historical source data                    |
| DET    | Dread Exposure Threshold        | [0,1] | Appropriate maximum dread intensity                      |
| HVF    | Haunt Vector Field              | [0,1] | Directional intensity of haunt phenomena                 |
| LSG    | Liminal Stress Gradient         | [0,1] | Stress across liminal boundaries                         |
| SHCI   | Spectral-History Coupling Index | [0,1] | Correlation between spectral phenomena and events        |

**Category B — Entertainment Metrics**

| Abbrev | Name                           | Range | Description                                              |
|--------|--------------------------------|-------|----------------------------------------------------------|
| UEC    | Uncertainty Engagement Coefficient | [0,1] | Uncertainty sustaining engagement                        |
| EMD    | Evidential Mystery Density     | [0,1] | Density of mysterious evidence                           |
| STCI   | Safe-Threat Contrast Index     | [0,1] | Contrast between safe and threatening moments            |
| CDL    | Cognitive Dissonance Load      | [0,1] | Cognitive load from contradictory cues                   |
| ARR    | Ambiguous Resolution Ratio     | [0,1] | Fraction of encounters left ambiguous                    |

**Category C — Chat Safety Invariants**

| Abbrev | Name                    | Constraint          | Description                          |
|--------|-------------------------|---------------------|--------------------------------------|
| SVI    | Sexual Violence Index   | Must be 0           | Sexual violence detection            |
| HVI    | Hate/Violence Index     | Tier-dependent cap  | Hate speech and gratuitous violence |
| SHI    | Self-Harm Index         | Tier-dependent cap  | Self-harm depiction/encouragement   |
| GXI    | Gore Explicitness Index | Tier-dependent cap  | Graphic gore/body horror            |
| NSI    | Nudity/Sexuality Index  | Tier-dependent cap  | Nudity and sexual content           |

### 1.5 Compliance Landscape

PEGI 2026 reforms and the DTSP 2025 Framework emphasize standardized risk categories, auditable logs, and deterministic policy engines, all of which Horror.Place implements via the contract-first stack. The immersive horror market is projected to grow from about 1.5 billion USD in 2025 to roughly 5 billion by 2033, indicating that governed horror experiences have commercial viability.

---

## 2. Repository Constellation

Horror.Place operates across thirteen repositories, each with an assigned tier and function.

| #  | Repository                          | Tier    | Purpose                                        |
|----|-------------------------------------|---------|------------------------------------------------|
| 1  | Horror.Place                        | Tier 1  | Main site, public docs and schemas            |
| 2  | Horror.Place-Orchestrator          | Tier 1  | CI/CD pipelines, workflows                     |
| 3  | HorrorPlace-Constellation-Contracts| Tier 1  | Core schemas, contracts, policy docs           |
| 4  | HorrorPlace-Codebase-of-Death      | Tier 2  | Core engine logic and intensity algorithms     |
| 5  | HorrorPlace-Black-Archivum         | Tier 2  | Historical archives and invariant bundles      |
| 6  | HorrorPlace-Spectral-Foundry       | Tier 2  | Style contracts, visual/audio pipelines        |
| 7  | HorrorPlace-Atrocity-Seeds         | Tier 2  | PCG seed vault, invariant-bound seeds          |
| 8  | HorrorPlace-Obscura-Nexus          | Tier 2  | Experimental styles and DSL variants           |
| 9  | HorrorPlace-Liminal-Continuum      | Tier 2  | Cryptographic marketplace, agent distribution  |
| 10 | HorrorPlace-Process-Gods-Research  | Tier 3  | Personality evolution research                 |
| 11 | HorrorPlace-Redacted-Chronicles    | Tier 3  | BCI/fMRI logs linked to metrics                |
| 12 | HorrorPlace-Neural-Resonance-Lab   | Tier 3  | Neural resonance and haptic tuning             |
| 13 | HorrorPlace-Dead-Ledger-Network    | Tier 3  | ZKP age-gating, DID/KYC, proof envelopes       |

Public repos never host Tier 2/3 rule bodies; they reference private content only via opaque IDs plus `deadledgerref`.

---

## 3. Module Cards — Ten Modules

Each module card defines purpose, key schemas, Lua APIs, CI integration, dependencies, and research objects. Gap status indicates how far each module is from implementation.

### 3.1 Module 1: Consent Profile & Comfort

**Module 1 — Consent Profile & Comfort**

Manages user age-gating, consent state, and per-tier explicitness caps, exposing a simple consent state machine and associated caps to all other modules.

- **Gap Status:** Schemas Drafted  
- **Target-repo:** HorrorPlace-Constellation-Contracts  

**Key Schemas**

- `schemas/consent/consent-state-machine.v1.json`  
- `schemas/consent/consent-explicitness-caps.v1.json`  
- `schemas/consent/consent-session-metrics.v1.json`  

**Lua API**

- `H.Consent.currentState()`  
- `H.Consent.checkCaps(tier, invariants)`  
- `H.Consent.bindSession(sessionId)`  

**CI Integration**

- Validate state transitions in `consent-state-machine` and reject invalid transitions.  
- Range checks on explicitness caps per consent tier.  

**Dependencies**

- Feeds → Age-Gated Tier Router (effective tier), Intensity Budget (caps), Telemetry binding.  

**Research Objects — Module 1**

- **RO 1.1: Consent State Machine Schema**  
  Defines states (minor, adult-basic, adult-horror, research), transitions, required proofs, and rollback rules, all cryptographically attested via Dead-Ledger envelopes.

- **RO 1.2: Per-Tier CIC×DET and Explicitness Caps**  
  Provides hard caps on CIC, DET, GXI, and explicitness bands per consent state, enforced across all policies.

- **RO 1.3: Telemetry Binding Without PII**  
  Binds consent state to session metrics using opaque session tokens, not PII, with clear rotation policies.

---

### 3.2 Module 2: Input Pattern & Banter Classifier

**Module 2 — Input Pattern & Banter Classifier**

Classifies user input into multidimensional labels (domain, intensity, mode, safety outcome) and routes them to policy decisions and banter adaptation.

- **Gap Status:** Schemas Drafted  
- **Target-repo:** HorrorPlace-Constellation-Contracts  

**Key Schemas**

- `schemas/classifier/input-label-taxonomy.v1.json`  
- `schemas/classifier/input-banter-policy.v1.json`  
- `schemas/classifier/banter-adaptation-policy.v1.json`  

**Lua API**

- `H.Banter.classify(input, sessionContext)`  
- `H.Banter.adaptStyle(sessionHistory)`  
- `H.Banter.checkPolicy(classifiedInput, tier)`  

**CI Integration**

- Taxonomy completeness and non-overlap checks.  
- Policy coverage tests ensuring every label combination maps to a deterministic outcome.  

**Dependencies**

- Receives ← Consent (tier), Telemetry (CDL, UEC).  
- Feeds → Content Boundary (labels), AI-Chat Template (banter style).  

**Research Objects — Module 2**

- **RO 2.1:** Canonical label taxonomy schema.  
- **RO 2.2:** Per-tier banter policy rules.  
- **RO 2.3:** Banter adaptation/down-shift logic tied to metrics.  
- **RO 2.4:** Invariant integration doc showing label-to-region context flows.

---

### 3.3 Module 3: Horror Intensity Budget

**Module 3 — Horror Intensity Budget**

Defines a per-session intensity budget \(B = f(\text{age\_tier}, \text{history\_length}, \text{recent DET}, \text{UEC})\) with independent sub-budgets per modality (violence, gore, sexual content, psychological).

- **Gap Status:** Schemas Drafted  
- **Target-repo:** HorrorPlace-Constellation-Contracts  

**Key Schemas**

- `schemas/budget/horror-intensity-budget-policy.v1.json`  
- `schemas/budget/horror-intensity-budget-metrics.v1.json`  
- `schemas/budget/horror-intensity-overbudget-policy.v1.json`  

**Lua API**

- `H.Budget.compute(sessionContext)`  
- `H.Budget.consume(eventType, amount)`  
- `H.Budget.shouldDownshift()`  
- `H.Budget.remaining()`  

**CI Integration**

- Range validation on modality caps.  
- Consistency checks against consent caps and ethics guardrails.  

**Dependencies**

- Receives ← Consent (caps), Telemetry (DET, UEC).  
- Feeds → Content Boundary, Selector, AI-Chat Template.  

**Research Objects — Module 3**

- **RO 3.1:** Budget formula and per-modality caps schema.  
- **RO 3.2:** Budget consumption telemetry schema.  
- **RO 3.3:** Over-budget behavior contract.  
- **RO 3.4:** Lua integration spec with full compute → consume → downshift flow.  
- **RO 3.5:** Cross-module composition rules with Banter and Consent.

---

### 3.4 Module 4: Content Boundary & Policy Engine

**Module 4 — Content Boundary & Policy Engine**

Final gatekeeper for outbound content, evaluating safety invariants and enforcing tier-specific rulesets.

- **Gap Status:** Schemas Drafted  
- **Target-repo:** HorrorPlace-Constellation-Contracts  

**Key Schemas**

- `schemas/safety/chat-safety-invariants-v1.json`  
- `schemas/safety/content-boundary-priority-rules-v1.json`  
- `schemas/safety/content-boundary-ruleset-v1.json`  

**Lua API**

- `H.Boundary.evaluate(content, sessionContext)`  
- `H.Boundary.applyRuleset(rulesetId, safetyVector)`  
- `H.Boundary.logDecision(sessionId, result)`  

**CI Integration**

- Priority rule consistency and ruleset completeness checks.  
- Leak tests for forbidden patterns.  

**Dependencies**

- Receives ← Banter labels, Budget state, Tier Router.  
- Feeds → AI-Chat Template decisions; enforces SVI=0 globally.  

**Research Objects — Module 4**

- **RO 4.1:** Chat-safety invariants schema with thresholds per tier.  
- **RO 4.2:** Deterministic priority and response strategies.  
- **RO 4.3:** Tiered rulesets and GitHub-safe vs vault-safe isolation.  
- **RO 4.4:** Engine wiring and signed safety decisions.  
- **RO 4.5:** Governance for public vs vault rule placement.

---

### 3.5 Module 5: Age-Gated Horror Tier Router

**Module 5 — Age-Gated Horror Tier Router**

Resolves effective horror tier from age gate, consent state, and repository-access capabilities, then routes to the appropriate logic stacks.

- **Gap Status:** Schemas Drafted  
- **Target-repo:** HorrorPlace-Constellation-Contracts  

**Key Schemas**

- `schemas/runtime/age-tier-routing-table.v1.json`  
- `schemas/security/repo-access-capability.v1.json`  
- `schemas/security/repo-access-proof-envelope.v1.json`  
- `schemas/runtime/age-tier-router-policy-v1.json`  
- `schemas/telemetry/age-tier-routing-metrics-v1.json`  

**Lua API**

- `H.TierRouter.resolve(sessionContext)`  
- `H.TierRouter.selectStack(routerState)`  
- `H.TierRouter.explain(routerState)`  

**CI Integration**

- Routing table completeness and fallback coverage.  
- Proof schema validation.  

**Dependencies**

- Receives ← Consent, Dead-Ledger proofs.  
- Feeds → Budget, Boundary, Selector, Templates.  

**Research Objects — Module 5**

Mapping tables, capability schemas, router policies, Lua wiring, and routing telemetry are all specified for implementation.

---

### 3.6 Module 6: Telemetry & Entertainment Metrics

**Module 6 — Telemetry & Entertainment Metrics**

Collects UEC, EMD, STCI, CDL, ARR and related signals, segments sessions, and enforces metric-based ethics guardrails.

- **Gap Status:** Schemas Drafted  
- **Target-repo:** HorrorPlace-Constellation-Contracts  

**Key Schemas**

- `schemas/telemetry/entertainmentmetricsv1.json`  
- `schemas/telemetry/session-segmentation-v1.json`  
- `schemas/policy/metrics-ethics-guardrails-v1.json`  
- `schemas/telemetry/session-metrics-envelope-v1.json`  

**Lua API**

- `H.Metrics.recordEvent(sessionId, event)`  
- `H.Metrics.updateFromSignals(sessionId, dt, signals)`  
- `H.Metrics.currentBands(sessionId)`  
- `H.Metrics.segmentTracker(sessionId)`  
- `H.Metrics.checkGuardrails(sessionId)`  

**CI Integration**

- Range checks on metrics.  
- Guardrail constraint validation and envelope schema checks.  

**Dependencies**

- Receives signals from all content modules.  
- Feeds → Budget, Selector, Templates, Ethics guardrails.  

**Research Objects — Module 6**

Canonical metric mappings, segmentation policy, guardrail schema, Lua API spec, and session envelope aggregation are all defined as research objects.

---

### 3.7 Module 7: History-Aware Content Selector

**Module 7 — History-Aware Content Selector**

Queries geo-historical invariants to choose thematically appropriate motifs and explicitness levels, with a DSL for invariant-to-pattern mapping and conflict resolution.

- **Gap Status:** Schemas Drafted  
- **Target-repo:** HorrorPlace-Constellation-Contracts  

**Key Schemas**

- `schemas/selector/history-selector-rule-v1.json`  
- `schemas/selector/history-selector-pattern-v1.json`  
- `schemas/policy/history-selector-conflict-policy-v1.json`  
- `schemas/telemetry/history-selector-decision-event-v1.json`  
- `schemas/policy/history-selector-pattern-governance-v1.json`  

**Lua API**

- `H.Selector.choosePattern(regionId, tileId, userIntent, sessionContext)`  
- `H.Selector.logDecision(sessionId, selectorResult)`  

**CI Integration**

Pattern library completeness, rule precedence, conflict coverage, and GitHub-safe decision logs.

**Dependencies**

- Receives ← Tier Router, Budget, Invariants.  
- Feeds → Templates, Content Boundary.  

**Research Objects — Module 7**

Selector DSL, conflict policy, decision events, Lua API spec, and pattern governance are enumerated for implementation.

---

### 3.8 Module 8: AI-Chat Template & Redaction

**Module 8 — AI-Chat Template & Redaction**

Defines prompt templates and post-filtering to prevent raw content leaks while allowing implied adult horror.

- **Gap Status:** Schemas Drafted  
- **Target-repo:** HorrorPlace-Constellation-Contracts  

**Key Schemas**

- `schemas/runtime/ai-chat-template-contract-v1.json`  
- `schemas/runtime/redaction-profile-v1.json`  
- `schemas/runtime/ai-chat-template-routing-v1.json`  
- `schemas/tooling/ai-chat-template-eval-config-v1.json`  
- `schemas/telemetry/ai-chat-template-metrics-v1.json`  

**Lua API**

- `H.Templates.select(sessionContext, intent)`  
- `H.Templates.apply(templateId, modelOutput)`  
- `H.Templates.logRedaction(sessionId, redactionEvents)`  

**CI Integration**

Leak tests, redaction-token coverage, and wiring consistency.

**Dependencies**

- Receives ← Tier Router, Selector, Budget, Boundary.  
- Produces → User-visible content.  

**Research Objects — Module 8**

Template contracts, redaction language, routing, evaluation harness, Lua integration, and telemetry research are all defined.

---

### 3.9 Module 9: DeadLedger/ALN Governance

**Module 9 — DeadLedger/ALN Governance**

Coordinates registry management, vault orchestration, and governance promotion with on-chain audit and build-flag bindings.

- **Gap Status:** Partially Ready  
- **Target-repos:** HorrorPlace-Constellation-Contracts + HorrorPlace-Dead-Ledger-Network  

**Key Schemas**

- `schemas/registry/descriptor-policy-v1.json`  
- `schemas/registry/public-registry-policy.json`  
- `schemas/governance/promotion-proposal-v1.json`  
- `schemas/governance/voting-rule-v1.json`  
- `schemas/governance/build-flag-binding-v1.json`  

**Lua API**

- `H.Registry.lookup(descriptorId)`  
- `H.Registry.checkPolicy(descriptor)`  
- `H.Governance.propose(promotionPayload)`  
- `H.Governance.vote(proposalId, vote)`  
- `H.Governance.lock(proposalId)`  

**CI Integration**

Descriptor policy checks, promotion schema validation, voting rule consistency, build-flag verification.

**Dependencies**

- Receives descriptors from all modules.  
- Feeds → Tier Router, Content Boundary, build systems.  

**Research Objects — Module 9**

Policy attributes, invariant-based registry tests, promotion proposal schemas, lock events, voting rules, and governance-to-build-flag bindings are specified.

---

### 3.10 Module 10: PCG Seed Generator

**Module 10 — PCG Seed Generator**

Generates invariant-bound procedural horror seeds stored in vaults and referenced by opaque IDs in public repos.

- **Gap Status:** Conceptual  
- **Target-repos:** HorrorPlace-Atrocity-Seeds + HorrorPlace-Constellation-Contracts  

**Key Schemas (proposed)**

- `schemas/pcg/pcg-seed-contract-v1.json`  
- `schemas/pcg/pcg-seed-generation-policy-v1.json`  
- `schemas/pcg/pcg-seed-validation-v1.json`  
- `schemas/pcg/pcg-seed-telemetry-v1.json`  

**Lua API (proposed)**

- `H.PCG.generateSeed(regionId, invariants, budgetState)`  
- `H.PCG.validateSeed(seedId, policy)`  
- `H.PCG.selectSeed(sessionContext, selectorResult)`  
- `H.PCG.logGeneration(sessionId, seedEvent)`  

**CI Integration**

Seed validation against invariants, forbidden-content scanning, budget compliance.

**Dependencies**

- Receives ← Selector, Budget, Tier Router.  
- Feeds → Templates, Boundary and downstream telemetry.  

**Research Objects — Module 10**

Seed contracts, generation policy, validation contracts, telemetry, and Lua wiring are laid out as concrete next steps.

---

## 4. Cross-Module Alignment

### 4.1 Shared Schema Spine

A unified schema spine in HorrorPlace-Constellation-Contracts ensures consistent organization and CI targeting.

| Directory              | Modules | Contents                                      |
|------------------------|---------|-----------------------------------------------|
| `schemas/consent/`     | 1       | Consent state and caps                        |
| `schemas/classifier/`  | 2       | Label taxonomy, banter policies               |
| `schemas/budget/`      | 3       | Budget formulas, metrics, overbudget rules    |
| `schemas/safety/`      | 4       | Safety invariants and boundary rulesets       |
| `schemas/runtime/`     | 5, 8    | Router tables, template contracts, profiles   |
| `schemas/telemetry/`   | 6 + all | Metrics, segmentation, envelopes, events      |
| `schemas/selector/`    | 7       | Selector rules, patterns, conflicts           |
| `schemas/registry/`    | 9       | Descriptor and registry policies              |
| `schemas/governance/`  | 9       | Proposals, voting rules, build-flag bindings  |
| `schemas/pcg/`         | 10      | Seed contracts, policies, validation          |
| `schemas/security/`    | 5       | Capabilities, proofs                          |
| `docs/`                | all     | APIs, explainers, research notes              |

### 4.2 Unified CI & GitHub Actions Pack

All modules share a CI pack with schema guards, range checks, leak tests, ethics checks, and governance lifecycle validation. These workflows are reusable across the constellation.

### 4.3 Standard NDJSON Telemetry Streams

Each module emits NDJSON events keyed by IDs such as `sessionId`, `regionId`, `seedId`, and `patternId`, enabling cross-module causal analysis.

### 4.4 Shared Lua API Surface

The `H.*` namespaces (`H.Consent`, `H.Banter`, `H.Budget`, `H.Boundary`, `H.TierRouter`, `H.Metrics`, `H.Selector`, `H.Templates`, `H.Registry`, `H.Governance`, `H.PCG`) use consistent naming, structured error objects, and shared logging hooks.

### 4.5 Research Feedback Loops

Each module defines telemetry-driven feedback loops to adjust caps, rules, patterns, and templates while preserving invariants and ethics constraints.

---

## 5. Compliance Integration

PEGI-aligned tier mapping, DTSP-aligned safety invariants, auditable logs, deterministic policy engines, and human-in-the-loop governance make the stack externally legible.

---

## 6. Gap Summary & Implementation Roadmap

A phased roadmap (Foundation → Content Pipeline → Intelligence → Governance) sequences module implementation, starting with Consent, Boundary, Tier Router, and shared CI.

---

## 7. Appendices

Appendices A–C catalogue invariant definitions, formula references, and hard prohibitions that all modules must respect.

**Document Control**  
Version: 1.0 • Status: Draft • Author: Doctor0Evil • Date: 6 April 2026  
Visibility: Tier 1 (GitHub-safe) • Review: Pending Module 9 promotion workflow.
