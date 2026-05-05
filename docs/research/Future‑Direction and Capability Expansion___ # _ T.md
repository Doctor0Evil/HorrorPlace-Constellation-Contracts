<img src="https://r2cdn.perplexity.ai/pplx-full-logo-primary-dark%402x.png" style="height:64px;margin-right:32px"/>

# Future‑Direction and Capability Expansion

| \# | Type | Item |
| :-- | :-- | :-- |
| 47 | RQ | How can we integrate the “field → repo coverage map” with a semantic search index (e.g., vector embeddings of field descriptions) so agents can find fields by natural language intent? |
| 48 | OI | If the ledger log in `HorrorPlace-Dead-Ledger-Network` is stored as SQLite, does the current wiring pattern protect against an agent accidentally executing a write query that corrupts CAN token audit trails? |
| 49 | DQ | Should the `for-ai-agents.md` entrypoint include a “recommended first queries” section that bootstraps agent knowledge in under 500 tokens? |
| 50 | DR | Precisely define “token budget efficiency” as a measurable metric for comparing the old prose‑based navigation against the new indexed/SQL‑based approach, so we can demonstrate the improvement to stakeholders. |


---

**Usage:**

- **AI agents** may load this document alongside `docs/for-ai-agents.md` to prioritize their immediate tasks.
- **Human maintainers** can use the items as a checklist for iterative project improvements.
- **Continuous integration** can transform selected DQs into automated lint rules or schema validators.

Answers build on your field index, CAN, and AI‑agent workflows.

***

## 47 · Field coverage + semantic search

You can layer a semantic index directly on top of `schema_fields` and `field_usage` without changing their SQL shape.

Proposed approach:

- Define a small `field_embeddings` table:
    - Columns:
        - `field_id` (FK → `schema_fields.field_id`).
        - `model` (e.g., `"text-embedding-3-small"`).
        - `dim` (embedding dimension).
        - `vector` (BLOB or space‑separated floats).
- Embedding source text (per field):
    - Concatenate:
        - `schema_fields.field_path` (e.g., `visual.maskRadius`),
        - `schema_fields.description` (short semantic summary),
        - Optional: top 1–2 `field_usage.note` texts for that field.
    - This yields a compact, low‑noise description suitable for vectorization.
- Workflow for agents:
    - Natural language → embedding via a service or offline model.
    - Run a nearest‑neighbor search over `field_embeddings` (either in memory or via a simple ANN index outside SQLite).
    - Use the resulting `field_id` list to query `field_usage` and see where those fields appear across repos.
- Storage and tooling discipline:
    - Keep embeddings out of the core constellation SQL if you want to stay pure‑SQLite; instead, treat them as an **adjacent cache** maintained by a separate tool (e.g., `tools/build-field-embeddings.sh` that reads `schema_fields` via SQLite and writes a small `field_embeddings.json` or `field_embeddings.sqlite`).
    - Reference this artifact in `docs/constellation-index.json` so agents find it cheaply.

This lets an agent ask “fields involved in visual tunnel vision” in natural language, embed that query, and rank `visual.maskRadius`, `visual.motionSmear`, etc., without scanning prose docs.

***

## 48 · Ledger SQLite writes and CAN audit safety

Current wiring strongly pushes agents toward **read‑only** inspection of logs and CAN registry data, but it does not, by itself, prevent an agent from executing a write on the ledger DB.

Relevant protections you already have:

- Separation of concerns:
    - CAN token registry index (`cantoken`, `cantokenregistry`) lives in Rotting‑Visuals‑BCI logging DB, not in the ledger.
    - Ledger tables are in a different repo (`HorrorPlace-Dead-Ledger-Network`) and schema file.
- Tooling policy:
    - `docs/agent-tooling-policy.md` instructs agents to use `sqlite3` for introspection and analysis, but does not explicitly forbid `INSERT`/`UPDATE`.
- Navigation patterns:
    - Workflows in `docs/for-ai-agents.md` show only `SELECT` queries for bcirequestframe, bcibindinggeometry, cantoken; no write examples are given.

To harden this:

- Add an explicit **read‑only rule** for production / audit DBs:
    - In `docs/agent-tooling-policy.md`, add:
        - “When connected to production or audit databases (ledger logs, CAN audit trails), AI agents must only issue `SELECT` statements. DDL or DML (`INSERT`, `UPDATE`, `DELETE`) are forbidden unless a task is explicitly tagged as migration and reviewed by humans.”
- Provide separate “scratch” DB recommendation:
    - Encourage agents to copy a ledger file or use a seed DB for experimentation, and to never modify the live audit database.
- Optional technical controls:
    - Run ledger SQLite in **WAL + file permission** mode where the process user for agents has read‑only access to the ledger file.
    - For programmatic access, expose a read‑only API (HTTP or IPC) for agents; keep raw write access restricted to the ledger service.

So: the wiring pattern helps (by separating registries and logs), but you still need explicit policy and OS‑level controls to fully prevent accidental write queries on the ledger.

***

## 49 · “Recommended first queries” in `for-ai-agents.md`

Yes, you should add a short “First queries” section; your existing workflows already hint at the right patterns.

Design constraints:

- Under ~500 tokens.
- Only `SELECT` queries.
- Guide agents to:
    - Confirm schemas.
    - See a tiny sample of BCI frames, bindings, and CAN tokens.

Example section for `docs/for-ai-agents.md`:

- “Recommended first queries”:
    - Confirm tables exist:

```sql
-- Count core Rotting-Visuals tables
SELECT COUNT(*) AS frames   FROM bcirequestframe;
SELECT COUNT(*) AS bindings FROM bcibindinggeometry;
SELECT COUNT(*) AS cantokens FROM cantoken;
```

    - Inspect one frame:

```sql
-- Look at a single recent BCI frame
SELECT frameid, timestampms, stressscore, visualoverload
FROM bcirequestframe
ORDER BY timestampms DESC
LIMIT 1;
```

    - Inspect bindings for that frame:

```sql
SELECT bindingkey, region, maskradius, motionsmear
FROM bcibindinggeometry
WHERE frameid = ?;  -- plug in the frameid from the previous query
```

    - Show a few CAN tokens:

```sql
SELECT tokenkey, bindingidhint, targetpath, maxgain, intent
FROM cantoken
ORDER BY tokenkey
LIMIT 10;
```


You can precede this with 3–4 bullets explaining what each query teaches (tables present, BCI shape, geometry shape, CAN control surface), keeping the section compact.

***

## 50 · “Token budget efficiency” metric

You already have a formalization for **context window budget** and a 10× parameter‑per‑turn gain for CAN vs JSON.[^1]
You can adapt that to measure navigation efficiency for prose vs indexed/SQL approaches.

Definitions:

- Let $C_{\text{max}}$ be the context window limit in tokens.[^1]
- Let $T_{\text{prompt}}$ be fixed system + instructions tokens.
- Let $T_{\text{resp}}$ be reserved tokens for the model’s answer.
- Let $T_{\text{nav}}$ be tokens consumed by **navigation artifacts**: docs, schemas, SQL queries, results.

For a given task class (e.g., “find all places where `visualOverloadIndex` is used across repos”):

- Old navigation:
    - Agents load prose docs, multiple schemas, and scan Rust; suppose this costs $T_{\text{nav,prose}}$ tokens.
- New navigation:
    - Agents load `docs/constellation-index.json`, `db/schema/field_index.sql` tables, and run a minimal query on `field_usage`; suppose this costs $T_{\text{nav,indexed}}$.

Token budget efficiency:

- Define **navigation efficiency** for a task as:

$$
E = \frac{T_{\text{max,nav}}}{T_{\text{nav}}}
$$

where $T_{\text{max,nav}} = C_{\text{max}} - T_{\text{prompt}} - T_{\text{resp}}$ is the maximum budget available for navigation artifacts.[^1]
- Old vs new:

$$
E_{\text{prose}} = \frac{T_{\text{max,nav}}}{T_{\text{nav,prose}}}, \quad
E_{\text{indexed}} = \frac{T_{\text{max,nav}}}{T_{\text{nav,indexed}}}
$$

and the **improvement factor** is

$$
I = \frac{E_{\text{indexed}}}{E_{\text{prose}}}
  = \frac{T_{\text{nav,prose}}}{T_{\text{nav,indexed}}}
$$

which directly compares how many times more navigation “work” you get from the same budget using the indexed approach.[^1]

Operationalizing this:

- Choose representative tasks (e.g., pipeline wiring, field coverage, CAN safety audit).
- For each, measure:
    - Tokens of prose / raw schemas needed before SQL refactor.
    - Tokens of `ai-metadata.json`, wiring JSON, and SQL result rows after refactor.
- Report to stakeholders as:
    - “For task X, we reduced navigation tokens from ~12 000 to ~1 200, giving a token budget efficiency improvement of ~10×.”
    - This mirrors the 10× CAN compression story you already have for bindings.[^1]

***

### Quick table

| \# | Topic | Recommendation |
| :-- | :-- | :-- |
| 47 | Field coverage + semantic search | Add `schema_fields`‑backed embeddings (adjacent cache), use natural‑language → embedding → nearest fields → `field_usage` lookups. |
| 48 | Ledger SQLite write safety | Current wiring separates logs and registry but does not fully prevent writes; add explicit read‑only policy and OS‑level protections for ledger DB. |
| 49 | First queries in `for-ai-agents.md` | Yes; add a short “Recommended first queries” section with 3–4 `SELECT` examples that establish tables and data shape under 500 tokens. |
| 50 | Token budget efficiency metric | Define efficiency as $E = T_{\text{max,nav}} / T_{\text{nav}}$; compare old prose vs new indexed SQL by measuring navigation tokens for representative tasks and reporting the improvement factor $I$. [^1] |

<div align="center">⁂</div>

[^1]: 55ecd068-6fbd-49be-b3e9-152b69bad05b.md

