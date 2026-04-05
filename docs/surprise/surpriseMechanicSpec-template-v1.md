---
title: Surprise Mechanic Spec – TEMPLATE
version: 1.0.0
doctype: spec-v1
schemaref:
  - schema.HorrorPlace-Constellation-Contracts.surpriseMechanicContract.v1.json
  - schema.HorrorPlace-Constellation-Contracts.registry-surpriseMechanics.v1.json
invariantsused:
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
metricsused:
  - UEC
  - EMD
  - STCI
  - CDL
  - ARR
tiers:
  - research
deadledgersurface:
  - bundleattestation
  - agentattestation
aiauthoringcontract: horrorplace-constellation-ai-authoring-contract-v1
---

# Surprise Mechanic Spec – TEMPLATE

## Purpose and Scope

This document defines a single surprise mechanic at the design-spec level, before it is narrowed into one or more surpriseMechanicContract instances and referenced from NDJSON registries. It is intended for use in HorrorPlace-Constellation-Contracts as a contract-first description that AI agents and humans can compile into concrete contracts and implementations.

## Mechanic Identity and Category

- Working name: (human-readable)
- Proposed mechanic ID prefix: (e.g., SURP.PERMISD.WATCHED-LANE.v1)
- Category: one of PerceptualMisdirection, SystemicTiming, EnvironmentTopology, SocialNPCBehavior, DiegeticSystem
- Intended intensity band: mild, moderate, severe, or adult
- Target tiers: public, vault, lab, research

Describe the core fantasy and use case for this mechanic in one or two short paragraphs, using implication and contract language rather than explicit horror content.

## Invariant and Metric Targets

### Invariant Slice

Describe which invariants are most important for this mechanic and how they should behave.

- CIC / MDI focus:
- AOS / RRM / FCF focus:
- DET expectations and caps:
- LSG / HVF / SHCI behaviors:

Then provide an expected numeric envelope in prose that will later be encoded as invariantPreconditions and detCaps/lsgCaps in surpriseMechanicContract.

### Entertainment Metric Intent

Describe the intended effects on UEC, EMD, STCI, CDL, and ARR.

- UEC band and direction:
- EMD (evidential mystery) effect:
- STCI (safe-threat contrast) behavior:
- CDL (cognitive dissonance) expectations:
- ARR (ambiguous resolution ratio) goals:

Indicate whether these are small adjustments inside a region envelope or strong pushes that require tighter policy envelopes.

## Coupling to History and SHCI

Explain how this mechanic is constrained by local history and SHCI.

- Required history anchors (regions, events, legends):
- Whether the mechanic is a loose echo, bound to a region, bound to an event chain, or legend-only:
- How SHCI should influence frequency, clarity, or form of manifestations:

This description will be mapped onto the shciMode and invariantPreconditions.SHCI fields in the contract.

## Placement and Adapter Expectations

Summarize how this mechanic should select eligible locations and adapters.

- Eligible spatial contexts (thresholds, ridges, borders, interiors):
- Expected LSG and HVF ranges:
- Required engine adapter types (e.g., godot-2d-forest, unreal-fps, lua-runtime):

Indicate any constraints that adapter implementations must respect, such as no direct asset references or required calls into H.Surprise APIs.

## UEC/EMD/STCI/CDL/ARR Caps and DET/LSG Limits

Define caps and safety considerations in prose that will be turned into detCaps, lsgCaps, and target metric bands.

- Maximum DET contribution per session and per region:
- Maximum LSG band this mechanic may operate in:
- Recommended UEC and ARR bands to avoid fatigue or burnout:
- Any additional guardrails for CDL and STCI:

These guardrails will be enforced by CI when narrowing this spec into concrete contracts.

## Expected Implementations and Telemetry

List expected implementation variants and telemetry requirements.

- Planned implementation descriptors (engine and ID labels only):
- Minimal telemetry to capture per activation (e.g., observed UEC/ARR deltas, DET spikes):
- How telemetry should feed back into future revisions of this spec:

Keep implementation notes abstract enough to remain GitHub-safe and contract-focused.

## CI and Validation Hooks

Describe how CI should validate any surpriseMechanicContract and registry entries derived from this spec.

- Required subset relationships to policyEnvelope and regionContractCard bounds:
- Required presence of deadledgerref for certain tiers or intensity bands:
- Any additional linting rules specific to this mechanic category:

CI will use these expectations to enforce the three-artifact path: spec, contract card, and implementation metadata.
