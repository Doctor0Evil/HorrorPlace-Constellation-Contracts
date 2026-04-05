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

# Pattern Governance, Entitlement, and Classic Horror Mechanics

This document explains how the Classic Survival Horror Pattern Library integrates with the HorrorPlace governance stack, especially the Dead-Ledger system, and how classic survival horror mechanics are translated into engine-agnostic, pattern-driven constructs.

The intent is to give AI tools, IDEs, and human developers a conceptual bridge between familiar horror design techniques and the invariant/metric contracts, seeds, agents, and entitlement profiles that govern their use in the VM-constellation.

---

## 1. Governance and Entitlement via Dead-Ledger

The governance of the HorrorPlace VM-constellation is centralized through the Dead-Ledger, a cryptographic ledger that enforces safety policies, manages entitlements, and ensures ethical compliance across all generated content. Within the context of the Classic Survival Horror Pattern Library, the Dead-Ledger plays a critical role in gatekeeping the use of patterns, especially those that may induce high levels of distress or simulate potentially sensitive scenarios. Its integration is not an afterthought but a foundational element of the architecture, ensuring that the creative freedom afforded by procedural generation is balanced with robust safety and ethical boundaries.

The primary mechanism for governance is the `deadledgerref` field, which is mandatory for any seed that exceeds a configured intensity threshold or belongs to a restricted category like the `research` tier. When a developer or runtime requests such a seed, the system checks for this reference. If present, the request is passed to the Dead-Ledger for attestation. The ledger holds cryptographic proofs that a given seed complies with the policies of a specific safety tier (`standard`, `mature`, `research`). For example, requesting a `Threshold Breaker` pattern seed with an `intensity_band` of 8 or higher would require a proof from the Dead-Ledger that confirms the seed adheres to the stricter rules of the `mature` tier, which might limit the frequency of stalker appearances, enforce downshift triggers, or mandate specific respite periods in the encounter sequence. This transforms the Dead-Ledger from a simple database into an active enforcement layer, guaranteeing that only authorized and vetted seeds are used in production environments while keeping all trauma details out of the ledger itself.

Pattern definitions within the library incorporate high-level guidance regarding typical tier usage. While specific policy details such as exact caps or required attestation types remain in documents within the HorrorPlace-Dead-Ledger-Network repository, the pattern library suggests general suitability. For instance, the `Sanctuary That Lies` pattern, with its potential for sudden, high-impact betrayals of safety expectations, would be marked as typically `mature` or `research`, signaling to developers that it likely requires a Dead-Ledger attestation and associated proofs. Conversely, a pattern for a simple, atmospheric archive setting like `Dead Exhibits` might be primarily `standard`, though a variant with more intense puzzle pressure and higher DET could be flagged for `mature` use. This tiering system allows for a spectrum of horror experiences, from mild unease suitable for broad audiences to intense, psychologically demanding sequences reserved for more mature contexts. The Dead-Ledger supports this by maintaining distinct policy profiles for each tier and validating that a seed’s invariant and metric envelopes fall within the acceptable bounds for its declared tier.

The Dead-Ledger also governs agents and personas that implement these patterns. An agent artifact’s attestation can specify its capabilities and limitations. A `Process Persona` designed to generate ambient dread in an industrial space might be granted a lower `intensity_band` and a relatively high `ARR` (Ambiguous Resolution Ratio) cap, reflecting its subtler threat model and focus on ambiguity. In contrast, a `Stalker Persona` from the `Threshold Breaker` pattern would be subject to stricter limitations on its aggression and breach frequency, with its attestation detailing constraints on how often it may violate door or hub safety contracts and what DET ceilings apply per session. This ensures that even the behavioral logic of an encounter is held to account.

Telemetry gathered by Tier 3 labs such as Neural-Resonance-Lab and Redacted-Chronicles is tagged with pattern IDs and linked back to the Dead-Ledger via ledger entries and policy updates. This allows continuous monitoring of how patterns perform in practice. If a `mature` tier seed is found to consistently cause excessive player distress beyond its intended scope according to BCI-derived overload flags or metric trajectories, this telemetry can feed back into the system, prompting a review of its attestation, a tightening of its intensity band, or even a reclassification to a higher tier. This closes the loop between creation, validation, deployment, and analysis, with the Dead-Ledger standing as the final arbiter of safety and entitlement, preserving the integrity of the entire HorrorPlace ecosystem.

---

## 2. Technical Translation of Classic Horror Mechanics

The theoretical framework of patterns, invariants, and metrics becomes practically useful when it can be translated into the specific mechanics that defined classic survival horror games. The proposed architecture is designed for this translation, taking iconic design elements from well-known survival horror titles and re-expressing them as modular, engine-agnostic constructs. This process involves deconstructing a beloved mechanic, identifying its core psychological function, and then rebuilding it using the vocabulary of invariants, metrics, seeds, and agents.

### 2.1 Door Oracle: Fixed Cameras and Cinematic Door Transitions

Fixed cameras and cinematic door transitions are quintessential techniques for controlling player perspective and building suspense. Historically, these patterns emerged from hardware limitations but evolved into powerful narrative and pacing tools. In the HorrorPlace framework, they are captured by the `Door Oracle` pattern (`pattern.transition.door-oracle.v1`).

A seed for this pattern is an abstract definition containing parameters for animation duration, audio triggers (such as creaks and impacts), camera framing styles, and hints for streaming or loading policies. A runtime like Codebase-of-Death consumes this seed and implements it using its native toolkit, whether UE5 Blueprints, Godot scripts, or a custom engine, to create a familiar smoke-and-mirrors transition. The invariant `LSG` (Liminal Stress Gradient) is intentionally elevated at these points, encoding their role as moments of heightened temporal and spatial uncertainty. The pattern’s metric targets ensure the sequence serves its purpose: high STCI (Safe-Threat Contrast Index) to make the transition feel distinct and liminal, and relatively low EMD (Evidential Mystery Density) so as not to over-explain what lies beyond the door. This preserves the “flavor” of classic door transitions—the slow, deliberate opening that builds dread—while making their implementation portable and contract-driven.

### 2.2 Cramped Kinetics: Linear Claustrophobia

The architectural design of narrow trains, maintenance tunnels, and long corridors can be captured by the `Cramped Kinetics` pattern (`pattern.corridor.cramped-kinetics.v1`). This pattern focuses on the horror of linear claustrophobia, where there is no “around” the threat, only “through”.

Technically, a `Cramped Kinetics` configuration is defined as a series of `region` seeds for individual corridor segments. Each segment has its own `LSG` and `DET` weighting, with some designated as “choke points” with higher `HVF` (Haunt Vector Field) pressure that biases entities into those spaces. Event seeds are pinned to these segments to force direct confrontations, with no alternate route seeds available. Agent personas spawned in these corridors are tuned to use line-of-sight prediction and path forecasting, appearing from plausible positions rather than arbitrarily spawning in front of the player. Lighting and style contracts emphasize blind corners, overhead fixtures, and gaps in visibility, forcing players to surrender information-gathering to anticipation. Because all of this is described schematically, a developer can generate many variations of claustrophobic corridors by sampling and composing seeds from this pattern family, each with slightly different geometry, lighting, and encounter density, while preserving the core psychological principle.

### 2.3 Industrial Indifference: Process-Driven Dread

The feeling of dread derived from impersonal, systemic threats in industrial complexes is formalized in the `Industrial Indifference` pattern (`pattern.process.industrial-indifference.v1`). This archetype focuses less on a monster pursuing the player and more on the oppressive weight of malfunctioning or indifferent systems.

The pattern’s invariant and metric profile is designed to foster ambiguity and sustained anxiety. High `RRM` (Ritual Residue Map) for repetitive industrial processes and high `CDL` (Cognitive Dissonance Load) for competing narratives about the facility’s purpose create a confusing, unsettling environment. The metric `ARR` (Ambiguous Resolution Ratio) is deliberately kept high, ensuring that the system’s motives remain fundamentally unclear. Agent personas for this pattern are `Process Personas`: state machines that respond to invariants and metrics instead of directly targeting the player. Their behavior might involve machinery starting and stopping, lights flickering in complex patterns, alarms activating based on overall `DET`, or subsystems changing state in ways that suggest deeper logic. There is minimal direct entity manifestation; the horror emerges from the environment itself behaving in strange, rule-driven yet opaque ways.

Because this is expressed as a pattern, a developer can use it to generate an entire zone of a game or experience that feels oppressive and inevitable without writing complex hostile AI. The environment, guided by pattern-linked invariants and metrics, becomes the primary antagonist. This demonstrates the framework’s ability to codify not just creature-based horror but also environmental and atmospheric dread within the same contract-first structure.

---

## 3. Strategic Synthesis and Future Directions

The establishment of a schema-compatible, engine-agnostic Classic Survival Horror Pattern Library represents a strategic initiative for the HorrorPlace VM-constellation. It moves beyond the creation of isolated, bespoke assets and toward a universal conceptual toolkit for designing interactive horror.

By codifying the principles behind classic survival horror into a formal system of patterns, invariants, and metrics, the project achieves several goals:

- It preserves “old recipes” of fear design in a reusable, documented form.
- It enables scalable production of high-quality horror content via procedural generation guided by pattern contracts.
- It establishes a common language and set of contracts that can be implemented by any engine or tool, from real-time game engines to film or virtual production systems.

The success of this approach depends on strict adherence to the contract-first, implication-only doctrine. Patterns are defined in terms of invariants and metrics, with no explicit scenes or trauma payloads. Runtime implementations must route through narrow APIs rather than hardwired access paths, keeping patterns portable and analyzable.

The architecture’s layered design, with clear boundaries between pattern contracts, historical grounding, seeds, style envelopes, agents, and entitlement logic, provides a framework for managing complexity. Empirical feedback from Neural-Resonance-Lab, including BCI-derived metrics, introduces a scientific dimension, allowing pattern implementations to be evaluated and optimized rather than tuned purely by intuition. The Dead-Ledger system provides the governance layer that ensures this experimentation remains within ethical and safety bounds, a key requirement for a genre centered on strong emotional responses.

Looking forward, the pattern library is intended as a living framework. The initial version covers foundational archetypes, but the architecture supports adding new patterns that capture novel flavors of horror. Future additions could include:

- `pattern.museum.living-exhibit.v1` for exhibits that respond to observation patterns.
- `pattern.corridor.mirror-ambush.v1` for delayed scare vectors via reflections.
- `pattern.hub.temporal-desync.v1` for hubs where perceived time is inconsistent.
- `pattern.process.bureaucratic-endless.v1` for administrative and procedural dread.

Patterns can also be extended to integrate narrative progression more tightly, linking storylet evolution directly to changes in invariants and metrics across a pattern’s lifecycle. Exploring how patterns can adapt dynamically to player choices and BCI signals to create branching metric trajectories would add player agency and replayability without compromising safety or governance.

In this way, the Classic Survival Horror Pattern Library serves as a blueprint for a new paradigm in horror content creation: systematic, scalable, cryptographically governed, and deeply respectful of both players and the histories it evokes.
