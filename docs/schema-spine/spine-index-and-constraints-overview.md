---
title: Spine Index Routing Triples and Constraint Families
version: 1.0.0
doctype: schema-spine-overview
tier: 1
role: Canonical overview of objectKind/tier/repo routing and invariant/metric constraint families for AI-authoring.
---

# Spine Index Routing Triples and Constraint Families

This document explains how the schema spine encodes routing triples `(objectKind, tier, repo)` and how invariant/metric constraint families are attached to those entries for AI-authoring, CI, and RotCave orchestration.

It is the human-readable counterpart to the machine-readable schemas:

- `schemas/spine/spine-triple-entry.v1.json`
- `schemas/spine/constraint-family.v1.json`

## 1. Spine Triple Entries

A spine triple entry is a single row in the schema spine that states, in a machine-checkable way:

- What kind of object is being authored (`objectKind`).
- Which tier it belongs to (`tier`).
- Which repository is authoritative for that artifact (`repo`).
- Which schemas, invariants, metrics, and constraint families apply.

Each entry is an instance of `spine-triple-entry.v1.json`. The `spineId` field is a stable key that downstream tools (RotCave, Wandering-entity, or external copilots) can select before generation.

Conceptually, choosing a spine entry like `"spine.moodContract.tier1.Horror.Place.v1"` forces AI-authoring to:

- Target the correct repository and path for the artifact.
- Use only the schemas and file extensions allowed for that contract family.
- Respect the invariant/metric bindings and constraint families specified by the entry.

### 1.1 Example Contract Families

The following table sketches common contract families and their typical routing triples.

| Spine Id                                      | objectKind         | tier   | repo                            | phase               |
|----------------------------------------------|--------------------|--------|---------------------------------|---------------------|
| spine.moodContract.tier1.Horror.Place.v1     | moodContract       | tier1  | Horror.Place                    | contract-authoring  |
| spine.eventContract.tier1.Constellation.v1   | eventContract      | tier1  | HorrorPlace-Constellation-Contracts | contract-authoring  |
| spine.regionCard.tier2.Black-Archivum.v1     | regionContractCard | tier2  | HorrorPlace-Black-Archivum      | pcg-authoring       |
| spine.seedCard.tier2.Atrocity-Seeds.v1       | seedContractCard   | tier2  | HorrorPlace-Atrocity-Seeds      | pcg-authoring       |
| spine.personaContract.tier2.Spectral.v1      | personaContract    | tier2  | HorrorPlace-Spectral-Foundry    | contract-authoring  |
| spine.policyEnvelope.tier1.Orchestrator.v1   | policyEnvelope     | tier1  | Horror.Place-Orchestrator       | runtime-policy      |

Each of these entries can additionally specify:

- `schemaRefs` to the canonical JSON Schemas.
- `allowedFileExtensions` (e.g., `.json` only for contract cards).
- `targetPathPatterns` (e.g., `schemas/contract/*.json`, `registry/*.ndjson`).
- `requiredInvariants` and `requiredMetrics`.
- `constraintFamilies` that must be enforced.

## 2. Invariant and Metric Bindings

The schema spine treats invariants (CIC, MDI, AOS, RRM, FCF, SPR, RWF, DET, HVF, LSG, SHCI) and metrics (UEC, EMD, STCI, CDL, ARR) as a shared vocabulary.

A spine triple entry can declare:

- `requiredInvariants`: invariants that must appear in the contract or its bound telemetry.
- `optionalInvariants`: invariants that may appear but are not required.
- `requiredMetrics`: metrics that must appear in associated telemetry or policy envelopes.
- `optionalMetrics`: metrics that can be tracked for analysis and evolution.

This lets contract families cleanly encode which parts of the invariant spine they rely on. For example:

- A `regionContractCard` entry will often require CIC, AOS, DET, and SHCI.
- A `seedContractCard` entry might require CIC, MDI, LSG, and HVF.
- A `policyEnvelope` entry focused on runtime adaptation may require UEC, ARR, and CDL.

By putting these bindings in the spine, AI-authoring tools can infer which fields must be populated before validation and telemetry wiring are considered complete.

## 3. Constraint Families and Formulas

Constraint families are named, versioned objects that describe how invariants and metrics interact. They are instances of `constraint-family.v1.json`.

Each family:

- Has a `constraintId` such as `CIC_AOS_DET_cap_v1`, `ARR_v1`, or `CDL_STCI_review_threshold_v1`.
- States which invariants and metrics it consumes via the `inputs` array.
- Identifies a primary `outputField` that is constrained.
- Declares its `kind` (cap, floor, band, review-threshold, telemetry-derived, composite-score).
- Optionally declares tier-specific behavior via `tiers` and parameterization in `parameters`.

The spine triple entry then references these families by `constraintId` in its `constraintFamilies` field.

### 3.1 Examples of Constraint Families

Below are example constraint families that can be instantiated as JSON objects under the `constraint-family.v1.json` schema.

1. `CIC_AOS_DET_cap_v1`  
   - Kind: `cap`  
   - Applies to: `invariant`  
   - Inputs: `["CIC", "AOS"]`  
   - Output field: `DET`  
   - Symbolic form: `DET <= f(CIC, AOS)` with parameters defining how historical consequence and archival opacity jointly cap tension.

2. `ARR_v1`  
   - Kind: `composite-score`  
   - Applies to: `metric`  
   - Inputs: `["UEC", "DET"]`  
   - Output field: `ARR`  
   - Symbolic form: `ARR = g(UEC_normalized, DET_normalized)` where `g` is a ratio or composite function with tunable coefficients.

3. `CDL_STCI_review_threshold_v1`  
   - Kind: `review-threshold`  
   - Applies to: `invariant-metric-pair`  
   - Inputs: `["CDL", "STCI"]`  
   - Output field: `CDL`  
   - Symbolic form: `if CDL >= threshold and STCI.upper >= confidence then flag for review`.  
   - Review policy: route to higher-tier research repositories for analysis.

The exact algebra is implemented in engine code and telemetry analysis stacks. The constraint family objects focus on naming, inputs, outputs, and parameterization so that tools can recognize and route them consistently.

## 4. Spine-Aware Validator Behind Reaper-Contracts

Reaper-contracts already encode:

- The request context (tiers, persona state, audit, routing plan).
- The list of repositories and artifacts under consideration.
- Wandering-entity hints for guard behavior.

A spine-aware validator sits behind Reaper-contracts and is responsible for:

1. Selecting the correct spine triple entry based on an authoring intent:
   - `objectKind` (e.g., `regionContractCard`).
   - `tier` (from the request and persona state).
   - Optional `repo` hints.

2. Resolving a concrete routing decision:
   - `targetRepo` (from the spine entry).
   - `targetPath` (matching `targetPathPatterns`).
   - `schemaRef` (from `schemaRefs`).
   - `invariantCaps` and `metricCaps` implied by the constraint families.

3. Returning either:
   - A normalized envelope that Reaper-contracts and RotCave can use to drive generation.
   - A machine error indicating that no valid spine entry exists or that constraints cannot be satisfied at the requested tier.

Wandering-entity guards and RotCave Lua do not re-implement routing semantics. They call the spine-aware validator as an oracle and apply guard logic to the resolved descriptor (repository existence, tier boundaries, allowed extensions, and artifact shape).

## 5. Using the Spine for AI-Authoring and CI

Once the spine index is populated with triple entries and constraint families:

- AI-authoring tools must select a `spineId` before proposing any file.
- Reaper-contracts call the spine-aware validator to resolve routing and constraint context.
- Wandering-entity guards validate the proposed artifacts against that context.
- CI regenerates a spine index artifact that can be used for impact analysis, consumer mapping, and drift detection.

This yields a deterministic, auditable path from a high-level request such as “Tier1 moodContract” to:

- A specific repository and path where the file must live.
- A specific schema it must validate against.
- A well-defined invariant/metric envelope and set of constraint families that govern both authoring and telemetry.

The same spine definitions can be safely shared with external copilots and AI-chat systems, allowing them to participate in the HorrorPlace constellation without direct access to vault content or implementation details.
