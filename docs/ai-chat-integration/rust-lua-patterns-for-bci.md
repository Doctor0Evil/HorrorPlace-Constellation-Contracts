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
    // 2. Parse feature_json -> BciFeatureEnvelopeV1
    // 3. Parse contract_json -> ConverterContract
    // 4. Apply EMA + calibration per transform.method
    // 5. Clamp all metrics to [0,1]; DET to [0,10]
    // 6. Serialize to BciMetricsEnvelopeV1 JSON
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

## Rust Geometry Kernel Pattern

```rust
// File: crates/bci_geometry/src/lib.rs

pub fn evaluate_mapping(
    binding: &BciGeometryBinding,
    inputs: &BciMappingInputs,
    safety: &BciSafetyProfile,
    state: &mut BciKernelState,
    dt: f32,
) -> BciMappingOutputs {
    // 1. Build weighted input vector from invariants/metrics/BCI
    // 2. Evaluate curve families per binding.curves
    // 3. Apply CSI/DET global caps
    // 4. Apply BciSafetyProfile caps (intensity, rate, recovery)
    // 5. Return clamped BciMappingOutputs
}
```

## Lua Geometry Entry Point Pattern

```lua
-- File: scripts/bci/geometry.lua

local function sample(player_id, region_id, tile_id)
  -- Assemble context
  local summary = BCI.getSummary(player_id)
  local invariants = H.Invariants.getSlice(region_id, tile_id)
  local metrics = H.Metrics.getBands(player_id, region_id)
  local csi = H.Timing.getCSI(player_id)
  local contract = H.Contract.getCurrentPolicy(player_id)
  
  -- Call Rust kernel via FFI
  local outputs = RustBci.evaluate_mapping(
    summary, invariants, metrics, csi, contract
  )
  
  -- Route to engine systems
  H.Visual.applyBciMask(player_id, outputs.visual)
  H.Audio.applyBciRtpcs(player_id, outputs.audio)
  H.Haptics.routeHaptics(player_id, outputs.haptics)
  
  return outputs
end

return { sample = sample }
```

## AI-Chat Authoring Rules

1. **Never invent new schema fields**. Use `$ref` to existing schemas or propose schema changes via PR.
2. **Never reference external libraries** (MNE, Python, BrainFlow) in engine code. Only Rust crates and Lua modules named in this doc.
3. **Always clamp metrics** to canonical ranges: bands in `[0,1]`, DET in `[0,10]`.
4. **Always log telemetry** via `BCIDebug.log*` functions; never silent-fail.
5. **Always validate** JSON against schema before FFI calls; use `hpc_jsonschema_validate`.

## File Naming Convention

- Rust crates: `crates/bci_*` (e.g., `bci_ema_smoothing`, `bci_geometry`)
- Lua modules: `scripts/bci/*.lua` or `scripts/hpc_bci_*.lua`
- Schemas: `schemas/telemetry/bci-*-v1.json` or `schemas/bci/bci-*-v1.json`
- Docs: `docs/telemetry/` or `docs/ai-chat-integration/`
