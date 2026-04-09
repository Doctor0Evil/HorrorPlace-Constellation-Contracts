---
invariants_used: [CIC, MDI, AOS, RRM, FCF, SPR, RWF, DET, HVF, LSG, SHCI]
metrics_used: [UEC, EMD, STCI, CDL, ARR]
tiers: [standard]
deadledger_surface: [zkpproof_schema, verifiers_registry]
---

# Telemetry Aggregator for Resurrection Runs  
## Research and Production Checklist

This checklist turns the conceptual closed-loop in Section 4.3 into a concrete plan for a production-grade Lua NDJSON aggregator that matches existing HorrorPlace tooling and can be safely called by CHATDIRECTOR and CI.

## 1. Research Objects and Style Alignment

Before writing code, review the existing constellation telemetry and lint ecosystem to match patterns for CLI behavior, error handling, and schema usage.

### 1.1 Lua telemetry and lint patterns

Study the following in Horror.Place and HorrorPlace-Constellation-Contracts:

- Existing Lua scripts under `scripts/` and `tooling/lua/`, especially:
  - `scripts/moodlint.lua` (argument parsing, exit codes, deterministic ordering).
  - Any NDJSON-processing tools (line-by-line parsing, dkjson usage).
- CLI norms:
  - `lua script.lua --help` prints brief usage and exits 0.
  - Non-zero exit codes for validation failures vs configuration errors.
  - Errors written to stderr, machine-readable output on stdout.

Outcome: a short internal note (or header comment template) capturing:

- Required Lua version and libraries (e.g., dkjson, file helpers).
- Preferred argument parsing pattern (simple `arg` table, `--flag=value` support).
- Canonical error-code mapping (`0 = success`, `1 = validation`, `2 = config`).

### 1.2 JSON Schema and telemetry artifacts

Align the aggregator’s input and output with existing telemetry schemas:

- Input: `resurrection-run.v1.json` (NDJSON log, one run per line).
  - Ensure fields for identity (`runId`, `sessionId`, `seedId`, `eventId`), invariants snapshot, metrics-before/after, gate result, quality block.
- Output: `resurrection-aggregate.v1.json` (one JSON object per group).
  - Use the canonical schema `schemas/telemetry/resurrection-aggregate.v1.json`.
  - Validate the field set and types against other telemetry envelopes such as:
    - `schemas/telemetry/session-metrics-envelope.v1.json`
    - `schemas/telemetry/runtime-drift-event.v1.json`

Decide:

- Which fields are **required** for aggregation (e.g., `seedId`, `resurrectionKind`, `quality.localScore`).
- How to handle malformed or incomplete lines (warn and skip vs hard fail).

### 1.3 CHATDIRECTOR telemetry loop and RWF

Consult CHATDIRECTOR’s telemetry and RWF usage:

- How `RWF` and `reviewStatus` are used to promote/demote artifacts.
- How telemetry slices are consumed by authoring tools and selectors.

Draft a small “resurrection quality spec” snippet:

- Definitions of `localScore`, `avgLocal`, `p25Local`, `p75Local`.
- Interpretation of `rwfScore` and `rwfHint.status` (`experimental`, `provisional`, `stable`, `quarantine`).
- Governance expectations: overflow penalties, required floors for “stable”.

This spec justifies the aggregator’s scoring math and can be linked from CHATDIRECTOR docs.

---

## 2. Search and Reading Tasks

Use targeted searches to tighten the aggregator’s implementation choices.

### 2.1 CLI ergonomics and NDJSON handling

Search:

- “Lua CLI tools json ndjson dkjson”
- “lua command line utilities patterns”

Goals:

- Confirm idiomatic ways to:
  - Accept `-` as stdin, or explicit `--input path`.
  - Stream NDJSON line-by-line (`io.lines`) with graceful error handling.
  - Distinguish `stdout` (aggregates) from `stderr` (diagnostics).

### 2.2 Percentile computation and performance

Search:

- “percentile approximation lua implementation”
- “streaming percentile estimation github lua”

Decisions:

- For initial version:
  - Use simple in-memory arrays and full sort for percentiles, assuming NDJSON logs are small or CI-bounded.
- Document the method explicitly:
  - “Nearest-rank” style: index \(i = \lfloor p * (n - 1) + 1 \rfloor\) over 1-based arrays.
- Add a comment/TODO:
  - For `N > X` (e.g., 100k records), consider migrating to:
    - A streaming/approximate percentile algorithm, or
    - A Rust/Python aggregator with the same percentile contract.

### 2.3 Telemetry pipelines and schema-driven aggregation

Search:

- “game telemetry aggregation pipelines ndjson”
- “schema-driven telemetry aggregation”

Objectives:

- Validate that NDJSON-in / NDJSON-out is compatible with:
  - Existing log collectors (e.g., orchestrator fan-in).
  - External big-data pipelines if logs are later shipped to Python/Rust.
- Confirm that emitting one aggregate object per line is acceptable for:
  - Registries (if stored as `registry/telemetry/resurrections.ndjson`).
  - Downstream AI tools that read aggregates.

### 2.4 RWF scoring mapping

Within CHATDIRECTOR-related docs, search for:

- “RWF reliability scoring telemetry design”
- “reviewStatus”, “quarantine”, “stable” status semantics.

Use findings to:

- Align `rwfScore` semantics with existing governance (0–1 numeric reliability).
- Derive default thresholds, such as:
  - `rwfScore >= 0.8` → `stable`
  - `0.6–0.8` → `provisional`
  - `0.4–0.6` → `experimental`
  - `< 0.4` → `quarantine`
- Decide penalties per overflow:
  - Each `detOverflow` reduces `rwfScore` by a fixed delta (e.g., 0.05).
  - Similar penalties for `cdlOverflows` and `arrTooLow`.

---

## 3. Structure Goals for the Lua Aggregator

The aggregator should be a single, small Lua program under `scripts/` with clear API and deterministic behavior.

### 3.1 File layout and entrypoint

- File path: `scripts/telemetry_resurrection_aggregate.lua`
- Single Lua module with:
  - `main()` as entrypoint, wired to `if ... then main() end`.
  - Internal helper functions:
    - `parse_args(arg)` – returns config (input path, output path, caps, config file).
    - `read_runs(source)` – yields or returns a list of parsed run records.
    - `group_runs(runs)` – groups by `(seedId, resurrectionKind, eventId?)`.
    - `compute_stats(group, caps)` – returns aggregate table for one group.
    - `compute_percentiles(values)` – returns `p25`, `p75` (documented method).
    - `compute_rwf(aggregate, rwf_cfg)` – returns `rwfScore`, `status`.
    - `emit_ndjson(aggregates, out)` – writes JSON lines in deterministic order.

Command-line interface:

- `--input PATH` (or `-i PATH`), `-` for stdin.
- `--output PATH` (or `-o PATH`), default stdout.
- Optional policy flags:
  - `--det-cap N`
  - `--cdl-cap N`
  - `--arr-min N`
  - `--config PATH` to load a small JSON policy file (see RWF section).

Exit codes:

- `0` – success, aggregates written.
- `1` – validation or parsing failure (malformed input beyond tolerances).
- `2` – configuration or argument error.

### 3.2 Schema-aligned data model and guards

Treat schemas as authoritative:

- `resurrection-run.v1` (input):
  - Expect at minimum:
    - `seedId` (string).
    - `eventId` (string or null).
    - `resurrectionKind` (string).
    - `quality.localScore` (0–1).
    - `metricsBefore` and `metricsAfter` with UEC, EMD, STCI, CDL, ARR.
    - Gate block fields (`gate.distance`, `gate.noveltyThreshold`, `gate.allowed`).
    - Safety indicators per run (e.g., flags for DET/CDL/ARR breaches).
- `resurrection-aggregate.v1` (output):
  - Required fields updated by the aggregator:
    - `id`
    - `version`
    - `recordedAt`
    - `seedId`
    - `eventId` (nullable)
    - `resurrectionKind`
    - `counters.*`
    - `scores.*`
    - `metricsRealized.*`
    - `safety.*`
    - `rwfHint.*`

Guard rails:

- On read:
  - If `run.seedId` or `quality.localScore` missing, log warning and skip that line.
  - If metrics are missing, skip per-run deltas but keep counts and scores where possible.
- On compute:
  - Clamp all computed probabilities and scores into valid ranges before emitting:
    - `localScore`, `avgLocal`, `p25Local`, `p75Local`, `rwfScore`, `avgARR` ∈ `[0, 1]`.
- On write:
  - Ensure `additionalProperties = false` schema constraints are honored in aggregate.

### 3.3 Grouping, determinism, and percentiles

Grouping key:

- Composite key `(seedId, resurrectionKind, eventId)`:
  - Use `""` or `null` consistently for missing `eventId`.
- Deterministic ordering of output:
  - Sort groups lexicographically by:
    1. `seedId`
    2. `resurrectionKind`
    3. `eventId` (with null/empty last).

Percentile computation:

- When there are `n` local scores:
  - Sort ascending.
  - `p25` index \(i_{25} = \lfloor 0.25 * (n - 1) + 1 \rfloor\).
  - `p75` index \(i_{75} = \lfloor 0.75 * (n - 1) + 1 \rfloor\).
- Document this explicitly at the top of the script and in comments so a Rust/Python port can match exactly.

### 3.4 Metric deltas and safety counters

Per-group metrics:

- For each run, compute deltas where both before and after exist:
  - `ΔUEC = after.UEC - before.UEC`
  - `ΔEMD`, `ΔSTCI`, `ΔCDL` similarly.
- Store per-run arrays and then compute:
  - `metricsRealized.avgUECdelta` = mean of `ΔUEC`.
  - `metricsRealized.avgEMDdelta`, `avgSTCIdelta`, `avgCDLdelta`.
  - `metricsRealized.avgARR` = mean of `after.ARR`.

Safety counters per group:

- `safety.detOverflows` – number of runs where:
  - `det > det_cap` (provided by CLI/config) or flagged by run record.
- `safety.cdlOverflows` – number of runs where:
  - `cdl > cdl_cap`.
- `safety.arrTooLow` – number of runs where:
  - `after.ARR < arr_min`.

These caps are inputs to the aggregator, not hardcoded, enabling environment-specific tuning.

---

## 4. RWF Scoring and Policy Layer

Encapsulate reliability scoring in a dedicated function with externalized policy.

### 4.1 Function contract

Lua function signature (conceptual):

- `compute_rwf(group_stats, policy) -> rwfScore, status`

Inputs:

- `group_stats`:
  - `avgLocal`, `p25Local`, `p75Local`
  - `attempts`, `allowed`, `blocked`
  - `detOverflows`, `cdlOverflows`, `arrTooLow`
  - Metric deltas if needed.
- `policy` (from config or defaults):
  - Base weights:
    - `w_avg`, `w_p25`, `w_p75`
  - Penalty magnitudes:
    - `penalty_per_det_overflow`
    - `penalty_per_cdl_overflow`
    - `penalty_per_arr_too_low`
  - Thresholds:
    - `stable_min`
    - `provisional_min`
    - `experimental_min`

Algorithm sketch:

1. Start from a quality score:
   - `q = w_avg * avgLocal + w_p25 * p25Local + w_p75 * p75Local`.
2. Apply penalties:
   - `q = q - penalty_per_det_overflow * detOverflows`
   - `q = q - penalty_per_cdl_overflow * cdlOverflows`
   - `q = q - penalty_per_arr_too_low * arrTooLow`
3. Clamp `q` into `[0, 1]` → `rwfScore`.
4. Map to status:
   - `rwfScore >= stable_min` → `"stable"`
   - `rwfScore >= provisional_min` → `"provisional"`
   - `rwfScore >= experimental_min` → `"experimental"`
   - Else `"quarantine"`.

### 4.2 Configuration and environment control

Allow policy to be supplied without code changes:

- Priority order:
  1. `--config PATH` JSON file, containing `rwfPolicy` object.
  2. Environment variables:
     - `RWF_STABLE_MIN`, `RWF_PROVISIONAL_MIN`, etc.
     - `RWF_PENALTY_DET`, `RWF_PENALTY_CDL`, `RWF_PENALTY_ARR`.
  3. Hard-coded, well-documented defaults (safe, conservative).

Ensure:

- The script prints a small debug note to stderr when non-default policy is loaded (for CI traceability).
- Policy is treated as data, not logic: CHATDIRECTOR and governance docs can revise it independently.

---

## 5. CI and CHATDIRECTOR Integration

Make the aggregator a first-class citizen in the existing CI and authoring pipelines.

### 5.1 GitHub Actions and CI wiring

Add a job to the relevant repository (e.g., Horror.Place-Orchestrator or a telemetry repo):

- Step 1: Run the aggregator.

  ```yaml
  - name: Aggregate resurrection telemetry
    run: |
      lua scripts/telemetry_resurrection_aggregate.lua \
        --input logs/resurrection-run.ndjson \
        --output registry/telemetry/resurrections.ndjson \
        --det-cap 8.0 \
        --cdl-cap 7.0 \
        --arr-min 0.4
  ```

- Step 2: Validate aggregates against schema:

  ```yaml
  - name: Validate resurrection aggregates
    run: |
      ajv validate \
        -s schemas/telemetry/resurrection-aggregate.v1.json \
        -d registry/telemetry/resurrections.ndjson \
        --spec=draft2020
  ```

- Optionally, fail lints if any `rwfHint.status` equals `"quarantine"` for Tier 1 builds.

### 5.2 CHATDIRECTOR consumption

Document in CHATDIRECTOR v1 docs (e.g., `docs/tooling/chatdirector-telemetry.md`) how aggregated rows are used:

- When generating edits for a seed or event:

  - CHATDIRECTOR:
    - Locates the aggregate row with matching `seedId` (and optionally `eventId`, `resurrectionKind`).
    - Exposes the following into the authoring context:
      - `scores.avgLocal`
      - `scores.p25Local`
      - `scores.p75Local`
      - `safety.detOverflows`, `safety.cdlOverflows`, `safety.arrTooLow`
      - `rwfHint.rwfScore`, `rwfHint.status`
      - `metricsRealized.*` deltas where useful.

- Authoring use cases:

  - Bias skeletons and prompts toward high-`avgLocal` / low-overflow ancestors.
  - Automatically suggest:
    - Tighter novelty bands for low-RWF ancestors.
    - Widening bands or relaxed caps for consistently high-RWF ancestors.

This closes the loop between runtime behavior and authoring defaults.

---

## 6. Performance and Safety Envelope

Set clear expectations and guardrails for v1.

### 6.1 Performance assumptions

- Initial assumption:
  - NDJSON log size is modest (e.g., ≤ 50k lines) in CI runs.
  - In-memory arrays and sorts are acceptable.
- In the script header, include:

  - A NOTE explaining:
    - Current O(N log N) percentile strategy.
    - A threshold (by lines) beyond which performance should be revisited.
  - A TODO pointing to:
    - Possible streaming/approximate percentile implementations.
    - A future Rust/Python drop-in replacement that preserves the same output contract.

### 6.2 Safety and content constraints

Ensure the script:

- Operates **only** on:

  - IDs (`runId`, `seedId`, `eventId`, `regionId`).
  - Numeric invariants and metrics.
  - Structured telemetry fields defined in schemas.

- Never:

  - Emits or parses raw narrative content, dialogue, or explicit horror text.
  - Depends on engine-specific paths or asset names.

This keeps the tool compliant with the “no raw horror in public tiers” doctrine and ensures it can live in Constellation-Contracts or other governance repos.

---

## 7. Advanced Lua and Library Extraction (Post-v1)

Once the basic aggregator is stable, consider higher-value refactors.

### 7.1 Shared telemetry helpers

Extract a reusable Lua library, e.g., `scripts/lib/hp_telemetry.lua`, providing:

- NDJSON helpers:
  - `read_ndjson(path_or_stdin)` → iterator or array.
  - `write_ndjson(path_or_stdout, array_of_tables)`.
- Statistical helpers:
  - `mean(values)`.
  - `percentiles(values, {0.25, 0.75})`.
- Grouping helpers:
  - `group_by(runs, key_fn)`.

This enables reuse by:

- Other aggregators (session metrics, failure atlases).
- Experimental tools in `research/` without duplicating logic.

### 7.2 Pluggable scoring strategies

Allow `compute_rwf` to load scoring strategies from data:

- Strategy source:
  - JSON file: `scripts/config/resurrection_rwf_policy.json`.
  - Or a small Lua module returning a policy table.
- Strategy fields:
  - Choice of inputs (e.g., include/exclude `avgUECdelta`, `avgCDLdelta`).
  - Weights and penalties.
  - Status thresholds per repo tier (`standard`, `research`).

This makes scoring itself an experiment controllable by governance, not by code changes.

### 7.3 Optional self-validation hook

For development and local runs:

- Add an optional `--validate` flag:
  - After writing NDJSON aggregates, call:
    - Local JSON Schema validation (if a Lua validator exists), or
    - `os.execute` to run `ajv` on the emitted file.
- Behavior:
  - On failure, exit with code `1` and print schema errors to stderr.
  - Intended only for development or CI; can be disabled for speed.

This mirrors CI behavior and shortens feedback loops for aggregator changes.

---

## 8. Minimal “Telemetry Script Style” Guideline (Inline Summary)

When implementing `telemetry_resurrection_aggregate.lua`, keep these style rules in mind:

- Single responsibility:
  - The script only aggregates; it does not interpret or apply governance.
- Determinism:
  - Same inputs → same outputs and group ordering.
- Strict, boring JSON:
  - No pretty-printing; one JSON object per line.
  - No additional fields beyond schema.
- Clear failure modes:
  - Argument errors and configuration failures exit with code `2`.
  - Irrecoverable input errors exit with `1`, with concise, structured stderr messages.
- Extensibility:
  - RWF scoring and caps treated as data, not embedded constants.
  - Percentile method and grouping keys documented at the top of the file.

This checklist should be kept next to the script and referenced in PRs adding or modifying the aggregator, ensuring that every implementation remains aligned with the constellation’s contract-first, telemetry-driven governance model.
