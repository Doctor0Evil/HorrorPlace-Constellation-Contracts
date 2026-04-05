---
title: Nature Seeds and NPC-Behavior Shards
doctype: protocoldoc
version: 1.0.0
status: draft
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
  - T1public
  - T2private
deadledger_surface:
  - zkpproofschema
  - verifiersregistry
  - bundleattestation
  - spectralseedattestation
---

# Nature Seeds and NPC-Behavior Shards

This document specializes the existing AI authoring and QPU Datashard vocabulary for “seeds-of-nature” and npc_behavior artifacts across the HorrorPlace VM-Constellation. It defines how Tier-1 contracts in HorrorPlace-Constellation-Contracts and Tier-2 seeds in HorrorPlace-Atrocity-Seeds should represent wilderness, weather, and environmental behaviors as implication-only, schema-bound NDJSON lines and derived Lua behavior modules.

The goal is to give AI-chat tools and human engineers a precise, cross-repo contract for nature seeds and NPC behaviors that is compatible with the existing invariants, entertainment metrics, Dead-Ledger governance, and one-file-per-request discipline.

---

## 1. Scope and repo roles

This document applies to two primary repositories:

- HorrorPlace-Constellation-Contracts (Tier 1 public): owns schemas and authoring contracts for nature-related QPU Datashards, AI authoring envelopes, and checklists.
- HorrorPlace-Atrocity-Seeds (Tier 2 private): owns PCG seeds for events, regions, and nature sequences, including invariant-bound “nature-seed” and “environment-controller” shards serialized as NDJSON or JSON files.

Other repos are consumers, not definers, of nature seeds:

- HorrorPlace-Codebase-of-Death (Tier 2): consumes nature seeds and produces Lua npc_behavior modules that bind to invariants and metrics but do not store seeds or lore.
- Horror.Place (Tier 1): mirrors public-facing registry entries that reference nature seeds via git URIs and Dead-Ledger references.
- HorrorPlace-Dead-Ledger-Network (Tier 3): records bundle and spectral/nature attestation entries referenced via deadledgerref fields but never stores seed payloads.

No nature seeds, environment contracts, or npc_behavior implementations may be defined directly in Horror.Place-Constellation-Contracts; this repo holds only schemas, AI authoring contracts, and vocabulary documents like this one.

---

## 2. Nature shard kinds in the QPU Datashard vocabulary

This section extends the QPU Datashard vocabulary with four shard kinds specialized for nature and environmental behavior. These shard kinds do not introduce new invariants or metrics; they reuse the existing invariants and entertainment metrics as state vectors and intent bands.

### 2.1 Shared envelope fields

All nature-related datashards must conform to schemas/qpudatashard-envelope-v1.json and use the common envelope fields:

- schema: always the canonical QPU Datashard schema URI.
- shardid: globally unique identifier, for example qpu.nature.seed.ravine.mist.v1.
- shardkind: one of the nature-specific kinds defined below.
- schemaref: URI or name of the primary contract schema implemented by the shard, such as nature-seed-contract-v1.json or npc-behavior-profile-v1.json.
- repotier: T1-core, T2-vault, or T3-lab.
- reponame: canonical repository name, such as HorrorPlace-Atrocity-Seeds.
- safetytier: standard, mature, or research.
- intensityband: integer from 0 to 10.
- deadledgerref: optional opaque handle; required for high-intensity shards.
- invariants: numeric invariants CIC, MDI, AOS, RRM, FCF, SPR, RWF, DET, HVF, LSG, SHCI.
- metrics: numeric entertainment metrics UEC, EMD, STCI, CDL, ARR.
- functionkind: a functional role label such as nature-seed, environment-controller, or npc-behavior-profile.
- functionid: stable identifier for a specific function or profile, such as npc.behavior.ravine.birds.v1.
- timestamp and version: creation time and semantic version of the shard definition.

All shards must be self-contained: every NDJSON line must include these envelope fields with no reliance on context inherited from adjacent lines.

### 2.2 Shardkind: nature-seed

The nature-seed shardkind describes a single nature seed that binds a region’s invariants to a procedural sequence involving weather, flora, fauna, or environmental motion. These shards live primarily in HorrorPlace-Atrocity-Seeds.

Additional required fields in the shard payload:

- payloadref:
  - registry: expected registry file name, such as registry/nature-seeds.ndjson.
  - id: seed ID, such as seed.nature.ravine.mist.band3.
  - hash: canonical content hash of the seed JSON in Atrocity-Seeds.
  - path: repository-relative path to the seed JSON file.
- nature:
  - regionid: ID of the region or bundle this seed applies to.
  - channel: high-level channel, such as weather, water, flora, fauna, or atmosphere.
  - mode: qualitative mode, such as persistent-background, episodic-surge, or threat-adjacent.
- invariants: full invariant vector for the region band this seed is tuned to.
- metrics: intended entertainment metric bands while the seed is active.

Nature-seed shards do not contain descriptive text, narrative, or explicit imagery. Behavior is implied by invariant and metric bands plus the channel and mode fields.

### 2.3 Shardkind: environment-controller

The environment-controller shardkind describes higher-level control envelopes for regional environment behavior. Controllers gather multiple nature seeds and describe how they should be scheduled or blended without specifying runtime code.

Additional required fields:

- payloadref:
  - registry: controller registry path, such as registry/environment-controllers.ndjson.
  - id: controller ID, for example env.controller.ravine.storm-cycle.v1.
  - hash and path: canonical hash and path for the controller definition JSON in Atrocity-Seeds or Liminal-Continuum.
- controller:
  - regionid: region or bundle ID.
  - phases: array of named environment phases such as calm, build, peak, and decay.
  - seedids: array of nature seed IDs referenced by the controller.
- invariants: invariant vector describing the region band in which this controller is valid.
- metrics: target metric envelopes across phases, especially UEC, EMD, and ARR.

Environment-controller shards must remain implication-only, expressing only IDs, invariant and metric bands, and phase labels.

### 2.4 Shardkind: npc-behavior-profile

The npc-behavior-profile shardkind describes environment-driven NPC behavior profiles that will later be implemented as Lua or C++ behavior logic in HorrorPlace-Codebase-of-Death.

Additional required fields:

- payloadref:
  - registry: NPC behavior registry path, such as registry/npc-behavior.ndjson.
  - id: profile ID, such as npc.behavior.ravine.carrion-birds.v1.
  - hash and path: content hash and path for the JSON profile in a vault repo (Codebase-of-Death or Atrocity-Seeds, depending on where behavior templates live).
- behavior:
  - species: neutral species label, for example bird, insect, herd-animal.
  - role: high-level role, such as sentinel, scavenger, or ambient.
  - coupling:
    - invariant_fields: list of invariants this behavior responds to, such as CIC, LSG, or HVF.
    - metric_fields: list of metrics this behavior responds to, such as UEC or ARR.
- invariants: target invariant bands in which this profile should be activated.
- metrics: expected effect on metrics while the NPC is active, constrained to allowed ranges.

Npc-behavior-profile shards contain no dialogue or narrative text; they describe bands, roles, and coupling fields only.

### 2.5 Shardkind: nature-telemetry-summary

Nature-telemetry-summary shards are optional Tier-3 lab artifacts that summarize how nature seeds and NPC behaviors affected metrics during test sessions.

Required fields:

- payloadref:
  - registry: telemetry registry path.
  - id: telemetry summary ID.
  - hash and path: canonical hash and path for the telemetry summary file.
- telemetry:
  - regionid: region ID.
  - seedids: list of nature seed IDs active during the session.
  - behaviorids: list of NPC behavior profile IDs active during the session.
  - duration_seconds: integer duration.
- invariants: invariant vector at the region for context.
- metrics:
  - uec_delta: observed UEC change band for the session.
  - arr_delta: observed ARR change band.
  - optional fields for EMD, STCI, and CDL deltas.

Telemetry shards must never contain raw logs, player identifiers, or BCI samples; they are summaries only.

---

## 3. Tier-2 nature seed NDJSON pattern (Atrocity-Seeds)

This section defines how HorrorPlace-Atrocity-Seeds should represent nature seeds and related entries as NDJSON or JSON files.

### 3.1 Nature seed JSON body

Nature seed files in HorrorPlace-Atrocity-Seeds should conform to a dedicated schema, for example schemas/nature-seed-contract-v1.json. A typical JSON body for events/nature/ravine-mist-band3.json contains:

- id: seed.nature.ravine.mist.band3
- schemaref: canonical URI for nature-seed-contract-v1.json.
- bundleref:
  - id: ID of the invariant bundle in HorrorPlace-Black-Archivum.
  - path: git URI into Black-Archivum for the bundle file.
- invariants: CIC, MDI, AOS, RRM, FCF, SPR, RWF, DET, HVF, LSG, SHCI in canonical ranges.
- metricsintent: UEC, EMD, STCI, CDL, ARR target bands.
- safetytier: standard, mature, or research, consistent with bundle attestation.
- intensityband: integer 0 to 10.
- nature:
  - regionid: region key in Horror.Place registries or Black-Archivum.
  - channel: weather, water, flora, fauna, or atmosphere.
  - mode: persistent-background, episodic-surge, or threat-adjacent.
- optional deadledgerref when intensityband is high and the charter requires explicit attestation.

Nature seed JSON files must not include description, narrative, loretext, dialogue, rawasseturl, or base64data fields.

### 3.2 Nature seed registry NDJSON

Atrocity-Seeds should maintain a registry/nature-seeds.ndjson file where each line references a seed JSON and its canonical hash. Each registry line is a simple JSON object with:

- id: seed ID.
- schemaref: nature-seed-contract-v1.json URI.
- path: git URI and fragment to events/nature/... seed file.
- invariantbundle: git URI to the Black-Archivum bundle.
- hash: canonical hash of the seed JSON.
- tier: T2private.
- deadledgerref: optional when required by safetytier and intensityband.
- tags: array of classification tags, such as water, ravine, fog, or migratory.

The registry is the only discovery surface; engines and directors must not query the Atrocity-Seeds repository directly at runtime.

### 3.3 QPU Datashard NDJSON view

When a nature seed must cross repo or tier boundaries, it should be projected into a QPU Datashard NDJSON line with shardkind nature-seed. The line must include the QPU envelope fields as described in section 2 and a payloadref pointing back to the seed JSON and registry entry.

This NDJSON projection is meant for orchestrators, CI, and ranking systems, not for player-facing engines.

---

## 4. NPC behavior Lua and C++ views

While seeds and QPU Datashards live in Atrocity-Seeds and Constellation-Contracts, NPC behavior implementations live in HorrorPlace-Codebase-of-Death as Lua modules or C++ components. This section defines how npc_behavior modules should bind to nature-related shards without embedding seed content.

### 4.1 Lua npc_behavior module shape

A canonical Lua behavior module, for example engine/npc_behavior/ravine_carrion_birds.lua, should:

- Accept environment invariants and metrics obtained through the narrow APIs H. and Telemetry. rather than direct table access.
- Accept one or more npc-behavior-profile QPU Datashards as configuration, pre-decoded into Lua tables.
- Expose deterministic functions such as init(profile), update(dt, invariants, metrics), and serialize_debug_state().

Behavior modules must not:

- Access Atrocity-Seeds repositories directly over the network or via git.
- Contain narrative text, dialogue, or explicit horror content.
- Bypass the invariants and Telemetry APIs to access raw invariant tables.

### 4.2 C++ behavior graph integration

C++ engine code that orchestrates npc_behavior modules should:

- Treat QPU Datashard NDJSON as configuration input: parse lines into in-memory structs matching the qpudatashard-envelope-v1 schema and npc-behavior-profile payload shape.
- Use the invariant and metric bands from the shard to determine activation conditions, such as whether a behavior is eligible in a region with given CIC, LSG, and HVF values.
- Never persist QPU shards with additional narrative content or assets.

The shared contract is that both Lua and C++ see the same functional fields from npc-behavior-profile shards, and that any changes to the shard schema are made in HorrorPlace-Constellation-Contracts only.

---

## 5. AI authoring directives for nature and NPC shards

AI-chat tools must follow additional constraints when generating nature seeds and npc_behavior-related files.

### 5.1 Artifact type and repo resolution

When the user requests a nature seed or NPC behavior artifact, AI-chat must:

- Map the request to an artifact type:
  - seedevent or seedregion for nature-seed JSON in Atrocity-Seeds.
  - registryentry for nature-seeds registry lines.
  - qpudatashard for nature-seed, environment-controller, npc-behavior-profile, or nature-telemetry-summary NDJSON lines.
  - enginecode for Lua npc_behavior modules in Codebase-of-Death.
- Choose targetrepo and targetpath from the constellation index:
  - Nature seeds: HorrorPlace-Atrocity-Seeds, under events/nature/ or regions/nature/.
  - Nature seed registries: HorrorPlace-Atrocity-Seeds, registry/nature-seeds.ndjson.
  - NPC behavior Lua: HorrorPlace-Codebase-of-Death, engine/npc_behavior/.
  - Nature QPU Datashards: any repo that needs cross-tier NDJSON, with schemaref set to qpudatashard-envelope-v1.

If the user requests a location that conflicts with these roles, AI-chat must propose a corrected targetrepo and path in its aifileenvelope description.

### 5.2 Research-first behavior for nature seeds

If AI-chat lacks sufficient information to generate a nature seed or NPC behavior profile safely, it must emit an airesearchplan envelope rather than an incomplete file. The research plan should include actions such as:

- Inspect existing invariant bundles for a region in HorrorPlace-Black-Archivum.
- Map user intent (for example, “ravine fog that increases dread but rarely attacks”) to invariant bands and metric intents.
- Determine safetytier and intensityband based on charter and platform rules.
- Confirm that the artifact belongs in HorrorPlace-Atrocity-Seeds or HorrorPlace-Codebase-of-Death.

Only after requiredactions are satisfied should AI-chat generate nature seed JSON, registry entries, QPU Datashard NDJSON, or Lua behavior code.

### 5.3 Seed and shard content rules

For all nature seeds and QPU Datashards:

- No raw lore or history: seeds and shards may only reference upstream data by IDs, hashes, and schema URIs.
- Only IDs, invariants, metrics, safetytier, intensityband, and tags may be used to characterize seeds and behaviors.
- Dead-Ledger references:
  - Required for any shard or seed with intensityband at or above thresholds defined in Atrocity-Seeds and Dead-Ledger policies.
  - Must match the proof envelope conventions of zkpproof schemas and verifiers registries.

If the user describes story content, AI-chat must transform it into invariant and metrics bindings, channel and mode choices, or tags, not narrative fields.

---

## 6. CI and validation hints

To keep nature seeds and NPC behavior shards consistent, repositories should integrate the following checks:

- Atrocity-Seeds:
  - Schema validation of nature seed JSON against schemas/nature-seed-contract-v1.json.
  - Hash verification tying registry/nature-seeds.ndjson entries to seed files.
  - Content-leak scans ensuring no narrative or asset URLs appear.
  - Validation of QPU Datashard NDJSON lines against qpudatashard-envelope-v1.json when used.

- Codebase-of-Death:
  - Lua linting and static checks to ensure npc_behavior modules only access invariants and metrics via the narrow H. and Telemetry APIs.
  - Optional checks ensuring QPU Datashard configuration is well-formed.

- Constellation-Contracts:
  - Schema validation for qpudatashard-envelope-v1.json and nature-specific schemas.
  - Validation of this protocol document’s examples against schemas when possible.

---

## 7. Summary of nature shard kinds

For reference, the nature-related shard kinds introduced in this document are:

- nature-seed: invariant-bound nature seed linking a region bundle to an environment channel and mode.
- environment-controller: control-level shard describing how multiple nature seeds are phased and combined.
- npc-behavior-profile: profile describing how NPC behaviors couple to invariants and metrics in nature contexts.
- nature-telemetry-summary: Tier-3 summary of how nature seeds and behaviors affected entertainment metrics in test sessions.

All four kinds reuse the existing invariant and metric vectors and must be serialized as self-contained QPU Datashard NDJSON lines when crossing repo or tier boundaries.
