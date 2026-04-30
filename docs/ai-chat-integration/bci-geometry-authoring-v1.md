# BCI Geometry Authoring for AI-Chat v1

## Purpose

This document defines how AI-chat agents should request and return BCI geometry bindings. All generated content must conform to the schemas in `schemas/ai/` and `schemas/bci/`.

## Request/Response Flow

1. AI receives an `ai-bci-geometry-request-v1` with target repo, path, and constraints.
2. AI generates one or more `bci-geometry-binding-v1` objects that:
   - Use only fields defined in the schema (`additionalProperties: false`)
   - Reference invariants (CIC, LSG, etc.) and BciSummary fields (stressBand, attentionBand, visualOverloadIndex, startleSpike, signalQuality)
   - Optionally use entertainment bands (UEC, EMD, STCI, CDL, ARR) only as gating signals
   - Use only recognized `familyCode` values: PKLIN, PKSIG, PKHYS, PKRHY, PKAMB, PKSTC
   - Reference safety profiles via `profileId` into `bci-safety-profile-v1.json`
3. AI returns an `ai-bci-geometry-response-v1` wrapping the bindings.

## Example Request

```json
{
  "schemaVersion": "1.0.0",
  "targetRepo": "HorrorPlace-Neural-Resonance-Lab",
  "targetPath": "examples/bci-geometry-bindings.corridor-high-cic.json",
  "bindingSchemaId": "hpnrl-bci-geometry-binding-v1",
  "experienceType": "Slowburn",
  "regionHints": {
    "regionClass": "corridor",
    "cicTarget": 0.85,
    "lsgTarget": 0.6
  },
  "constraints": {
    "allowedTiers": ["standard", "mature"],
    "maxSafetyProfile": "corridor_med"
  },
  "authorNotes": "High-CIC corridor with sigmoid tunnel curve for visual mask."
}
```

## Example Response

```json
{
  "schemaVersion": "1.0.0",
  "bindings": [
    {
      "schemaId": "hpnrl-bci-geometry-binding-v1",
      "schemaVersion": "1.0.0",
      "bindingId": "corridor_high_cic_sigmoid_tunnel",
      "regionClass": "corridor",
      "tier": "standard",
      "priority": 7,
      "invariantFilter": {
        "cicMin": 0.7, "cicMax": 1.0,
        "lsgMin": 0.4, "lsgMax": 0.8
      },
      "bciFilter": {
        "stressBand": ["Medium", "High"],
        "visualOverloadMax": 0.7
      },
      "inputWeights": {
        "stressScore": 0.4, "visualOverloadIndex": 0.3, "cic": 0.2, "lsg": 0.1
      },
      "curves": {
        "visual": {
          "familyCode": "PKSIG",
          "params": [0.0, 0.8, 0.5, 2.0]
        },
        "audio": {
          "pressureLf": { "familyCode": "PKLIN", "params": [0.1, 0.6, 0.0, 1.0] }
        },
        "haptics": {
          "intensity": { "familyCode": "PKHYS", "params": [0.2, 0.5, 0.1, 0.3] }
        }
      },
      "safetyProfile": "corridor_med"
    }
  ],
  "meta": {
    "authorAgent": "hpc-ai-chat-v3",
    "notes": "Binding tuned for high-CIC corridors with medium-high stress."
  }
}
```

## AI-Chat Rules

1. **Never invent new schema fields**. Use `$ref` to existing schemas or propose schema changes via PR.
2. **Never reference external libraries** (MNE, Python, BrainFlow) in engine code. Only Rust crates and Lua modules named in `rust-lua-patterns-for-bci.md`.
3. **Always clamp metrics** to canonical ranges: bands in `[0,1]`, DET in `[0,10]`.
4. **Always validate** generated JSON against the target schema before returning.
5. **Always include `schemaId` and `schemaVersion`** in every binding object.
6. **Safety logic belongs in Rust**. Bindings reference `safetyProfile` by ID; they do not inline caps.

## Integration Points

- Rust: `crates/bci_geometry/src/lib.rs` evaluates bindings via `evaluate_mapping()`.
- Lua: `scripts/bci/geometry.lua` orchestrates binding resolution and output routing.
- CI: All bindings in `examples/` must pass `jsonschema` validation against `bci-geometry-binding-v1.json`.
