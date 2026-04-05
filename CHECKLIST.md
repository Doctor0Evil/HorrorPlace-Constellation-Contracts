# Horror$Place VM-Constellation Checklist
HorrorPlace-Constellation-Contracts  
Living Engineering and Research To‑Do List

---

## A. Core Vision and Doctrine

**Goal:** Keep the VM‑constellation’s purpose clear and machine‑enforceable so AI‑chat, tools, and humans all interpret Horror$Place the same way.

- [ ] Define a canonical invariants schema in `schemas/invariantsv1.json` with:
  - CIC, MDI, AOS, RRM, FCF, SPR, SHCI, HVF, LSG, DET.
- [ ] Define a canonical entertainment metrics schema in `schemas/entertainmentmetricsv1.json` with:
  - UEC, EMD, STCI, CDL, ARR.
- [ ] Add a short doctrine file `docs/DOCTRINE.md` that:
  - States: “No behavior without `QueryHistoryLayer → SetBehaviorFromInvariants`.”
  - Explains invariants and metrics in 1–2 lines each.
- [ ] Build a “failure atlas” in `docs/FailureAtlas.md`:
  - For each common problem (spawn camping, flat maps, inconsistent sequels, burnout), document:
    - Description.
    - Likely invariant failures (e.g., low LSG, zero HVF, low DET).
    - Relevant moods or contracts that would prevent it.

---

## B. Invariant‑Driven SFX and FX

**Goal:** Make SFX/FX the primary carrier of dread, always driven by invariants and metrics.

- [ ] Add `schemas/mood_contract.schema.json` to standardize mood contracts.
- [ ] Ensure `horror_audio.lua` (or equivalent) exists in a public API repo and:
  - Reads CIC, MDI, AOS, HVF, LSG, DET, UEC, EMD, ARR before choosing ambience or stingers.
  - Returns abstract tags/RTPC bundles, not engine specifics.
- [ ] Document SFX mapping rules in `docs/SFX_Invariants.md`:
  - How CIC influences ambience density.
  - How AOS influences muffling/static.
  - How LSG/HVF influence threshold and directional cues.
- [ ] Plan research tasks:
  - [ ] Collect at least 3 external references on horror sound design and atmosphere.
  - [ ] For each, note which aspects map to specific invariants/metrics.
- [ ] Design 3 audio “mood presets” as contracts:
  - `mood.audio.subdued_dread.v1`.
  - `mood.audio.constant_pressure.v1`.
  - `mood.audio.combat_nightmare.v1`.

---

## C. AI‑Mood Contracts (AI-Mood Layer)

**Goal:** Maintain a stable library of mood contracts (personalities) backed by schemas and Lua hooks.

- [ ] Keep `docs/AI-Mood.md` synchronized with JSON mood files under `moods/`.
- [ ] For each mood section in `AI-Mood.md`, ensure:
  - A corresponding JSON contract exists in `moods/`.
  - A corresponding Lua module name is registered in `registry/moods.json`.
- [ ] Validate all `moods/mood.*.json` against `schemas/mood_contract.schema.json`:
  - Enforce `[0.0, 1.0]` ranges.
  - Forbid unknown keys.
- [ ] Maintain `registry/moods.json` with:
  - `mood_id`.
  - `path` to JSON contract.
  - `lua_module` for behavior.
  - `requires_hooks` array (e.g., `["on_player_spawn", "on_tick"]`).
- [ ] Keep `scripts/mood_lint.lua` up to date so it:
  - Requires each `lua_module`.
  - Fails CI if hooks in `requires_hooks` are missing.
- [ ] Add a short guide `docs/HowTo_Add_Mood.md` describing:
  - Step 1: Define JSON contract.
  - Step 2: Implement Lua module with required hooks.
  - Step 3: Register in `registry/moods.json`.
  - Step 4: Run schema and mood lint CI.

---

## D. DreadForge for Action Hybrids

**Goal:** Use DreadForge to enforce atmosphere integrity in action titles (Battlefield‑style, extraction shooters).

- [ ] Store DreadForge contract as `moods/mood.dreadforge_resonance.v1.json`.
- [ ] Document DreadForge in `docs/moods/DreadForge_Resonance.md`:
  - Mood intent.
  - Invariant targets for battlefront, spawn, liminal tiles.
  - Experience targets for UEC, EMD, STCI, CDL, ARR.
- [ ] Ensure a corresponding Lua module exists:
  - `moods/DreadForge_Resonance.Contract.lua` (in target engine repo).
- [ ] Add a “spawn integrity” linter in this repo’s CI:
  - Validate any tile marked as spawn has CIC and LSG within DreadForge ranges.
  - Fail builds that violate thresholds.
- [ ] Provide a reference integration example in `docs/examples/DreadForge_Shooter.md`:
  - Example pseudo‑code for `on_player_spawn` hooks.
  - Explanation of how HVF and LSG discourage spawn camping.
- [ ] Track open research tasks:
  - [ ] Gather examples of map exploitation and spawn camping from public sources.
  - [ ] For each, define a DreadForge parameter change that would mitigate it.

---

## E. Schemas, Contracts, and CI

**Goal:** Make schemas and contracts the single source of truth for behavior and AI‑chat guidance.

- [ ] Keep these in `schemas/`:
  - `invariantsv1.json`.
  - `entertainmentmetricsv1.json`.
  - `mood_contract.schema.json`.
  - `event_contractv1.json`.
  - `personacontractv1.json`.
  - `stylecontractv1.json`.
- [ ] Ensure all contracts live under `contracts/` or `moods/`:
  - One object per file.
  - No inline lore or raw horror content.
- [ ] Maintain CI workflows that:
  - Run JSON Schema validation on all contracts.
  - Run custom linters (`mood_lint.lua`, future `event_lint.lua`, etc.).
- [ ] Document contract conventions in `docs/Contract_Patterns.md`:
  - Naming (`mood.*.json`, `event.*.json`, `persona.*.json`).
  - Required fields and common patterns.
- [ ] Provide a “contract‑first” flow in `docs/Workflow_Contract_First.md`:
  - Schema → Contract → Lint → Code → Engine wiring.

---

## F. Lua/C++ Adapter Patterns and Engine Integration

**Goal:** Keep Horror$Place APIs narrow and stable across engines, with clear adapter responsibilities.

- [ ] Standardize naming conventions:
  - `H.*` for invariants/metrics (e.g., `H.CIC`, `H.AOS`, `H.LSG`).
  - `Audio.*` for SFX logic (e.g., `Audio.compute_dread_rtpcs`).
  - `H.EngineAudio.*` for engine audio calls (RTPCs, events).
  - `Director.*` for event scheduling.
- [ ] Maintain adapter specs in `docs/API_Sockets.md`:
  - Describe what the Lua layer expects from C++/engine.
  - Describe what engine adapters may assume from Horror$Place.
- [ ] Provide engine‑agnostic adapter stubs in `cpp/`:
  - Example: `HorrorAudioDirector.h/.cpp`.
- [ ] Create per‑engine notes in `docs/engines/`:
  - Godot, Unreal, Frostbite‑like, etc.
  - Where to call Horror$Place APIs in each engine’s lifecycle.
- [ ] Add logging recommendations in `docs/Debug_Logging.md`:
  - Standard fields for debug logs (region, tile, mood_id, invariant sample).

---

## G. Telemetry, Sustainability, and Evolution

**Goal:** Use metrics and telemetry to improve horror quality and protect teams from overwork.

- [ ] Define a telemetry schema in `schemas/telemetry_sessionv1.json`:
  - Session ID, timestamp.
  - Aggregated UEC, EMD, STCI, CDL, ARR.
  - Optional tension curves and pacing labels.
- [ ] Document how to map telemetry to moods in `docs/Telemetry_Guide.md`:
  - How to detect when sessions fall outside a mood’s `experience_targets`.
  - How to adjust seeds or contracts in response.
- [ ] Encourage sustainable design:
  - Add a short note in `DOCTRINE.md` about limiting maximum DET exposure per session.
- [ ] Plan research:
  - [ ] Collect references on burnout and sustainable game dev.
  - [ ] Translate lessons into contract constraints (e.g., recommended DET bands per rating).
- [ ] Prototype a basic report script (external repo or tools directory) that:
  - Reads telemetry logs.
  - Highlights where target bands are consistently missed.

---

## H. AI‑Chat Integration and Guardrails

**Goal:** Make AI‑chat a reliable co‑designer for Horror$Place contracts and code, not a source of chaos.

- [ ] Add `docs/AI_CoDesigner_Charter.md`:
  - Explain invariants and metrics.
  - Describe repository layout (`schemas/`, `contracts/`, `moods/`, `registry/`, `scripts/`).
  - State rules: schema‑first, no raw content, narrow APIs only.
- [ ] Document recommended prompt patterns:
  - “Extend this schema.”
  - “Create a new mood contract constrained by this schema.”
  - “Generate Lua hooks for this mood contract only.”
- [ ] Maintain a curated set of “exemplar outputs” in `docs/examples/`:
  - Good contracts.
  - Good Lua modules.
  - Good adapter snippets.
- [ ] Add guardrail scripts where feasible:
  - Check for forbidden patterns (raw content markers, unknown keys) in new files.
- [ ] Periodically review and update the checklist:
  - Add new research tasks and constraints as the constellation evolves.

---

This checklist is intended as a living document. Update it whenever new schemas, moods, adapters, or telemetry practices are introduced so that HorrorPlace-Constellation-Contracts remains the authoritative spine for the entire VM‑constellation.
