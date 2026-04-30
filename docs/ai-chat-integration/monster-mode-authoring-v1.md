# Monster-Mode BCI Geometry Authoring (Rotting-Visuals)

This document defines how AI-assisted tools must author BCI geometry bindings for monster-mode encounters using the Rotting-Visuals style pack. All authoring MUST go through the `ai-bci-geometry-request-v1` and `ai-bci-geometry-response-v1` envelopes and MUST produce bindings that validate against `bci-geometry-binding-v1.json` and the bindings collection metaschema.

## Authoring contract

When generating monster-mode bindings, the assistant SHALL:

- Accept requests shaped as `ai-bci-geometry-request-v1.json`.
- Return responses shaped as `ai-bci-geometry-response-v1.json`.
- Populate the `bindings` array only with objects that conform to `bci-geometry-binding-v1.json`.
- Never introduce new top-level fields or new metric spaces.
- Express all BCI conditions only in terms of:
  - `BciSummary` fields: `stressScore`, `stressBand`, `attentionBand`, `visualOverloadIndex`, `startleSpike`, `signalQuality`.
  - Invariants: `CIC`, `AOS`, `DET`, `LSG` and other spine-defined invariants.
  - Optional entertainment bands: `UEC`, `EMD`, `STCI`, `CDL`, `ARR` as gates.
- Reference safety profiles only by `profileId` defined in `bci-safety-profile-v1.json`.
- Use only approved curve family codes and four-parameter vectors.

### Monster-mode / Rotting-Visuals specialization

For Rotting-Visuals style bindings:

- `experienceType` in the request MUST be `"monster-mode"`.
- `regionHints.style` SHOULD be `"Rotting-Visuals"` or a compatible style tag.
- `targetPath` SHOULD point to a Rotting-Visuals style directory, for example:
  - `examples/bci/styles/Rotting-Visuals/bci-geometry-bindings.rotting-visuals.lab.sample.json`
- Bindings SHOULD:
  - Target corridor or threshold region classes where infection and close-quarters contact dominate.
  - Use visual curves to create:
    - Necrotic tunnel vision (shrinking mask radius, stronger vignette).
    - Edge desaturation and mild motion smear in the periphery.
  - Use audio curves to:
    - Boost low-frequency “body / hunger” pressure.
    - Slightly muffle squad/ally channels when `stressBand` is `High` or `Extreme` and `attentionBand` is `Focused` or `HyperFocused`.
  - Respect neurorights safety:
    - Clamp intensity upward more slowly as `visualOverloadIndex` rises.
    - Reduce intensity or hold steady when `stressBand` is `Extreme` or `signalQuality` is `Unavailable`.

## Required request shape (ai-bci-geometry-request-v1)

An authoring request for this style MUST look like:

```json
{
  "schemaVersion": "1.0.0",
  "targetRepo": "HorrorPlace-Neural-Resonance-Lab",
  "targetPath": "examples/bci/styles/Rotting-Visuals/bci-geometry-bindings.rotting-visuals.lab.sample.json",
  "bindingSchemaId": "hpnrl-bci-geometry-binding-v1",
  "regionHints": {
    "regionClass": "corridor",
    "style": "Rotting-Visuals",
    "sampleInvariants": {
      "cic": 0.7,
      "aos": 0.4,
      "det": 4.5,
      "lsg": 0.3
    }
  },
  "experienceType": "monster-mode",
  "constraints": {
    "allowedTiers": ["lab"],
    "allowedSafetyProfiles": ["monster-mode-standard"],
    "maxCurveComplexity": "medium"
  }
}
```

## Required response shape (ai-bci-geometry-response-v1)

The assistant MUST respond with a single JSON object shaped as:

```json
{
  "schemaVersion": "1.0.0",
  "bindings": [ /* one or more bci-geometry-binding-v1 objects */ ],
  "meta": {
    "authorAgent": "ai-chat",
    "notes": "Short explanation of the Rotting-Visuals binding intent.",
    "createdAt": "ISO-8601 timestamp"
  }
}
```

The next section provides a complete, lab-ready example pair for Rotting-Visuals.
