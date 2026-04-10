# Seed-Ritual Cross-Stage Audiovisual Intensity Envelopes

## 1. Purpose and Position

This document specifies the cross-stage audiovisual intensity envelope layer for Seed-Rituals. It refines the existing `stageIntensityEnvelopes` bands in `seed-ritual-contract-v1.json` by introducing a focused, implication-only `min_intensity_envelope` concept, a style-coupling strategy, and runtime enforcement rules that bind DreadForge atmosphere profiles, Seed styles, and Director decisions into a single contract.

The goal is to guarantee non-zero atmospheric pressure across Probe → Evidence → Confrontation → Aftermath → Residual, without encoding descriptive horror content. Engines see only numeric floors and references to style profiles that are known to maintain audio/visual presence.

---

## 2. Schema Shape: `min_intensity_envelope`

For v1, the intensity layer is represented by a new property in the ritual body:

```json
"min_intensity_envelope": {
  "probe":        { "min_audio_pressure": 0.15, "min_visual_presence": 0.10 },
  "evidence":     { "min_audio_pressure": 0.30, "min_visual_presence": 0.25 },
  "confrontation": {
    "min_audio_pressure": 0.60,
    "min_visual_presence": 0.55,
    "max_DET_delta": 4.0
  },
  "aftermath":    { "min_audio_pressure": 0.25, "min_visual_presence": 0.20 },
  "residual":     { "min_audio_pressure": 0.10, "min_visual_presence": 0.05 }
}
```

Recommended JSON Schema fragment for `seed-ritual-contract-v1.json`:

```json
"min_intensity_envelope": {
  "type": "object",
  "description": "Minimum audiovisual intensity floors per stage, expressed as normalized bands.",
  "additionalProperties": false,
  "required": [
    "probe",
    "evidence",
    "confrontation",
    "aftermath",
    "residual"
  ],
  "properties": {
    "probe":        { "$ref": "#/definitions/minIntensityStage" },
    "evidence":     { "$ref": "#/definitions/minIntensityStage" },
    "confrontation":{ "$ref": "#/definitions/minIntensityStageWithDet" },
    "aftermath":    { "$ref": "#/definitions/minIntensityStage" },
    "residual":     { "$ref": "#/definitions/minIntensityStage" }
  }
}
```

With supporting definitions:

```json
"minIntensityStage": {
  "type": "object",
  "additionalProperties": false,
  "required": ["min_audio_pressure", "min_visual_presence"],
  "properties": {
    "min_audio_pressure": {
      "type": "number",
      "minimum": 0.0,
      "maximum": 1.0
    },
    "min_visual_presence": {
      "type": "number",
      "minimum": 0.0,
      "maximum": 1.0
    }
  }
},
"minIntensityStageWithDet": {
  "allOf": [
    { "$ref": "#/definitions/minIntensityStage" },
    {
      "type": "object",
      "properties": {
        "max_DET_delta": {
          "type": "number",
          "minimum": 0.0
        }
      }
    }
  ]
}
```

These values are floors, not targets. Engines may exceed them but must never drop below them once a Seed is instantiated under the ritual for that stage.

---

## 3. Style Coupling: DreadForge and Allowed Profiles

To make intensity floors operational instead of aspirational, Seed-Rituals must constrain the style layer. This is accomplished by adding an optional style coupling block:

```json
"style_binding": {
  "allowed_style_refs": [
    "dreadforge:rumble_threshold_v1",
    "dreadforge:shader_distortion_high_v1"
  ],
  "required_channels": [
    "audio_rumble",
    "audio_hiss",
    "visual_distortion"
  ]
}
```

Conceptual JSON Schema fragment:

```json
"style_binding": {
  "type": "object",
  "description": "Coupling between ritual intensity floors and DreadForge/Style profiles.",
  "additionalProperties": false,
  "required": ["allowed_style_refs"],
  "properties": {
    "allowed_style_refs": {
      "type": "array",
      "items": { "type": "string" },
      "minItems": 1,
      "uniqueItems": true
    },
    "required_channels": {
      "type": "array",
      "items": {
        "type": "string",
        "enum": [
          "audio_rumble",
          "audio_hiss",
          "audio_static",
          "visual_distortion",
          "visual_grain",
          "visual_particles"
        ]
      },
      "uniqueItems": true
    }
  }
}
```

Constraints for CI:

- Any Seed bound to a ritual must reference at least one style profile in `allowed_style_refs`.
- Style schemas in Spectral-Foundry must declare per-channel baseline intensities; CI checks that those baselines satisfy the ritual’s `min_intensity_envelope` for the relevant stage.
- If a style cannot meet the floor (for any `required_channels`), the Seed–Ritual binding is rejected.

This yields a verifiable chain: ritual → style IDs → style baselines → engine RTPCs, with no prose.

---

## 4. Runtime Enforcement in `H.shouldtriggersequence`

At runtime, `H.shouldtriggersequence(regionId, tileId, playerState)` must extend its decision pipeline with audiovisual checks:

1. Select a candidate Seed `s` and its parent ritual `R` and stage `k`.
2. Resolve the effective style mix for this Seed and stage (from Seed, biome, and ritual style_binding).
3. Compute channel intensities:

   - `a_rumble(s,R)`, `a_hiss(s,R)`, etc. for audio.
   - `v_distort(s,R)`, `v_grain(s,R)`, etc. for visuals.

4. Look up ritual floors for stage `k`:

   - `a_rumble_min(R,k)`, `a_hiss_min(R,k)` derived from `min_audio_pressure`.
   - `v_distort_min(R,k)`, `v_presence_min(R,k)` derived from `min_visual_presence`.

5. Enforce inequalities (conceptually):

   - `a_channel(s,R) ≥ a_channel_min(R,k)` for all configured audio channels.
   - `v_channel(s,R) ≥ v_channel_min(R,k)` for all configured visual channels.

If any inequality fails, the Seed is rejected for this stage and context, and the Director must:

- Try another eligible Seed in the same ritual and stage, or
- Defer stage progression until context allows a compatible Seed.

The same function already enforces entertainment floors and GXI caps. Intensity enforcement becomes a parallel set of numeric guards, not an extra decision plane.

---

## 5. Eligibility Scoring and DET Budget

To prioritize high-impact but budget-compliant Seeds, Directors can compute a scalar eligibility score:

- Inputs:

  - Current metrics (e.g., `STCI_cur`, `CDL_cur`).
  - Seed deltas (`ΔSTCI(s)`, `ΔCDL(s)`, `ΔDET(s)`).
  - Ritual floors per stage (`m_STCI^{R,k}`, `m_CDL^{R,k}`).
  - DET budget (`B_DET`) and accumulated DET (`C_DET`).
  - Stage weight (`w_s`) and tuning weights (`λ_1`, `λ_2`, `λ_3`).

- Behaviour:

  - Positive contributions for closing gaps between current metrics and ritual floors.
  - Negative contributions for overshooting DET budgets.

Seeds with `E_{R,k}(s) ≤ 0` still must satisfy all hard ritual constraints to be eligible; the score is used only for ranking within the eligible set.

---

## 6. Telemetry: Intensity Compliance Metrics

To prove the envelope is working and to tune future rituals:

- Instrument telemetry to record, per Seed activation:

  - `ritualId`, `stage`, `seedId`.
  - Projected vs realized metric deltas.
  - Channel intensities at fire time.
  - Booleans: `intensity_floor_met`, `entertainment_floor_met`.

- Define, per ritual `R`, a satisfaction ratio `ρ_R`:

  - Fraction of activations that meet all metric and intensity floors.

Recommended CI gate:

- For public or operational tiers, maintain `ρ_R` above a configured threshold over a rolling window.
- If `ρ_R` drops below the threshold, flag the ritual for human review or auto-retuning of floors, style bindings, or DET caps.

This closes the loop: contract → Seed/style selection → runtime enforcement → telemetry → contract updates.

---

## 7. Integration Roadmap

To fully activate this pillar:

1. Update `seed-ritual-contract-v1.json` to include `min_intensity_envelope` and `style_binding` definitions and add minimal structural checks.
2. Extend the existing Seed-Ritual linter to:

   - Verify presence and ranges of `min_intensity_envelope`.
   - Cross-check allowed styles against style metadata once those schemas expose baseline intensities.

3. Extend `H.shouldtriggersequence` and any engine-specific adapters to compute and enforce channel-level intensity floors per ritual stage.
4. Expand `entertainmentmetricsv1` telemetry to carry intensity compliance flags and per-channel realized values to support `ρ_R` and related analytics.

This keeps v1 compact but end-to-end: authors must provide a full cross-stage intensity envelope from day one, engines must respect it numerically, and telemetry must track how well reality matches the envelope.
