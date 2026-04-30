---
title: Monster-Mode BCI Geometry Authoring – Rotting-Visuals
doctype: ai-chat-authoring-guide
version: 1.1.0
role: "AI authoring constraints for BCI monster-mode (Rotting-Visuals style pack)"
status: "lab"
targetSchema:
  request: "schemas/ai/ai-bci-geometry-request-v1.json"
  response: "schemas/ai/ai-bci-geometry-response-v1.json"
  binding: "schemas/bci/bci-geometry-binding-v1.json"
  safety: "schemas/bci/bci-safety-profile-v1.json"
stylePackId: "monster-mode.rotting-visuals.v1"
---

# 0. Overview

This guide defines how AI-assisted tools must author BCI geometry bindings for **monster-mode** encounters using the **Rotting-Visuals** style pack.
All authoring flows through `ai-bci-geometry-request-v1` and `ai-bci-geometry-response-v1` envelopes and produces bindings that validate against `bci-geometry-binding-v1.json` and the bindings-collection metaschema.

The Rotting-Visuals pack specializes monster-mode toward necrotic tunnel vision, edge decay, and muffled squad perception, while preserving the existing BCI pipeline, metrics, and safety doctrine.

---

# 1. Authoring contract

When generating monster-mode bindings (including Rotting-Visuals), the assistant SHALL:

- Accept requests shaped as `ai-bci-geometry-request-v1`.
- Return responses shaped as `ai-bci-geometry-response-v1`.
- Populate the `bindings` array only with objects conforming to `bci-geometry-binding-v1.json` and the bindings-collection metaschema.
- Never introduce new top-level fields or new metric spaces.
- Express all BCI conditions only in terms of:
  - BciSummary fields: `stressScore`, `stressBand`, `attentionBand`, `visualOverloadIndex`, `startleSpike`, `signalQuality`.
  - Invariants: `CIC`, `AOS`, `DET`, `LSG` and other spine-defined invariants referenced by existing schemas.
  - Optional entertainment metrics as gates: `UEC`, `EMD`, `STCI`, `CDL`, `ARR`.
- Reference safety profiles only by `profileId` defined in `bci-safety-profile-v1.json`.
- Use only approved curve families and parameter vectors defined in the Rust geometry kernel.
- Respect neurorights and existing BCI intensity and exposure policies.

For Rotting-Visuals, these constraints apply in addition to the generic monster-mode authoring rules.

---

# 2. Monster-mode / Rotting-Visuals specialization

Rotting-Visuals is a **style pack** layered on top of the generic monster-mode experienceType:

- `experienceType` MUST be `"monster-mode"`.
- `infectionProfile.style` SHOULD be `"shambler"` or `"seer"` for default Rotting-Visuals behavior, or `"runner"` for more jagged variants.
- `infectionProfile.durationHintSec` SHOULD be in the 10–120 second range for lab scenarios.
- `infectionProfile.coopEmphasis` SHOULD usually be `"squad-first"` to preserve squad readability unless a specific solo-focused experiment is requested.

Style-specific expectations:

- Visual focus:
  - Necrotic tunnel vision: shrinking `maskRadius`, strong vignette via `maskFeather`.
  - Edge desaturation and granular decay via `colorDesat` and `decayGrain`.
  - Occasional vein-like overlays via `veinOverlay`, but with strict caps.
- Audio focus:
  - Emphasis on low-frequency body pressure (`heartbeatGain`, `breathGain`).
  - Mild squad channel muffling (`squadMuffle`) when infection conditions are met.
  - Controlled ringing (`ringingLevel`) that never spikes abruptly.
- Haptic focus:
  - Slow, weighted `chestPressure`.
  - Creeping `spineCrawl` at moderate intensity.
  - Low-amplitude `tremorRate` for subtle infection “ticks”.

Bindings for this style pack SHOULD be placed under a dedicated style path, for example:

- `examples/bci/styles/Rotting-Visuals/bci-geometry-bindings.rotting-visuals.lab.sample.json`

---

# 3. BCI surface you may use

Monster-mode (including Rotting-Visuals) bindings may only consume canonical BciSummary and invariant fields already exposed to Lua and Rust geometry kernels.

Allowed BciSummary fields:

- `stressScore`
- `stressBand`
- `attentionBand`
- `visualOverloadIndex`
- `startleSpike`
- `signalQuality`

Allowed invariants for gating and region classification:

- `CIC`
- `AOS`
- `DET`
- `LSG`

Allowed entertainment metrics as optional gates:

- `UEC`
- `EMD`
- `STCI`
- `CDL`
- `ARR`

Prohibitions:

- Do not invent new metric names.
- Do not read raw biosignal features or device-specific data.
- Do not bypass the existing envelopes (feature, metrics, intensity).

---

# 4. Allowed outputs for monster-mode (Rotting-Visuals)

All outputs are normalized in `[0,1]` and are interpreted by engine helpers such as `H.Visual.applyBciMask`, `H.Audio.applyBciRtpcs`, and `H.Haptics.routeHaptics`.

Visual outputs:

- `maskRadius`
- `maskFeather`
- `decayGrain`
- `colorDesat`
- `veinOverlay`
- `motionSmear`

Rotting-Visuals emphasis:

- Use `maskRadius` and `maskFeather` to produce a constricting, necrotic tunnel under high stress and focused attention.
- Use `colorDesat` and `decayGrain` to flatten and rot the periphery, keeping central vision more intact.
- Use `veinOverlay` sparingly and with strong caps to avoid visual overload.
- Use `motionSmear` only at low-to-medium levels to suggest sluggish perception, not full blur.

Audio outputs:

- `infectedChannelGain`
- `squadMuffle`
- `heartbeatGain`
- `breathGain`
- `ringingLevel`
- `directionBias`

Rotting-Visuals emphasis:

- Use `heartbeatGain` and `breathGain` as the primary “body / hunger” channels.
- Use `squadMuffle` to gently attenuate ally chatter when BciSummary suggests infection perspective, but never fully silence.
- Use `infectedChannelGain` and `directionBias` to tilt attention toward nearby threats.
- Use `ringingLevel` only as a soft stress ring, capped and smoothed.

Haptic outputs:

- `chestPressure`
- `spineCrawl`
- `tremorRate`

Rotting-Visuals emphasis:

- `chestPressure` should track cumulative tension in a slow, damped way.
- `spineCrawl` should appear as intermittent pulses, more common in high-DET corridors.
- `tremorRate` should remain low to avoid fatigue and motor interference.

AI-authored bindings must not target engine-specific fields or reference internal RTPC names directly; they only set these normalized channels.

---

# 5. ExperienceType, style, and routing fields

Requests targeting Rotting-Visuals MUST satisfy:

- `experienceType`: `"monster-mode"`.
- `targetRepo`: SHOULD be `"HorrorPlace-Neural-Resonance-Lab"` for lab packs or `"Death-Engine"` for engine examples.
- `targetPath`: SHOULD point to a Rotting-Visuals style path, for example:

  - `examples/bci/styles/Rotting-Visuals/bci-geometry-bindings.rotting-visuals.lab.sample.json`

Style routing hints can be carried either via:

- `infectionProfile.style`: `"shambler"` or `"seer"` preferred.
- Optional `meta.stylePackId`: `"monster-mode.rotting-visuals.v1"` in the response `meta` block.

---

# 6. Region classes and binding IDs

Monster-mode bindings must declare region classes from the canonical taxonomy used by BCI geometry bindings:

Recommended regionClass values for Rotting-Visuals:

- `corridor-highCIC-highAOS-build`
- `threshold-highDET-apex`
- `safe-lowDET-cooldown`
- `infection-hub`
- `infection-approach`
- `infection-retreat`

Binding IDs must follow:

- `monster-mode.rotting-visuals.<regionClass>.<flavor>.<version>`

Examples:

- `monster-mode.rotting-visuals.corridor-highCIC-highAOS-build.shambler.v1`
- `monster-mode.rotting-visuals.threshold-highDET-apex.runner.v1`
- `monster-mode.rotting-visuals.safe-lowDET-cooldown.seer.v1`

Region classes are inferred at runtime from invariants.
Authoring-time instructions may assume these semantics but must not invent new `regionClass` strings without a schema update.

---

# 7. BCI filter slices for infection windows

Each monster-mode binding declares a `bciFilter` that defines when it is eligible to activate.
For Rotting-Visuals, use infection windows that emphasize high stress and focused attention, while avoiding fully overloaded states.

Recommended slices:

- `stressBandAllowed`: include `"High"` and/or `"Extreme"`.
- `attentionBandAllowed`: include `"Focused"` and/or `"HyperFocused"`.
- `visualOverloadMin`: typically between `0.2` and `0.7`.
- `visualOverloadMax`: typically between `0.5` and `0.8`, leaving headroom above.
- `signalQualityAllowed`: include `"Good"`; `"Degraded"` may be allowed for lab-only experiments, but bindings must degrade gracefully.

Authoring rules:

- Always specify `bciFilter` with explicit ranges.
- Never activate monster-mode when `signalQuality` is `"Unavailable"`.
- For Rotting-Visuals safe-room bindings, narrow the stress bands and tighten `visualOverloadMax` to encourage recovery.

Optional entertainment metric gates for Rotting-Visuals:

- Use `UEC` and `ARR` to prevent monster-mode from activating in already saturated threat states.
- Use `CDL` and `STCI` as soft guards to avoid pushing confusion and tension beyond policy thresholds.

---

# 8. Curve families and parameter expectations

Monster-mode output curves must use only the approved curve families implemented in the Rust geometry kernel:

- `linear`
- `sigmoid`
- `hysteresis`
- `noise`

Each curve:

- References a single input from the allowed BciSummary fields.
- Declares an `outputRange` in `[0,1]`.
- Uses parameter vectors that stay within schema and CI bounds.

Rotting-Visuals guidelines:

- `sigmoid`:
  - Use for tunnel onset and recovery on `maskRadius`, `colorDesat`, and `chestPressure`.
  - Choose midpoints in the mid stress band and avoid cliffs near band boundaries.
- `hysteresis`:
  - Use for stabilizing infection windows for `maskRadius`, `infectedChannelGain`, and `chestPressure`.
  - Ensure separate rising/falling behavior to prevent rapid mode flipping.
- `noise`:
  - Use only for `decayGrain` and `motionSmear` at low amplitude.
  - Do not attach noise directly to channels that control loudness spikes or full-screen occlusions.
- `linear`:
  - Use for gentle ramps on `squadMuffle`, `heartbeatGain`, and `spineCrawl` where simple proportional response is sufficient.

Always set `outputRange.min` and `outputRange.max` to stay within `[0,1]` and within the caps implied by the active safety profile.

---

# 9. Safety profiles and neurorights

Monster-mode bindings must reference dedicated safety profiles such as:

- `monster-mode-standard`
- `monster-mode-safe-room`

For Rotting-Visuals:

- Use `monster-mode-standard` for corridor and threshold bindings.
- Use `monster-mode-safe-room` for safe-room and cooldown bindings.

Safety expectations:

- Visual caps: limit maximum tunnel strength and decay severity so that center vision is never fully removed.
- Audio caps: enforce conservative `maxAudioDeltaPerSecond`, especially for `ringingLevel` and `infectedChannelGain`.
- Haptic caps: limit `chestPressure` and `tremorRate` to prevent fatigue or discomfort.
- DET/CSI caps: infection windows must respect dread and cumulative session exposure limits.

AI must not:

- Invent new `profileId` values.
- Attempt to circumvent caps by proposing out-of-range curve parameters.
- Reduce or remove safety constraints encoded in the profiles.

---

# 10. Required request shape (Rotting-Visuals, ai-bci-geometry-request-v1)

A lab-oriented authoring request for this style SHOULD look like:

```json
{
  "schemaVersion": "1.0.0",
  "targetRepo": "HorrorPlace-Neural-Resonance-Lab",
  "targetPath": "examples/bci/styles/Rotting-Visuals/bci-geometry-bindings.rotting-visuals.lab.sample.json",
  "bindingSchemaId": "bci-geometry-binding-v1",
  "regionHints": {
    "regionClass": "corridor-highCIC-highAOS-build",
    "style": "Rotting-Visuals",
    "sampleInvariants": {
      "cic": 0.7,
      "aos": 0.4,
      "det": 4.5,
      "lsg": 0.3
    }
  },
  "experienceType": "monster-mode",
  "infectionProfile": {
    "style": "shambler",
    "durationHintSec": 60,
    "coopEmphasis": "squad-first"
  },
  "constraints": {
    "allowedTiers": ["lab"],
    "allowedSafetyProfiles": ["monster-mode-standard"],
    "maxCurveComplexity": "medium"
  }
}
```

The authoring engine may add additional governance fields (such as change tickets or policy references), but AI-facing tooling must treat the above as the minimal viable shape.

---

# 11. Required response shape (ai-bci-geometry-response-v1)

The assistant MUST respond with a single JSON object shaped as:

```json
{
  "schemaVersion": "1.0.0",
  "bindings": [
    /* one or more bci-geometry-binding-v1 objects */
  ],
  "meta": {
    "authorAgent": "ai-chat",
    "stylePackId": "monster-mode.rotting-visuals.v1",
    "notes": "Short explanation of the Rotting-Visuals binding intent.",
    "createdAt": "2026-01-01T00:00:00Z"
  }
}
```

Requirements:

- Each element of `bindings` MUST validate against `bci-geometry-binding-v1.json`.
- Each binding MUST:
  - Have a `bindingId` following the Rotting-Visuals naming pattern.
  - Declare a `regionClass` from the allowed taxonomy.
  - Declare a `bciFilter`.
  - Reference a valid `safetyProfileId` from `bci-safety-profile-v1.json`.
  - Provide `outputCurves.visual`, `outputCurves.audio`, and `outputCurves.haptics` as needed.

---

# 12. Lua and runtime expectations

Runtime behavior is unchanged from generic monster-mode:

- Lua code calls `BCI.getSummary(playerId)` to obtain BciSummary.
- Lua code calls `BciGeometry.sample(playerId, regionId, tileId)` to evaluate bindings, including Rotting-Visuals ones.
- Binding selection may be cached per `(playerId, regionId, tileId)` to avoid thrashing.
- Debug overlays may present:
  - Current `stressBand`, `attentionBand`, `visualOverloadIndex`, `signalQuality`.
  - Selected `bindingId` and `profileId`.
  - Current state such as `NORMAL`, `RECOVERY`, or `SAFEZONE`.

Rotting-Visuals bindings must assume:

- They are a skin on top of existing BCI safety and intensity logic.
- They can be disabled or clamped by upstream intensity policies when caps are reached.
- They should degrade gracefully (e.g., fade out masks, reduce audio emphasis) when signal quality drops or recovery states engage.

---

# 13. AI-chat authoring prompt stub (Rotting-Visuals)

The following stub can be embedded in AI-chat configs for Rotting-Visuals:

> You are authoring BCI geometry bindings for the experienceType `"monster-mode"` using the Rotting-Visuals style pack.
>
> You must:
> - Accept an `ai-bci-geometry-request-v1` request where `experienceType` is `"monster-mode"` and `regionHints.style` is `"Rotting-Visuals"`.
> - Produce an `ai-bci-geometry-response-v1` response with a `bindings` array of `bci-geometry-binding-v1` objects.
> - Use only BciSummary fields: `stressScore`, `stressBand`, `attentionBand`, `visualOverloadIndex`, `startleSpike`, `signalQuality`.
> - Use only invariants: `CIC`, `AOS`, `DET`, `LSG`, and optional entertainment metrics `UEC`, `EMD`, `STCI`, `CDL`, `ARR` as gates.
> - Target only these outputs:
>   - Visual: `maskRadius`, `maskFeather`, `decayGrain`, `colorDesat`, `veinOverlay`, `motionSmear`.
>   - Audio: `infectedChannelGain`, `squadMuffle`, `heartbeatGain`, `breathGain`, `ringingLevel`, `directionBias`.
>   - Haptics: `chestPressure`, `spineCrawl`, `tremorRate`.
> - Use curve families: `linear`, `sigmoid`, `hysteresis`, `noise`.
> - Reference safety profiles: `monster-mode-standard` or `monster-mode-safe-room`.
>
> You must not:
> - Introduce any new metric names or engine-specific fields.
> - Exceed safety caps encoded in safety profiles.
> - Activate monster-mode when `signalQuality` is `"Unavailable"`.
>
> For Rotting-Visuals, you should:
> - Emphasize necrotic tunnel vision and edge decay in visuals.
> - Emphasize low-frequency body pressure and mild squad muffling in audio.
> - Emphasize chest and spine pressure with low tremor intensity in haptics.
> - Keep all outputs in `[0,1]` and use `bciFilter` slices that require high stress and focused attention, with mid-band visual overload.

---

# 14. Next wiring actions

Once this document is committed:

- Ensure `ai-bci-geometry-request-v1.json` and `ai-bci-geometry-response-v1.json` are present under `schemas/ai/` and wired into AI-chat orchestration for BCI work.
- Add a Rotting-Visuals example binding pack at:
  - `examples/bci/styles/Rotting-Visuals/bci-geometry-bindings.rotting-visuals.lab.sample.json`
- Add a lab corridor replay scene that exercises Rotting-Visuals bindings and logs BciSummary, selected binding IDs, and applied safety caps for offline analysis.
