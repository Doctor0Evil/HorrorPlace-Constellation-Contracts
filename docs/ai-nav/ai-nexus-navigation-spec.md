## Horror.Place Nexus Navigation Spec (AI‑Facing)

This document tells AI agents how to **think and talk** about the Horror.Place constellation without wasting tokens.

### 1. Mental model: graph, not text blob

Treat the constellation as a **graph**:

- **Nodes**:  
  - Repos (e.g., `Horror.Place`, `HorrorPlace-Atrocity-Seeds`).  
  - Contract families / objectKinds (e.g., `moodContract`, `seedContractCard`).  
  - Tiers (`T1-core`, `T2-vault`, `T3-lab`).  
  - Governance / metrics (`Dead-Ledger`, invariants, metrics).

- **Edges**:  
  - `route(objectKind, tier) → repo`: defined by `hpc-routing-spine-v1.json`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/019e6ee1-b215-4c82-a4b5-0cf4931b980d/this-research-focuses-on-devel-Eh3ThsQBQg.r6HDAZZhx5w.md)
  - `manifest(repo) contains objectKind`: defined by `repo-manifest.hpc.*.json`.  
  - `governs(profile) → contractFamily`: defined by Dead‑Ledger profiles. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/019e6ee1-b215-4c82-a4b5-0cf4931b980d/this-research-focuses-on-devel-Eh3ThsQBQg.r6HDAZZhx5w.md)

When answering questions, **navigate the graph** in 3 hops:

1. Identify **objectKind** and **tier** (or “type of thing” and “sensitivity level”).  
2. Follow the routing spine to get the **target repo** and **schemaRef**. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/019e6ee1-b215-4c82-a4b5-0cf4931b980d/this-research-focuses-on-devel-Eh3ThsQBQg.r6HDAZZhx5w.md)
3. Check the repo’s manifest to see if that repo **accepts** that objectKind and tier. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/019e6ee1-b215-4c82-a4b5-0cf4931b980d/this-research-focuses-on-devel-Eh3ThsQBQg.r6HDAZZhx5w.md)

If you cannot identify all three (objectKind, tier, repo), **stop and ask for clarification** instead of guessing.

***

### 2. Cheap token discipline: three registers

Use three mental “registers” to minimize tokens:

1. **Intent register (I)** – tiny summary of what the user wants.  
2. **Routing register (R)** – minimal tuple of routing info.  
3. **Plan register (P)** – compressed list of steps.

Format them **implicitly in your own head**, and only surface them when asked, or when the user clearly wants structured plans.

#### 2.1 Intent register (I)

Internally track:

- `I.kind` – which **objectKind** or family?  
- `I.tier` – which tier is implied?  
- `I.repo` – which repo is the natural target?

Examples of your own mental resolution (no need to print unless asked):

- “Design a new region card for the marsh” →  
  - `I.kind = regionContractCard`, `I.tier = T2-vault`, `I.repo = HorrorPlace-Black-Archivum`.  
- “Tune BCI geometry for this corridor” →  
  - `I.kind = bci-geometry-tileset`, `I.tier = T1-core`, `I.repo = Horror.Place`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/019e6ee1-b215-4c82-a4b5-0cf4931b980d/this-research-focuses-on-devel-Eh3ThsQBQg.r6HDAZZhx5w.md)

If any of these are ambiguous, ask a short disambiguation question instead of generating long reasoning.

#### 2.2 Routing register (R)

Always be able to form the routing tuple:

- `R = (objectKind, tier, repo, schemaRef, defaultPath)`.

You **don’t need to print it** by default; just ensure your answer is consistent.  
When the user is talking about file placement, commits, or CI wiring, it is appropriate to **print** R in 1–2 lines.

Example:

- “Where should this seed go?”  
  - Internally: `R = (seedContractCard, T2-vault, HorrorPlace-Atrocity-Seeds, seed-contract-card-v1.json, seeds/)`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/019e6ee1-b215-4c82-a4b5-0cf4931b980d/this-research-focuses-on-devel-Eh3ThsQBQg.r6HDAZZhx5w.md)

#### 2.3 Plan register (P)

Keep plans short and structured:

- `P.steps` – 3–5 steps max.  
- Prefer **“derive → validate → wire”** rather than long chains.

Example internal plan:

1. Derive the correct objectKind/tier/repo.  
2. Map to schema and default path.  
3. Generate the minimal contract or manifest snippet.  
4. Suggest one CI / lint step.

Only print the plan if the user asks for “steps”, “actions”, or “wiring plan”.

***

### 3. Nexus‑graph reasoning patterns

Use these **patterns** whenever dealing with constellation work.

#### 3.1 “Where does this live?”

User asks where to put an artifact or feature.

1. Resolve **objectKind** (what is it?) from the description.  
2. Resolve **tier** (T1 public, T2 vault, T3 lab) from sensitivity.  
3. Use the routing spine: find `route(objectKind, tier)` and read `repo`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/019e6ee1-b215-4c82-a4b5-0cf4931b980d/this-research-focuses-on-devel-Eh3ThsQBQg.r6HDAZZhx5w.md)
4. Confirm the repo’s manifest allows that objectKind and tier. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/019e6ee1-b215-4c82-a4b5-0cf4931b980d/this-research-focuses-on-devel-Eh3ThsQBQg.r6HDAZZhx5w.md)
5. Answer with: repo name + relative path + schemaRef, in one short sentence.

Example answer style:

- “This should live in `HorrorPlace-Atrocity-Seeds` under `seeds/`, using `seed-contract-card-v1.json` as the schema.”

No need to narrate your full reasoning unless explicitly requested.

#### 3.2 “How do these connect?”

User asks how repos or artifacts relate.

1. Treat it as a **graph query**: nodes = repos, edges = shared objectKinds or references.  
2. Identify the **shared objectKinds** or IDs (e.g., region IDs referenced in multiple repos).  
3. Describe 1–2 key edges in plain language:  
   - “Atrocity-Seeds seeds reference region IDs from Black-Archivum and spectral moods from Spectral-Foundry.”  
4. Keep the explanation to 2–4 sentences.

#### 3.3 “What should I do next?”

User wants next coding or design actions.

1. Check if the question is about:  
   - schema,  
   - manifests,  
   - routing,  
   - runtime adapters,  
   - governance/Dead-Ledger.  
2. Pick **one layer** first (e.g., manifests), suggest 3–4 concrete actions there.  
3. Mention how those actions affect routing or invariants **in one sentence**.

Keep the plan under 6–8 total sentences.

***

### 4. Low‑token strategies for planning and wiring

You do not need long chain‑of‑thought. Use **compressed, structured thinking**.

#### 4.1 Avoid long narrated reasoning

Prefer:

- Short, decisive answers referencing:  
  - objectKind,  
  - tier,  
  - repo,  
  - file path,  
  - schemaRef.

If you need to explore options, present at most **two**:

- “You can either treat this as a `moodContract` in T1 and route to Horror.Place, or as a higher-intensity `eventContract` in T2 routed to Atrocity-Seeds.”

#### 4.2 Reuse known patterns

When generating new things:

- For **new manifests**, copy an existing manifest of similar repo type (vault, lab, orchestrator) and only change:  
  - `repoName`,  
  - `allowedObjectKinds`,  
  - `defaultPaths`,  
  - `schemaWhitelist`.  
- For **new routes**, pattern after existing entries in `hpc-routing-spine-v1.json` for the same tier (e.g., reuse invariants and metrics bands). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/019e6ee1-b215-4c82-a4b5-0cf4931b980d/this-research-focuses-on-devel-Eh3ThsQBQg.r6HDAZZhx5w.md)

Explicitly say “pattern this after X” instead of re‑explaining everything.

#### 4.3 Use small tables instead of long prose

When comparing repos, objectKinds, or paths, use a **small markdown table** with 3–6 rows and 3–5 columns. This is often cheaper and clearer than multiple paragraphs.

Example columns:

- `objectKind` | `tier` | `repo` | `path` | `notes`  

Populate with minimal text; one short phrase per cell.

***

### 5. Behavior for constellation maintenance

When the user asks about maintenance, upgrades, or refactors:

1. **Identify layer**:  
   - Schemas  
   - Routing spine  
   - Manifests  
   - CI / lint  
   - Runtime (Lua / Rust)  
   - Governance (Dead-Ledger, policies)

2. For each answer, **touch at most two layers** at once.  
3. Always mention how changes affect:  
   - routing consistency,  
   - manifests’ `allowedObjectKinds`,  
   - and CI (routing lint / constellation audit).

Keep the answer like:

- “Add this objectKind to the routing spine, update manifests for these two repos, and extend the audit tool to check for it.”

***

### 6. Quick reference checklist for every new request

Internally (no need to print unless asked), run this checklist:

1. **What is this?**  
   Map to `objectKind` or say it’s “unclear” and ask.

2. **How sensitive is it?**  
   - Public → `T1-core`  
   - Vault / explicit horror → `T2-vault`  
   - Experimental / research → `T3-lab`

3. **Where should it live?**  
   Use routing spine to get `repo`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/019e6ee1-b215-4c82-a4b5-0cf4931b980d/this-research-focuses-on-devel-Eh3ThsQBQg.r6HDAZZhx5w.md)

4. **What contract/schema governs it?**  
   Identify `schemaRef` and a default path.

5. **What invariants/metrics matter?**  
   Name 2–4 relevant invariants (CIC, AOS, LSG, SHCI…) and metrics (UEC, ARR, etc.) only if the question is about tuning or safety. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cdb90fc3-8a6a-46e2-89a6-187b2f85f988/019e6ee1-b215-4c82-a4b5-0cf4931b980d/this-research-focuses-on-devel-Eh3ThsQBQg.r6HDAZZhx5w.md)

6. **What small set of actions help most?**  
   Suggest 3–5 concrete edits or files, tied to 1–2 layers.

***

### 7. How to respond to future constellation questions

For **any future request about Horror.Place repos or constellation maintenance**:

- First, silently determine the routing tuple `(objectKind, tier, repo)`.  
- Second, decide which **one** of these the user really wants:

  - A file skeleton (schema, manifest, contract, Lua module).  
  - A wiring plan (paths, routes, CI steps).  
  - A maintenance plan (next actions across repos).  
  - An explanation (how/why the routing and invariants apply).

- Answer with the **smallest artifact** that unblocks them:  
  - One file,  
  - One table,  
  - Or one 3–5 step plan.

Only expand beyond that if the user explicitly requests more detail (“show more steps”, “explain why”, “give me the full spec”).
