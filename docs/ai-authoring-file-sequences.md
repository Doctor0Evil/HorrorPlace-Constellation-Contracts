# AI Authoring File Sequences and Graph-Aware Planning

This document defines how ai-safe-authoring-contract.v1 and discovery-contract.v1 encode file-sequence ordering and constellation-graph intent for AI authoring sessions in the Horror$Place VM-constellation.

## 1. Overview

External AI platforms must treat authoring as a contract-first, multi-phase pipeline rather than emitting free-form code or JSON. Each session begins with a discovery-contract.v1 envelope that proves which spines, manifests, registries, and graph indices were loaded, followed by an ai-safe-authoring-contract.v1 plan that declares every artifact, routing triple, and sequence position before any payload is generated.

CHAT_DIRECTOR uses these contracts to normalize the plan into one or more AiAuthoringRequest envelopes and to enforce that every artifact in an AiAuthoringResponse maps back to a planned artifactId. Any attempt to generate unplanned files or exceed bundle size limits is treated as a validation error and is rejected before merge.

## 2. Discovery as a Hard Gate

The discovery-contract.v1 schema captures which schema spines, invariants spines, metrics spines, routing spines, manifests, registries, and constellation-graph indices the agent has actually read. Each entry includes a ref and a hash so that CHAT_DIRECTOR and CI can confirm that authoring was planned against a precise version of the routing spine and manifests, not stale or guessed structures.

A discovery envelope must validate against its JSON Schema and must resolve all spineRefs and manifestRefs on disk before CHAT_DIRECTOR.plan will accept any ai-safe-authoring-contract.v1 payload. This turns discovery into a hard precondition: if the routing spine or manifests are missing or mismatched, there is no legal way for external platforms to propose a plan or emit files.

The runtimeCapabilities section in discovery-contract.v1 expresses the effective capabilities for the session derived from manifests, including allowed objectKinds, allowed tiers, allowed repos, maxFilesInBundle, and maxBundlesPerSession. ai-safe-authoring-contract.v1 must stay within these limits, otherwise CHAT_DIRECTOR rejects the session as incompatible with the currently loaded routing state.

## 3. Authoring Contract as Session Spine

The ai-safe-authoring-contract.v1 schema defines a session-level envelope that binds agent identity, authoring profile, discoveryRef, consent, safety constraints, bundlePlan, and telemetry. The profile and consent sections capture tier caps, max bundle sizes, intensity bands, and Dead-Ledger surfaces so that safety and entertainment metrics are considered at planning time instead of being patched after payload generation.

Each planned artifact in bundlePlan.artifacts encodes its objectKind, tier, targetRepo, targetPath, schemaRef, CHAT_DIRECTOR phase, sequence metadata, graphPlan, invariantsTouched, metricsTouched, and optional noveltyTargets. This turns the plan into a fully machine-checkable blueprint: routing triples are explicit, schema bindings are pinned, and the CI layer can verify that no artifact attempts to cross tier or repo boundaries beyond what manifests allow.

CHAT_DIRECTOR normalizes this bundlePlan into concrete AiAuthoringRequest envelopes, each with a bounded set of artifacts taken from the contract. During validate-response, it insists that every PrismEnvelope and file produced by AI maps back to exactly one artifactId, that no extra artifacts appear, and that file count per bundle never exceeds the declared maxFilesInBundle. The contract thus becomes the spine for the entire session.

## 4. File-Sequence Ordering

File-sequence ordering is represented explicitly by the sequence object on each planned artifact. The phaseIndex field encodes the authoring phase order within the bundle, where lower indices correspond to earlier phases, such as schema definitions or registry entries, and higher indices correspond to later phases, such as runtime bindings or PCG implementations.

The dependsOnArtifacts field defines a per-artifact dependency set that expresses graph-like ordering constraints at the artifact level. For example, a seedContractCard that references a regionContractCard must list that region artifactId in dependsOnArtifacts so CHAT_DIRECTOR can ensure that the region card is generated, validated, and accepted before the seed card is allowed to proceed.

CHAT_DIRECTOR enforces phase ordering by accepting only those artifacts whose dependencies have already completed successfully in earlier phases. If an AiAuthoringResponse includes an artifact whose dependencies are not yet satisfied, or that jumps ahead of its phaseIndex, the response is rejected with a structured error code. This makes file-sequence ordering a first-class property of the plan rather than a soft convention.

## 5. Constellation-Graph Planning

The graphPlan block on each artifact declares how the artifact intends to interact with the constellation graph before any nodes or edges are created. For graphIntent values such as create-nodes or attach-edges, the nodes array lists new or modified graph nodes with nodeId, graphNodeKind, and optional regionAnchorId, while the edges array lists typed relationships such as region-seed, region-persona, seed-event, or event-style.

CHAT_DIRECTOR and graph-aware Rust helpers can run pre-generation validation on graphPlan, checking that node IDs are unique within the proposed bundle, that node kinds are permitted, and that edge kinds connect only allowed pairs of nodes. They also verify that region-anchored nodes respect region-level invariant bands and entertainment metric envelopes derived from the invariants and metrics spines.

Because graphPlan is part of the authoring contract, CI can assert that no artifact attempts to create forbidden edges or modify nodes outside the allowed scope for the session. This turns the constellation graph into a governed contract surface instead of an emergent property of filenames and IDs, and it directly ties AI authoring intent to graph-level invariants and metrics.

## 6. Invariants, Metrics, and Telemetry

The invariantsTouched and metricsTouched lists on each planned artifact provide a fine-grained declaration of which invariants and entertainment metrics the artifact intends to read or modify. Each invariant entry carries a code and bandRef, while each metric entry carries a code and targetBand, both resolved against the invariants and metrics spines.

During validation, CHAT_DIRECTOR can intersect these requested bands with authoring profile caps and consent, and can reject any artifact whose requested range exceeds the allowed bands for its objectKind and tier. This ensures that CIC, HVF, LSG, DET, UEC, ARR, and related metrics are governed at plan time rather than after files land in the repo.

The telemetry object links the authoring contract to NDJSON and SQLite streams that record chat-director-validation-metrics and Dead-Ledger policy decisions. By including expected code-band envelopes in expectedBands, the contract can hint to Reaper-contracts which spectral code bands the session expects to occupy, tying AI authoring behavior back into the same spectral-boundary framework used for pipeline health and reaping policies.

## 7. Cross-Platform Integration Pattern

Because both discovery-contract.v1 and ai-safe-authoring-contract.v1 are defined as JSON Schemas with additionalProperties set to false, any external platform that can read JSON, perform HTTP requests, and honour schema validation can integrate with the constellation. The integration sequence is:

First, the platform loads the public spines and selected manifests, then constructs and submits a discovery-contract.v1 envelope. Once CHAT_DIRECTOR validates discovery and confirms that all refs resolve, the platform submits an ai-safe-authoring-contract.v1 plan that stays within the capabilities declared in discovery.

Second, the platform requests skeletons for each planned artifact from a constellation-side skeleton generator. The model fills these skeletons while respecting structure, invariants, and metric bands, and returns them wrapped in AiAuthoringResponse and PrismEnvelope objects. CHAT_DIRECTOR and CI perform schema, routing, invariant, metric, graph, and safety checks before any merge.

This pattern forces Perplexity, Qwen, Copilot, and local LLMs into the same contract-first behavior: they must first declare what will change, where it lives in the constellation graph, and how it obeys invariants and metrics, and only then are they allowed to generate code or JSON. File-sequence ordering and graph planning become enforceable properties, not informal norms, which improves predictability, auditability, and horror metric alignment across the entire VM-constellation.
