# Wiring Patterns for AI-Chat and Tooling

This document explains how wiring patterns let AI-chat agents and tools perform structured, multi-step operations (queries + file writes + DB inserts) in a single, deterministic compile step. It also defines the naming conventions for `goal.*`, `query.*`, and `insert.*` namespaces.

Wiring patterns are described by the `wiring-pattern.v1.json` schema and live under:

- `schemas/tooling/wiring-pattern.v1.json`
- `wiring-patterns/*.json`

## 1. What a wiring pattern is

A wiring pattern is a declarative recipe that tells an agent:

1. Which database queries to run first (`inputQuery.dbQueries`).
2. Which goal fields must be present (`inputQuery.requiredFields`).
3. Which files to emit in which repos (`outputTemplate.files`).
4. Which DB inserts to perform and in what dependency order (`outputTemplate.dbInserts`).

Patterns are identified by:

- `patternId` (e.g., `add-region-v1`)
- `patternKind` (e.g., `Add-Region`)

Agents select a pattern based on the user’s intent (goal) and the patternKind, then execute it as a small compile step: gather facts, fill templates, and emit outputs.

## 2. Namespaces: goal.*, query.*, insert.*

Template expressions inside wiring patterns use three main namespaces:

- `goal.*` – fields provided by the user or calling tool.
- `query.*` – results of `inputQuery.dbQueries`.
- `insert.*` – results of prior `dbInserts` in the same pattern.

These namespaces are read-only views into different parts of the execution context. They are not arbitrary; every reference must resolve to a known value before a pattern can be applied.

### 2.1 The goal.* namespace

`goal.*` fields come from the user request or higher-level planning context. Examples:

- `goal.regionId`
- `goal.cicTarget`
- `goal.detTarget`
- `goal.personaName`

The pattern’s `inputQuery.requiredFields` declares which `goal.*` keys must be present and non-empty before execution. An agent or orchestrator must check this list and either:

- Prompt the user for missing fields, or
- Abort and choose a different pattern.

Example:

```json
"requiredFields": [
  "regionId",
  "cicTarget",
  "aosTarget",
  "detTarget"
]
```

This means `goal.regionId`, `goal.cicTarget`, `goal.aosTarget`, and `goal.detTarget` must exist before running the pattern.

### 2.2 The query.* namespace

`query.*` contains the results of SQL queries declared in `inputQuery.dbQueries`. Each query has:

- `queryId` – a short name used under `query.*`.
- `sql` – the statement to execute against the constellation DB.
- `bindParams` – template expressions for parameters (often from `goal.*`).
- `expectRows` – expected cardinality (`zero-or-one`, `one`, `many`, `any`).

Results are exposed as JSON-like structures:

- `query.<queryId>` is an array of rows.
- `query.<queryId>[0].column_name` accesses a specific column in the first row.

Example:

```json
{
  "queryId": "get-atrocity-seeds-node",
  "sql": "SELECT vm_node_id AS repo_id, tier FROM constellation_vm_node WHERE vm_name = 'HorrorPlace-Atrocity-Seeds' LIMIT 1",
  "bindParams": [],
  "expectRows": "one"
}
```

Template usage:

- `{{query.get-atrocity-seeds-node[0].repo_id}}` – repo id for Atrocity-Seeds.
- `{{query.get-atrocity-seeds-node[0].tier}}` – tier label.

The orchestrator must enforce `expectRows`:

- `one`: error if zero or more than one row.
- `zero-or-one`: error if more than one row.
- `many`: error if zero rows.
- `any`: no cardinality restriction.

### 2.3 The insert.* namespace

`insert.*` refers to values returned from earlier `dbInserts` in the same pattern. Each `dbInserts` entry can declare:

- `table` – the table to insert into.
- `rowTemplate` – template for the row.
- `expectReturning` – fields expected from the insert (e.g., auto IDs).
- `dependsOn` – names of previous inserts this one references.

The orchestrator executes `dbInserts` in order, resolving `dependsOn` and collecting returned fields under:

- `insert.<table_name>.<field_name>`

Example:

```json
{
  "table": "constellation_file_manifest",
  "rowTemplate": {
    "vm_node_id": "{{query.get-atrocity-seeds-node.repo_id}}",
    "repository": "HorrorPlace-Atrocity-Seeds",
    "file_path": "regions/{{goal.regionId}}.json",
    "object_kind": "regionContractCard",
    "schema_uri": "region-contract-card.v1.json"
  },
  "expectReturning": [
    "file_id"
  ]
}
```

A later insert can reference:

```json
{
  "table": "constellation_file_node",
  "rowTemplate": {
    "file_id": "{{insert.constellation_file_manifest.file_id}}",
    "region_id": "{{goal.regionId}}",
    "cic": "{{goal.cicTarget}}",
    "aos": "{{goal.aosTarget}}",
    "det": "{{goal.detTarget}}"
  },
  "dependsOn": [
    "constellation_file_manifest"
  ]
}
```

The orchestrator must ensure:

- Inserts are run in a sequence that satisfies `dependsOn`.
- Any `insert.*` reference refers to a field listed in `expectReturning`.

## 3. Pattern selection by AI-chat agents

AI-chat agents should select wiring patterns as follows:

1. Interpret the user’s goal (e.g., “add a new region to Atrocity-Seeds”).
2. Map the intent to a `patternKind` (e.g., `Add-Region`).
3. Query the wiring pattern registry (e.g., `wiring-patterns/*.json`) to find candidate patterns with matching `patternKind`.
4. Check `inputQuery.requiredFields` against current `goal.*` values.
5. If fields are missing, ask the user or parent orchestrator for them.
6. Once a pattern is satisfied, commit to that pattern and move to execution.

Agents should not improvise SQL or file paths when a wiring pattern exists. The pattern is the source of truth.

## 4. Execution flow

Once a pattern is chosen and `goal.*` is complete, the orchestrator executes it in four phases:

1. **Prepare context**
   - Build a context object containing `goal.*`.
   - Load the pattern definition.

2. **Run `inputQuery.dbQueries`**
   - For each query:
     - Render `bindParams` using `goal.*`.
     - Execute the SQL against the constellation DB.
     - Validate result count against `expectRows`.
     - Store results under `query.<queryId>`.

3. **Render and emit `outputTemplate.files`**
   - For each file:
     - Render `repoId`, `path`, `templateRef`, `schemaId`, and `tier` using `goal.*` and `query.*`.
     - Lookup `templateRef` in a separate template catalog.
     - Render the file content using the same context.
     - Validate the file against `schemaId` before writing.
     - Write or stage the file in the target repo.

4. **Execute `outputTemplate.dbInserts`**
   - For each insert in order:
     - Ensure dependencies from `dependsOn` have already run.
     - Render `rowTemplate` using `goal.*`, `query.*`, and `insert.*`.
     - Execute the insert in the constellation DB.
     - If `expectReturning` is non-empty, capture the returned fields and store them under `insert.<table>.<field>`.

All of these steps should be wrapped in a transaction where possible, so that either all inserts and file registrations succeed, or none do.

## 5. Error handling and validation

Agents and orchestrators must handle errors deterministically:

- Missing `goal.*`: pattern not applicable; request more input or choose a different pattern.
- `expectRows` violation: pattern failure; do not attempt to improvise another SQL statement.
- Template reference error (unknown `goal.*`, `query.*`, or `insert.*`): pattern bug; report as a tooling error.
- Schema validation failure: reject the pattern output; if possible, present a structured error to the agent (field name, expected type/range).

Patterns should be kept small and composable. If a single pattern starts to mix multiple unrelated operations (e.g., add a region plus a persona plus a DeadLedger proof), split it into multiple patterns and orchestrate them at a higher level.

## 6. Naming conventions and placement

Recommended conventions:

- Pattern files:
  - Directory: `wiring-patterns/`
  - Filenames: `<patternId>.json` (e.g., `add-region-v1.json`)
- `patternId`:
  - Use lowercase with hyphens and a version suffix (`add-region-v1`).
- `patternKind`:
  - Use PascalCase with a verb and a noun (`Add-Region`, `Add-Persona`).
- Query ids:
  - `get-<object>-<qualifier>` (e.g., `get-atrocity-seeds-node`).
- Table names in `dbInserts`:
  - Use actual DB table names (e.g., `constellation_file_manifest`).

By following these conventions, AI-chat agents and tooling can treat wiring patterns as a small, stable language for “how to make changes” in the constellation, rather than guessing SQL, file paths, or schema usage.
