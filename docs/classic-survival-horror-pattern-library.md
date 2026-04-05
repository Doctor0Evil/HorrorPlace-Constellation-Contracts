---
invariants_used:
  - CIC
  - MDI
  - AOS
  - RRM
  - FCF
  - SPR
  - RWF
  - DET
  - HVF
  - LSG
  - SHCI
metrics_used:
  - UEC
  - EMD
  - STCI
  - CDL
  - ARR
tiers:
  - standard
  - mature
  - research
deadledger_surface:
  - bundle_attestation
  - agent_attestation
  - spectral_seed_attestation
---

# Classic Survival Horror Pattern Library v1

This document defines a reusable library of survival horror archetypes as schema-friendly pattern definitions for the HorrorPlace VM-constellation.  
Each pattern is expressed in terms of invariants, entertainment metrics, and recommended seed and agent layouts, so that any engine or lab can realize classic survival horror “recipes” without hardcoding specific games or explicit content.

The goal is to let vaults, labs, and engines reference patterns by ID and attach them to invariant bundles, seeds, and agents, while keeping all horror implication-only and GitHub-safe.

---

## 1. Pattern Model and Naming

Each pattern is identified by a stable ID and described as a cross-repo contract, not a specific level.  
Patterns are intended to be referenced from Tier 2 vaults (Atrocity-Seeds, Spectral-Foundry, Codebase-of-Death) and Tier 3 labs (Neural-Resonance-Lab, Redacted-Chronicles) via pattern_id fields inside contracts and seeds.

### 1.1 Pattern schema (conceptual)

A future JSON Schema (e.g., `schemas/patterncontractv1.json`) should support fields like:

- `pattern_id`: stable identifier, e.g., `pattern.hub.sanctuary-that-lies.v1`
- `archetype`: high-level category (`hub`, `corridor`, `stalker`, `process_space`, `museum`, `threshold`, `transition`, `bci_pacing`)
- `intended_tier`: `standard`, `mature`, `research`
- `invariant_bands`: recommended invariant ranges (per CIC, AOS, etc.)
- `metric_targets`: desired metric bands for UEC, EMD, STCI, CDL, ARR
- `seed_layout`: guidance for event/region seeds
- `agent_layout`: guidance for persona/agent artifacts
- `style_hooks`: expected style contract tags
- `notes`: implementation notes for engines and labs

This library defines the content for those fields; the actual schema lives alongside other contracts in this repository.

---

## 2. Hub Patterns

### 2.1 Sanctuary That Lies (Hub That Fails)

**Pattern ID:** `pattern.hub.sanctuary-that-lies.v1`  
**Archetype:** hub

A central hub space that initially behaves as safe, then progressively violates that expectation.

- **Invariant bands (recommended):**
  - CIC: medium (4–7) – historically important but not maximal catastrophe.
  - MDI: medium–high (5–8) – many rumors and stories cluster here.
  - AOS: high (7–9) – records are contradictory or heavily redacted.
  - RRM: medium (4–7) – repeated administrative or ritual behavior.
  - SHCI: medium–high (6–8) – entities are coupled to this hub’s history.
  - DET: low–medium (0.2–0.5) – exposure is tolerable; hub feels safe.
  - LSG: high (7–9) at exits – thresholds around doors and stairwells are tense.
  - HVF: medium (4–6) – directional pressure into off-limits wings.

- **Metric targets:**
  - UEC: high (0.6–0.9) – strong uncertainty about the hub’s true nature.
  - EMD: medium (0.4–0.7) – clues accumulate but do not resolve.
  - STCI: very high contrast (0.7–1.0) – hub initially feels clearly safer than surrounding spaces.
  - CDL: medium (0.4–0.7) – multiple plausible explanations for anomalies.
  - ARR: high (0.6–0.8) – some hub mysteries should remain unresolved.

- **Seed layout:**
  - Region seeds:
    - One or more `region.hub` seeds with `pattern_id` set to this pattern.
    - Early phase: mark hub regions with `safetytier` at or below surrounding areas but `intensityband` 1–3.
    - Later phases: derivative hub seeds that increase SHCI and reduce STCI by enabling anomalies and incursions.
  - Event seeds:
    - Initial events: minor anomalies (lighting, distant sounds) with low DET and low EMD (suggestive, not explanatory).
    - Escalation events: one or two “contract breaks” such as a trusted NPC changing state or an intrusion into the hub.
    - All hub events should reference the same invariant bundle and gradually adjust metric targets (STCI decreasing, CDL increasing).

- **Agent layout:**
  - Personas:
    - One “helper” or “guide” persona with strong coupling to AOS and RWF (they know more than they admit).
    - Potential stalker/Process personas that are initially barred from the hub by policy, later granted conditional access.
  - Agents should track hub state transitions and gate behavior by hub-phase metric bands (e.g., only breach hub once STCI has peaked twice).

- **Style hooks:**
  - Style contracts with tags like `hub_sanctuary`, `administrative_grandeur`, `museum_overlay`.
  - Visual and audio styling should reinforce initial safety (warm lighting, clear diegetic music) then subtly destabilize it.

---

## 3. Corridor Patterns

### 3.1 Cramped Kinetics (No “Around”, Only “Through”)

**Pattern ID:** `pattern.corridor.cramped-kinetics.v1`  
**Archetype:** corridor

A narrow, linear path that removes lateral options, forcing encounters “through” rather than “around”.

- **Invariant bands:**
  - CIC: medium–high (5–8) – corridor is associated with transport or transit under stress.
  - MDI: low–medium (2–5) – not many myths, mostly practical fear.
  - AOS: mid (4–7) – incomplete records of what happened here.
  - RRM: high (7–9) in trains or maintenance tunnels: repeated routines.
  - LSG: very high (8–10) – liminal stress at each doorway, bend, or car boundary.
  - DET: medium–high (0.5–0.8) – prolonged exposure is draining.
  - SHCI: high (7–9) – entities must manifest in constrained, repeatable ways.

- **Metric targets:**
  - UEC: medium (0.4–0.7) – player understands the threat but not timing or placement.
  - EMD: medium–high (0.5–0.8) – openings, sounds, and visual hints accumulate.
  - STCI: moderate (0.4–0.6) – contrast between “empty segment” vs “contested segment”.
  - CDL: low–medium (0.2–0.5) – single dominant explanation: “this corridor is lethal”.
  - ARR: medium (0.4–0.7) – some events resolve, others leave residue.

- **Seed layout:**
  - Region seeds for “car segments” or corridor segments:
    - Each segment with its own LSG and DET weighting.
    - Some segments designated “choke points” with higher HVF into them.
  - Event seeds:
    - Encounter seeds pinned to segments that force direct confrontation (no alternate routes).
    - Backtracking events: events that trigger only when corridor is re-used, increasing SHCI and DET.

- **Agent layout:**
  - Pursuer or ambush personas tuned to:
    - Use line-of-sight and predicted movement to appear in front or behind, never “from nowhere”.
    - Adjust aggression based on DET and BCI-derived fear index (more delay if overload is detected).

- **Style hooks:**
  - Styles for trains, industrial tunnels, hospital corridors, or maintenance shafts.
  - Lighting styles that emphasize blind corners, overhead fixtures, and intermittent visibility.

---

## 4. Stalker and Threshold Patterns

### 4.1 Threshold Breaker (Door-Rule Violator)

**Pattern ID:** `pattern.stalker.threshold-breaker.v1`  
**Archetype:** stalker / threshold

An entity or system that occasionally violates the implicit “door is safety” or “load screen = reset” contract.

- **Invariant bands:**
  - CIC: medium–high (6–8) – threshold region is historically charged.
  - AOS: high (7–9) – door/threshold events are poorly documented.
  - RRM: medium (4–7) – repeated access rituals such as patrols or repeated evacuations.
  - LSG: extremely high (9–10) – thresholds are stress amplifiers.
  - SHCI: very high (8–10) – stalker behavior is tightly bound to specific doors or passages.
  - DET: medium (0.4–0.7) – repeated use of thresholds is taxing.

- **Metric targets:**
  - UEC: high (0.6–0.9) – player is never sure which threshold is safe.
  - EMD: medium (0.4–0.7) – occasional clues hint at where rules can fail.
  - STCI: extremely high (0.8–1.0) initially, then gradually eroded by rule violations.
  - CDL: high (0.6–0.9) – multiple explanations for how or why doors fail.
  - ARR: high (0.6–0.9) – many questions about thresholds remain partially unresolved.

- **Seed layout:**
  - Region seeds for “safe rooms” and “transition corridors”:
    - Marked with designed STCI peaks.
    - Some seeds flagged for “rare threshold break” with low probability.
  - Event seeds:
    - Rare events that allow stalker into spaces normally considered safe (e.g., hub rooms).
    - Door-transition events that include stalker presence behind an opened door.

- **Agent layout:**
  - Stalker personas:
    - Must respect Dead-Ledger entitlement policies (threshold breaks per session, per tier).
    - Behavior parameterized by “rule breach budget” controlling how often doors/hubs can be violated.
  - Director integration:
    - Uses Surprise.Events! phases to schedule threshold breaks only after certain UEC and STCI conditions are met.

- **Style hooks:**
  - Styles for subtle rule-breaking: lights flickering across closed doors, doors that open slightly then stop, background audio that implies presence behind safety barriers.

---

## 5. Museum and Archive Patterns

### 5.1 Dead Exhibits (Former Museum, Now Authority Space)

**Pattern ID:** `pattern.museum.dead-exhibits.v1`  
**Archetype:** museum / archive hub

Institutional spaces that used to be museums, galleries, or archives and now serve other roles, leaving unsettling artifacts and architectural oddities.

- **Invariant bands:**
  - CIC: medium (4–6) – past events are significant but not pure catastrophe.
  - MDI: very high (8–10) – dense mythic layering from multiple eras.
  - AOS: very high (8–10) – extensive missing, contradictory, or censored records.
  - FCF: high (7–9) – overlapping motifs, cross-cultural artifacts.
  - SPR: high (7–9) – strong diegetic justification for spectral manifestations.
  - SHCI: high (7–9) – entities reenact motifs from prior exhibits.
  - RWF: low–medium (2–5) – sources are unreliable, memory conflicts.

- **Metric targets:**
  - UEC: high (0.7–0.9) – uncertainty is a primary engagement driver.
  - EMD: high (0.7–1.0) – rooms are dense with unresolved clues.
  - STCI: moderate (0.4–0.6) – some rooms feel curated-safe, others oppressive.
  - CDL: high (0.7–1.0) – many plausible explanations for any given anomaly.
  - ARR: very high (0.8–1.0) – most threads remain partially unresolved.

- **Seed layout:**
  - Region seeds:
    - “Exhibit” rooms with dedicated invariant bundles.
    - “Conversion” rooms where museum logic and institutional logic clash.
  - Event seeds:
    - Puzzle-like events that require interacting with exhibits in specific orders.
    - Discovery events that expose fragments of the prior museum’s curatorial logic without fully explaining it.

- **Agent layout:**
  - Archivist or curator personas:
    - Implement redaction, contradiction, and drip-feeding of storylets based on AOS and RWF.
    - Adjust CDL and UEC curves by choosing when to stabilize or destabilize narratives.
  - Process personas:
    - Reflect bureaucratic or institutional behavior layered on top of older myths.

- **Style hooks:**
  - Styles for galleries, sculpture halls, archival stacks.
  - Emphasis on lighting that isolates objects, audio that “frames” rooms as curated spaces.

---

## 6. Transition and Door Patterns

### 6.1 Door Oracle (Cinematic Transition As Governance)

**Pattern ID:** `pattern.transition.door-oracle.v1`  
**Archetype:** transition / door

Door-opening sequences that both hide streaming and act as pacing/governance points.

- **Invariant bands:**
  - CIC: depends on adjoining regions; door itself may have low CIC but high LSG.
  - LSG: very high (8–10) – door thresholds are liminal stress peaks.
  - DET: moderate (0.3–0.6) – repeated transitions gradually wear down comfort.
  - SHCI: medium (5–7) – doors may be bound to a subset of events.

- **Metric targets:**
  - UEC: medium (0.4–0.7) – player uncertain about what lies beyond each door.
  - EMD: low–medium (0.2–0.5) – door sequences should not over-explain.
  - STCI: high (0.7–0.9) – door sequences clearly distinct from normal play; perceived as liminal.
  - CDL: low–medium (0.2–0.5) – their role is mostly structural.
  - ARR: moderate (0.4–0.7) – some doors remain unexplained.

- **Seed layout:**
  - Transition seeds:
    - Abstract “door transition” seeds referencing source and target regions.
    - Fields for animation duration, audio profile, and streaming policy hints.
  - Events:
    - Optional rare events for “door anomalies” (longer sequence, different audio, unusual behavior) under certain metric/invariant conditions.

- **Agent layout:**
  - Director logic:
    - Uses door transitions as points to apply Dead-Ledger entitlement checks.
    - Adjusts transition duration or style based on BCI or metric state (e.g., longer fades when BCI overload is detected).

- **Style hooks:**
  - Styles describing camera framing, zoom behavior, and sound design for doors.
  - Distinct profiles per safetytier or intensityband.

---

## 7. BCI Pacing and Neural Resonance Patterns

### 7.1 Neural Resonance Rails (BCI-Governed Surprise.Events!)

**Pattern ID:** `pattern.bci.neural-resonance-rails.v1`  
**Archetype:** BCI pacing / director

BCI-informed pacing logic that stretches or compresses Surprise.Events! phases to maintain target engagement and resolution bands without overloading the player.

- **Invariant bands:**
  - Invariants follow the underlying region; this pattern constrains only how events respond to BCI and metrics, not where they may occur.
  - DET caps are enforced: e.g., DET.max 0.7 for mature tier, 1.0 for research tier.

- **Metric targets:**
  - UEC: target band per scenario (e.g., 0.55–0.8).
  - ARR: target band (e.g., 0.6–0.8) to ensure a high proportion of ambiguity.
  - CDL: medium (0.4–0.7) – cognitive load is kept uncanny but tractable.
  - STCI and EMD are modulated dynamically:
    - If BCI indicates overload: STCI and EMD should be eased.
    - If BCI indicates under-stimulation: STCI and EMD can be increased.

- **Seed layout:**
  - Event seeds:
    - Surprise.Events! with explicit phase durations and variability ranges.
    - Fields for `bci_adjustment_profile` that reference Neural-Resonance-Lab contracts.
  - Region seeds:
    - Optional BCI “comfort envelopes” that describe acceptable fear/arousal ranges per region.

- **Agent layout:**
  - Director and persona logic:
    - Reads BCI-derived indices (fear, overload) via BCI.
    - Applies deterministic rules:
      - Shorten Shock/Echo phases if fear index exceeds threshold.
      - Insert additional micro-anomalies if fear remains low.
    - Enforces downshift triggers and rest intervals when overload or DET caps are approached.

- **Style hooks:**
  - Styles for haptic feedback and subtle sensory changes linked to BCI states.
  - Contracts for “respite” styles to be used when downshift triggers fire.

---

## 8. Process and Industrial Space Patterns

### 8.1 Industrial Indifference (Process Space)

**Pattern ID:** `pattern.process.industrial-indifference.v1`  
**Archetype:** process / industrial

Cold, industrial spaces (factories, plants, depots) where dread arises from impersonal systems rather than overt entities.

- **Invariant bands:**
  - CIC: high (7–9) – accidents, failures, or long-term environmental damage.
  - MDI: medium (4–7) – rumors of disasters, corruption, or disappearances.
  - AOS: high (7–9) – censored logs, missing incident reports.
  - RRM: very high (8–10) – repetitive industrial processes.
  - SHCI: high (7–9) – processes are tightly tied to past events, not to characters.
  - DET: medium–high (0.5–0.8) – long exposure is draining.
  - HVF: strong directional vectors toward machinery and critical infrastructure.

- **Metric targets:**
  - UEC: medium (0.4–0.7) – uncertainty about intent and risk.
  - EMD: medium (0.4–0.7) – scattered clues about what systems are doing.
  - STCI: moderate (0.4–0.6) – “quiet” vs “active” process zones.
  - CDL: high (0.7–1.0) – multiple competing narratives about cause/effect.
  - ARR: high (0.6–0.9) – system motives remain ambiguous.

- **Seed layout:**
  - Region seeds:
    - Machinery halls, waste areas, control rooms, catwalks.
    - Each with distinct RRM and HVF profiles.
  - Event seeds:
    - Process-type events (systems changing state, alarms, flows starting or stopping).
    - Minimal or no direct entity manifestation; focus on environment behavior.

- **Agent layout:**
  - Process personas:
    - State machines that respond to invariants and metrics rather than player identity.
    - Designed to raise CDL and UEC modestly, maintaining high ARR without collapsing STCI.

- **Style hooks:**
  - Styles for industrial light, noise, and movement.
  - Audio emphasis on repetitive sounds, mechanical rhythms, and distant impacts.

---

## 9. Integration with Seeds, Agents, and Dead-Ledger

### 9.1 Pattern usage in seeds

Atrocity-Seeds and Spectral-Foundry should:

- Add a `pattern_id` field to event and region contracts (or a pattern reference object) for any seed that implements one of these archetypes.
- Ensure invariant and metric bands declared for seeds remain within the pattern’s recommended ranges, or document and justify deviations in a lab-specific way.
- For high-intensity or restricted seeds (e.g., high DET, high CIC), attach a `deadledgerref` so Dead-Ledger can attest and gate them appropriately.

### 9.2 Pattern usage in agents

Codebase-of-Death and related vaults should:

- Encode pattern IDs in persona and agent artifacts so behavior can be classified and ranked by pattern.
- Use pattern definitions as constraints when evolving agents:
  - Invariant fidelity is mandatory: agents must not violate pattern-implied invariant or metric bands.
  - Metrics (UEC, ARR, CDL, etc.) are tuned within the pattern envelope, not arbitrarily.

Liminal-Continuum can later use this to compare agent variants that implement the same pattern and select best-in-class implementations.

### 9.3 Pattern-aware entitlement and telemetry

Dead-Ledger and Tier 3 labs should:

- Incorporate pattern IDs into ledger entries where appropriate (e.g., `pattern_id` in bundle or spectral seed attestations) to allow policy and research to reason at the pattern level.
- In Neural-Resonance-Lab and Redacted-Chronicles, tag telemetry summaries with pattern IDs so metric and BCI responses can be analyzed per pattern.
- Adjust policy profiles (e.g., for standard, mature, research tiers) based on observed safety and engagement properties of each pattern.

---

## 10. Extensibility

This v1 library does not define every classic survival horror pattern. Future extensions can add, for example:

- `pattern.corridor.mirror-ambush.v1` – glass or reflective surfaces as delayed scare vectors.
- `pattern.hub.temporal-desync.v1` – hubs where time flows inconsistently.
- `pattern.process.bureaucratic-endless.v1` – paper, queues, and administrative loops as dread sources.
- `pattern.museum.living-exhibit.v1` – exhibits that respond to observation patterns.

All extensions must remain contract-first, invariant-bound, and consistent with Rivers of Blood Charter doctrine, with no explicit content or raw trauma data.  
Patterns should remain engine-agnostic, so that any VM, engine, or tool in the HorrorPlace constellation can consume them via schemas and narrow APIs.
