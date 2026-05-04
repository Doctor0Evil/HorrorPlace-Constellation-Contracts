**Filename:** `research-agenda-repo-health-ai-chat.md`  
**Destination:** `analysis/research/` inside the `HorrorPlace‑Constellation‑Contracts` repository

Below is a focused set of research objectives and questions designed to lift the project’s repository health, usability, discoverability, and AI‑Chat compatibility—directly extending the iptables, blast‑radius, governance‑corridor, and SQLite patterns already in place.

---

## 1. Research Objectives (Priority Topics)

| Objective | Short Rationale | Impact on … |
|-----------|----------------|--------------|
| **OBJ‑1** – Auto‑generated, AI‑friendly schema documentation with token‑aware examples | LLMs need compact schema summaries and per‑field “why” explanations, not just JSON Schema files. | Usability, AI‑Chat compatibility |
| **OBJ‑2** – Canonical, token‑efficient graph serialisation for all MCP tools | Responses from `blast_radius`, `find_path` must follow a strict envelope that optimises for LLM context windows (sub‑1k token summaries for graphs ≤200 nodes). | AI‑Chat compatibility, Usability |
| **OBJ‑3** – Time‑travel query interface for policy intent and drift audits | Past blast neighbourhoods and governance decisions must be reproducible via snapshot‑aware queries without ad‑hoc scripts. | Repository health, Indexing |
| **OBJ‑4** – Schema versioning strategy with CI‑enforced backward‑compatibility checks | Every JSON Schema (`policy-intent-v1`, `sandbox-profile-v1`, etc.) must declare a version policy, and CI must block breaking changes that would invalidate existing AI‑generated proposals. | Repository health, Usability |
| **OBJ‑5** – LLM‑readable error and governance‑corridor explanation templates | When an AI proposal is rejected by a corridor check, the system must return structured, human‑ and AI‑digestible reasons (tier escalation, missing consent, drift threshold exceeded) with direct references to the violating fields. | Usability, AI‑Chat compatibility |
| **OBJ‑6** – Index‑first thought: priority for probabilistic reachability, conntrack‑aware queries, and corridor validation | Determine whether the SQLite indexing scheme (e.g., indexes on `graph_edge`, `flows`, `iptables_rules`) should be optimised primarily for probabilistic queries, conntrack‑aware branching, governance corridor pre‑computation, or a carefully balanced combination. | Indexing, Query performance |
| **OBJ‑7** – Hybrid human‑AI policy workflow specification | Define the exact interaction model when a human manually edits a policy intent, creates diffs, and how those changes coexist with AI‑generated proposals in the same `policyintent` table, ensuring audit trails and merge‑safe semantics. | Usability, Repository health |
| **OBJ‑8** – Tenant‑modular wiring for iptables and blast‑taxonomy dependencies | Clarify whether the canonical `iptables_rules` table and blast‑tier taxonomy are mandatory for all tenants or can be disabled/overridden, and how that affects tool availability (`MCP_blast_radius`, `propose`) without breaking governance guarantees. | Indexing, Repository health |

---

## 2. Research Questions

These questions bridge the objectives into concrete investigations.

1. **Should the new schema‑indexing methods prioritise query performance for probabilistic reachability calculations, conntrack‑aware simulations, or governance corridor validations—or a balanced approach across all three?**  
   *(from provided list)*

2. **Are the new directory structures intended to support only AI‑driven policy proposals (via `policyintent` and diffs), or also manual human‑authored rule changes that must remain compatible with the same SQLite‑backed workflow?**  
   *(from provided list)*

3. **Regarding the wiring aspects: should integration with the canonical `iptables_rules` table and blast‑tier taxonomy be treated as hard dependencies for the new directories, or as optional modules that can be enabled per tenant?**  
   *(from provided list)*

4. **How can the SQLite schema be extended with per‑table change journals (insert/update/delete logs) so that AI agents can reconstruct the history of a governance policy change without receiving the entire `policyintent` history in one prompt, and what is the token overhead of such a journal in system prompts?**

5. **What naming conventions for JSON Schema `$id` values and SQLite table names best enable AI‑Chat tools to discover and navigate the constellation’s policy and blast resources without pre‑training on the specific repository layout, and can those conventions be enforced by a linter?**

6. **Should the indexing strategy for `flows` and `iptables_rules` include pre‑computed materialised views (e.g., per‑agent effective blast tier summaries) that are updated transactionally, and how can these views be exposed through MCP tools to reduce the latency of blast‑radius queries that originate from AI planning loops?**

---

## 3. Additional 3 Questions (explicitly augmenting the provided list)

### Q‑A1  
**What is the optimal strategy for exposing repository‑wide metadata (schema versions, corridor constants, tenant‑specific blast‑tier taxonomies) to AI agents via a dedicated “constellation‑manifest” endpoint, and how often must that manifest be refreshed to remain trustworthy during prolonged Chat sessions?**

### Q‑A2  
**How can the MCP tool descriptions (e.g., `blast_radius`, `propose`, `find_path`) be augmented with terse, LLM‑optimised usage examples and pre‑condition summaries so that coding agents can autonomously select the correct tool without trial‑and‑error, even when they have never seen the full system documentation?**

### Q‑A3  
**In a repository where both human‑authored and AI‑generated policy intents coexist, what merge‑safety and conflict‑resolution policies must be encoded in the schema (or the policy‑diff application engine) to prevent silent overwrites and to ensure that a human‑imposed drift threshold is never lowered by an unsupervised AI proposal?**

---

These objectives and questions are intended to be prioritised alongside the existing blast‑radius and corridor work. They directly address the friction points that currently limit the system’s score on “repository health” (versioning, CI, schema evolution), “usability” (clear AI contracts, manual vs. automatic workflows), “indexing” (query trade‑offs, materialisation), and “AI‑Chat compatibility” (token‑aware responses, explainability, discoverability).
