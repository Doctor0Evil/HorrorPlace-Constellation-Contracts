# Net-Change Corridors and Policy Intent

This document explains how `policy-intent-v1`, `policy-intent-diff-v1`, and `net-change-corridor-v1` fit together, and how the Rust simulator in Death-Engine evaluates proposed changes.

## Role of policy-intent-v1

`policy-intent-v1.json` is the contract for immutable, versioned network policy intents. Each intent:

- Belongs to a `policyName` and has a monotonic `version`.
- Contains a logical, device-agnostic `intentSpec` with ordered rules and high-level constraints.
- May include a compiled representation (`compiled`) and blast-aware net-change metrics (`netChange`).
- Is stored in SQLite via the `policyintent` table, with `isCurrent = 1` marking the active intent per `policyName`.

AI agents and human authors always work at the `intentSpec` level. Compilers and simulators derive lower-level artifacts and metrics from it.

## Role of policy-intent-diff-v1

`policy-intent-diff-v1.json` describes a structured diff between two intents:

- `fromIntentId` is the base intent.
- `toIntentId` is the intended successor.
- `changes` holds:
  - `ruleAdds`: new rules to insert.
  - `ruleRemovals`: rule IDs to delete.
  - `ruleUpdates`: per-rule field updates using simple `set` / `unset` operations.
  - `metaUpdates`: optional tweaks to top-level metadata like `status` or `goal`.

A validator can load `fromIntentId`, apply the diff deterministically, and either:

- Produce a candidate `toIntentId` intent and run it through blast/net-change checks, or
- Reject the diff with explicit reasons.

This pattern keeps AI proposals narrow and auditable.

## Role of net-change-corridor-v1

`net-change-corridor-v1.json` defines a governance corridor for blast-aware policy changes. It encodes:

- A target `tier` and optional `scope` (policy names and zone kinds).
- Numeric thresholds for net-change metrics:
  - `maxRoH` and `minRoH` over Radius of Harm (delta HS reach).
  - `maxDeltaMaxDet` and `maxDeltaMaxCic`.
  - `maxDeltaReachProb`.
  - `maxAbsNef` for NEF (Network Exposure Factor).
- Optional NEF weights:
  - `wReachProb`, `wMaxDet`, `wRoH`.

The corridor is stored in `netchangecorridor` and can be referenced from `policyintent.governanceCorridorRef`.

## Rust simulator in Death-Engine

The crate `hpc_policy_sim` in Death-Engine provides a minimal net-change simulator:

- `NetChangeCorridor` represents a corridor document.
- `NetChangeSnapshot` holds blast-derived deltas for a specific policy change.
- `NetChangeEvaluation` carries:
  - `within_corridor` (boolean decision),
  - `nef` and raw deltas,
  - a list of human-readable `violations`.

The simulator:

1. Computes NEF using corridor weights:
   - `NEF = wReachProb * deltaReachProb + wMaxDet * deltaMaxDet + wRoH * normalizedRoH`.
2. Checks all corridor thresholds against the snapshot.
3. Returns an evaluation with `within_corridor = true` only if no violations occur.

An FFI function `hpc_policy_eval_netchange` exposes this logic over a small C ABI, accepting JSON corridor and snapshot inputs and returning JSON evaluation.

## Typical validation flow

A typical pipeline for a new AI-generated policy change is:

1. **Base selection**  
   - Identify the current intent for `policyName` from `policyintent` (`isCurrent = 1`).

2. **Diff proposal**  
   - AI agent emits a `policy-intent-diff-v1` object targeting `fromIntentId`.

3. **Intent synthesis**  
   - A deterministic worker:
     - Applies the diff to the base intent.
     - Produces a candidate `policy-intent-v1` with incremented `version`.
     - Stores it in `policyintent` as `status = 'proposed'` and `isCurrent = 0`.

4. **Blast and net-change analysis**  
   - A blast simulator:
     - Computes HS reachability, max DET/CIC, and reach probabilities before and after.
     - Produces a `NetChangeSnapshot` summarizing `roh`, `deltaMaxDet`, `deltaMaxCic`, `deltaReachProb`.
   - The Rust simulator:
     - Loads the appropriate `net-change-corridor-v1` document.
     - Evaluates the snapshot to produce a `NetChangeEvaluation`.
   - The worker writes the result back into the candidate intent’s `netChange` block.

5. **Decision and promotion**  
   - If `within_corridor = false`, the change is rejected or escalated for manual review.
   - If `within_corridor = true`, and other checks (schema, blast radius, consent) pass:
     - The candidate intent is promoted to `status = 'approved'`.
     - Its `isCurrent` is set to 1, and the previous current intent is cleared.

## How AI agents should use these schemas

- Always declare `schemaRef` correctly:
  - `policy-intent-v1`,
  - `policy-intent-diff-v1`,
  - `net-change-corridor-v1`.
- For **intent authoring**:
  - Work only inside `intentSpec.rules` and `intentSpec.constraints`.
  - Do not touch `compiled`, `netChange`, or SQLite fields directly; those are filled by compilers and simulators.
- For **diff authoring**:
  - Use `ruleAdds`, `ruleRemovals`, and `ruleUpdates` rather than rewriting the full intent.
  - Keep changes minimal and local; large refactors should be split into multiple diffs.
- For **governance-aware reasoning**:
  - Treat corridor thresholds as hard limits.
  - Use NEF and violations as structured signals to adjust proposals before they are emitted.

This slice provides enough structure to:

- Capture high-level policy intent in a schema-first way,
- Apply precise, blast-aware net-change checks,
- And let Death-Engine host a narrow, deterministic kernel for acceptance decisions.
