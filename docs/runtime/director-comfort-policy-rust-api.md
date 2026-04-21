Below is a docs outline you can drop in as `docs/runtime/director-comfort-policy-rust-api.md`. The prose includes explicit logic that coding‑agents and AI‑chat tools must obey when generating code or running the orchestrator. No citations appear in the file itself; you can treat this as final text.

***

## File path and scope

- Target repo: `HorrorPlace-Constellation-Contracts`  
- Path: `docs/runtime/director-comfort-policy-rust-api.md`  

This document explains how engine code, Lua, coding‑agents, and AI‑chat tools must use the Rust `h_director` crate and `director-comfort-policy.v1.json` to enforce governed horror behavior.

***

## 1. Purpose and guarantees

This API ensures that:

- Director personas never improvise intensity; they always read a `directorComfortPolicy` and act within its numeric limits.  
- All downshifts, softening, implication‑only behavior, and refusals are traceable to explicit fields in the policy and telemetry metrics (DET, CDL, ARR).  
- Coding‑agents and AI‑chat tools generate code and orchestration logic that call the adapter instead of branching directly on raw metrics or budget flags.

When `h_director` is in use:

- Every AI‑chat horror step that involves a director persona must pass through the Rust adapter (or an equivalent, tested implementation) before choosing templates or dungeon events.  
- No code or agent is allowed to choose `allow`/`soften`/`imply`/`downshift`/`refuse` by ad‑hoc rules; they must derive that choice from the adapter’s `StrategyResult`.

***

## 2. Data types and JSON contracts

### 2.1 director-comfort-policy.v1.json

`director-comfort-policy.v1.json` is the authoritative definition of a director persona’s comfort envelope. The Rust crate mirrors it via `DirectorComfortPolicy`:

Key fields (conceptual):

- `id`, `schemaRef`, `objectKind`, `tier`, `phase`  
- `directorKind`, `appliesToTiers`  
- `maxDetPerSession`, `maxDetPerBeat`  
- `cooldownRatio` (optional)  
- `cdlComfortBand`, `arrComfortBand` (range 0–1)  
- `strategyThresholds`  
  - `soften.detDeltaCeiling`, `soften.cdlUpper`  
  - `imply.nearDetSoftCap`, `imply.explicitnessCeiling`  
  - `downshift.detHardCapFraction`, `downshift.cdlHardUpper`  
  - `refuse.hardBoundaryViolated`, `refuse.detAbsoluteCap`  
- `prismMeta`  
- `deadLedgerRef`

The `h_director::policy::DirectorComfortPolicy` struct is a direct serde mapping of this schema. Policies must be loaded from the registry, not constructed ad‑hoc at runtime.

### 2.2 Runtime inputs and outputs

The Rust adapter lives in `h_director::adapter` and uses four types:

- `SafetyDecision`  
  - `decision: Option<String>` – upstream decision: `"allow"`, `"soften"`, `"imply"`, `"downshift"`, `"refuse"`  
  - `reason: Option<String>` – upstream reason text (optional)

- `MetricsSnapshot`  
  - `det: f32` – cumulative DET for this session  
  - `det_delta: f32` – proposed DET change for this beat/turn  
  - `cdl: f32` – current Cognitive Dissonance Load (0–1)  
  - `arr: f32` – current Ambiguous Resolution Ratio (0–1)

- `BoundaryState`  
  - `hard_boundary_violated: bool` – true if any content boundary/consent hard stop is currently in effect

- `StrategyResult`  
  - `strategy: Strategy` – one of `allow`, `soften`, `imply`, `downshift`, `refuse`  
  - `reason: Option<String>` – aggregated reasons from upstream and adapter

These are all serde‑serializable and are also used as JSON payloads for the Lua/FFI bridge.

***

## 3. Fetching the JSON policy from the registry

### 3.1 Engine and orchestrator behavior

At session binding time (when a director persona is attached):

1. The orchestrator resolves:
   - `directorPersonaContract` for the session.  
   - Its `comfortPolicyRef` field.

2. Using `comfortPolicyRef`, the orchestrator fetches the corresponding `director-comfort-policy.v1.json` artifact from the registry or local cache.

3. The engine deserializes the JSON into `DirectorComfortPolicy` via `serde_json`:

```rust
use h_director::policy::DirectorComfortPolicy;

fn load_comfort_policy(json_str: &str) -> DirectorComfortPolicy {
    serde_json::from_str(json_str).expect("valid director comfort policy JSON")
}
```

4. The resulting `DirectorComfortPolicy` must be stored in session state and reused; agents and code must not mutate its numeric fields at runtime.

### 3.2 Rules for coding‑agents and AI‑chat tools

When generating code that loads comfort policies:

- Always treat the JSON policy as read‑only configuration. Do not inline constants in code that duplicate policy values.  
- Do not hardcode per‑persona limits (e.g., specific DET caps) in Rust, Lua, or Godot; always read from `DirectorComfortPolicy`.  
- When proposing new comfort policy files, ensure they conform to the schema and keep intensity within the tier’s allowed bands.  

***

## 4. Calling the Rust adapter from Godot/C++ and Lua

### 4.1 Direct Rust call (native orchestrator)

If the orchestrator is written in Rust or can call Rust directly:

```rust
use h_director::adapter::{
    apply_safety_decision,
    SafetyDecision,
    MetricsSnapshot,
    BoundaryState,
    StrategyResult,
};
use h_director::policy::DirectorComfortPolicy;

fn choose_strategy(
    policy: &DirectorComfortPolicy,
    det: f32,
    det_delta: f32,
    cdl: f32,
    arr: f32,
    hard_boundary: bool,
    upstream_decision: Option<String>,
    upstream_reason: Option<String>,
) -> StrategyResult {
    let safety = SafetyDecision {
        decision: upstream_decision,
        reason: upstream_reason,
    };

    let metrics = MetricsSnapshot {
        det,
        det_delta,
        cdl,
        arr,
    };

    let boundary = BoundaryState {
        hard_boundary_violated: hard_boundary,
    };

    apply_safety_decision(&safety, &metrics, policy, &boundary)
}
```

The caller then uses `StrategyResult.strategy` to drive AI‑chat template selection and dungeon routing.

### 4.2 C/FFI entry point for Godot / Lua bridge

When the `lua-ffi` feature is enabled, `h_director` exports:

```c
// C signature
char* h_director_apply_safety_decision_json(
    const char* policy_json,
    const char* safety_decision_json,
    const char* metrics_json,
    const char* boundary_json
);

void h_director_free_string(char* ptr);
```

Usage pattern (conceptual C++/Godot pseudo‑code):

1. Build four JSON strings:

- `policy_json` – the `DirectorComfortPolicy` JSON.  
- `safety_decision_json` – object with `decision`, `reason`.  
- `metrics_json` – object with `det`, `det_delta`, `cdl`, `arr`.  
- `boundary_json` – object with `hard_boundary_violated`.

2. Call the FFI function:

```cpp
char* result_cstr = h_director_apply_safety_decision_json(
    policy_json_cstr,
    safety_json_cstr,
    metrics_json_cstr,
    boundary_json_cstr
);
```

3. Parse the returned JSON into a small struct with fields `strategy` and `reason`.

4. Free the returned string with `h_director_free_string(result_cstr)`.

### 4.3 Lua‑side logic

Lua itself should not re‑implement strategy logic. Instead, it should:

- Prepare JSON strings (via a thin C bridge or binding).  
- Call the FFI function or delegate to C++ to do so.  
- Use the resulting `strategy` string as a mode token when selecting templates and events (`"allow"`, `"soften"`, `"imply"`, `"downshift"`, `"refuse"`).

***

## 5. Wiring the strategy into AI‑chat templates and dungeon routing

### 5.1 Template selection rules

Runtime logic for AI‑chat templates must obey:

- If `strategy == "refuse"`:
  - Use refusal templates only.  
  - Do not generate any new horror imagery or explicit content.  
  - Optionally transition to meta‑safety or non‑horror content as dictated by other modules.

- If `strategy == "downshift"`:
  - Route to lower‑intensity moods or scenes.  
  - Prefer templates marked as cooldown, safe‑harbor, or ambient.  
  - Do not select templates flagged as high DET or high CDL.

- If `strategy == "imply"`:
  - Use implication‑only templates that avoid explicit descriptions of harm or trauma.  
  - Maintain mystery while respecting consent and boundaries.

- If `strategy == "soften"`:
  - Select templates for the same event/scene that use gentler descriptions.  
  - Reduce explicitness and avoid stacking multiple intense elements in one turn.

- If `strategy == "allow"`:
  - Use normal templates, but still respect global budgets and consent modules.

Coding‑agents generating template selection code must always branch on this strategy token, not on raw DET/CDL alone.

### 5.2 Dungeon routing rules

When dungeon or event routing is active:

- `refuse`:
  - Do not advance to any new horror event nodes.  
  - If necessary, move to safe nodes or terminate the horror run.

- `downshift`:
  - Bias or force routing to nodes tagged as low DET / cooldown.  
  - Avoid nodes whose planned DET increase plus current DET would exceed the policy’s `detHardCapFraction` or `detAbsoluteCap`.

- `imply`:
  - Keep the planned event, but choose variants that are less direct, leaving outcomes implied.

- `soften`:
  - If an event increases DET beyond `maxDetPerBeat` or `soften.detDeltaCeiling`, adjust its intensity or substitute with a milder node.

- `allow`:
  - Follow normal routing, still within global invariant and budget constraints.

Dungeon routing code must never bypass these rules based on internal heuristics. All “is it safe to go there?” decisions must respect the adapter’s strategy.

***

## 6. Rules for coding‑agents and AI‑chat orchestrators

This section is normative for AI‑based coding assistants and orchestration agents.

### 6.1 Prohibited behaviors

When generating code or orchestrating runs, agents must not:

- Implement their own strategy selection logic based on DET/CDL/ARR without invoking `h_director` or an equivalent, tested adapter.  
- Hardcode numeric thresholds for “too intense” or “too confusing” instead of reading from `DirectorComfortPolicy`.  
- Modify `DirectorComfortPolicy` values at runtime. Policies are configuration and governance artifacts, not runtime knobs.  
- Ignore `hard_boundary_violated` or treat boundary violations as soft hints.

### 6.2 Required behaviors

Agents must:

1. Treat `directorPersonaContract` and its `comfortPolicyRef` as mandatory when horror content is involved.  
2. Load the referenced `director-comfort-policy.v1.json` and deserialize into `DirectorComfortPolicy` (or pass through as JSON to FFI).  
3. For every AI‑chat turn or dungeon beat that may change DET or CDL:
   - Compute or read the current `MetricsSnapshot`.  
   - Read the current `BoundaryState`.  
   - Provide an upstream `SafetyDecision` from budget/boundary modules if available.  
   - Call `apply_safety_decision` (Rust or FFI) and obtain `StrategyResult`.  
   - Use `StrategyResult.strategy` to choose templates, events, or routing options, as described above.  
   - Log `strategy` and `reason` into telemetry.

4. When generating new comfort policies:
   - Ensure they keep DET and CDL within tier‑safe bounds.  
   - Set `refuse.detAbsoluteCap` at or below the tier’s DET ceiling.  
   - Provide meaningful `soften`, `imply`, and `downshift` thresholds so the adapter has room to operate.

### 6.3 Telemetry expectations

Every time the adapter is used:

- Log the following into the session route or metrics envelope:
  - Comfort policy ID (`DirectorComfortPolicy.id`).  
  - Director persona ID and kind.  
  - Metrics snapshot (DET, DET_delta, CDL, ARR).  
  - Upstream safety decision (if any).  
  - Final strategy and reason.

Coding‑agents generating telemetry code must ensure these fields are present, so future tuning of comfort policies can be data‑driven.

***

## 7. Example end‑to‑end flow (orchestrator perspective)

1. Session starts; player selects an experience contract and a director persona.  
2. Orchestrator:
   - Resolves the persona’s `comfortPolicyRef`.  
   - Loads and deserializes `DirectorComfortPolicy`.  
   - Stores it in session state.

3. Each turn:

   - Metrics and boundaries are updated by the budget and consent modules.  
   - Orchestrator constructs `SafetyDecision`, `MetricsSnapshot`, and `BoundaryState`.  
   - Orchestrator calls the Rust adapter (direct or via FFI).  
   - The resulting `strategy` is attached to the AI‑chat turn context.  
   - Template and dungeon routers read `strategy` and select content accordingly.  
   - Telemetry logs the entire decision.

4. At the end of the session:

   - Session route and metrics envelopes include a timeline of strategies.  
   - Designers and governance tools can review whether comfort policies behaved as intended, and adjust `director-comfort-policy.v1.json` files rather than engine code.

This doc establishes a clear contract: the comfort policy lives in JSON and registry, the adapter in Rust, and all horror‑relevant code and agents must route through both before deciding how far to push.
