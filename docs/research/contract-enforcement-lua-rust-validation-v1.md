---
invariants_used:
  - CIC
  - DET
  - UEC
  - ARR
  - SHCI
  - RWF
metrics_used:
  - UEC
  - ARR
  - CDL
  - STCI
  - EMD
tiers:
  - standard
  - mature
  - research
deadledger_surface:
  - zkpproof_schema
  - verifiers_registry
  - bundle_attestation
---

# Contract Enforcement & Lua/Rust Mechanic Validation in Horror$Place

Schema Reference: `schema://HorrorPlace-Constellation-Contracts/research_doc_v1.json`  
Tier: `T1_public`

This document maps the existing contract spine and CHATDIRECTOR design into concrete next steps for enforcing invariant‑driven mechanics across Lua and Rust, and for treating code and document quality as a non‑negotiable property of the VM‑constellation.[file:45]

## 1. Next‑Step Research Directions

This section extends the existing inventory into clear research paths that tighten contract enforcement, systemic timing, and cross‑language validation.[file:45]

### 1.1 Unify “narrowing” as a first‑class constraint

The current contract spine already treats `policyEnvelope → regionContractCard → seedContractCard → styleContract → eventContract` as a parent→child band chain for invariants and metrics.[file:45] The next move is to make **narrowing** an explicit, shared rule visible in both schemas and validators.[file:45]

First, every contract schema in the constellation, including `styleContract`, `eventContract`, and `sequence-seed-v1`, should grow normalized `invariants` and `metricTargets` blocks that carry `min`, `max`, and optional `target` values for each code (CIC, DET, UEC, ARR, SHCI, RWF, etc.).[file:45] These blocks must be defined in a reusable schema fragment so that band semantics stay identical across contract types and tiers.[file:45] By locking this into Tier‑1 schemas, CHATDIRECTOR and downstream tooling can treat band ranges as data, not comments.[file:45]

Second, every contract card must carry a `parentRef` (or equivalent pointer) so that validators can walk the parent→child chain and enforce interval inclusion across all invariants and metrics.[file:45] At validation time, the narrowing rule is simple: for each field, the child’s `[min, max]` interval must be a subset of the parent’s, with explicit special cases such as “DET may not widen at Tier 1” encoded as table‑driven policies rather than ad‑hoc conditionals.[file:45] This makes the narrowing behavior auditable and easy to extend when new metrics or invariants are added.[file:45]

Research work here is to formalize narrowing as a pure band‑inclusion check that both CHATDIRECTOR and Lua linters can share.[file:45] That includes defining a canonical JSON shape for bands, specifying allowed comparison tolerances, and encoding tier‑specific exceptions (for example, permitting controlled widening of UEC bands only under strict DET ceilings in research tiers). Any widening or illegal drift discovered by the narrowing check must be treated as a hard error that blocks merges in CI and prevents contract promotion in Orchestrator flows.[file:45]

### 1.2 Systemic timing mechanics as invariant contracts

The systemic timing family (Coyote Time, Rule of Three, Adaptive Damage) already has contract‑like formulas; the missing step is to anchor them directly to CIC, DET, UEC, ARR, and the timing‑parameter schema drafted in earlier work.[file:32][file:45] The first task is to treat timing behavior as its own mechanic contract with a dedicated schema, for example `mechanic_contract_timing_params_v1.json`, sitting alongside existing style and event contracts.[file:45] This schema should define explicit invariant preconditions (DET caps, ARR floors) and acceptable bands for UEC and CDL so that each timing variant is constrained by the same geometry as other mechanics.[file:32][file:45]

On the implementation side, Lua helpers such as `systemic_timing.lua` must be refactored to compute probabilities and multipliers strictly via the formulas encoded in the mechanic contract.[file:32][file:45] All weights, caps, and bounds should be read from the timing contract instance, not hard‑coded in Lua, so that contract edits become the only allowed path for tuning behavior.[file:32][file:45] This change ensures that every lethal or near‑lethal decision in the runtime is a direct function of invariant geometry and metric bands, rather than code‑local magic numbers scattered across scripts.[file:32][file:45]

### 1.3 Formal verification hooks for timing formulas

Once timing formulas are schema‑driven, formal verification becomes tractable.[file:32] The immediate mathematical objective is to encode key functions such as \( p_{\text{coyote}} \), \( p_{\text{downgrade}} \), and \( m_{\text{damage}} \) in a proof assistant like Lean or Coq with numeric ranges pulled from `invariants_v1` and `mechanic_contract_timing_params_v1`.[file:32] Within that environment, proofs of monotonicity (for example, increased DET should not decrease safety clamps) and boundedness (outputs must remain inside contract bands) can be constructed for the full legal parameter space.[file:32]

These proofs should then be tied directly to the Rust implementations that CHATDIRECTOR and engine helpers call.[file:32] One viable pattern is proof‑by‑code‑generation, where verified functions in the theorem prover are exported into Rust, or proof‑carrying code patterns where the Rust kernels are small enough to be checked against the proven formulas.[file:32] Lua must rely only on these Rust functions through a narrow FFI surface, ensuring that high‑risk timing decisions cannot drift away from mathematically constrained behavior that respects DET caps and ARR floors.[file:32]

### 1.4 Runtime Lua surface as a minimal invariant/meter contract

On the Lua side, existing H. API blueprints and runtime‑surface research already sketch a minimal, engine‑agnostic contract layer: `H.CIC`, `H.DET`, `H.metrics`, `H.contract`, `H.Selector`, `Policy.DeadLedger`, and related entry points.[file:33][file:45] A near‑term goal is to finalize a `H.runtime-core-lua-api-minimal-v1` specification that pins function names, argument envelopes, and schema bindings for all invariant reads, metric reads, contract lookups, selector decisions, and Dead‑Ledger gating calls.[file:33][file:45] That spec should live in Constellation‑Contracts and be referenced by Lua linting tools and engine integrations alike.[file:33]

To enforce this contract, CI scanners need to forbid any direct access to raw invariant tables, vault data, or engine‑local globals that bypass H.[file:33][file:45] All runtime horror logic must flow through a small, vetted H. surface, whose functions are themselves backed by JSON Schemas and Rust validators.[file:33] This approach unifies Lua behavior across engines and enables CHATDIRECTOR and Rust analysis tools to reason about mechanics purely in terms of structured, contract‑aware function calls.[file:33]

## 2. Code‑Quality Improvements: Rust, Lua, NDJSON

This section outlines concrete changes that raise code and document quality and tie mechanics tightly back to contracts and metrics.[file:45]

### 2.1 Rust CHATDIRECTOR validation pipeline

The CHATDIRECTOR spec defines a multi‑layer validation pipeline with stages for schema checks, phase consistency, invariants/metrics, manifests, and envelopes.[file:45] In Rust, `validate_invariants.rs` should be completed with three passes: an initial raw range check against the invariant spine, an application of all cross‑metric XMIT rules, and a post‑XMIT re‑check that emits diagnostics annotated with `interactionEffects` and `fixOrder` hints.[file:45] Each pass should produce machine‑readable error codes and structured payloads so that AI tools and human authors receive consistent guidance.[file:45]

At the module level, `validate/mod.rs` needs to strictly enforce layer ordering and implement aggressive short‑circuiting when early stages fail.[file:45] Diagnostics surfaced from this pipeline should be ranked by severity, validation layer, and estimated fix cost so that automated tooling knows which corrections to prioritize.[file:45] With these changes in place, CHATDIRECTOR becomes a stable compiler surface that CI, AI‑chat, and manual workflows can trust as the ground truth for numeric and structural violations.[file:45]

### 2.2 Rust narrowing validator and narrowing telemetry

To make narrowing operational, a dedicated Rust narrowing validator should walk parent→child chains of the form `policyEnvelope → region → seed → style → event → sequenceSeed`, evaluating interval inclusion for every invariant and metric field.[file:45] This validator should plug into the validation pipeline ahead of manifest routing so that any band misalignment is caught before content is dispatched to engines or vaults.[file:45] Diagnostics from this pass must include both parent and child ranges, as well as any DET/SHCI cross‑field rules that were violated, to support precise repairs.[file:45]

For telemetry, narrowing failures and near‑misses should be logged as structured NDJSON events keyed by invariant and metric IDs.[file:32][file:45] Aggregating these logs over time will highlight which invariants and metrics are most often misused by authors or AI, informing better default bands, smarter authoring hints, and refined training data for contract‑aware generation.[file:32][file:45] This feedback loop moves narrowing from a purely static rule into a live governance signal.[file:32]

### 2.3 Lua contract linters as front‑line compilers

On the Lua side, existing sketches for `hpc-contract-cards.lua`, `hpc-spineclient.lua`, and related tools should be elevated into proper linters.[file:45] The linter entry point should load a contract card and its parent, use the spine client to fetch canonical bands, apply the shared narrowing rules, and print a concise band‑check summary suitable for both humans and AI‑chat consumption.[file:45] This summary should reference invariant and metric codes, show parent and child ranges, and classify violations by severity.[file:45]

These Lua linters should be wired into pre‑commit hooks and authoring scripts so that designers and AI pipelines see narrowing and DET/ARR band violations before CI runs.[file:45] By providing fast, engine‑agnostic diagnostics, the linters reduce noisy CI failures and create a gentle on‑ramp for learning the invariant and metric geometry.[file:45] They also act as a thin compiler front‑end for contract cards, backed by the same schemas and narrowing policies enforced in Rust.[file:45]

### 2.4 NDJSON schemas and telemetry shapes

Telemetry specs such as `execution-string-run.v1` and `systemic-timing-run.v1` already exist conceptually and should be formalized as JSON Schemas with `additionalProperties: false` and normalized naming conventions for invariants and metrics.[file:33][file:45] These schemas need explicit “before/after” structures (for example, `metrics_before` and `metrics_after`) so that delta analysis for UEC, ARR, CDL, and related fields is straightforward.[file:33] Normalizing field names and ranges across telemetry streams avoids brittle, one‑off parsing code.[file:33]

Small Rust or CLI tools should validate NDJSON streams against these schemas and compute aggregate trajectories such as ΔUEC/ΔARR/ΔCDL per mechanic and contract ID.[file:33][file:45] These tools can also tag runs with tier, region class, and persona context, making it easier to mine telemetry for patterns that suggest where mechanics or contracts need retuning.[file:33] Together, strict schemas and analysis tools turn NDJSON into a first‑class contract surface, not a by‑product.[file:33]

## 3. Enforcement Logic: Contract → Lua → Rust → Telemetry

This section connects contract definitions to concrete enforcement points in Lua and Rust and shows how they close the loop via telemetry.[file:33][file:45]

### 3.1 Contract‑driven systemic timing in Lua

Given the timing formulas specified in the mechanic contracts, Lua helpers in `engine/systemic_timing.lua` should treat all timing behavior as pure functions of `(region_id, player_id, contract_parameters)` plus invariant and metric values read via H. and Metrics (for example, DET, ARR, UEC).[file:33][file:45] The helper functions should avoid local state that is not captured in telemetry, keeping behavior reproducible and audit‑friendly.[file:33] Contract parameters become the only tunable inputs; hard‑coded constants are considered contract violations.[file:33]

Every helper decision—probabilities, random draws, branch choices, and resulting damage multipliers—should be logged into `systemic-timing-run.v1` records bound to the active mechanic contract ID.[file:33][file:45] This enables Rust analysis tools and notebooks to correlate per‑decision telemetry with entertainment metrics and invariant states over time.[file:33] The key quality rule is that helpers must not embed undocumented branches; each path must be explainable in terms of contract parameters and invariant/metric bands.[file:33]

### 3.2 Rust mechanic kernels and safety envelopes

In Rust (for example, in Death‑Engine or Codebase‑of‑Death), timing and damage computations should be encapsulated in small mechanic kernels that accept DET, ARR, UEC, timing parameters, and RWF/invariant bands and return probabilities or multipliers guaranteed to be in range.[file:32][file:45] These kernels should perform all clamping and safety checks, ensuring outputs respect both invariant ceilings and contract‑specified limits.[file:32] Unit tests must cover edge cases across the entire allowed parameter and invariant space.[file:32]

The API surface exposed to Lua and ALN nodes must be narrow, routing all calls through a small set of approved kernel functions rather than re‑implementing math in scripts.[file:32][file:45] CHATDIRECTOR can then verify that mechanics only use these kernels by scanning call graphs and FFI bindings.[file:32] Once the kernels are verified (and, where feasible, formally proved), they become the “privileged math” layer where systemic timing behavior is allowed to change, with all higher‑level code constrained to call them.[file:32]

### 3.3 Telemetry‑driven refinement and contract updates

With telemetry and kernels in place, Process‑Gods and related personas can use notebooks to analyze how often timing helpers grant mercy, downgrade lethal hits, or scale damage, and how those choices affect UEC, ARR, and CDL across regions and tiers.[file:32][file:45] These analyses should be framed in terms of contract IDs and mechanic versions to keep interpretations aligned with concrete artifacts.[file:32] Findings can then feed back into contract default bands and tuning policies.[file:32]

Contract updates should be driven through AI authoring envelopes and mechanic contract cards rather than direct Lua edits.[file:45] When a timing profile needs adjustment, AI or human authors propose new parameters in `mechanic_contract_timing_params` cards, CHATDIRECTOR validates them (including narrowing checks), and the approved cards are promoted to production.[file:45] This keeps tuning data‑driven and contract‑first and prevents code drift between Lua and Rust implementations.[file:45]

## 4. Procedural Continuity & GitHub Sovereignty

This section ties the above work back into the three‑artifact model and procedural enforcement already defined for the VM‑constellation.[file:45]

### 4.1 Three‑artifact changesets and phase rules

The “spec → contract card → implementation” model, tied to phases 0–4 and enforced by CHATDIRECTOR, remains the core quality gate for mechanics.[file:45] To keep timing and mechanic validation aligned with this model, any new or changed timing behavior must be represented by: (1) a spec update (docs and schema for timing parameters), (2) updated mechanic contract cards, and (3) implementation changes in Lua/Rust that are strictly limited to calling the new parameters and kernels.[file:45] These changes should be bundled into a single, reviewable changeset.[file:45]

CI and CHATDIRECTOR must make it impossible to land Lua/Rust changes in timing helpers unless the corresponding timing contract has been updated and passes narrowing and invariant checks.[file:45] This preserves the doctrine that “horror is contracts, not scripts” and keeps the repositories structurally clean by ensuring every behavior change casts a shadow in Tier‑1 schemas and registries.[file:45]

### 4.2 CI and ZKP hardening

Current CI pipelines already enforce schema validation, invariant range checks, manifest routing, and Dead‑Ledger hooks, and the research blueprint outlines a future ZKP layer.[file:34][file:45] For timing and enforcement, CI workflows should be extended so that any mechanic contract or timing parameter file is validated against both the timing schema and the invariant spine, and any such change triggers re‑validation of dependent Lua/Rust files through CHATDIRECTOR.[file:45] This creates a closed loop where contract edits automatically exercise all affected runtimes.[file:45]

On the ZKP side, Dead‑Ledger circuits can be designed to prove off‑chain that a given set of contracts and mechanic parameters respect invariant ranges and narrowing rules.[file:34][file:45] HorrorPlace‑Dead‑Ledger‑Network would then store these proofs for high‑impact content or Tier‑3 labs as opaque `zkpproof` envelopes referenced via `deadledgerref`, without exposing raw player data or trauma content.[file:34] GitHub remains the sovereign host of specs and code, while Dead‑Ledger becomes the layer where “contract health” is cryptographically attested.[file:34]

## 5. Personas, Telemetry, and Cross‑Repo Reasoning

This section shows how personas, telemetry, and cross‑repo workflows support the enforcement patterns described above.[file:33][file:45]

### 5.1 Persona contracts and enforcement roles

Persona contracts already define metric levers and invariant read/write capabilities and can be treated as “enforcement roles” for mechanics.[file:33][file:45] For example, Custodian or Ledger‑Warden personas can be assigned responsibility for monitoring DET caps and RWF thresholds, Archivist can focus on UEC/ARR effects of mechanics, and Process‑Gods can observe CI and failure patterns.[file:33][file:45] These roles should be encoded directly in persona contract fields, not just implied in documentation.[file:33]

CHATDIRECTOR and runtime telemetry can provide structured inputs to these personas in the form of band deltas, narrowing violations, and RWF changes.[file:33][file:45] Tuned as governance agents rather than purely narrative voices, personas can then act as interpretable controllers that recommend or veto contract changes based on observed patterns, keeping mechanical safety and horror pacing grounded in characters designers can think with.[file:33]

### 5.2 Cross‑repo loops and YAML workflows

To improve computational reasoning capacity across all twelve repos, small, schema‑aware workflows can be added that regularly scan contract and telemetry schemas and build summary indices such as `telemetry-metric-coverage.json` and `schema-spine-index.json`.[file:45] These indices would catalog which invariants, metrics, personas, and mechanics exist, their allowed bands, and their relationships, and would be committed as versioned artifacts in Constellation‑Contracts or a central governance repo.[file:45]

Rust, Lua, or HTTP “discovery” APIs can then serve these summary artifacts to AI‑chat and orchestration tools, ensuring that any file generation starts with a clear view of the invariant and metric landscape.[file:33][file:45] This reinforces the intended pattern—discover → constrain → generate → validate → log → refine—and gives Archivist and Process‑Gods a richer context for cooperative learning and contract‑aware co‑development.[file:33][file:45]

By following these directions, systemic timing and mechanic validation become an invariant‑driven, contract‑first discipline enforced consistently across schemas, Lua, Rust, and NDJSON, while CHATDIRECTOR and the persona layer gain the structured signals they need to keep Horror$Place evolving procedurally, safely, and coherently over time.[file:33][file:45]
