# BCI Binding Lifecycle, Schema Evolution, and Input-Domain Constraints

## 1. Lifecycle of lab vs standard vs mature bindings

### 1.1 Binding tiers

HorrorMappingConfig.BCI bindings are explicitly tiered:

- `tier: "lab"`: Experimental mappings used in offline replay and synthetic traces only.
- `tier: "standard"`: Production-capable mappings usable in live sessions under policy caps.
- `tier: "mature"`: Long‑lived, well‑validated mappings with strong telemetry evidence and stable behavior.

The tier is recorded in each binding entry under a `lifecycle` block:

```json
{
  "id": "bci-map-sprint-inhibition-001",
  "schemaVersion": "HorrorMappingConfig.BCI.v1",
  "lifecycle": {
    "tier": "lab",
    "promotionCriteriaRef": "bci-binding-promotion-criteria-v1",
    "createdBy": "human-reviewer-or-tool-id",
    "createdAt": "2026-04-30T00:00:00Z"
  }
}
```

### 1.2 Promotion path and checks

Promotion from `lab` to `standard` and from `standard` to `mature` follows a fixed path:

- `lab → standard`:
  - Schema gate: binding must validate against `HorrorMappingConfig.BCI.v1` with `additionalProperties: false`.
  - Invariants/metrics gate: input/output ranges must sit within canonical invariant and entertainment metric spines.
  - Numerical gate: mapping must pass synthetic fixtures and differential envelope checks for worst‑case tension and exposure.
  - Policy gate: mapping must respect `bci-intensity-policy-v1` caps and DeadLedger BCI safeguards.

- `standard → mature`:
  - All `lab → standard` checks.
  - Telemetry evidence: minimum number of sessions, cohorts, and exposure hours under DeadLedger‑approved policies, with no safety‑critical violations.
  - Stability checks: low variance in key metrics (e.g., DET, overloadFlag incidence) under similar contexts across sessions.

Promotion decisions are encoded by updating the `lifecycle` block:

```json
"lifecycle": {
  "tier": "standard",
  "promotionCriteriaRef": "bci-binding-promotion-criteria-v1",
  "promotedFrom": "lab",
  "promotedAt": "2026-06-15T00:00:00Z",
  "promotionEvidenceRef": [
    "bci-telemetry-summary-v1:dataset-123",
    "bci-telemetry-summary-v1:dataset-456"
  ]
}
```

### 1.3 `promotionCriteria` field

The schema gains a formal `promotionCriteria` subobject under `lifecycle`, which is purely descriptive and references shared criteria artifacts:

```json
"lifecycle": {
  "type": "object",
  "required": [ "tier", "promotionCriteriaRef" ],
  "properties": {
    "tier": { "type": "string", "enum": [ "lab", "standard", "mature" ] },
    "promotionCriteriaRef": {
      "type": "string",
      "description": "ID of a shared promotion criteria artifact that defines quantitative thresholds."
    },
    "promotedFrom": { "type": "string", "enum": [ "lab", "standard" ] },
    "promotedAt": { "type": "string", "format": "date-time" },
    "promotionEvidenceRef": {
      "type": "array",
      "items": { "type": "string" },
      "description": "References to telemetry summaries and reports that justify promotion."
    }
  },
  "additionalProperties": false
}
```

The shared criteria artifact (e.g., `bci-binding-promotion-criteria-v1.json`) defines numeric thresholds for CI checks and telemetry evidence.

## 2. Schema evolution and compatibility

### 2.1 Schema versioning strategy

When `HorrorMappingConfig.BCI.v1` is extended with new fields (such as additional output channels or family codes), evolution follows these rules:

- Backwards-compatible changes (additive, optional fields):
  - Increment minor version (e.g., `v1.1`) and keep `v1` as a stable alias for older bindings.
  - New fields must be optional and must not change semantics of existing fields.
  - Tooling must treat older bindings as valid but potentially feature-limited.

- Breaking changes (renames, removals, semantics shifts):
  - Introduce a new major schema (e.g., `HorrorMappingConfig.BCI.v2`) with a new `$id`.
  - Keep `v1` read-only and mark it as deprecated but supported for replay and migration.

Each binding’s `schemaVersion` field is mandatory and must match the concrete schema ID, for example:

```json
"schemaVersion": "HorrorMappingConfig.BCI.v1"
```

### 2.2 Gating AI-chat via `schemaVersion`

AI-chat assistance is constrained using `schemaVersion` gating:

- Prompts for mapping edits must include the target `schemaVersion` and forbid changing it.
- AI is only allowed to add or modify fields that exist in that schema version.
- For new features or family codes, maintainers update the schema and CI, then revise AI playbooks to target the new `schemaVersion`.

This gating mechanism ensures that AI-generated bindings cannot silently drift into undocumented fields or bypass the schema spine.

### 2.3 Feature flags in manifests

For runtime and gradual rollout, a lightweight `features` block is added to binding manifests:

```json
"features": {
  "enableNewOutputChannel": false,
  "enableExperimentalFamilyCode": false
}
```

These flags:

- Are advisory for runtime behavior (e.g., gating evaluation of optional outputs).
- Must never allow AI to add fields outside the schema.
- Are themselves governed by CI and policy gates, ensuring that enabling a feature flag without the required evidence fails CI.

SchemaVersion gating remains the primary constraint for AI and CI; feature flags are a controlled runtime overlay.

## 3. Input-domain constraints in schema vs code

### 3.1 Schema-level constraints

The schema encodes “hard” structural and numeric bounds:

- Field presence and types (`required`, `type`).
- Canonical ranges for normalized metrics (e.g., `0.0` to `1.0` for `stressScore`).
- Enumerations for modes, family codes, and tier values.
- `additionalProperties: false` for all structural nodes.

Examples:

```json
"stressScore": {
  "type": "number",
  "minimum": 0.0,
  "maximum": 1.0
},
"visualOverloadIndex": {
  "type": "number",
  "minimum": 0.0,
  "maximum": 1.0
}
```

These constraints guarantee that:

- NDJSON and config artifacts are structurally consistent.
- Values outside known safe numeric domains are rejected before runtime.

### 3.2 Runtime assertions and derived constraints

Rust runtime code enforces “derived” constraints and safety margins that cannot be expressed easily in JSON Schema:

- Differential inequality checks relating parameters to exposure envelopes (e.g., ensuring that worst-case tension `T_max` over `T_session` stays below `T_cap`).
- Cross-field relationships (e.g., certain combinations of `CIC`, `DET`, and `SHCI` must fall inside defined corridors).
- Rate-of-change limits and Lipschitz-style bounds for mappings.

These assertions live in dedicated Rust crates (e.g., `bci-mapping-safety`) and are invoked in CI and runtime guardrails. They treat schema-validated values as inputs and reject configurations that violate analytic safety conditions.

### 3.3 `inputDomainVersion` tags tied to invariants/metrics spine

To align bindings with the invariants/metrics spine, bindings carry an explicit `inputDomainVersion` tag:

```json
"inputDomain": {
  "version": "invariants-metrics-spine-v2",
  "metricsSchemaRef": "bci-metrics-envelope-v2",
  "invariantsSchemaRef": "invariants-metrics-spine-v2",
  "notes": "Defines ranges and correlation rules for CIC, DET, SHCI, UEC, EMD, STCI, CDL, ARR."
}
```

This tag:

- Binds a binding to a specific invariants/metrics version.
- Allows CI to ensure that bindings are only used with compatible telemetry and pipelines.
- Provides a clear migration path when invariants or metrics evolve.

Schema-level constraints ensure that the tag is present and syntactically valid; Rust and CI enforce semantic compatibility by verifying that the referenced spine versions exist and that input ranges and relationships match the declared spine.

## 4. Summary of responsibilities

- Schema (`HorrorMappingConfig.BCI.v1+`):
  - Structure, required fields, basic numeric bounds, tier and lifecycle encoding, schemaVersion and inputDomainVersion, and feature flag presence.

- CI and Rust safety crates:
  - Differential envelopes, cross-field inequalities, policy caps, telemetry evidence thresholds, and promotion decisions.

- AI-chat tooling:
  - Locked to explicit `schemaVersion` and `inputDomainVersion`.
  - May only adjust fields allowed by those schemas and must not alter lifecycle tier or promotion metadata without human-approved workflows.
