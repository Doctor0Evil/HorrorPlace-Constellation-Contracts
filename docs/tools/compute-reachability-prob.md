# Tool: compute_reachability_prob

This document defines the MCP-facing contract for the `compute_reachability_prob` tool, including the JSON schemas, MCP tool name, and how it connects to the SQLite blast graph with probabilistic edges and blast profiles.

The goal of this tool is to let agents answer questions like:

> What is the probability that data from node X reaches a high-sensitivity zone under the current topology and link reliabilities?

while keeping all math and governance enforcement inside Death-Engine and the SQLite blast graph.

---

## MCP tool name and intent

**Tool name (MCP / AI-Chat):**

- `compute_reachability_prob`

This tool is a pure function over JSON:

- Input: `compute-reachability-prob-request-v1` envelope.
- Output: `compute-reachability-prob-response-v1` envelope.

The tool:

- Never exposes raw SQL or probabilistic edge internals to agents.
- Always evaluates against a single, committed graph snapshot.
- Respects governance constraints encoded in blast profiles and corridor policies (when used in policy simulations).

Agents use it to:

- Estimate risk of reachability to sensitive zones before proposing network changes.
- Explain probabilistic blast effects of intermittent links (e.g., VPNs).
- Support net-change calculations that combine structural blast radius and probabilistic reach.

---

## Request envelope

The request schema is defined in:

- `schemas/tools/compute-reachability-prob-request-v1.json`

**Canonical shape (informal):**

```json
{
  "id": "req-uuid-1",
  "schemaRef": "compute-reachability-prob-request-v1",
  "tenantProfile": { "...": "..." },
  "query": {
    "tenantId": "tenant-A",
    "snapshotId": 42,
    "sourceNodeId": 10,
    "targetFilter": {
      "zoneKind": "highSensitivity"
    },
    "maxDepth": 6,
    "maxPaths": 8,
    "minEdgeSuccessProb": 0.3,
    "includePathSamples": true
  },
  "edges": {
    "kind": "sqlite"
  },
  "meta": {
    "toolName": "MapDesignWizard",
    "sessionId": "sess-123",
    "traceId": "trace-abc"
  }
}
```

### Fields

- `id`  
  Stable request identifier (e.g., UUID or Dead-Ledger-linked ID).

- `schemaRef`  
  Must be `"compute-reachability-prob-request-v1"`.

- `tenantProfile`  
  Tenant profile document (typically `tenant-profile-v1`) that constrains what the kernel may see and how it should interpret the graph (tiers, blast taxonomies, allowed zones).

- `query`  
  Structured parameters for the probabilistic reachability calculation:

  - `tenantId`  
    Tenant namespace for scoping the graph and policies.

  - `snapshotId`  
    ID of the graph snapshot to use. The kernel must evaluate exclusively against this snapshot.

  - `sourceNodeId`  
    Integer ID of the source node in the blast graph.

  - `targetFilter`  
    Defines which nodes count as "targets" for the probability:

    - Either `{"nodeIds": [101, 205, ...]}` for explicit target nodes.
    - Or `{"zoneKind": "highSensitivity"}` for a zone-class-based target set.

  - `maxDepth` (optional)  
    Hard cap on path length, in hops. If omitted, the Rust kernel uses a policy-specific default.

  - `maxPaths` (optional)  
    Maximum number of highest-probability paths to keep in the calculation. Used to bound compute.

  - `minEdgeSuccessProb` (optional)  
    Lower bound on edge success probability; edges below this threshold are ignored.

  - `includePathSamples` (optional)  
    If true, the response may include a small sample of top contributing paths for explainability.

- `edges` (optional)  
  Selects where probabilistic edges come from:

  - `{ "kind": "sqlite" }`  
    Use the blast graph and probabilistic edge columns from SQLite, filtered by `tenantId` and `snapshotId`.

  - `{ "kind": "inline", "items": [...] }`  
    Provide a small in-memory edge list (e.g., lab experiments). Each item includes at least:
    - `sourceNodeId`, `targetNodeId`, `successProb`, and optional hints like `isIntermittent`, `probSource`.

- `meta` (optional)  
  Tool metadata for observability (tool name, session, trace).

Agents should always:

- Set `schemaRef` correctly.
- Pass an explicit `snapshotId` obtained from a separate snapshot-discovery tool or orchestration layer.
- Prefer `edges.kind = "sqlite"` in production; `inline` is for experiments and tests.

---

## Response envelope

The response schema is defined in:

- `schemas/tools/compute-reachability-prob-response-v1.json`

**Canonical success (informal):**

```json
{
  "id": "resp-uuid-1",
  "schemaRef": "compute-reachability-prob-response-v1",
  "requestId": "req-uuid-1",
  "ok": true,
  "error": null,
  "data": {
    "tenantId": "tenant-A",
    "snapshotId": 42,
    "sourceNodeId": 10,
    "maxDepthUsed": 5,
    "pathsConsidered": 24,
    "truncated": false,
    "targets": [
      { "nodeId": 101, "reachabilityProb": 0.18, "zoneKind": "highSensitivity" },
      { "nodeId": 205, "reachabilityProb": 0.25, "zoneKind": "highSensitivity" }
    ],
    "aggregate": {
      "targetKind": "highSensitivityZone",
      "reachabilityProb": 0.31
    },
    "pathSamples": [
      {
        "targetNodeId": 101,
        "nodes":,[2]
        "edgeSuccessProb": [0.7, 0.9, 0.9],
        "pathSuccessProb": 0.567
      }
    ],
    "limits": {
      "maxDepth": 6,
      "maxPaths": 8,
      "minEdgeSuccessProb": 0.3
    },
    "meta": {
      "engineVersion": "hpc-reach-prob-0.1.0",
      "runtimeMs": 1.7,
      "warnings": []
    }
  }
}
```

**Canonical error (informal):**

```json
{
  "id": "resp-uuid-1",
  "schemaRef": "compute-reachability-prob-response-v1",
  "requestId": "req-uuid-1",
  "ok": false,
  "error": {
    "code": "INVALID_INPUT",
    "message": "snapshotId must be a positive integer",
    "detail": null
  },
  "data": null
}
```

### Fields

- `id`  
  Response identifier (can mirror `requestId` or be independent).

- `schemaRef`  
  Must be `"compute-reachability-prob-response-v1"`.

- `requestId`  
  The `id` of the original request.

- `ok`  
  Boolean success flag.

- `error`  
  `null` on success. Otherwise, an object with:

  - `code` (e.g., `INVALID_INPUT`, `FFI_ERROR`, `GRAPH_UNAVAILABLE`).
  - `message` human-readable explanation.
  - `detail` optional machine-friendly payload (numeric return codes, parse traces).

- `data`  
  `null` on error. On success:

  - `tenantId`, `snapshotId`, `sourceNodeId`  
    Echo back the resolved tenant, snapshot, and source.

  - `maxDepthUsed`  
    Actual maximum depth reached in BFS/path enumeration.

  - `pathsConsidered`  
    Number of paths used in the probability computation.

  - `truncated`  
    True if computation hit internal limits (depth, paths, or time budget).

  - `targets`  
    Per-target marginal reachability probabilities:

    - `nodeId`  
      Target node identifier.

    - `reachabilityProb`  
      Probability that at least one safe path from `sourceNodeId` reaches this node.

    - `zoneKind` (optional)  
      Zone classification, when available from the graph.

    - `viaIntermittent` (optional)  
      True if all contributing paths rely on at least one intermittent edge.

  - `aggregate` (optional but recommended)  
    Aggregate probability across the target set:

    - `targetKind` e.g., `"highSensitivityZone"`.
    - `reachabilityProb` probability that any target node is reached.

  - `pathSamples` (optional)  
    Sample of contributing paths for explainability:

    - `targetNodeId`  
      Which target this path reaches.

    - `nodes`  
      Ordered node IDs from source to target.

    - `edgeSuccessProb`  
      Per-edge success probabilities along the path.

    - `pathSuccessProb`  
      Product of `edgeSuccessProb`.

  - `limits` (optional)  
    Echo of applied limits (maxDepth, maxPaths, minEdgeSuccessProb).

  - `meta` (optional)  
    Engine version, runtime, and any warnings.

---

## Relationship to SQLite blast graph and probabilistic edges

The `compute_reachability_prob` tool sits on top of the SQLite blast graph and its probabilistic extensions:

- The blast graph already models nodes (contracts, zones, tiles) and edges with invariants and distances.
- For probabilistic reachability, the edge schema is extended with:
  - `successProb` in `[0, 1]`.
  - `probSource` (e.g., `measured`, `config`, `default`).
  - `isIntermittent` flag for unstable links like VPNs.

The Rust kernel that backs this tool:

1. Resolves the graph snapshot for `(tenantId, snapshotId)` and applies tenant scoping.

2. Filters edges:

   - Only governance-allowed edges are considered (blast policies and corridor rules may apply).
   - Edges with `successProb < minEdgeSuccessProb` are discarded.
   - BFS / path enumeration is depth-limited by `maxDepth` and any blast policy caps.

3. Enumerates a bounded set of paths from `sourceNodeId` to nodes matching `targetFilter`:

   - Computes per-path success probabilities as the product of edge success probabilities.
   - Aggregates per-target reachability using standard probability combination (no double counting of overlapping paths under the independence assumption, up to approximation limits governed by `maxPaths`).

4. Derives:

   - Per-target probabilities for the `targets` array.
   - An optional aggregate probability over the entire target set.
   - Optional path samples for explainability.

5. Ensures all reads are served from a single, committed snapshot, so the result is deterministic for a given `(tenantId, snapshotId, request)` triple.

The probabilistic blast graph schema and the blast-radius policies (caps, tiers) remain authoritative; the tool is only a narrow, JSON-defined surface over those structures.

---

## Typical agent usage

An AI agent or tool typically:

1. Determines `tenantId` and obtains or selects a graph `snapshotId`.
2. Identifies the `sourceNodeId` (e.g., a zone or service node).
3. Chooses a target filter:

   - For questions like “risk to high-sensitivity zones”, use `{"zoneKind": "highSensitivity"}`.
   - For specific nodes of interest, list them in `nodeIds`.

4. Sets reasonable limits:

   - `maxDepth` based on blast policy (e.g., 4–8).
   - `maxPaths` tuned for latency and approximation quality (e.g., 8–32).

5. Calls `compute_reachability_prob` with:

   - A tenant profile document.
   - `edges.kind = "sqlite"` in production, or `inline` edges in lab setups.

6. Interprets the response:

   - Uses `targets` and `aggregate.reachabilityProb` as structured signals.
   - Treats high probabilities to high-sensitivity zones as a reason to tighten policies or adjust wiring plans before proposing changes.

When combined with net-change and corridor tooling, this allows an agent to:

- Compare reachability probabilities before and after a proposed change.
- Reject or down-tune changes that increase reachability beyond corridor thresholds.
- Provide human-auditable explanations using `pathSamples` and `meta.warnings`.

---

## Integration points

- **Schemas:**
  - `schemas/tools/compute-reachability-prob-request-v1.json`
  - `schemas/tools/compute-reachability-prob-response-v1.json`

- **Rust (Death-Engine):**
  - FFI kernel implementing the probabilistic blast computation against SQLite snapshots.

- **Lua (Death-Engine):**
  - `H.Blast.computeReachabilityProb` and a tool wrapper that:
    - Accepts request envelopes.
    - Calls the FFI.
    - Returns response envelopes matching the response schema.

- **Policy simulators:**
  - Net-change simulators may call this tool twice (before/after) to populate `deltaReachProb` and related fields in policy and corridor evaluation.

This document is the normative reference for MCP and AI-Chat integration: all agents should treat the two JSON Schemas as the only valid input and output shapes for probabilistic reachability queries.
