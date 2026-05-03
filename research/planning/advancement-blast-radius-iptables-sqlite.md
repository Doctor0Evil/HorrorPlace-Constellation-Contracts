Below is a set of **50** targeted items — research‑questions, definition‑requests, detail‑queries, and objection‑identifiers — derived from the advanced blast‑radius patterns (iptables + SQLite + AI agents) previously outlined.  
They are designed to push that work toward a production‑grade, governable system while respecting the AlienFS‑style constraints (governance corridors, telemetry‑driven tightening, immutable AI‑intent layers).

**Filename:** `advancement-blast-radius-iptables-sqlite.md`  
**Destination:** `research/planning/` inside `HorrorPlace‑Constellation‑Contracts` (or the repo that holds the firewall‑governance modules)

---

## Research‑Questions *(12 items)*

1. **RQ‑01** – How can the SQLite‑backed blast‑radius graph be augmented with **probabilistic reachability** (e.g., link failure probabilities or unknown intermediate hops) so that an AI agent can answer “what is the risk that data reaches a high‑sensitivity zone given an intermittent VPN link?”

2. **RQ‑02** – What is the most **token‑efficient representation** for exporting the blast‑radius graph to a large language model as a prompt, while preserving governance‑filtered paths and met‑rules, and can this representation achieve sub‑1k‑token footprints for typical graphs of ≤200 nodes?

3. **RQ‑03** – How should **conntrack‑state‑aware** blast‑radius queries be structured so that an AI can simulate “if this new long‑lived TCP session is established, what new zones become reachable *as a result of that state entry*?”

4. **RQ‑04** – What is the ideal **governance corridor definition** for network‑policy changes (RoH, Veco, and a new “Network Exposure Factor”) such that a coding agent can safely propose rule modifications without exceeding a human‑approved blast‑radius expansion threshold?

5. **RQ‑05** – How can the **SQLite‑based policy intent tables** be made to support **versioned diffs** (immutable history) while still allowing AI‑generated proposals to be compared against the current compiled iptables state within a few milliseconds?

6. **RQ‑06** – What is the best way to **prove termination and uniqueness** of the graph‑based “effective blast radius” BFS algorithm when the underlying network graph contains both explicit rules and implicit default‑deny policies, and how can the proof be embedded as a Rust property test?

7. **RQ‑07** – How can the blast‑radius analysis be extended to **multi‑tenant constellations** where each tenant’s policies are stored in the same SQLite file but must be strictly isolated from each other in queries and AI‑proposals?

8. **RQ‑08** – What is the **minimal set of AI‑Chat command tools** (beyond `blast_radius`, `find_path`, `propose`) needed for a coding agent to autonomously debug a “connection refused” error without human intervention, while keeping the blast radius of its investigation bounded?

9. **RQ‑09** – How can the telemetry‑driven tightening loop be made **resilient to feedback‑induced oscillation** when an AI agent’s proposed tightening inadvertently causes a service disruption that then triggers a relaxation proposal from another agent?

10. **RQ‑10** – What is the most appropriate **serialisation format for the “propose” result** (Rust struct ↔ JSON ↔ iptables‑restore diff) so that a human auditor can review the impact of a proposed rule change directly in a diff‑like view without understanding raw iptables syntax?

11. **RQ‑11** – How should the **sandbox‑profile system** be extended to support **hierarchical profiles** where one profile inherits from another but restricts egress further, and what is the correct governance validation for profile inheritance?

12. **RQ‑12** – Can the concept of a **“blast‑radius impact score”** (computable from the graph) be integrated into the AlienFS ReadSession governance corridor so that the system denies any read request whose score exceeds a session‑specific budget, thereby enforcing both code and network boundaries in one pass?

---

## Definition‑Requests *(13 items)*

13. **DR‑01** – Provide a normative JSON Schema for a **network policy intent document** that a coding agent would submit as a proposed change, covering fields such as `change_id`, `proposed_rules` (zone‑to‑zone flows), `desired_blast_tier`, and `governance_corridor_check`.

14. **DR‑02** – Define the precise **data model for the “blast‑radius graph”** stored in SQLite, including types for `nodes`, `edges`, and `graph_properties` (governance‑filtered flags, effective policy tier), as a formal ERD with foreign‑key constraints and index strategies.

15. **DR‑03** – Formalise the **“governance corridor” mathematical definition** for network policy changes as a set of constraints (RoH ≤ 0.30, Veco ≤ 0.30, plus a new constraint on maximum new egress count) and show how they can be enforced via SQL `CHECK` or application‑level validation.

16. **DR‑04** – Define the **canonical representation of an iptables rule** in SQLite (`iptables_rules` table) that is losslessly reversible from `iptables-save` output, including all match modules (`owner`, `ipset`, `conntrack`, `tcp`, `udp`), and specify how to handle non‑linear table traversal (e.g., chain jumps).

17. **DR‑05** – Provide a precise specification for the **“sandbox profile” JSON schema**, covering required fields (`profile_id`, `max_zones`, `allowed_elements`, `max_egress_flows`), and how these map to an iptables ruleset applied to a network namespace.

18. **DR‑06** – Formally define the **“effective blast radius” metric** for a given agent UID, including its computation from the graph (reachable nodes after governance filtering), a formula for normalisation to a 0‑1 score, and the acceptable staleness window before recomputation.

19. **DR‑07** – Define the **telemetry event schema** for `flows` logged into SQLite, specifying required fields (timestamp, 5‑tuple, uid, inferred zone, verdict, chain) and the indexing required to support per‑agent blast‑radius queries at ingestion rates of 1000 events/second.

20. **DR‑08** – Request a **“policy drift” definition** that quantifies the difference between the rows in `policy_intent` and the compiled `iptables_rules` as a single scalar, including a threshold above which the system must refuse to allow new AI proposals until drift is resolved.

21. **DR‑09** – Define a **machine‑readable “blast‑tier taxonomy”** (T0–T3) mapping each tier to a set of allowed egress zones, after which all blast‑radius queries are expressed. This taxonomy must be stored as JSON in SQLite.

22. **DR‑10** – Specify the **exact API contract** for the `MCP_blast_radius` tool: input parameters (uid, max hops, governance_corridor_id), output JSON schema, error codes, and the guarantee that the output is consistent with the committed snapshot of the SQLite graph.

23. **DR‑11** – Provide a formal **definition of “safety” for an AI‑proposed rule addition** in terms of the blast‑radius graph; for example, a proposed rule is safe if the resulting effective blast radius for any agent does not include any node with a blast tier higher than the agent’s allowed tier.

24. **DR‑12** – Define the **semantics of “governance filtering”** on graph edges: when a human‑created policy says “edge from AI‑agents zone to backend zone is allowed,” how is that encoded as a boolean `governance_allowed` flag in the SQLite edges table and what provenance fields are required.

25. **DR‑13** – Specify a **canonical **“iptables‑save to SQL ingest”** transformation** that handles all common table/chain constructs, ensures idempotency, and produces a `rule_hash` for each rule to support diff detection.

---

## Detail‑Queries *(13 items)*

26. **DQ‑01** – In the Rust implementation, how exactly should the `bfs_with_constraints` function handle large graphs (500+ nodes) to stay within a 2‑millisecond time budget suitable for an interactive AI‑Chat tool? Should it use incremental partial results or pre‑computed materialised paths?

27. **DQ‑02** – What is the recommended SQLite configuration (journal mode, synchronous, mmap size) for a database that ingests 1,000 telemetry flow rows per second while simultaneously serving 10 read‑only blast‑radius queries per second from AI agents?

28. **DQ‑03** – How should the `policy_intent` table’s `desired_blast_tier` be validated against the agent’s own UID when an AI proposes a rule, and what SQL query returns a rejection reason if the agent attempts to escalate beyond its tier?

29. **DQ‑04** – What is the precise algorithm to convert a `zone_policies` row (“A‑to‑B, port 5432, ACCEPT”) into an iptables rule while automatically generating the correct `-m set --match-set` references and ensuring the rule is inserted in the correct chain order without collision?

30. **DQ‑05** – In the `propose` tool, how should the system diff the candidate intent rows against the existing `iptables_rules` implementation to produce a minimal set of iptables‑restore instructions, and what is the diff algorithm when multiple intents target the same chain?

31. **DQ‑06** – How should the telemetry ingestion daemon deduplicate flow events before inserting them into the `flows` table, given that a single TCP stream will generate many LOG entries, and what is the appropriate batch size to keep SQLite write‑ahead log under control?

32. **DQ‑07** – What is the exact list of Rust crates needed to implement the full blast‑radius service (SQLite, graph algorithms, MCP server, iptables parser/serialiser) and what version compatibility constraints must be observed so that AlienFS’s governance layer can reuse the same crate set?

33. **DQ‑08** – How should the `blast_radius` command handle the case where an agent requests a graph query with `max_hops=3` but some nodes have high out‑degree that would explode the result set; should it impose a fan‑out limit, and if so, how is that communicated back to the AI as a non‑error truncation?

34. **DQ‑09** – What is the best way to represent **iptables chain jumps** (user‑defined chains) in the SQLite edges table so that the blast‑radius BFS can transparently follow them without requiring a pre‑computed jump‑to‑rule mapping that must be re‑synchronised on every policy change?

35. **DQ‑10** – For the `find_path` tool, what weighting function should be applied to edges (e.g., by blast‑tier impact, rule hit count from telemetry) to produce a meaningful “least‑risky path” result, and how are those weights stored in SQLite to be queryable?

36. **DQ‑11** – How should the sandbox‑profile system integrate with the **network namespace lifecycle** (creation, teardown, cleanup) so that a crashed AI agent does not leave orphaned rules in the host’s iptables; what hook or garbage‑collection must be implemented in the Rust service?

37. **DQ‑12** – What are the exact character limits and sanitisation requirements for `rule_comment` in the `iptables_rules` table to prevent SQL injection when the comment originates from an AI agent’s proposed change?

38. **DQ‑13** – How should the “policy drift” metric be maintained transactionally: if the telemetry ingester and the policy reconciler both update the same drone table simultaneously, what concurrency control (explicit `BEGIN IMMEDIATE`, application‑level mutex, or a separate reconciliation lock table) is required?

---

## Objection‑Identifiers *(12 items)*

39. **OI‑01** – The current blast‑radius graph design stores edges as simple source/destination pairs without timestamps or versioning; this makes auditability of past blast‑radius queries impossible. **Objection:** The schema must include `created_at`, `valid_from`, `valid_until` columns on edges to support historical querying and AI‑explainability of changes.

40. **OI‑02** – The proposal for AI agents to **never touch iptables directly** relies on a separate reconciler daemon applying changes; if that daemon crashes or is misconfigured, AI‑generated proposals remain unenforceable. **Objection:** The system must provide a healthcheck that is continuously monitored by AI‑Chat tools, and the MCP server must refuse to accept proposals if the reconciler is not in a healthy state.

41. **OI‑03** – The Rust implementation example uses a simple in‑memory graph for blast‑radius queries; as the number of rules grows beyond 10,000, this will exceed memory limits and cause performance degradation. **Objection:** The graph must be stored in SQLite with on‑demand traversal (e.g., recursive CTEs or a dedicated graph‑processing Rust module that streams from SQLite) rather than loading all edges.

42. **OI‑04** – The “effective blast tier” metric derived from telemetry may be **computed incorrectly** if a flow crosses a zone boundary where the zone assignment is ambiguous (e.g., a packet matched by multiple ipsets). **Objection:** A deterministic tie‑breaking rule must be defined and proven consistent; otherwise AI agents could be misled about their true reach.

43. **OI‑05** – The `MCP_blast_radius` tool returns results based on the current SQLite snapshot, but **iptables rules may have been applied externally** (e.g., by an admin) after the last sync, leading to a stale model. **Objection:** The system must continuously verify the hash of the current iptables ruleset against the last known hash in SQLite, and refuse to serve queries if a mismatch is detected for more than 60 seconds.

44. **OI‑06** – The proposed **sandbox profiles** assume that the AI agent’s process UID remains constant, but if the agent restarts or spawns child processes under different UIDs, the sandbox could be escaped. **Objection:** The launcher must enforce a one‑to‑one mapping between a session token and a single network namespace with a pinned UID/GID that is destroyed on session termination.

45. **OI‑07** – Telemetry‑driven tightening could be exploited by a malicious AI agent that **deliberately triggers benign‑looking flows** to train the system into allowing a broader blast radius later. **Objection:** Proposal‑tightening algorithms must incorporate a “safety margin” and never mechanically tighten beyond a pre‑defined minimum blast tier without human review.

46. **OI‑08** – The JSON Schema for policy intent documents (DR‑01) is still undefined; without it, every agent may produce different, unvalidatable formats. **Objection:** The schema must be finalised and stabilised before any further implementation, and a CI lint check must reject any code that produces intent documents not validated against this schema.

47. **OI‑09** – The MCP server relies on SQLite for both configuration and telemetry; if the database file grows too large, blast‑radius queries will slow down. **Objection:** A retention and archiving policy for the `flows` table must be part of the initial design, including a background job that moves old telemetry to a separate historical DB.

48. **OI‑10** – The **governance corridor constants** (RoH ≤ 0.30, Veco ≤ 0.30) were borrowed from AlienFS without empirical justification for network policy changes. **Objection:** A tuning experiment must be conducted to determine appropriate thresholds for network blast‑radius expansion, and the values must be documented with their derivation before being hard‑coded.

49. **OI‑11** – The plan to generate an iptables `-m owner` rule per agent UID will quickly lead to **rule‑order sensitivity** if an agent’s traffic matches multiple owner rules; iptables has no concept of “most specific owner match wins.” **Objection:** The policy compiler must guarantee that for any given UID, exactly one rule matches at each priority, or else include explicit `-j RETURN` or `-j ACCEPT` chain exits to avoid ambiguity.

50. **OI‑12** – The blast‑radius graph edges currently lack a **confidence or provenance tag**; an AI agent may propose an edge (e.g., “AI‑tools zone → production DB”) that appears safe based on an outdated schematic but in reality violates a higher‑level security covenant. **Objection:** Every edge must carry a `provenance` field (e.g., “declared by infra‑team, verified on 2025-12-01”) and the governance validator must refuse to accept edges with unknown or expired provenance.

---

These 50 items span the immediate next‑step engineering needs (detail queries), the conceptual blanks (definition requests), the ambitious explorations (research questions), and the critical risks (objection identifiers) that will turn the blast‑radius + SQLite + AI pattern into a trusted, production‑hardened component of the constellation.
