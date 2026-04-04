# Registry Styles Specification

This document defines the structure, semantics, and usage rules for the `styles` registry in the HorrorPlace constellation. Styles are aesthetic and presentation contracts that govern visual, audio, and narrative tone for regions, events, or personas.

## Schema Reference

All `styles` registry entries must conform to `schemas/registry/registry-styles.v1.json`, which extends `schemas/registry/registry-entry-base.v1.json` with style-specific fields.

Canonical `$id`: `https://horrorplace.constellation/schemas/registry/registry-styles.v1.json`

## Required Fields (Beyond Base)

| Field | Type | Description |
|-------|------|-------------|
| `styleCategory` | enum | One of: `"visual"`, `"audio"`, `"narrative"`, `"haptic"`, `"mixed"`. |
| `targetTier` | enum | Access tier this style is valid for: `"public"`, `"vault"`, `"lab"`. |
| `aestheticDescriptors` | array[string] | Lowercase, hyphenated tags describing the style's feel (e.g., `["grainy-16mm","low-frequency-drone","fragmented-narrative"]`). |
| `invariantAlignment` | object | Map of invariant name → how the style responds to it (e.g., `{"AOS":{"audioPitchShift":">0.7"}}`). |

## Optional Fields

| Field | Type | Description |
|-------|------|-------------|
| `metricTargets` | object | Map of metric name → expected impact range (e.g., `{"UEC":{"+":[10,30]}}` means style should raise UEC by 10–30 points). |
| `engineHints` | object | Engine-specific rendering or audio hints (e.g., `{"unreal":{"postProcess":"bleach-bypass"},"godot":{"audioBus":"horror-low"}}`). Kept minimal and non-binding. |
| `prismMetaRef` | string | Reference to a `prismMeta` document describing style linkage and dependency graph. |
| `experimentalFlags` | array[string] | Tags for research-grade styles (e.g., `["neural-resonance-test","haptic-feedback-alpha"]`). Only valid in `lab` tier. |

## Style Category Semantics

### `visual`
Controls rendering parameters: color grading, film grain, lighting falloff, particle effects. Used to evoke historical periods (e.g., `soviet-era-16mm`) or environmental conditions (e.g., `salt-storm-haze`).

### `audio`
Controls audio mixing: reverb profiles, frequency filters, ambient stem selection, dynamic music cues. Tied to `invariantAlignment` for responsive sound design (e.g., higher AOS → lower frequency drone).

### `narrative`
Controls text presentation, dialogue pacing, lore reveal patterns. Used to modulate EMD (Evidential Mystery Density) and CDL (Cognitive Dissonance Load).

### `haptic`
Controls controller vibration, VR haptics, or other tactile feedback. Experimental; requires `lab` tier and `experimentalFlags`.

### `mixed`
Combines multiple categories. Must declare `aestheticDescriptors` for each sub-category.

## Invariant Alignment Format

`invariantAlignment` describes how a style adapts to invariant values at runtime:

```json
{
  "AOS": {
    "audioPitchShift": ">0.7",
    "visualGrain": "linear-scale"
  },
  "DET": {
    "musicIntensity": "stepwise-3-7-10"
  }
}
```

- Keys are invariant names from `invariants-spine.v1.json`.
- Values are style-response rules: thresholds, scaling functions, or discrete steps.
- The linter validates that referenced invariants exist and response rules use approved syntax.

## Validation Rules

The registry linter (`hpc-lint-registry.py`) enforces:

1. `styleCategory` matches one of the allowed enums.
2. `targetTier` is consistent with the entry's `tier` field (a `public` style cannot have `targetTier: "vault"`).
3. `aestheticDescriptors` are lowercase, hyphenated, and from the approved list in `docs/overview/design-principles.md`.
4. `invariantAlignment` keys match invariant names; response rules use approved syntax.
5. If `experimentalFlags` is non-empty, then `tier` must be `"lab"`.

## Example Entry

```json
{"id":"STY-ARAL-0001","schemaref":"https://horrorplace.constellation/schemas/registry/registry-styles.v1.json","deadledgerref":"zkp:sha256:sty001...","artifactid":"ipfs:bafysty001...","createdAt":"2026-01-15T03:00:00Z","status":"active","styleCategory":"mixed","targetTier":"vault","aestheticDescriptors":["grainy-16mm","low-frequency-drone","fragmented-narrative","salt-haze"],"invariantAlignment":{"AOS":{"audioPitchShift":">0.7","visualGrain":"linear-scale"},"DET":{"musicIntensity":"stepwise-3-7-10"}},"metricTargets":{"UEC":{"+":[10,30]},"CDL":{"+":[1,3]}},"engineHints":{"unreal":{"postProcess":"bleach-bypass","audioReverb":"large-industrial"},"godot":{"audioBus":"horror-low","shader":"film-grain"}},"prismMetaRef":"prism:sty-aral-0001"}
```

## Cross-Registry Linking

Styles are referenced by regions, events, and personas via ID. A region entry may list `styleIds: ["STY-ARAL-0001"]` to declare which aesthetic contracts apply. The spine index enables tools to answer: "What styles are valid for a high-CIC, vault-tier region?"

## Runtime Integration

At runtime, the engine loads the style's `aestheticDescriptors` and `invariantAlignment` to configure:

- Post-process volumes (Unreal) or shaders (Godot) for visual styles.
- Audio bus routing and DSP chains for audio styles.
- Dialogue system pacing rules for narrative styles.

The engine also reads `metricTargets` to tune adaptive systems: if a style aims to raise UEC by 10–30 points, the drama manager can prioritize events that support that goal.

## Related Documents

- `schemas/registry/registry-styles.v1.json`: Canonical schema for style registry entries.
- `registry/formats/styles.example.ndjson`: Minimal valid example entries.
- `docs/schema-spine/invariants-and-metrics-spine.md`: Invariant definitions referenced by styles.
- `docs/integration/engine-agnostic-integration.md`: How engines consume style hints without being locked to one platform.
