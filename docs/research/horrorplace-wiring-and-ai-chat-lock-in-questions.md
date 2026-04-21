# Research Questions, Definitions, Queries, and Objections for Horror\$Place Wiring & AI‑Chat Lock‑In

This document collects **100** focused research, definition, query, and objection items derived from the technical conversation about memory architectures, dungeon run contracts, selector patterns, and director persona integration. Use these to guide next‑step implementation, validation, and architectural refinement.

---

## 1. Research Questions (25)

1. How should `DungeonMemory` layer integrate with `H.Run`’s state machine to support at‑most‑once surprise triggers while remaining deterministic across Godot and C++/Rust engines?
2. What is the minimal set of invariant fields (`CIC`, `MDI`, `AOS`, `RRM`, `FCF`, `SPR`, `RWF`, `DET`, `HVF`, `LSG`, `SHCI`) that must be stored per `DungeonNode` to enable selector‑pattern scoring?
3. How can `dungeonRunContract` metric targets (`UEC`, `EMD`, `STCI`, `CDL`, `ARR`) be validated against observed telemetry to tune surprise‑trigger density without manual playtesting?
4. What are the performance implications of calling `H.Invariants.sample_region_tile` from a Godot frame loop for each candidate trigger?
5. Can the same `historySelectorPattern` scoring logic used for `H.Node.choose_next` be reused to rank `SurpriseTrigger` candidates during runtime?
6. How should `CHATDIRECTOR` resolve conflicts when a chat session locked to `dungeonRunContract` requests a pattern that is invalid under current `aiChatHorrorProfile` bands?
7. What telemetry fields must be added to `history-selector-decision-event` to measure the effectiveness of liminal‑tag‑based node selection on player `ARR` and `EMD`?
8. How does the `DungeonMemory` ring’s planning envelope stay synchronized with live invariants when the player’s `HVF.mag` or `LSG` changes due to narrative events?
9. What is the optimal frequency for `H.Node.describe` calls in AI‑Chat sessions to avoid overwhelming the LLM context while providing accurate node‑level detail?
10. How can `directorPersonaContract` comfort policy (`maxDetPerSession`, `requiredCooldownRatio`) be enforced across multiple disconnected dungeon‑run chat sessions without session bleed?
11. What is the fallback behavior when `H.Selector.selectPattern` returns `NO_ELIGIBLE_PATTERN` during a dungeon‑design chat locked to a `mapStyle` enum?
12. How should `prismMeta` blocks be extended to capture which AI‑Chat template (`MapDesignWizard`, `DungeonNodeWalkthrough`) was active during each contract edit?
13. Can `dungeon-node-contract` `liminalTags` be used to automatically adjust Godot `LSG` coefficients without designer intervention?
14. What are the failure modes when `H.Node.choose_next` is called on a node with `topology.connections` that reference missing `dungeon-node-contract` IDs?
15. How can the `history-selector-pattern` `mechanicHookPreferences` be validated at load time to ensure referenced mechanic IDs exist in the registry?
16. What is the impact of `sessionLock: true` in `dungeonRunContract` on `H.Budget` consumption across multiple turns within the same map?
17. How should AI‑Chat session route envelopes capture `designMode` transitions to correlate `UEC`/`CDL` metrics with user satisfaction in map‑authoring conversations?
18. What are the security implications of exposing `H.Node.describe` via Lua to AI‑Chat agents that may attempt to enumerate node IDs beyond the current session?
19. How can `directorPersonaContract` `toneLabel` be mapped to concrete `StyleRouter` profile selections when generating node descriptions in AI‑Chat?
20. What is the minimum required `dungeonRunContract` metadata to allow `CHATDIRECTOR` to automatically promote a completed map‑design session to a `regionContractCard`?
21. How should `H.Dungeon.sample_tile` cache invariant snapshots to avoid redundant `H.Invariants` calls in a high‑frequency engine loop?
22. What are the boundary cases for `H.Director.constrainMetrics` when `intendedMetrics` contain `nil` values for some fields but not others?
23. How can `historySelectorPattern` `entertainmentTargets` be used to throttle the number of `SurpriseTrigger` activations in a single dungeon run?
24. What schema additions are needed in `ai-chat-template-contract` to enforce that a template can only be used when current `LSG` and `SHCI` match a specific band?
25. How can `DeadLedgerRef` be used to cryptographically attest that a `dungeonRunContract` was generated under a specific `aiChatHorrorProfile` and `mapStyle`?

---

## 2. Definition Requests (25)

1. Provide a formal definition for `SurpriseTrigger` in the context of `dungeonRunContract`, including required fields and validation rules for at‑most‑once execution.
2. Define the precise relationship between `tileClass` (in `dungeonRunContract`) and `role` (in `dungeon-node-contract`) to avoid contradictory invariant band assignments.
3. Define the expected behavior of `sessionLock` in `dungeonRunContract` when a chat session attempts to switch to a different `regionRef` mid‑conversation.
4. Define the term “liminal structure” as used in `liminalTags` and specify how it influences `LSG` and `HVF.mag` calculations.
5. Provide a precise definition for `historyCoupling` block in `historySelectorPattern`, including how `minSHCI` and `preferredBand` interact with `H.SHCI` snapshots.
6. Define the expected data shape returned by `H.Node.score_next_from_history` when a neighbor node has no `mechanicHooks` or `liminalTags`.
7. Define what constitutes a “valid” `mechanicHook` reference in `dungeon-node-contract` and how it resolves to a `mechanicContract` object.
8. Define the term “comfort policy” in the context of `directorPersonaContract` and distinguish it from `aiChatHorrorProfile` safety bands.
9. Define the contract lifecycle phase for `dungeon-node-contract` (currently phase 72) and its implications for promotion to higher tiers.
10. Define the difference between `topology.connections` kind `ritual_locked` and `locked`, and how each affects `H.Node.choose_next` scoring.
11. Provide a definition for `metricTargets` in `dungeonRunContract` vs. `dungeon-node-contract` and specify which takes precedence when narrowing is applied.
12. Define the expected output of `H.Dungeon.sample_tile` when the requested `nodeId` does not have an explicit `tileClass` in the contract.
13. Define the term “selector pattern” and distinguish `historySelectorPattern` from `StyleRouter` pattern selection.
14. Define the meaning of `routeStage` labels (`GoalClarification`, `RegionInvariants`, `RoomGraph`, `MechanicDrilldown`, `TelemetryReview`) in the context of AI‑Chat map‑design templates.
15. Define the contract validation rules for `dungeonRunContract` `tileClasses` arrays to ensure they reference valid node IDs from the attached graph.
16. Provide a definition for `invariantAlignment` as computed in `H.Dungeon.sample_tile` and specify the scoring algorithm precisely.
17. Define the term “experienceMode” for AI‑chat templates and list all allowed values relevant to dungeon design.
18. Define the expected behavior of `H.Director.applySafetyDecision` when `safetyDecision.decision` is `imply` and budget is near soft cap.
19. Define the `prismMeta` required fields for `dungeonRunContract` to enable auditability in a multi‑author constellation.
20. Define the term “narrowing” as applied to `dungeon-node-contract` invariant bands relative to parent `regionContractCard` envelopes.
21. Provide a precise definition for `ARR` (Ambiguous Resolution Ratio) in the context of a `dungeonRunContract` with `NODE_GRID` map style.
22. Define the expected behavior when `H.Node.choose_next` is called with a `patternId` that does not exist in the registry.
23. Define the difference between `strategy` verbs `soften` and `downshift` in `H.Director.applySafetyDecision`.
24. Define the term “AI‑chat session route envelope” and its required fields for tracking dungeon‑design conversations.
25. Provide a definition for `deadLedgerRef` usage in `director-comfort-policy` to link policy attestations to a verifiable ledger.

---

## 3. Detail Queries (25)

1. In the `H.Dungeon.can_spawn_trigger` example, how is `trigger_spec.metricBands` expected to be passed from the Lua surface to the C++ `SceneVault`?
2. What are the exact C++ function signatures for `H.Dungeon.sample_tile` and `H.Dungeon.can_spawn_trigger` when called via Lua‑C++ FFI in Godot?
3. In `dungeon-node-contract.v1.json`, should `invariantBands.HVF_mag` be a single band or a full object with `mag` sub‑field to match `HVF` invariant definition?
4. How does `H.Node.describe` determine `feel` classification when `CIC` and `AOS` fall into overlapping bands (e.g., CIC 0.5, AOS 0.45)?
5. What is the exact algorithm for `H.Node.score_next_from_history` to combine `liminalBonus`, `preferredBonus`, and `historyCoupling` weights into a single score?
6. How should `CHATDIRECTOR` populate `selectorContext` fields (`descriptorId`, `consentTier`, `routerStateId`) when the user is an anonymous chat participant?
7. What is the expected JSON structure of a `historySelectorPattern` instance that prefers `liminal:stairwell` and `mechanicHooks` containing `sanity_tick`?
8. In the `directorPersonaContract` schema, should `invariantBands.DET` be a single band or also include sub‑fields for different event types (ambient, event, climax)?
9. How does `H.Director.constrainMetrics` handle the case where `intendedMetrics` contains a value that is outside both the persona band and the consent cap?
10. What is the exact Lua table structure expected by `H.Selector.recordDecision` for the `selectorDecision` parameter?
11. How should `H.Run.next_turn` integrate `H.Node.choose_next` when the current run is in a `NODE_GRID` map style versus a `WIRE_AUTOMAP` style?
12. What is the exact format of the `explanation` string returned by `H.Node.choose_next` when multiple reasons (liminal, hooks, SHCI) contribute?
13. In `dungeonRunContract` instance `mansion_1f_lobby_DC`, why are `boss` tile classes empty, and what is the intended way to add a boss arena node later?
14. How does the `H.Dungeon` module register multiple `dungeonRunContract` instances for different dungeons within the same region?
15. What is the expected behavior of `H.Node.score_next_from_history` when `currentNodeId` has `topology.connections` that include one‑way edges marked `hidden`?
16. How does `H.Director.loadPersona` resolve `comfortPolicyRef` when the referenced policy ID is not yet loaded into the runtime registry?
17. What is the exact Lua code for `H.Contract.load("dungeonNodeContract", nodeId)` as used in `H.Node.describe`?
18. How should telemetry events from `H.Selector.recordDecision` be correlated with `dungeonRunContract` metrics to evaluate selector pattern effectiveness?
19. In the `director-comfort-policy` schema, how should `requiredCooldownRatio` be interpreted when `highDetMinutes` is 0?
20. What is the expected schema validation error when a `dungeon-node-contract` `metricTargets.UEC.max` exceeds the parent `regionContractCard` `UEC.max`?
21. How does `H.Node.describe` populate `metricsSnapshot` when `sessionId` is provided but no `H.Metrics.current_bands` data exists for that session?
22. What is the exact file path convention for storing `historySelectorPattern` instances that correspond to specific `mapStyle` enum values?
23. In the `history-selector-rule` example, what does `SEL-R005` rule binding `budgetfit` actually evaluate?
24. How should `CHATDIRECTOR` validate that an `aiChatTemplate` with `experienceMode: MapDesignWizard` is only used when a `dungeonRunContract` is active?
25. What is the expected JSON output of `H.Node.describe` when the node has `mechanicHooks` that reference `mechanicId` strings not yet resolved to full contracts?

---

## 4. Objection Identifiers (25)

1. The `dungeonRunContract` `sessionLock` boolean does not specify how a session is terminated if the user attempts to leave the map—this could lead to undefined behavior.
2. `H.Dungeon.sample_tile` relies on `H.Invariants.sample_region_tile`, but no contract defines the expected output format of that function; missing schema for invariant snapshots.
3. The `mapStyle` enum in `dungeonRunContract` mixes visual presentation (`ISO_CANVAS`) with navigation logic (`NODE_GRID`), which may cause conflicts when both are needed.
4. `H.Node.choose_next` explanation string concatenates reasons without localization support or structured output, making it brittle for non‑English AI‑Chat.
5. `dungeon-node-contract` `topology.connections` lacks a `weight` or `distance` field, limiting selector scoring to binary presence/absence of tags rather than traversal cost.
6. The `directorPersonaContract` `allowedTier` is a single value, but a director might need to operate across multiple tiers with different band constraints.
7. `H.Director.applySafetyDecision` returns a `styleHint` that is not tied to any enumerated vocabulary, making it unreliable for consistent AI‑Chat tone control.
8. The `historySelectorPattern` scoring algorithm in `H.Node.score_next_from_history` adds weights linearly, which may produce arbitrary total scores when combined with base score `1.0`.
9. `dungeonRunContract` `tileClasses` are defined as arrays of node IDs, but no validation ensures a node ID appears in only one class.
10. `H.Node.describe` `feel` classification uses hard‑coded thresholds (0.7, 0.4) that are not aligned with any configurable policy.
11. The `prismMeta` block in all schemas is defined as `additionalProperties: true`, which undermines strict schema validation and contract integrity.
12. `H.Dungeon.can_spawn_trigger` does not check the global `SceneVault` `triggered_events` list, only the local `is_active` flag; missing cross‑node trigger deduplication.
13. The `director-comfort-policy` `requiredCooldownRatio` uses `lowDetMinutes` and `highDetMinutes` without defining the time unit or how it maps to `H.Run` turns.
14. `historySelectorPattern` `appliesTo.minCIC`, `minAOS`, `minSPR` are declared but not used in the provided Lua scoring logic.
15. `H.Node.score_next_from_history` does not handle the case where `patternConfig.liminalTagPreferences` is `nil`, causing potential Lua errors.
16. The `dungeon-node-contract` schema uses `invariantBands.HVF_mag` as a simple band, but `HVF` is defined as an object with `mag` sub‑field elsewhere; inconsistency across schemas.
17. `H.Selector.selectPattern` expects `context.banterPlan` from `H.Banter.planResponse`, but no specification defines `H.Banter` API or its integration with selector.
18. `dungeonRunContract` `nodeHints` field is optional and not validated against node IDs in `tileClasses`, allowing references to non‑existent nodes.
19. `H.Director.constrainMetrics` may return `effectiveMetrics` with values clamped to `min` but the original `intendedMetrics` could be `nil`; ambiguity in required fields.
20. The `history-selector-pattern` `mechanicHookPreferences.weights` uses `preferredBonus` and `perMatchingHook` redundantly; scoring may double‑count preferred hooks.
21. `H.Node.describe` assumes `H.metrics` returns a full set of metrics, but in early session stages some metrics may be undefined.
22. `directorPersonaContract` `invariantBands` includes only `CIC`, `AOS`, `DET`, `SHCI`, but the `H.Director` API stub mentions all 11 invariants; missing fields.
23. `dungeonRunContract` `phase` is fixed to 72, but contract promotion rules require dynamic phase values; hard‑coded enum prevents lifecycle progression.
24. `H.Node.choose_next` returns a `debug.ranked` field that includes internal scoring details; exposing this in production could leak design intent or selector weights.
25. The `CHATDIRECTOR` wiring for map‑design templates relies on `routeStage` labels, but no registry or enumeration of allowed stage values exists, making template filtering ad‑hoc.

---

**Total: 100 items (25 per category)**  
Use this document as a checklist for refining the Horror\$Place wiring and AI‑chat lock‑in components. Each item can be assigned to a research spike, schema refinement task, or validation test.
