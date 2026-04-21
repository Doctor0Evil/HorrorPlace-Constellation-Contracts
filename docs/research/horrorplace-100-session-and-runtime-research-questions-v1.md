---
invariants-used:
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
metrics-used:
  - UEC
  - EMD
  - STCI
  - CDL
  - ARR
modules:
  - H.Consent
  - H.Budget
  - H.Banter
  - H.Safety
  - H.Selector
  - H.Director
  - H.Node
  - Policy.DeadLedger
  - H.Metrics
target-repos:
  - HorrorPlace-Constellation-Contracts
  - Horror.Place
  - HorrorPlace-Codebase-of-Death
  - HorrorPlace-Atrocity-Seeds
  - HorrorPlace-Spectral-Foundry
  - Horror.Place-Orchestrator
---

# HorrorPlace: 100 Session and Runtime Research Questions v1

This document lists 100 focused research and implementation questions to guide AI-Chat tools, coding agents, and engine teams when wiring session logic, director personas, history-aware selectors, and runtime encounters across the HorrorPlace VM-Constellation.

Each question is tagged by primary focus:

- `[TECH]` – technical implementation and wiring
- `[DESIGN]` – design methodology, metrics, and contracts
- `[RUNTIME]` – runtime behavior, encounters, and pacing

Numbering is global (1–100) for easier cross-referencing.

---

## 1. Technical Implementation & Wiring (1–40)

1. `[TECH] (Engine)` How should `H.Director.loadPersona` cache director persona envelopes per `sessionId` so that repeated calls from AI-Chat and engine scripts remain cheap while still reflecting live consent and budget changes mid-session?  

2. `[TECH] (Engine)` What minimal Lua table shape should `H.Metrics.snapshot(sessionId)` expose so that selector, director, and image-helper modules can all read UEC, EMD, STCI, CDL, and ARR consistently without each defining its own metric view?  

3. `[TECH] (Engine)` How can the H. runtime surface enforce the rule “no behavior without `QueryHistoryLayer → SetBehaviorFromInvariants`” in Lua, e.g., via a central `H.requireInvariants(regionId, tileId)` helper that must be called before any mood, selector, or node logic executes?  

4. `[TECH] (Engine)` What is the best pattern for sharing selector rules across AI-Chat and engine runtime: a Lua loader over `history-selector-rule-v1.json`, or an HTTP-backed schema cache, and how can we prove both surfaces stay in lockstep with Constellation-Contracts?  

5. `[TECH] (Engine)` How can `H.Node.describe` be implemented so that it pulls CIC, AOS, SHCI, and LSG from the history layer without ever exposing raw region tables, and still returns enough detail for AI-Chat to narrate why a node feels liminal or heavy?  

6. `[TECH] (Engine)` What standard Lua interface should dungeon node contracts implement (e.g., `NodeContract.get_connections(nodeId)`) so that `H.Node.score_next_from_history` can operate identically in Death-Engine, Godot, and Unreal bindings?  

7. `[TECH] (Engine)` How should `H.Selector.selectPattern` be extended to accept an optional `directorPersonaId` and `experienceContractId` while preserving backward compatibility with existing selectors that do not know about director personas yet?  

8. `[TECH] (Safety)` Which shared error-code schema should all H. modules use (`SESSION_NOT_FOUND`, `CONSENT_INCOMPATIBLE`, `BUDGET_EXCEEDED`, etc.), and how will CI enforce that Lua and Rust implementations return only codes declared in that schema?  

9. `[TECH] (Engine)` What is the minimal metadata contract for exposing region-level invariants to shaders (e.g., for screen-space FX) via a `H.Invariants.toRenderParams(regionId, tileId)` helper without leaking any internal registry details?  

10. `[TECH] (Engine)` How can we design a simple, engine-agnostic `H.Audio` Lua façade that maps CIC, AOS, DET, and LSG into a small set of RTPCs (e.g., `horror_pressure`, `liminal_intensity`) which Death-Engine and external engines can all implement in their own audio middlewares?  

11. `[TECH] (CI)` What static analysis rules should be added to Codebase-of-Death so that any Lua script accessing invariants or contracts outside the H. API (e.g., reading raw JSON tables) fails CI by default?  

12. `[TECH] (CI)` How should Constellation-Contracts define a reusable CI job that validates that every new history-selector rule, pattern, or director persona references only invariants and metrics declared in the canonical spine, never ad-hoc fields?  

13. `[TECH] (Authoring)` What JSON structure should AI-Chat use as a “session planning skeleton” that lists planned objectKinds, targetRepos, tiers, and invariants/metrics touched, and how can CHATDIRECTOR verify that all subsequent authoring responses stay within that plan?  

14. `[TECH] (Authoring)` How can we expose a lightweight CLI tool that takes a single session plan JSON, validates it against `ai-safe-authoring-contract`, and prints a concise summary for human designers before any AI or code runs?  

15. `[TECH] (Engine)` What is the cleanest way for Godot and Unreal plugins to present H. functions: directly exposing Lua functions through bindings, or wrapping them in native functions that internally call into an embedded Lua VM?  

16. `[TECH] (Telemetry)` How should session metrics envelopes annotate which director persona and comfort policy were active so offline analysis can correlate different downshift strategies with UEC, CDL, and ARR trajectories?  

17. `[TECH] (Engine)` What base interface should an “image-helper” module implement (e.g., `H.ImageHelper.describe(context, invariants, metrics, inventory)`) so AI-Chat, Godot, and Unreal all see the same contract for generating clue-rich prompts while keeping raw image generation out of Tier 1 repos?  

18. `[TECH] (Engine)` How can H. modules expose a uniform `context` table shape (sessionId, regionId, tileId, nodeId, patternId, directorPersonaId) that all decision functions accept, so new modules remain pluggable without combinatorial parameter growth?  

19. `[TECH] (Engine)` What mechanism should be used to propagate Dead-Ledger decision tokens from `Policy.DeadLedger.requestBundleAccess` into H.Selector and H.Director calls, so selectors never activate bundles without a fresh entitlement?  

20. `[TECH] (Engine)` How should the `H.Director.applySafetyDecision` helper be wired so that it can optionally log its own decisions as telemetry events (e.g., strategy `soften`, `imply`, `refuse`) without requiring persona scripts to emit their own NDJSON?  

21. `[TECH] (CI)` What schema-level invariants should be added to `director-persona-contract.v1.json` and `director-comfort-policy.v1.json` to ensure that CDL spans and DET caps never widen beyond those allowed by the consent and budget modules for a given tier? [file:9]  

22. `[TECH] (Engine)` How can H.Budget and H.Director share a common representation of “high DET event” versus “low DET mood” so that required cooldown ratios are enforceable in Lua without each module reinventing DET band thresholds?  

23. `[TECH] (Engine)` What is the most straightforward way for a runContract (story-driven session bundle) to make its difficulty profile accessible to Lua (e.g., via `H.Run.getDifficultyBands(runId, difficultyName)`) so persona skill bands and interpretation tools can be applied consistently?  

24. `[TECH] (Authoring)` How should AI-Chat request a prevalidated skeleton for a director persona or runContract, and what fields must that skeleton always contain so that AI-Chat only fills values inside schema and invariant bounds?  

25. `[TECH] (Engine)` How can `H.Selector.selectPattern` and `H.Node.choose_next` share an internal scoring kernel so that pattern scoring and node scoring do not drift in behavior between pattern-level and node-level selection?  

26. `[TECH] (Telemetry)` What additional fields should a `history-selector-decision` telemetry event carry (e.g., `patternId`, `nodeId`, `directorPersonaId`, metric snapshot) to reconstruct full decision trees for playtest analysis?  

27. `[TECH] (Engine)` How can AI-Chat and engines agree on a canonical `stage` enumeration for runs (e.g., OuterThreshold, Locus, Rupture, Fallout) so templates, mood contracts, and selectors all talk about progression using the same vocabulary?  

28. `[TECH] (Engine)` What interface should `H.Run` expose (e.g., `start`, `step`, `currentStage`, `isComplete`) so that directors, selectors, and AI-Chat templates know exactly where they are in a runContract without re-deriving state from metrics alone?  

29. `[TECH] (Authoring)` How can we define a small markdown-to-metadata contract for design docs so that AI-Chat can parse which invariants and metrics a doc references and use that information to shape its authoring behavior?  

30. `[TECH] (Engine)` What is the simplest way to provide a `H.Debug.logDecision(context, details)` helper that writes to structured logs, and how can we restrict its use so it never leaks sensitive or vault-only IDs into public telemetry?  

31. `[TECH] (CI)` How should CI enforce “three artifacts per request per repo” in a way that is visible to AI-Chat, so chat-based authoring tools know when they must split work into multiple phases rather than overfilling a single prism envelope?  

32. `[TECH] (Engine)` How can we standardize the representation of “persona inventory” (list of interpretation tools) in Lua so both image-helper and director persona logic can reason about which clues and contradictions are allowed this run? [file:9]  

33. `[TECH] (Engine)` What data structure should be used for representing node graphs (adjacency lists vs. explicit edge objects) so H.Node functions can easily apply pattern-based scoring without heavy transformations at runtime?  

34. `[TECH] (Engine)` How can Death-Engine expose H. functions to Blueprint or VisualScript-like systems so designers can easily wire selector and director calls without writing Lua by hand, while still keeping Lua as the single behavior authority?  

35. `[TECH] (Authoring)` What small JSON manifest should each repo maintain listing which H.* namespaces it expects to use (e.g., `H.Audio`, `H.Selector`, `H.Director`), so AI-Chat knows which runtime surfaces are appropriate when generating code for that repo?  

36. `[TECH] (CI)` How can we build a linter that checks that any AI-Chat-generated Lua file only imports approved modules (H.*, engine adapters, Dead-Ledger client), and never uses forbidden standard library calls (e.g., filesystem, networking) in constellated repos?  

37. `[TECH] (Engine)` How should we model and store cross-session lore progress (e.g., completed lore threads) so that selectors can favor deeper threads over time, and how can this state be exposed to AI-Chat without leaking user-level PII?  

38. `[TECH] (Engine)` What is the best representation for a “comfort outcome” summary (`withinBand`, `tooIntense`, `tooFlat`) at the end of a session, and how should H.Director and H.Metrics compute it from CDL, ARR, consent wave, and budget overages? [file:9]  

39. `[TECH] (Engine)` How can we extend the AI-safe authoring contract so each session planning envelope explicitly declares which H.* namespaces it will use, and how can CHATDIRECTOR refuse plans that attempt to touch any not whitelisted for that profile?  

40. `[TECH] (Telemetry)` What small set of derived metrics (e.g., max DET overshoot, average UEC deviation from target, count of safety-triggered downshifts) should each session emit, and how can this be standardized across all experience types and director personas?  

---

## 2. Design Methodology & Metric Calibration (41–70)

41. `[DESIGN] (Design)` How should we numerically distinguish a “Slowburn Dread” experience from a “Short Shock” experience in terms of UEC, EMD, STCI, CDL, and ARR bands, and what default runContracts should ship for each as reference implementations?  

42. `[DESIGN] (Design)` How can we define a small set of named experience contracts (e.g., Guided Ritual Investigation, Process-Driven Catastrophe Run) whose metric bands and invariant envelopes can be easily communicated to users as sliders or presets in an AI-Chat UI? [file:9]  

43. `[DESIGN] (Design)` What numeric patterns in SHCI, CIC, and AOS should indicate that a region is suitable for “overseer-style” director personas, and how do we keep those regions rare enough that these personas feel special rather than ubiquitous? [file:9]  

44. `[DESIGN] (Design)` How should we calibrate DET bands for different tiers so that “standard” sessions feel tense but not overwhelming, while “mature” and “research” tiers can support higher peaks without causing runaway distress?  

45. `[DESIGN] (Design)` What guidelines should we give to narrative designers for setting ARR bands so endings feel meaningfully ambiguous but not cheap, and how can we use ARR and CDL together to distinguish satisfying ambiguity from simple confusion?  

46. `[DESIGN] (Design)` How can persona skill profiles (analysisSkill, composure, worldviewRigidity) be mapped into metric modifiers so that different personas experience the same runContract as easier or harder without altering core invariants? [file:9]  

47. `[DESIGN] (Design)` What range of UEC and EMD should be considered “comfortable mystery” versus “cognitive overload,” and how should this influence default difficulty profiles for novice vs. expert runs in AI-Chat sessions?  

48. `[DESIGN] (Design)` How can experience contracts ensure that every session includes at least one “cooldown window” where DET and CDL drop below certain thresholds, and how should the Director persona be required to acknowledge these moments in-world?  

49. `[DESIGN] (Design)` What clear language can we give to designers to describe the difference between high SHCI but low AOS (deep, static haunt) and high AOS but moderate SHCI (restless but shallow haunt), and how should each be used in session layouts?  

50. `[DESIGN] (Design)` How should we define “liminal nodes” in node contracts (tagging with liminal:doorway, liminal:threshold, etc.), and what target LSG and HVF ranges should make a corridor feel unsettling versus neutral?  

51. `[DESIGN] (Design)` What small catalog of “Director strategies” (e.g., clinical recounting, ritual invocation, archival detachment) should we define, and how should each map to characteristic metric bands and tone labels? [file:9]  

52. `[DESIGN] (Design)` How can we ensure that AI-Chat templates for different experience contracts never drift into content that violates explicitness ceilings and prohibited themes even when targeting high UEC and EMD, using only implication and tone?  

53. `[DESIGN] (Design)` What does an “ideal” session curve look like for UEC, STCI, and DET over 20–40 turns, and how can we provide reference graphs designers can treat as targets when authoring new runContracts?  

54. `[DESIGN] (Design)` How should we differentiate “branch density” (number of narrative forks) from “mystery density” (EMD) so that sessions can be highly replayable without becoming exhausting, and what ARR ranges make repeated play interesting rather than frustrating?  

55. `[DESIGN] (Design)` What design patterns are best for signaling consent and safety changes in-world (e.g., lights changing, music shifts, meta-narration) so users feel informed without breaking immersion, and how should directors vary these patterns by persona type?  

56. `[DESIGN] (Design)` How can we encourage designers to think of node graphs as “dread fields” shaped by CIC, SHCI, and LSG, and what visualization tools (heatmaps, gradient fields) should we provide to make that intuition tangible?  

57. `[DESIGN] (Design)` What naming conventions should we use for moodContracts and eventContracts so AI-Chat can infer their likely metric effects (e.g., `mood.liminal_hum.v1` suggests moderate UEC, low DET, medium LSG)?  

58. `[DESIGN] (Design)` How should we describe “death” in story-driven runs so that it remains purely an implied branch (never explicit depiction), and how can metrics and director behavior make non-lethal but high-stakes outcomes feel just as consequential? [file:9]  

59. `[DESIGN] (Design)` What principles should guide the design of image-helper prompts so that each generated image contains at least one interpretable clue for novice difficulty, while still allowing more abstract symbolism at higher difficulties without breaking CDL caps? [file:9]  

60. `[DESIGN] (Design)` How can we construct a small library of “interpretation tools” (Pattern Recognition Lens, Doubt Amplifier, Ritual Decoder, etc.) and define their metric effects so designers can mix and match them to create distinct analytical personas? [file:9]  

61. `[DESIGN] (Design)` What guidelines should we give designers for using SHR (SHCI) and SPR together to define how “present” a spectral history is at a location, and how should this influence whether an encounter is described as a glitch, echo, or full apparition?  

62. `[DESIGN] (Design)` How should we calibrate ARR ceilings for different experience types to ensure some runs always end with strong closure (low ARR) while others deliberately leave unresolved threads (high ARR), and how should UI communicate this to players?  

63. `[DESIGN] (Design)` What policy should we adopt for limiting cumulative DET across a multi-session campaign, and how should directors and selectors coordinate to enforce campaign-level exposure caps while still allowing intense individual episodes?  

64. `[DESIGN] (Design)` How can we define a small set of “safety personas” that AI-Chat can switch into when guardrails trigger, and what metric bands and tone descriptions should they follow so downshifts feel caring rather than punitive?  

65. `[DESIGN] (Design)` How should we balance the desire for “metagame puzzle solving” (players analyzing invariants, sessions, and runs) against the risk of making horror feel purely mechanical, and what ranges of UEC and EMD best support that meta layer?  

66. `[DESIGN] (Design)` What naming and tagging scheme should runContracts use to make it clear which invariants are central to their themes (e.g., DET-heavy prison, SHCI-heavy cemetery), and how should AI-Chat present those tags as user-facing descriptors?  

67. `[DESIGN] (Design)` How should we define the difference between a “guided tour” AI-Chat mode and a “co-authored story” mode in terms of metric bands, branch counts, and director persona roles, and what constraints should each impose on runtime decisions?  

68. `[DESIGN] (Design)` What simple, consistent way can we summarize an experience contract to players (e.g., intensity meter, ambiguity meter, mystery meter) using projections of UEC, DET, and ARR, and how can we keep those summaries honest as contracts evolve?  

69. `[DESIGN] (Design)` How should we incorporate real-world folklore and historical events into seeds and regions without ever representing real victims or trauma directly, relying instead on synthetic invariants and fictionalized metadata to preserve ethical boundaries?  

70. `[DESIGN] (Design)` What is the right balance of shared vs. per-user lore: how much should SHCI and CIC-based haunt patterns persist across players for a given region, and how much should selector and director behavior adapt to each player’s prior routes and comfort outcomes?  

---

## 3. Runtime Behavior, Nodes, and Encounters (71–100)

71. `[RUNTIME] (Engine)` How should H.Selector and H.Node coordinate to ensure that a “liminal hooks” pattern never routes players through more than N consecutive high-LSG nodes without inserting a lower-stress node to respect DET and CDL limits?  

72. `[RUNTIME] (Engine)` What runtime rules should govern when a spectral entity is allowed to “step up” from a subtle presence (e.g., audio glitch) to a more direct manifestation, based solely on historical invariants (CIC, SHCI, SPR) and not on arbitrary timing?  

73. `[RUNTIME] (Engine)` How can director personas use `H.Director.constrainMetrics` each turn to slowly steer UEC and EMD toward target bands without causing abrupt jumps that feel artificial or manipulative to players?  

74. `[RUNTIME] (Engine)` What algorithm should `H.Node.choose_next` use when multiple nodes are tied for top score—random selection, deterministic tie-breaking, or director persona preferences—and how can this be made tunable per experience contract?  

75. `[RUNTIME] (Engine)` How should runtime logic decide when to terminate a run early (e.g., if comfortOutcome trends to `tooIntense`) while still offering a narratively satisfying wrap-up, and what signals should be emitted to AI-Chat to switch into epilogue mode?  

76. `[RUNTIME] (Engine)` How can we design a simple “heartbeat” mechanism where each turn AI-Chat calls into `H.Metrics.snapshot` and `H.Director.applySafetyDecision` to adjust not just text but also audio, VFX, and pacing in-game?  

77. `[RUNTIME] (Engine)` How should selectors and directors react if BCI-derived signals (in lab or research tiers) indicate that a player is more distressed than DET and CDL predicted—should they downshift more aggressively, or adjust metric targets for future runs?  

78. `[RUNTIME] (Engine)` What runtime strategies can be used to prevent players from “camping” in safe nodes with very low DET and CDL to avoid progression, while still respecting their choice not to confront high-intensity content?  

79. `[RUNTIME] (Engine)` How should AI-Chat communicate in-world that a downshift has occurred due to safety or budget constraints (e.g., “the house seems to hold its breath”), and how can this messaging vary across director personas without revealing implementation details?  

80. `[RUNTIME] (Engine)` What runtime checks should precede any node transition to ensure that required Dead-Ledger proofs are still valid (e.g., entitlement tokens not expired) before activating certain high-intensity or research-tier content?  

81. `[RUNTIME] (Engine)` How should selectors use encounter history (e.g., nodes visited, patterns resolved) to avoid repetitive beats in a single session, and what thresholds constitute “too much repetition” for different experience types?  

82. `[RUNTIME] (Engine)` How can node scoring incorporate both local invariants and session-level metrics, e.g., awarding bonus weight to nodes that bring UEC or CDL closer to target bands, without producing obviously “gamey” routing behavior?  

83. `[RUNTIME] (Engine)` What runtime rule should govern when to introduce puzzles or clue-based imagery (via image helper) versus pure atmospheric beats, given current EMD, UEC, and CDL, so puzzle sequences do not overwhelm the horror pacing?  

84. `[RUNTIME] (Engine)` How should we decide when to introduce “false safe” moments (nodes that look low-DET but carry subtle spectral implications) while still ensuring that overall session DET remains within comfort policy bounds?  

85. `[RUNTIME] (Engine)` How can AI-Chat and engine logic coordinate so that user choices at narrative forks directly influence metric evolution (e.g., brave choices increasing DET and UEC, cautious choices lowering DET but raising ARR) in predictable, tunable ways?  

86. `[RUNTIME] (Engine)` What runtime policies should be in place to prevent a director persona from aggressively pushing ARR (ambiguity) at the expense of closure when telemetry shows that many players rate those runs as unsatisfying?  

87. `[RUNTIME] (Engine)` How should BCI or physiological data, when available, be incorporated as optional “hint signals” into metric adjustments (e.g., nudging DET or CDL bands) without ever becoming a direct or sole driver of horror intensity?  

88. `[RUNTIME] (Engine)` What runtime strategy should be used to align template selection with selector and director decisions, so that narrative outputs, audio mood, and node transitions all tell a consistent story of rising or falling tension?  

89. `[RUNTIME] (Engine)` How should the system handle “failed” puzzles or branches (user misses key clues repeatedly) in a way that preserves horror tone—e.g., by folding failure into the narrative rather than dumping the player out-of-world?  

90. `[RUNTIME] (Engine)` What runtime data should be logged for each turn (nodeId, patternId, directorStrategy, metric snapshot, comfortPolicy status) to enable detailed reconstruction of sessions for debugging and research?  

91. `[RUNTIME] (Engine)` How can we design a “micro-run” concept (short, 5–10 turn experiences) for AI-Chat that still respects all invariants and metrics but is tuned for casual interaction, and what runtime differences should they have compared to full runs?  

92. `[RUNTIME] (Engine)` How should selectors and directors behave when a session is resumed after a long gap—should they reintroduce context gently, re-establish DET and CDL from lower baselines, or pick up where metrics left off?  

93. `[RUNTIME] (Engine)` What runtime policy should govern cross-session adaptation—for example, lowering DET target bands in subsequent runs if prior sessions repeatedly ended with comfortOutcome `tooIntense`, even when consent tier remains unchanged?  

94. `[RUNTIME] (Engine)` How can director personas use `requiredCooldownRatio` to decide when to insert explicitly low-DET “sanctuary” segments, and how can those segments remain thematically aligned with the rest of the horror experience?  

95. `[RUNTIME] (Engine)` What runtime behaviors should be reserved exclusively for research-tier sessions (e.g., certain mapping kernels, BCI-driven modulation) to prevent them from accidentally leaking into public-tier experiences?  

96. `[RUNTIME] (Engine)` How should we structure runtime experiments (A/B tests) around director strategies, selector patterns, and node scoring tweaks so that changes to metric mappings can be evaluated without risking large shifts in user comfort?  

97. `[RUNTIME] (Engine)` What runtime safeguards should exist so that if H. modules or external services fail (e.g., selector unavailable), AI-Chat defaults to a clearly safe, low-intensity fallback mode rather than silently continuing with incomplete constraints?  

98. `[RUNTIME] (Engine)` How should AI-Chat adapt its pacing (frequency of messages, length of turns) based on current metrics and comfort policy—for example, slowing down during high DET peaks and speeding up in low-DET cooldowns?  

99. `[RUNTIME] (Engine)` What runtime logic should be used to mark specific nodes or patterns as “unlocking” new director persona behaviors (e.g., deeper insight into a region’s history) while ensuring those behaviors remain bounded by the persona’s invariant and metric envelopes?  

100. `[RUNTIME] (Engine)` How should selectors and directors cooperate to ensure that final beats of a run (epilogue, last node) satisfy both metric targets for ARR/CDL and the requirement that all safety modules have fully de-escalated DET to within comfort policies before exit?  

---
