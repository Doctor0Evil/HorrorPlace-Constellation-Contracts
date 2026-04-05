# NDJSON Authoring Patterns

Repository: HorrorPlace-Constellation-Contracts  
Path: docs/ndjson-authoring-patterns.md  

This guide defines the normative patterns for authoring NDJSON registries in the VM-constellation. It is written for AI agents, tools, and humans who emit or review NDJSON files in any constellation-aware repository.

NDJSON here means:

- One complete JSON object per line.
- UTF-8 encoding, LF line endings.
- No comments, no pretty-printing, no trailing commas.

NDJSON registries are the **only** discovery surface for entities such as regions, events, personas, styles, surprise mechanics, seeds, and related contracts.

---

## 1. Core NDJSON rules

Every NDJSON registry in the constellation must follow these structural rules:

- Exactly one JSON object per line.
- Each line is self-contained: no multi-line strings, no trailing commas, no comments.
- Files end with a final newline and contain only UTF-8 text.
- All objects validate against a canonical registry schema (`schema...registry-*.v1.json`).
- No raw horror content, narrative prose, or large assets in any registry line.

At minimum, a registry entry must include:

- `id`: stable, globally unique identifier.
- `schemaref`: canonical schema URI this entry conforms to.
- `tier`: one of `public`, `vault`, `lab`, `research` (or documented superset).
- At least one implementation reference: `artifactid`, `cid`, or `deadledgerref`.
- Optional `metadata` or domain-specific metadata defined by the schema.

IDs must be immutable once published. Deprecation or replacement is handled by new entries, not by editing old IDs in place.

---

## 2. Canonical fields and shapes

The schema spine defines a shared base shape for NDJSON registry entries, extended by domain-specific schemas. Common patterns:

### 2.1 Base registry entry

All NDJSON registries share the same base registry concepts:

- `id`: string (e.g., `EVT-ARAL-0001`, `REG-ARAL-0001`, `PER-ARAL-1001`, `STY-DREAD01`).
- `schemaref`: canonical schema URI for this registry entry type.
- `tier`: `public | vault | lab | research`.
- `artifactid`: opaque identifier for the underlying contract or asset reference (repo path, IPFS CID, etc.).
- `cid`: optional content-address hash (IPFS-style or equivalent).
- `deadledgerref`: optional opaque reference to a Dead-Ledger proof envelope, required for certain tiers/intensity bands.
- `metadata`: small, structured object with human-readable tags (names, status, timestamps), not free-form content.

Domain-specific registry schemas add their own required fields (e.g., `regionIds`, `personaType`, `styleCategory`, `mechanicContractId`), but the base pattern and field names remain stable.

### 2.2 Common registry families

Typical NDJSON registries include:

- Regions: `registry/regions.ndjson` (or equivalent), referencing region contracts and invariant bundles.
- Events: `registry/events.ndjson`, linking event contracts to regions, personas, and metrics.
- Personas: `registry/personas.ndjson`, describing persona wiring and invariant coupling.
- Styles: `registry/styles.ndjson`, binding style contracts to tiers and platforms.
- Surprise mechanics: `registry/registry-surprise-mechanics.ndjson`, indexing surpriseMechanicContract instances.
- Example registries: `registry/examples/.../*.example.ndjson`, used strictly as schema-conformant examples.

All follow the one-object-per-line NDJSON pattern and validate against their specific `registry-*.v1.json` schemas.

---

## 3. Authoring workflow for NDJSON lines

When generating or editing NDJSON, follow this exact sequence:

1. **Choose the registry and schema**

   - Identify the correct registry file (e.g., `registry/regions.ndjson`, `registry/personas.ndjson`, `registry/registry-surprise-mechanics.ndjson`).
   - Load the canonical schema for that registry (e.g., `schemas/registry/registry-personas.v1.json`).

2. **Resolve IDs and cross-links**

   - Select or generate a unique `id` following the registry’s pattern (e.g., `PER-ARAL-1001`).
   - Resolve any referenced IDs (regions, events, styles, bundles, contracts) from their own registries first; never invent unknown references.

3. **Populate required fields only**

   - Fill `id`, `schemaref`, `tier`, and at least one of `artifactid`, `cid`, or `deadledgerref`.
   - Add domain-specific required fields from the schema (e.g., `personaType`, `mechanicContractId`, `category`, `intensityBand`).
   - Populate optional fields only if they are meaningful and schema-permitted.

4. **Serialize as compact NDJSON**

   - Emit the object as a single-line JSON string with no spaces except inside string values.
   - Ensure there are no comments, no trailing commas, and no extra punctuation.

5. **Validate**

   - Run schema validation against the corresponding `registry-*.v1.json`.
   - Run registry linting: check ID uniqueness, required fields, `schemaref` prefix, presence of reference fields, and any invariant/metric range rules.

6. **Commit via prism envelope (for AI agents)**

   - Wrap any new NDJSON lines inside a prism envelope containing `targetRepo`, `targetPath`, `schemaref`, `tier`, and `referencedIds`.
   - Respect changeset limits and agent profile capabilities (e.g., `maxChangesetSize`, `allowsRegistryWrite`).

---

## 4. Example NDJSON lines

These examples illustrate the expected NDJSON style: one object per line, compact, and structurally valid. They are safe to use as patterns when authoring new entries.

### 4.1 Persona example (personas.example.ndjson)

A minimal persona registry line (single NDJSON object):

```ndjson
id=PER-ARAL-1001,schemaref=https://horror.place/schemas/registry/registry-personas.v1.json,deadledgerref=deadledgerrefPER-ARAL-1001v1,artifactid=cid:personaPER-ARAL-1001v1,createdAt=2026-01-15T03:10:00Z,status=active,personaType=narrative-voice,historicalAnchor=eventRefs:EVT-ARAL-0001|regionRefs:REG-ARAL-0001,role=archivist-narrator,reliabilityWeight=0.72,behaviorProfile=modes:reveal|redact|contradict,telemetryHooks=emitOn:eventTypes:EVT-TYPE-INVESTIGATION|EVT-TYPE-TRANSITION,invariantCoupling=CIC.hintDensityRange:0.6-1.0|AOS.redactionRange:0.5-1.0,metricInfluence=UEC.deltaRange:0.05-0.15|EMD.deltaRange:0.03-0.10|CDL.deltaRange:0.05-0.12|ARR.deltaRange:0.02-0.08,prismMetaRef=prism:persona:PER-ARAL-1001
```

Note: the actual shape (JSON vs key=value shorthands) must match the canonical `registry-personas.v1.json` schema. The important properties here are: stable ID, `schemaref`, opaque references, and invariant/metric coupling represented as numbers and small ranges.

### 4.2 Surprise mechanic registry example (registry-surprise-mechanics.ndjson)

A single NDJSON line indexing a surprise mechanic contract:

```ndjson
{"id":"SURP-PERMISD-0001","schemaref":"schema.HorrorPlace-Constellation-Contracts.registry-surpriseMechanics.v1.json","tier":"research","mechanicContractId":"SURP.PERMISD.WATCHED-LANE.v1","category":"PerceptualMisdirection","intensityBand":"moderate","styleid":"STY-DARK-FOG-01","regionHint":"REG-ARAL-0001","implementationDescriptor":{"adapters":[{"engine":"godot-2d-forest","id":"surp_permisd_watched_lane_godot_v1"},{"engine":"unreal-fps","id":"surp_permisd_watched_lane_unreal_v1"}]},"deadledgerref":"dlref:surp-permisd-0001-attested"}
```

This line:

- Points to the registry schema via `schemaref`.
- References a concrete `mechanicContractId` defined by `surpriseMechanicContract.v1.json`.
- Declares `category`, `intensityBand`, optional `styleid` and `regionHint`.
- Provides an `implementationDescriptor` with engine adapters as opaque IDs.
- Includes `deadledgerref` for governance and proof.

---

## 5. Quality and safety expectations

When authoring NDJSON in any constellation repo:

- Treat registry entries as **contracts**, not free-text rows.
- Never embed raw horror, narrative transcripts, or explicit descriptions in NDJSON; use IDs and hashes only.
- Respect invariant and metric ranges defined in the schema spine (e.g., invariants in `[0,1]`, `DET` in `[0,10]`, metrics in `[0,1]`), when present in registry lines.
- Keep NDJSON files focused: a single entity family per registry file (e.g., only personas in `personas.ndjson`).
- Use example files (`*.example.ndjson`) as canonical teaching surfaces for AI agents and humans; keep them in lockstep with schemas.

CI and pre-commit packs are expected to:

- Validate all NDJSON registries against their schemas.
- Enforce ID uniqueness, `schemaref` prefixes, and `deadledgerref` structure.
- Reject NDJSON entries with missing required fields or out-of-range invariant/metric values.
- Block raw content and unsafe references in contract-only repositories.

By following these patterns, NDJSON remains the stable, machine-queryable backbone of the VM-constellation, and AI-assisted authoring can safely scale across repositories and engines.
