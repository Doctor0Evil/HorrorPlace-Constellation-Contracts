# Rust/Lua Patterns for BCI Development

## Purpose

This document provides trimmed, envelope-free code snippets that AI-chat should target when generating patches for BCI subsystems. All generated code must conform to these patterns.

## Rust FFI Pattern: `hpc_bci_process`

```rust
// File: crates/bci_ema_smoothing/src/ffi.rs
// DO NOT modify signature; used by Lua FFI

#[no_mangle]
pub extern "C" fn hpc_bci_process(
    feature_json: *const c_char,
    contract_json: *const c_char,
    out_metrics_json: *mut c_char,
    out_cap: usize,
) -> i32 {
    // 1. Validate inputs (null checks)
    // 2. Parse feature_json -> BciFeatureEnvelopeV1 (schema: bci-feature-envelope-v1)
    // 3. Parse contract_json -> ConverterContract
    // 4. Apply EMA + calibration per transform.method
    // 5. Clamp all metrics to [0,1]; DET to [0,10]
    // 6. Serialize to BciMetricsEnvelopeV1 JSON (schema: bci-metrics-envelope-v1)
    // 7. Write to out_metrics_json (respect out_cap)
    // 8. Return 0 on success, negative error code on failure
}
```

**Key constraints:**
- Output JSON MUST validate against `bci-metrics-envelope-v1.json`
- Never introduce new top-level fields; use `transform.featureMappingRules` for extensions
- Error codes: `-1` parse error, `-2` validation error, `-3` processing error

## Lua Adapter Pattern: `hpc_bci_adapter.lua`

```lua
-- File: scripts/hpc_bci_adapter.lua
-- DO NOT add numerics here; only orchestration

local ffi = require("ffi")
local json = require("dkjson")

ffi.cdef[[
  int32_t hpc_bci_process(
    const char* feature_json,
    const char* contract_json,
    char* out_metrics_json,
    size_t out_cap
  );
]]

local function process_feature_envelope(feature_env, contract_ctx)
  -- Encode inputs
  local feature_json = json.encode(feature_env)
  local contract_json = json.encode(contract_ctx)
  
  -- Prepare output buffer
  local out_cap = 65536
  local out_buf = ffi.new("char[?]", out_cap)
  
  -- Call Rust FFI
  local ret = ffi.C.hpc_bci_process(
    feature_json,
    contract_json,
    out_buf,
    out_cap
  )
  
  if ret ~= 0 then
    error("BCI processing failed with code: " .. tostring(ret))
  end
  
  -- Decode and validate output
  local metrics_json = ffi.string(out_buf)
  local metrics_env = json.decode(metrics_json)
  
  -- Apply contract caps (UEC/EMD/etc. in [0,1])
  return MetricsState.clamp_to_contract(metrics_env, contract_ctx)
end

return {
  process_feature_envelope = process_feature_envelope
}
```

## Rust Geometry Kernel Pattern (Aligned Schema)

```rust
// File: crates/bci_geometry/src/lib.rs

pub fn evaluate_mapping(
    binding: &BciGeometryBinding,  // from bci-geometry-binding-v1 schema
    inputs: &BciMappingInputs,     // { summary, invariants, metrics, csi }
    safety: &BciSafetyProfile,     // from bci-safety-profile-v1 schema
    state: &mut BciKernelState,
    dt: f32,
) -> BciMappingOutputs {
    // 1. Build weighted input vector from invariants/metrics/BCI using binding.inputWeights
    // 2. Evaluate curve families per binding.curves (structured visual/audio/haptics)
    // 3. Apply CSI/DET global caps from safety.timingCaps
    // 4. Apply BciSafetyProfile caps (intensity, rate, recovery)
    // 5. Return clamped BciMappingOutputs
}
```

## Lua Geometry Entry Point Pattern

```lua
-- File: scripts/bci/geometry.lua

local function sample(player_id, region_id, tile_id)
  -- Assemble context
  local summary = BCI.getSummary(player_id)  -- bci-summary-v1
  local invariants = H.Invariants.getSlice(region_id, tile_id)
  local metrics = H.Metrics.getBands(player_id, region_id)
  local csi = H.Timing.getCSI(player_id)
  local contract = H.Contract.getCurrentPolicy(player_id)
  
  -- Call Rust kernel via FFI (binding resolution happens in Rust)
  local outputs = RustBci.evaluate_mapping(
    summary, invariants, metrics, csi, contract
  )
  
  -- Route to engine systems via helpers
  H.Visual.applyBciMask(player_id, outputs.visual)
  H.Audio.applyBciRtpcs(player_id, outputs.audio)
  H.Haptics.routeHaptics(player_id, outputs.haptics)
  
  return outputs
end

return { sample = sample }
```

## AI-Chat Authoring Rules

1. **Schema-first**: Always look up or reconstruct the JSON Schema before proposing Rust/Lua types.
2. **Pipeline layering**: External EEG → `bci-feature-envelope-v1` → `bci-metrics-envelope-v1` → `bci-summary-v1` → geometry bindings → outputs. No direct feature consumption in Death-Engine.
3. **Geometry binding constraints**:
   - Use only fields defined in `bci-geometry-binding-v1.json`
   - Express conditions only in terms of invariants (CIC, LSG, etc.) and BciSummary fields
   - Use only recognized `familyCode` values and four-parameter arrays per curve
   - Reference safety profiles via `profileId` into `bci-safety-profile-v1.json`
4. **Safety enforcement in Rust only**: All caps, cooldowns, and safe zones belong in Rust geometry kernels.
5. **No citations in generated files**: Never include `[web:..]`, `[file:..]`, or similar in repo content.

## File Naming Convention

- Rust crates: `crates/bci_*` (e.g., `bci_ema_smoothing`, `bci_geometry`)
- Lua modules: `scripts/bci/*.lua` or `scripts/hpc_bci_*.lua`
- Schemas: `schemas/telemetry/bci-*-v1.json` or `schemas/bci/bci-*-v1.json`
- AI envelopes: `schemas/ai/ai-bci-*-v1.json`
- Docs: `docs/telemetry/` or `docs/ai-chat-integration/`
