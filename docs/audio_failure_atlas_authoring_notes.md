# Audio Failure Atlas – Authoring Notes (Tier‑1)

This document gives concrete, numeric examples for three common audio failure patterns – “flat corridor”, “lifeless firefight”, and “jump‑scare spam” – and shows how they are expressed against the invariant and metrics schemas and enforced by the linters bound to mood, audio style, and mapping‑family contracts.

The goal is to make these failures checkable, not just aesthetic. Every example below is written so that:

- Invariants are drawn from the canonical spine (CIC, MDI, AOS, RRM, HVF, LSG, SHCI, DET).  
- Entertainment metrics come from the standard schemas (UEC, EMD, STCI, CDL, ARR).  
- Audio behavior is expressed via moodcontractv1, audiolandscapestylev1, and audiortpc‑mapping‑family‑v1.json, plus audio telemetry schemas.  
- Linters have explicit numeric predicates they can encode and flag in CI.

---

## 1. “Flat Corridor” – Underdriven Atmosphere

### 1.1. Intuition

A “flat corridor” is a traversal space that should feel tense or anticipatory but instead sounds like a bare ambience loop. There is no meaningful variation in pressure, whispers, or hiss; the player’s UEC and EMD stagnate, and STCI never rises enough to mark thresholds or transitions.

This is not a single bug. It is the joint outcome of:

- Region invariants that permit a tension band (CIC, AOS, LSG not at zero).  
- Mood contracts that nominally ask for non‑zero pressure and subtle features.  
- Audio mapping families (RTPC curves) that compress all inputs into an effectively flat output.  
- Telemetry that shows negligible metric deltas across the corridor window.

### 1.2. Example invariant and metric bands

Consider a corridor region class `threshold_corridor` bound in a region invariant pack:

- CIC in \[0.30, 0.55] – medium trauma ambience density.  
- AOS in \[0.40, 0.65] – archival noise and sense of history present.  
- LSG in \[0.25, 0.55] – some liminal structure and thresholds.  
- DET capped at 0.30 – corridor should not be full combat, but also not safe.

For the corresponding mood contract (e.g., `mood.subdued_dread.v1`), we expect per‑session statistics in this corridor window to satisfy:

- UEC median ≥ 0.50.  
- EMD median ≥ 0.45.  
- STCI median ≥ 0.40.  
- ARR in \[0.60, 0.80].  
- CDL not dropping below 0.25 for long stretches.

### 1.3. Audio RTPC behavior

A correct corridor mood will declare non‑zero bands in its RTPC section, for example:

- `pressure` in \[0.15, 0.40].  
- `whisperDensity` in \[0.05, 0.20].  
- `hissLevel` in \[0.10, 0.35].

A flat corridor failure occurs when, under these bands, the realized RTPC traces (from audio mapping telemetry) satisfy:

- For corridor windows of at least 8–12 seconds,  
  - `pressure(t)` stays within ±0.02 of a baseline (e.g., 0.18).  
  - `whisperDensity(t)` stays within ±0.02 of near‑zero.  
  - `hissLevel(t)` stays within ±0.02 of baseline.  
- There are no significant spikes or dips aligned with player movement, BCI changes, or invariant transitions.

At the same time, the entertainment metrics show:

- ΔUEC ≈ 0 across the window (median uplift < 0.02).  
- ΔEMD ≈ 0 across the window.  
- STCI never exceeds 0.35 despite the corridor’s LSG and AOS windows permitting subtle threat cues.

### 1.4. Linter predicates

The flat corridor pattern can be encoded into the authoring and CI linters as follows:

**Contract‑level checks (static):**

1. For any `moodcontractv1` whose `applicableRegionClasses` includes a class with LSGmin ≥ 0.20 and AOSmin ≥ 0.35:  
   - Require `pressure.max − pressure.min ≥ 0.15`.  
   - Require at least one secondary RTPC band (`whisperDensity` or `hissLevel`) with `max − min ≥ 0.10`.  
   - Reject moods whose RTPC bands are effectively flat (e.g., `max − min < 0.05` on all atmospheric channels).

2. For any `audiolandscapestylev1` attached to those moods:  
   - Require at least one invariant‑sensitive mapping family that reads CIC or AOS and outputs to an atmospheric RTPC.  
   - Reject configurations where all mapped curves ignore CIC/AOS and only depend on DET.

**Telemetry‑level checks (dynamic / linter with data replay):**

Define a corridor window as a contiguous set of frames where:

- `regionClass = threshold_corridor`.  
- `persona` in a non‑combat state.  
- DET ≤ 0.35.

For each mapping profile used in such windows, compute:

- `var_pressure` – variance of `H.Audio.pressure`.  
- `var_whisper` – variance of `H.Audio.whisperDensity`.  
- `var_hiss` – variance of `H.Audio.hissLevel`.  
- Median ΔUEC and ΔEMD across the window.

Flag an audio failure if:

- `var_pressure ≤ 0.0004` (σ ≤ 0.02) and  
- `var_whisper ≤ 0.0004` and  
- `var_hiss ≤ 0.0004` and  
- Median ΔUEC < 0.02 and median ΔEMD < 0.02.

A “flat corridor” entry in the Atlas corresponds to a profile–region–mood triple that hits these predicates in telemetry and whose contracts fail or barely meet the RTPC spread rules above.

Authoring guidance:

- If a linter reports flat corridor for a given mood + style + mapping family, adjust either:  
  - The RTPC bands in the mood contract (widen ranges and raise minima).  
  - The mapping families’ curve parameters (increase weights on CIC/AOS, increase gain, or reduce DET‑driven compression).  
  - The style’s canonical palettes to ensure content exists for the full range.

---

## 2. “Lifeless Firefight” – Underdriven Combat

### 2.1. Intuition

A “lifeless firefight” is a high‑intensity segment (combat, chase, breach) where the visuals indicate chaos and danger, but the audio remains constrained to mid‑level or even subdued bands. Shots and impacts may play, but the macro‑level intensity curve looks like quiet traversal.

In invariant terms, the region and Seeds are in a high‑DET, high‑CIC regime that should push audio closer to the top of its envelope. In metric and telemetry terms, DET increases and combat events fire, but the RTPC outputs and UEC/EMD/STCI curves do not respond accordingly.

### 2.2. Example invariant and metric bands

Consider a region class `warruin_breach` and a combat mood like `mood.combat_nightmare.v1`. In this space we expect:

- CIC in \[0.70, 1.00].  
- AOS in \[0.50, 1.00].  
- RRM in \[0.45, 0.90] (ritual motifs).  
- DET in \[0.60, 0.95].

For the combat mood contract:

- UEC median ≥ 0.70 and upper quartile ≥ 0.80 during combat windows.  
- EMD median ≥ 0.65.  
- STCI median ≥ 0.75.  
- CDL median ≥ 0.50 but not > 0.85 (avoid white‑noise chaos).  
- ARR in \[0.50, 0.80].

### 2.3. Audio RTPC behavior

The combat mood declares aggressive RTPC bands:

- `pressure` in \[0.65, 0.95].  
- `whisperDensity` in \[0.35, 0.90].  
- `ritualMotif` in \[0.40, 0.95].

A lifeless firefight occurs when the realized behavior, across combat windows, looks like:

- `pressure(t)` rarely exceeds 0.55 and spends most time near 0.45–0.50 despite DET ≥ 0.70.  
- `whisperDensity(t)` and `ritualMotif(t)` stay in the lower half of their bands (e.g., 0.35–0.50) with limited spikes.  
- RTPC velocity caps are effectively too tight (e.g., maxDeltaPerSecond so small that values cannot reach the combat envelope before the encounter ends).

Meanwhile, entertainment metrics show:

- ΔUEC modest (0.05–0.10) relative to pre‑combat baseline.  
- EMD not significantly higher than corridor values.  
- STCI median ≤ 0.60 even in high‑CIC, high‑DET regions.

### 2.4. Linter predicates

**Contract‑level checks:**

1. For any `moodcontractv1` with `intensityBand ≥ 6` and `applicableRegionClasses` including a class with `DETmin ≥ 0.60`:  
   - Require `pressure.min ≥ 0.60` and `pressure.max ≥ 0.85`.  
   - Require at least one auxiliary RTPC (`whisperDensity`, `ritualMotif`, or equivalent) with `min ≥ 0.30` and `max ≥ 0.80`.  
   - Reject moods whose combat envelopes overlap heavily with traversal envelopes (e.g., `pressure.max` ≤ 0.70 when traversal moods go to 0.65).

2. For any associated `audiortpc‑mapping‑family‑v1` bound to combat moods:  
   - Require at least one family where DET weight is strongly positive (e.g., `kDET ≥ 4.0` in sigmoid families, with sign chosen to increase pressure as DET increases).  
   - Reject combat families with DET‑scaled ceilings that clamp outputs below 0.70 under `DET ≥ 0.80`.

3. Safety envelopes for combat families should not be so restrictive that values never leave mid‑range:

   - For combat mapping families, linter should assert `outputMax ≥ 0.90` for `pressure` and per‑second delta caps ≥ 0.50, unless explicitly tagged as “short burst only”.

**Telemetry‑level checks:**

Define a combat window as frames where:

- `regionClass` ∈ {`killroom_ritual`, `contamination_zone`, `warruin_breach`, etc.} and  
- DET ≥ 0.60 and  
- Combat or chase tags are present in the region or Seed telemetry.

For each combat window and mapping profile:

- Compute percentiles of `pressure(t)` (25th, 50th, 75th).  
- Compute fraction of frames where `pressure(t) ≥ 0.75`.  
- Compute median STCI and EMD.

Flag a lifeless firefight if:

- 75th percentile of `pressure` < 0.70 and  
- Fraction of frames with `pressure ≥ 0.75` ≤ 0.10 and  
- Median STCI ≤ 0.65 and  
- Median ΔUEC < 0.15 and median ΔEMD < 0.15.

The Atlas entry should attach:

- The mapping profile and family IDs (e.g., `audio.family.combat_nightmare.pressure.v1`).  
- The mood ID and region class.  
- The observed distributions (percentiles and frame fractions).  

Authoring guidance:

- If linters report lifeless firefight, options include:  
  - Raising `pressure.min` and `pressure.max` in the mood contract.  
  - Increasing gain or weights on DET and CIC in the mapping family.  
  - Relaxing per‑second delta caps so RTPCs can reach the top of the envelope quickly.  
  - Adjusting persona behavior so Spectral Foundry personas do not down‑bias combat profiles inappropriately.

---

## 3. “Jump‑Scare Spam” – Overdriven Peaks

### 3.1. Intuition

“Jump‑scare spam” describes audio behavior where the system repeatedly fires near‑maximal spikes – sharp volume, sudden stingers, or percussive hits – in close temporal succession, regardless of contextual DET, BCI overload, or entertainment metrics. The experience is tiring rather than tense: CDL climbs into noisy confusion while ARR drops, and overload probability rises.

In contract terms, this happens when mapping families have excessive gain and insufficient hysteresis or cooldown, and when mood/style contracts fail to constrain the frequency and amplitude of spikes. In telemetry, we see frequent RTPC jumps, repeated peaks near 1.0, and degraded CDL/overload metrics.

### 3.2. Example invariant and metric context

Jump‑scare spam can appear in multiple region classes, but the most concerning pattern arises in mid‑intensity spaces where DET is not supposed to saturate:

- Region class `threshold_corridor` or `archive_highAOS`.  
- DET in \[0.20, 0.55].  
- CIC in \[0.30, 0.70].  

Metrics ideally should show:

- UEC sustained in \[0.50, 0.75].  
- EMD moderate.  
- STCI rising and falling with structure.  
- CDL not exceeding ~0.60.  
- Overload probability low outside of explicit combat peaks.

Jump‑scare spam instead shows:

- Repeated RTPC peaks (e.g., `pressure`, `stingerLevel`, `impactCluster`) jumping from ≤0.30 to ≥0.90 dozens of times in a short window.  
- CDL median ≥ 0.70 and rising.  
- Overload flags from BCI mapping families increasing in frequency.

### 3.3. Audio RTPC behavior

A typical problematic mapping family might look like:

- Sigmoid family with very high gain on `bcifearindex` and CIC, small x0, and insufficient DET moderation.  
- Oscillatory or noise‑augmented stinger channels without damping.  
- Hysteresis families missing “cooldown” behavior (identical thresholds for on/off, causing rapid toggling).

In telemetry, over a 60‑second mid‑intensity segment, we might see:

- `pressure(t)` with > 30 spikes where `pressure` goes from < 0.40 to > 0.90 in less than 0.25 seconds.  
- `stingerLevel(t)` or `impactCluster(t)` with similar bursts.  
- `maxDeltaPerSecond` effectively not enforced or set to very high values (e.g., 5.0) for those channels.  
- BCI overload flags triggered in > 20% of frames during this window.

### 3.4. Linter predicates

**Contract‑level checks:**

1. For non‑combat moods (intensityBand ≤ 4) where DETmin ≤ 0.60:  
   - Restrict `outputMax` for “stinger” or “impact” RTPCs (e.g., force `outputMax ≤ 0.70` unless the channel is explicitly marked as `isCombatOnly`).  
   - Enforce `maxDeltaPerSecond` ≤ 0.80 on high‑salience channels for these moods.  

2. For sigmoid or oscillatory mapping families used outside combat:  
   - Impose an upper bound on global gain (e.g., `gain ≤ 4.0`) and on the sum of absolute weights.  
   - Require hysteresis families to have `xOn ≥ xOff + ε` (e.g., ≥ 0.05) to prevent rapid on/off chatter.  

3. Encourage “cooldown” parameters or windows in style contracts and mapping families:  
   - Add optional `minCooldownSeconds` metadata for profiles that drive stingers.  
   - Linters can warn when `minCooldownSeconds` is missing on channels flagged as high‑impact outside combat.

**Telemetry‑level checks:**

Define a mid‑intensity window as frames where:

- DET in \[0.20, 0.55].  
- Region class not in explicit combat categories.  
- Mood intensityBand ≤ 4.

For each mapping profile and RTPC channel classified as high‑impact:

- Compute the number of “large jumps”  
  - A large jump is a frame where `|rtpc(t) − rtpc(t − Δt)| ≥ 0.50`.  
- Compute the count of peaks where `rtpc(t) ≥ 0.90`.  
- Compute the fraction of frames where BCI overload is active.  
- Compute median CDL.

Flag jump‑scare spam if, over a 60‑second window:

- Number of large jumps ≥ 15 for any high‑impact RTPC and  
- Number of peaks ≥ 10 and  
- Median CDL ≥ 0.70 and  
- BCI overload fraction ≥ 0.15 or overload probability estimate exceeds the configured bound for the Dead‑Ledger attestation profile.

The Atlas entry for jump‑scare spam should include:

- Mapping profile and family codes.  
- Mood and region metadata.  
- Counts of large jumps and peaks per channel.  
- Observed CDL and overload statistics.

Authoring guidance:

- To fix jump‑scare spam, adjust one or more of:  
  - Lower `outputMax` and `gain` for stinger/stress channels in non‑combat moods.  
  - Tighten `maxDeltaPerSecond` so jumps cannot exceed 0.30–0.40 per frame outside combat.  
  - Introduce hysteresis and cooldown in mapping families to enforce `minCooldownSeconds` between high‑amplitude events.  
  - Ensure Dead‑Ledger attestation thresholds for overload are respected; profiles that produce repeated overload flags in mid‑intensity contexts should be demoted or disabled.

---

## 4. How the Atlas Connects to Schemas and Linters

Each failure entry in the Audio Failure Atlas should map to:

- One or more `moodcontractv1` instances (mood IDs, RTPC bands).  
- One or more `audiolandscapestylev1` style IDs (canonical palettes and invariant bindings).  
- One or more `audiortpc‑mapping‑family‑v1` family IDs and profile IDs.  
- Telemetry schemas for per‑frame records and summary aggregates.

Linters then operate at three layers:

1. **Schema‑level linting (CI, static):**

   - Validate contracts against their JSON Schemas.  
   - Apply the static predicates described above for each failure class.  
   - Refuse to admit moods, styles, or mapping families that structurally encode flat corridors, lifeless firefights, or jump‑scare spam.

2. **Profile‑level validation with synthetic traces (offline):**

   - Drive mapping families with synthetic invariant and BCI trajectories (e.g., rising DET, oscillating CIC, changes in fear index).  
   - Record RTPC outputs into audio telemetry envelopes.  
   - Check whether the synthetic outputs trigger failure predicates before deployment.

3. **Runtime and lab telemetry analysis (Orchestrator + Dead‑Ledger):**

   - Aggregate audio telemetry across sessions per `(profileId, regionClass, persona, BCI phase)` and compute UEC/ARR/CDL/overload deltas.  
   - If a profile repeatedly exhibits failure patterns (e.g., flat corridor across many corridors, lifeless firefight across breaches, jump‑scare spam across mid‑intensity spaces), downgrade or revoke its attestation.  
   - Only profiles whose telemetry stays outside these failure regions and satisfies uplift/safety predicates are granted or retained as “safe defaults”.

By tying every Atlas entry to precise numeric predicates and schema‑backed contracts, authors and tools can reason about horror audio behavior as a searchable, evolvable design space rather than a collection of ad‑hoc heuristics.
