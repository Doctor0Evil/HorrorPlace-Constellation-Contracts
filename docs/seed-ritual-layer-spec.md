# Seed-Ritual Layer Specification (Tier-1)

## 1. Purpose and Position in the Spine

The Seed-Ritual layer defines a thin, contract-first envelope above `seedContractV1`. It does not replace or duplicate Seed contracts. Instead, it binds one or more Seeds into a five-stage horror arc (Probe, Evidence, Confrontation, Aftermath, Residual) and adds three things the Seed layer does not:

- Minimum entertainment bands per stage and over a session-scale envelope.
- Audiovisual (and optional haptic/BCI) intensity envelopes per stage.
- GXI and consent-tier governance bands that constrain gore/violence by implication-only rules.

In the VM-constellation spine:

- `seedContractV1` is the tile-scale, history-bound unit (invariants, metric deltas, engine profile, Dead-Ledger hooks).
- `seed-ritual-contract-v1` is a Tier-1 orchestration contract that:

  - References Seeds by `seedId`.
  - Declares minimum UEC/EMD/STCI/CDL/ARR bands per stage.
  - Declares per-stage audio/visual envelopes.
  - Caps implied gore/violence via `gxiCap` and `consentTier`.

- The resurrection spine (`resurrection-spine.v1.json` and related contracts) sits orthogonally and can optionally be attached via `resurrectionProfileId` to define how rituals may evolve or be reused over time.

Seed-Rituals are declarative. They never contain descriptive horror content, assets, or scripts. Engines, AI, and tooling consume them purely as numeric and referential constraints.

---

## 2. Schema Overview: seed-ritual-contract-v1.json

The canonical schema lives at:

- `schemas/seed-ritual-contract-v1.json`
- `$id: https://horror.place/schemas/seed-ritual-contract-v1.json`

At a high level, the contract includes:

- Identity and wiring:

  - `schema`: fixed URI for validator resolution.
  - `ritualId`: unique ritual identifier.
  - `version`: semantic version.
  - `targetRepo`, `path`: where the ritual contract lives in Git.
  - `deadledgerRef` (optional): Dead-Ledger entry for high-intensity / high-GXI rituals.

- Governance:

  - `consentTier`: `Tier1Public | Tier2Internal | Tier3Vault`.
  - `gxiCap`: scalar cap on implied gore/violence intensity (0–10).
  - `explicitViolenceForbidden`: boolean flag mirroring Seed and style layers.
  - `implicationStyles` (optional): hints like `offscreen`, `aftermath_only`, `symbolic`, `medicalized`, etc.

- Resurrection hooks (optional):

  - `resurrectionProfileId`: reference into the resurrection spine.
  - `resurrectionFreshnessTarget`: normalized novelty/freshness target (0–1).

- Seed bindings:

  - `stageBindings`: array of `{ stage, seedId, weight?, priority? }` mapping the five ritual stages to one or more `seedContractV1` Seeds.

- Entertainment metric floors:

  - `stageMetricBands`: per-stage bands for UEC, EMD, STCI, CDL, ARR.
  - `sessionEnvelope`: optional time-windowed bands across the session.

- Audiovisual envelopes:

  - `stageIntensityEnvelopes`: per-stage audio/visual intensity bands (and optional haptic/BCI hints).

The schema enforces type and range correctness. Cross-object constraints (such as continuity across stages, or GXI vs consent tier consistency) are enforced by separate linters and CI rules.

---

## 3. Relationship to SeedContractV1

### 3.1. What Seeds own

`seedContractV1` (Tier-1) remains the authoritative contract for:

- Identity: `seedid`, `regionid`, `tileid`, `version`.
- History coupling: `bundleref`, invariant vector at tile scope.
- Entertainment intent: `metrictargets` with UEC/EMD/STCI/CDL deltas and ARR bands.
- Safety and intensity: `safetytier`, `intensityband`, `explicitviolenceforbidden`.
- Persona and narrative affordances: `personahooks`, `narrativeaffordances`.
- Engine profile: trigger cooldowns, max triggers per session, required player state.
- Governance: optional `deadledgerref`, `charterprofileid`, `validatorsignature`.

Seeds are small, tile-local contracts whose metric deltas and invariants are already validated against global spines.

### 3.2. What Seed-Rituals add

Seed-Rituals do not duplicate any of the above. They:

- Group Seeds into a staged arc via `stageBindings`.
- Require per-stage minimum metric bands via `stageMetricBands`:
  - Each stage has `UEC_min`, `EMD_min`, `STCI_min`, `CDL_min`, `ARR_min`, `ARR_max`.
- Optionally define session-scale target envelopes via `sessionEnvelope`.
- Define per-stage audio/visual envelopes via `stageIntensityEnvelopes`.
- Add GXI and consent governance via `consentTier`, `gxiCap`, `explicitViolenceForbidden`, and `implicationStyles`.

Seeds remain the “what” at tile level. Rituals define the “how much, at minimum, and how intense across stages” for a particular arc.

### 3.3. CI cross-checks between Seeds and Rituals

CI for Constellation-Contracts must perform the following checks whenever a Seed-Ritual is authored or updated:

1. Schema validation:

   - Validate the ritual JSON against `seed-ritual-contract-v1.json`.
   - Validate all referenced Seeds against `seedContractV1`.

2. Stage → Seed compatibility:

   For each `stageBinding`:

   - Resolve `seedId` to a Seed.
   - Check that the Seed’s `stage` matches the ritual’s `stage` semantics (e.g., `stage3confrontation` for `confrontation`), or at least is consistent under your mapping policy.
   - Estimate whether the Seed’s `metrictargets` can plausibly achieve or exceed the ritual’s `stageMetricBands` floors for that stage without violating ARR bands.

   A minimal implementation can treat this as:

   - If `UECdelta` is non-positive and the stage’s `UEC_min` is significantly above the current design baseline, flag `UNDERPOWERED_STAGE`.
   - Similar checks for `EMD`, `STCI`, `CDL`.
   - If a Seed’s `ARRmin` / `ARRmax` are incompatible with the ritual’s ARR band, flag `INCOMPATIBLE_ARR_BAND`.

3. GXI and consent:

   - Ensure `gxiCap` lies within allowed ranges per `consentTier` (e.g., stricter caps for `Tier1Public`).
   - Ensure `explicitViolenceForbidden` is `true` for `Tier1Public` and `Tier2Internal` rituals.
   - Lint against any known style or biome profiles (if resolved in the same CI run) to catch mismatches between `gxiCap` and attached style contracts.

4. Cross-stage continuity:

   - Verify that audio/visual intensity envelopes are monotone or staircase-shaped from Probe → Evidence → Confrontation, with allowed falloff in Aftermath and Residual.
   - Ensure no stage intensity band is empty or inverted (`min` ≤ `max`).

Any violation should fail CI and prevent the ritual from being merged or promoted.

---

## 4. Engine Consumption (Lua / Rust / Engine Adapters)

Seed-Rituals are meant to be consumed by engines through a narrow facade, not via ad-hoc JSON parsing.

### 4.1. Recommended Lua facade

A minimal Lua-facing API (conceptually under `HRituals`) might expose:

- `HRituals.load(ritualId) -> ritual`  
  Returns a ritual object with read-only fields mirroring the schema.

- `HRituals.for_stage(ritualId, stage) -> stageBindings, stageBands, intensity`  
  Returns the bindings, metric bands, and intensity envelope for a given stage.

- `HRituals.effective_metric_floor(ritualId, stage, sessionState) -> bands`  
  Computes the effective minimum bands at this point in the session, combining `stageMetricBands` with any `sessionEnvelope` windows that overlap the current session time.

- `HRituals.effective_intensity(ritualId, stage, tInStage) -> audioTarget, visualTarget`  
  Computes normalized audio/visual target intensities based on the stage envelope and current time within the stage. Engines map these onto RTPCs or equivalent.

- `HRituals.governance(ritualId) -> consentTier, gxiCap, flags`  
  Exposes governance information (consent, GXI cap, `explicitViolenceForbidden`, `implicationStyles`).

These calls should be implemented as thin adapters over preloaded ritual JSON and should never expose internal Dead-Ledger or archival data.

### 4.2. Integration with H.shouldtriggersequence

`H.shouldtriggersequence(regionId, tileId, playerState)` already performs:

1. Invariant sampling and Seed filtering.
2. Metric intent evaluation.
3. Stage logic and pacing.

The Seed-Ritual layer extends this pipeline:

1. Seed selection remains as-is but is annotated with its parent `ritualId` and `stage`.

2. Before finalizing a candidate Seed:

   - Query `HRituals.for_stage(ritualId, stage)` and `HRituals.effective_metric_floor`.
   - Use a small runtime metric predictor (e.g., approximate application of `metrictargets` against current metrics) to determine whether firing this Seed keeps UEC/EMD/STCI/CDL above the ritual’s minima and ARR within the allowed band.
   - If not, either:

     - Select another eligible Seed in the same ritual stage, or
     - Delay the stage and keep searching on future ticks/tiles.

3. For audiovisual output:

   - Use `HRituals.effective_intensity` to derive normalized `audioTarget` and `visualTarget` scalars.
   - Feed those into engine-specific audio and VFX controllers as RTPCs or intensity weights.

Rituals thus become hard constraints. A Seed that cannot support the ritual’s floor in the current context should be treated as ineligible for that stage at that time.

---

## 5. Dead-Ledger and Governance Integration

### 5.1. When to require deadledgerRef

The schema treats `deadledgerRef` as optional to keep v1 flexible. Governance policy must tighten this:

- For `consentTier = Tier1Public`:

  - `deadledgerRef` is optional; `gxiCap` must remain below a conservative threshold.
  - Rituals may still reference Seeds that carry their own `deadledgerref` where required by intensity.

- For `consentTier = Tier2Internal` or `Tier3Vault`:

  - Any ritual with `gxiCap` or implied intensity above configured thresholds (e.g., `gxiCap >= 4` or stages with high audio/visual intensities) must carry a non-null `deadledgerRef`.
  - The referenced ledger entry should attest:

    - `ritualId` and ritual hash.
    - The set of `seedId`s bound via `stageBindings`.
    - A compliance report hash confirming:

      - Ritual bands are within global policy.
      - All Seeds validate and respect Charter rules.
      - GXI and implication doctrine are satisfied.

Dead-Ledger proofs remain sealed at Tier-3. Tier-1 only sees `deadledgerRef` tokens and simple pass/fail decisions from a proof-verification service.

### 5.2. Consent and GXI policy

Governance documents (outside this schema) should define, per `consentTier`:

- Allowed ranges for `gxiCap`.
- Any additional constraints on `stageMetricBands` (e.g., minimal ARR floors).
- Required proof types in the ledger (age gating, Charter acknowledgements, research-only flags).

CI for Constellation-Contracts should:

- Check that each ritual’s `gxiCap` lies within its tier’s policy.
- Optionally cross-reference style/biome GXI profiles when available.
- Warn or fail if a ritual’s `stageMetricBands` are inconsistent with policy (e.g., ARR too low for public tiers).

---

## 6. Composition with the Resurrection Spine

The resurrection spine defines how resurrected contracts (Seeds, regions, personas) must differ from their ancestors and how novelty bands, metric bands, and safety caps are enforced.

Seed-Rituals integrate with this spine via:

- `resurrectionProfileId`: linking rituals to resurrection profiles that define:

  - Novelty bands (distance in invariant/metric space).
  - Metric bands and safety caps.
  - Maximum resurrection counts per lineage.

- `resurrectionFreshnessTarget`: per-ritual target for how “fresh” a resurrected arc should feel.

Recommended usage:

- For v1, treat these fields as optional at the schema level but enforce them in governance policy for:

  - High-intensity rituals.
  - Research or experimental tiers.

- When a ritual is resurrected (v2, v3, etc.):

  - CI must check that:

    - `stageMetricBands` and `stageIntensityEnvelopes` are not trivial clones of the ancestor.
    - GXI and consent tiers still respect global policy.
    - The resurrection profile’s novelty thresholds are met.

The resurrection spine remains the canonical source of novelty rules; the ritual layer merely carries references and a target freshness scalar.

---

## 7. Telemetry and Feedback Loops

To close the loop between design and runtime behavior, telemetry schemas should treat `ritualId` as a first-class key:

- For each Seed activation:

  - Log `ritualId` (if present), `stage`, `seedId`, `regionId`, `tileId`.
  - Record pre- and post-event UEC/EMD/STCI/CDL/ARR.
  - Record realized audio/visual intensities (and haptic/BCI signals where available).

- For each session:

  - Aggregate metric traces by `ritualId` and stage.
  - Compare realized curves against:

    - `stageMetricBands`.
    - `sessionEnvelope` windows.
    - `stageIntensityEnvelopes`.

Offline tools can then:

- Compute over/under-shoot statistics for each ritual.
- Suggest adjustments to `stageMetricBands` and intensity envelopes.
- Feed into resurrection profiles and future versions of the ritual.

This telemetry loop should mirror the existing Seed-layer patterns: deploy, observe, refine, but now at the higher granularity of multi-Seed arcs.

---

## 8. Authoring Guidelines and CI Rules

### 8.1. Authoring guidelines

Authors should:

- Start from templates (`templates/seed-ritual.template.json`) that:

  - Pre-fill the five stages.
  - Provide safe default metric floors and intensity envelopes for common ritual families.

- Choose a small, coherent set of Seeds per ritual:

  - Prefer Seeds bound to the same or closely related bundles and regions.
  - Ensure stage semantics roughly match Seed stages.

- Set metric floors:

  - Use conservative but meaningful UEC/EMD/STCI/CDL minima for each stage.
  - Keep ARR bands consistent with the intended ambiguity pattern (high in Probe/Evidence, carefully controlled in Confrontation, non-zero in Residual).

- Set intensity envelopes:

  - Follow a clear rising shape into Confrontation.
  - Allow Aftermath and Residual to fall but not collapse to zero.

- Choose `consentTier`, `gxiCap`, and `implicationStyles` in line with existing Charter and gore governance doctrine.

### 8.2. CI rules (summary)

Reusable CI jobs for Constellation-Contracts should:

1. Validate all ritual contracts against `seed-ritual-contract-v1.json`.

2. For each ritual:

   - Resolve all referenced Seeds and assert they exist and validate.
   - Run stage-level checks for underpowered or incompatible Seeds.
   - Enforce GXI and consent tier policy.
   - Enforce cross-stage continuity constraints.
   - Optionally verify that any non-null `deadledgerRef` matches ledger schemas.

3. Deny merges or promotions when:

   - Schema validation fails.
   - Seeds are missing or invalid.
   - Metric floors cannot be met by referenced Seeds.
   - GXI, consent tier, or intensity policies are violated.

Seed-Rituals are thus treated as first-class, machine-audited contracts in the same way as Seeds, events, regions, and resurrection profiles.

---

## 9. Roadmap and Extensions

Future versions of the Seed-Ritual schema and spec may:

- Add explicit references to gore/violence style governance profiles, once GXI is formalized there.
- Tighten integration with BCI envelopes and safety spines.
- Add explicit compatibility hooks with surprise mechanic contracts and DreadForge mood stacks.

v1 keeps the core commitments small and auditable:

- Bind Seeds into five-stage arcs.
- Enforce numeric metric floors.
- Enforce numeric audiovisual envelopes.
- Enforce GXI and consent caps.
- Provide hooks for Dead-Ledger and resurrection without over-committing.

Engines and vaults that implement this spec can rely on Seed-Rituals as a compact policy layer that guarantees “no low-impact filler” and “no free-floating splatter,” while staying fully implication-only and GitHub-safe.
