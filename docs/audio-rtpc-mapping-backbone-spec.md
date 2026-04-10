Audio RTPC Mapping Backbone Specification v1
===========================================

1. Purpose and Position
-----------------------

This specification defines the invariant-driven audio RTPC mapping backbone shared across Horror.Place, Spectral-Foundry, Atrocity-Seeds, Black-Archivum, Neural-Resonance-Lab, Orchestrator, and Dead-Ledger. It formalizes a single, hex-coded audio spine so that every repo can target the same mapping families, telemetry fields, and attestation predicates instead of inventing ad-hoc horror audio behaviors. The backbone is contract-first and schema-governed: audiortpc-mapping-family-v1.json, audio-rtpc-mapping-telemetry-frame-v1.json, and related registries define the only allowed mapping shapes and logging formats.

The spine sits on top of the existing invariant and metrics vocabulary CIC, MDI, AOS, RRM, HVF, LSG, SHCI, DET and UEC, EMD, STCI, CDL, ARR. It does not change those schemas, but constrains audio and BCI-aware mappings to be expressed as simple, bounded functions of these fields plus normalized BCI inputs, with DET-scaled safety envelopes enforced in Rust and ranked in Kotlin analysis.

2. Mapping Families and Curves
------------------------------

Audio RTPC mappings are grouped into small, pluggable mapping families, identified by short hex-coded familyCode values and detailed by audiortpc-mapping-family-v1.json. Each family transforms a vector of invariants and BCI inputs into one or more normalized RTPC outputs, while staying within a safety envelope that CI can reason about.

The initial family codes are:
- 0xPKLIN: linear kernels for pressure-like channels, typically combining bcifearindex, CIC, LSG, and DET into a scalar p that is later clamped to outputMin and outputMax.
- 0xPKSIG: logistic or sigmoid kernels for smooth, saturating mappings such as whisper density or archival hiss intensity that must approach asymptotes.
- 0xPKHYS: hysteresis kernels that support different rising and falling behavior, useful for latching states like “voices present” vs “voices absent” without rapid toggling.
- 0xPKOSC: oscillatory kernels that introduce controlled, low-frequency modulation for tremor, flutter, or breathing-like patterns in ambience or whispers.

Each family has a concrete parameter block constrained in audiortpc-mapping-family-v1.json, so AI tools can only tune bounded scalars rather than inventing new formulas. Rust evaluators treat these parameters as data, computing outputs of the form \(y = f(x; \theta)\) where \(x\) includes normalized BCI and invariant fields and \(\theta\) is the family parameter vector. CI jobs in Neural-Resonance-Lab and Constellation-Contracts can then apply analytic or numeric checks to derive Lipschitz estimates, rate limits, and worst-case excursions.

3. Inputs, Outputs, and Safety
------------------------------

The inputs block in audiortpc-mapping-family-v1.json restricts mapping families to a named subset of BCI and invariant fields. BCI inputs include normalized values such as bciattentionfocus, bcifearindex, bcistartlespike, bcicognitiveload, bcivisualoverloadscore, bciheartsyncratio, and bcibreathphase, all derived from canonical BCI envelopes. Invariants are drawn from the standard horror context snapshot CIC, MDI, AOS, RRM, HVF, LSG, SHCI, and DET, already present in Atrocity-Seeds, Black-Archivum, and Dread Conductor style specs.

Outputs are declared as a small list of RTPC channels, such as H.Audio.pressure, H.Audio.whisperDensity, H.Audio.hissLevel, H.Audio.filterCutoff, or engine-specific equivalents. All outputs are normalized to the 0, 1 band and then mapped into engine-specific ranges by adapters like HorrorAudioDirector in Horror.Place and Codebase-of-Death. This keeps the mapping family definitions engine-agnostic while still letting per-engine adapters bind RTPCs to concrete Wwise or FMOD parameters.

Safety is encoded in the safety block of audiortpc-mapping-family-v1.json. detCeilingScale and outputMin, outputMax define hard bounds on outputs based on DET, while maxDeltaPerSecond approximates a bound on the derivative of RTPC values over time, limiting how quickly audio intensity can change. lipschitzEstimate is a conservative bound on the mapping’s sensitivity to input changes; CI jobs can reject families whose theoretical or estimated Lipschitz constant exceeds policy limits for a given tier. These fields allow Rust and Kotlin tools to implement differential envelope checks and velocity caps without depending on language-specific implementations.

4. Telemetry and Evolution Signals
----------------------------------

To make mapping evaluation and evolution a data-driven process, every mapping decision frame is logged using audio-rtpc-mapping-telemetry-frame-v1.json. Each NDJSON row captures a sessionId, frameTime, regionClass, personaId, bciPhase, invariants snapshot, BCI inputs, the mapping familyId, RTPC outputs, and the entertainment metrics before and after application.

The metricsBefore and metricsAfter blocks provide UEC, EMD, STCI, CDL, and ARR values for this frame, enabling downstream analyses to compute per-profile deltas such as \(\Delta \text{UEC} = \text{UEC}_\text{after} - \text{UEC}_\text{before}\) and \(\Delta \text{ARR}\) as part of composite scoring functions in Kotlin experiment tools. The overloadFlag field indicates whether the BCI intensity envelope considered this frame overloaded, supporting direct estimates of overload probability for given profileId, regionClass, persona, and bciPhase combinations.

Kotlin analysis code in Neural-Resonance-Lab treats mapping profiles and families as experiment dimensions. It ingests the NDJSON telemetry frames, groups them by profileId, regionClass, persona, and BCI phase, and computes expectations and risk metrics. From these, it constructs composite scores S that combine expected uplift in UEC and ARR with penalties for overload probability and excess CDL, ranking mapping profiles and selecting those that approach go-point thresholds for deployment.

5. Orchestrator and Dead-Ledger Attestation
-------------------------------------------

Orchestrator uses this backbone as the basis for profile aggregation and policy evaluation. It consumes Tier-2 summaries that aggregate telemetry per (profileId, regionClass, personaId, bciPhase) and evaluate profiles against formally declared uplift and safety predicates. Profiles that satisfy configured inequalities for expected ΔUEC, ΔARR, and overload probability across a minimum number of sessions can be marked as safe and effective candidates.

Dead-Ledger integrates by hashing and signing these summaries, producing attestation records that bind profileId and familyId combinations to specific invariant and BCI conditions under which they are proven safe and effective. The audio mapping backbone does not expose raw BCI features or per-frame logs; it only uses the normalized telemetry aggregates and statistical predicates to decide whether a given mapping profile should be admitted into public or internal tiers as a “safe default”.

Downstream repos, including Spectral-Foundry and third-party adopters, can then treat attested profiles as trusted audio behavior presets. They reference familyId and profileId pairs in persona contracts, style IDs, and region packs, confident that the underlying mappings have been vetted through shared invariants, metrics, and BCI safety math. This closes the loop promised by the invariant-driven audio backbone: a single, evolvable audio spine that every part of Horror.Place can plug into without fragmenting or reinventing horror audio behavior.
