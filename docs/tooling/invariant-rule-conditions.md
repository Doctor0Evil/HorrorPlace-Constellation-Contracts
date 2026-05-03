# Invariant Rule Conditions (`match_json`)

This document explains how to use the `match_json` field for `invariant_rules` as a small domain-specific language (DSL) for matching regions, tiles, invariants, and metrics. It also shows how these JSON conditions can be evaluated in SQL and in engine-side code.

The JSON shape is defined by:

- `schemas/tooling/invariant-rule-condition.v1.json`

## 1. Purpose

`match_json` lets you express more flexible rule conditions than a fixed set of `min_` / `max_` columns. It supports:

- Region and tile filters (equals / in-list).
- Range and comparison operators for invariants and metrics.
- Simple `AND` / `OR` logic over all specified conditions.

This keeps the `invariant_rules` table compact, while making the matching behavior explicit and machine-readable.

## 2. Base structure

A `match_json` block has the following top-level fields:

- `region` (optional): region match criteria.
- `tile` (optional): tile match criteria.
- `invariants` (optional): per-invariant numeric predicates.
- `metrics` (optional): per-metric numeric predicates.
- `logic` (optional): `"AND"` or `"OR"`, default `"AND"`.

### 2.1 Region and tile matchers

`region` and `tile` both follow this structure:

```json
"region": {
  "equals": "region-001",
  "in": ["region-001", "region-002"]
},
"tile": {
  "equals": "tile-A",
  "in": ["tile-A", "tile-B", "tile-C"]
}
```

Rules:

- If `region` is omitted, the rule does not constrain region.
- If `region.equals` is present, region id must equal that value.
- If `region.in` is present, region id must be one of the listed values.
- If both `equals` and `in` are present, they are interpreted together (e.g., `equals` should also appear in `in`).

The same applies to `tile`.

### 2.2 Invariant and metric predicates

`invariants` and `metrics` are maps keyed by code (e.g., `CIC`, `DET`, `UEC`), each with optional numeric comparisons:

```json
"invariants": {
  "CIC": {
    "min": 0.4,
    "max": 0.8
  },
  "DET": {
    "gte": 3.0,
    "lt": 8.5
  }
},
"metrics": {
  "UEC": {
    "min": 0.2,
    "max": 0.7
  }
}
```

Supported operators per code:

- `min`: value ≥ min
- `max`: value ≤ max
- `gt`: value > gt
- `gte`: value ≥ gte
- `lt`: value < lt
- `lte`: value ≤ lte

If multiple operators are present for the same code, they are combined with AND (all must pass).

### 2.3 Logic mode

The `logic` field controls how region/tile/invariants/metrics constraints combine:

```json
"logic": "AND"
```

- `"AND"` (default): all specified constraints must pass.
- `"OR"`: rule matches if any single constraint passes.

Within each sub-block, the semantics are:

- For `region` and `tile`, all sub-conditions in each block are ANDed together.
- For each invariant/metric code, all comparison operators are ANDed.
- `logic` controls how these top-level blocks combine (region, tile, invariants, metrics).

## 3. Concrete examples

### 3.1 Example 1: High-DET tiles in specific regions

Rule: Match any tile in regions `region-north` or `region-south` where `DET` is at least 6.0.

```json
{
  "region": {
    "in": ["region-north", "region-south"]
  },
  "invariants": {
    "DET": {
      "min": 6.0
    }
  },
  "logic": "AND"
}
```

Interpretation:

- Region must be one of the listed values.
- DET must be ≥ 6.0.
- No constraints on tile, metrics, or other invariants.

Possible SQL (if `invariants_snapshot` holds the values):

```sql
SELECT *
FROM invariants_snapshot AS s
WHERE s.region_id IN ('region-north', 'region-south')
  AND s.det >= 6.0;
```

### 3.2 Example 2: Low-CIC OR high-AOS anywhere

Rule: Match snapshots where either `CIC` is below 0.3 or `AOS` is above 0.7, regardless of region or tile.

```json
{
  "invariants": {
    "CIC": {
      "lt": 0.3
    },
    "AOS": {
      "gt": 0.7
    }
  },
  "logic": "OR"
}
```

Interpretation:

- No region or tile constraints.
- Rule matches if:
  - `CIC < 0.3` OR
  - `AOS > 0.7`.

Possible SQL (approximate):

```sql
SELECT *
FROM invariants_snapshot AS s
WHERE s.cic < 0.3
   OR s.aos > 0.7;
```

### 3.3 Example 3: Specific tile with invariant and metric bands

Rule: Match tile `tile-42` in region `region-west` where:

- `CIC` is between 0.5 and 0.9 (inclusive).
- `DET` is between 2 and 5 (inclusive).
- `UEC` is between 0.3 and 0.8.

```json
{
  "region": {
    "equals": "region-west"
  },
  "tile": {
    "equals": "tile-42"
  },
  "invariants": {
    "CIC": {
      "min": 0.5,
      "max": 0.9
    },
    "DET": {
      "min": 2.0,
      "max": 5.0
    }
  },
  "metrics": {
    "UEC": {
      "min": 0.3,
      "max": 0.8
    }
  },
  "logic": "AND"
}
```

Possible SQL:

```sql
SELECT *
FROM invariants_snapshot AS s
JOIN runtime_metrics AS m
  ON m.snapshot_id = s.snapshot_id
WHERE s.region_id = 'region-west'
  AND s.tile_id = 'tile-42'
  AND s.cic >= 0.5 AND s.cic <= 0.9
  AND s.det >= 2.0 AND s.det <= 5.0
  AND m.uec >= 0.3 AND m.uec <= 0.8;
```

The actual table storing metrics may differ; the key idea is that each code in `metrics` translates to a band condition on the corresponding value.

### 3.4 Example 4: Region-only rule with OR logic

Rule: Match any snapshot either in `region-alpha` OR with `DET > 8.0`.

```json
{
  "region": {
    "equals": "region-alpha"
  },
  "invariants": {
    "DET": {
      "gt": 8.0
    }
  },
  "logic": "OR"
}
```

Interpretation:

- If snapshot is in `region-alpha`, rule matches even if `DET` is low.
- If `DET > 8.0`, rule matches even if region is something else.

SQL approximation:

```sql
SELECT *
FROM invariants_snapshot AS s
WHERE s.region_id = 'region-alpha'
   OR s.det > 8.0;
```

## 4. Engine-side evaluation sketch

In engine or offline code (Lua, GDScript, Rust), evaluation of `match_json` can follow this pattern:

1. Parse the JSON into a struct.
2. Evaluate `region` and `tile` predicates against the snapshot.
3. Evaluate each invariant and metric predicate against the numeric values.
4. Combine results using `logic`.

Pseudocode:

```pseudo
function matches(match_json, snapshot, metrics):
    // region
    let region_ok = true
    if match_json.region exists:
        region_ok = check_equals_and_in(snapshot.region_id, match_json.region)

    // tile
    let tile_ok = true
    if match_json.tile exists:
        tile_ok = check_equals_and_in(snapshot.tile_id, match_json.tile)

    // invariants
    let inv_ok_all = true
    let inv_ok_any = false
    if match_json.invariants exists:
        for each (code, cond) in match_json.invariants:
            value = snapshot.invariants[code]
            if value is undefined:
                continue
            ok = check_numeric_cond(value, cond)
            inv_ok_all = inv_ok_all AND ok
            inv_ok_any = inv_ok_any OR ok
    else:
        inv_ok_all = true
        inv_ok_any = false

    // metrics
    let met_ok_all = true
    let met_ok_any = false
    if match_json.metrics exists:
        for each (code, cond) in match_json.metrics:
            value = metrics[code]
            if value is undefined:
                continue
            ok = check_numeric_cond(value, cond)
            met_ok_all = met_ok_all AND ok
            met_ok_any = met_ok_any OR ok
    else:
        met_ok_all = true
        met_ok_any = false

    let logic = match_json.logic or "AND"

    if logic == "AND":
        return region_ok AND tile_ok AND inv_ok_all AND met_ok_all
    else: // "OR"
        return region_ok OR tile_ok OR inv_ok_any OR met_ok_any
```

Where `check_equals_and_in` and `check_numeric_cond` implement the obvious semantics.

## 5. Authoring guidelines for rule designers

- Prefer narrow, focused rules:
  - Use `region.equals` and `tile.equals` for precise control.
  - Use `invariants` and `metrics` only for codes that matter for the rule.
- Use `logic = "AND"` for most safety-critical rules.
- Reserve `logic = "OR"` for fallback or emergency rules (e.g., “if DET is extremely high OR region is flagged unsafe, block spawns”).
- Keep numeric ranges aligned with the canonical invariant and metric spines:
  - Invariants typically normalized 0.0–1.0, except DET with 0–10.
  - Metrics normalized 0.0–1.0.

By consolidating matching logic into `match_json`, you can evolve rule semantics and debugging views without altering table layouts, and AI-chat tools can generate and reason about rules using a single, documented JSON structure.
