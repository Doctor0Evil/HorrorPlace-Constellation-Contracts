# SQLite Patterns for AI-Chat Agents and Coding Tools

This note shows how to extend the constellation SQLite schema with a few agent-friendly tables and views that make it easier to generate rich code, navigate repositories, and reason about invariants and metrics at a higher level.

The SQL examples assume the manifest tables described in the constellation wiring docs are present:

- `constellationvmnode`
- `constellationschemaregistry`
- `constellationfilemanifest`
- `constellationfilenode`
- `metric_recommendations`
- `chunks`

## 1. Tagging files for AI-Chat navigation

### 1.1 File tags table

Add a small tagging table so AI-Chat can discover “zones” of the codebase (e.g., BCI adapters, Lua APIs, Godot scenes) without hardcoding paths.

```sql
CREATE TABLE IF NOT EXISTS constellation_file_tag (
    tag_id      INTEGER PRIMARY KEY,
    file_id     INTEGER NOT NULL,
    tag         TEXT    NOT NULL,
    importance  INTEGER NOT NULL DEFAULT 1,
    created_at  TEXT    NOT NULL,
    FOREIGN KEY (file_id) REFERENCES constellation_file_manifest(file_id)
);

CREATE INDEX IF NOT EXISTS idx_file_tag_tag
    ON constellation_file_tag (tag);
```

Examples of tags:

- `bci-adapter`
- `godot-scene`
- `lua-api`
- `schema-spine`
- `prism-contract`
- `telemetry-etl`

### 1.2 View: files by tag and repo

```sql
CREATE VIEW IF NOT EXISTS v_files_by_tag AS
SELECT
    t.tag,
    f.file_id,
    f.repository,
    f.filepath,
    f.objectkind,
    f.schemauri,
    v.vmname,
    v.tier
FROM constellation_file_tag AS t
JOIN constellation_file_manifest AS f
    ON f.file_id = t.file_id
JOIN constellation_vmnode AS v
    ON v.vmnodeid = f.vmnodeid;
```

AI-Chat can ask, “show me BCI adapters in Codebase-of-Death” with:

```sql
SELECT repository, filepath
FROM v_files_by_tag
WHERE tag = 'bci-adapter'
  AND vmname = 'HorrorPlace-Codebase-of-Death';
```

## 2. Linking chunks to metric recommendations

### 2.1 View: chunk + metric recommendation context

Bind `chunks` to `metric_recommendations` so coding agents see the expected metric bands for each chunk’s object kind and tier.

Assumptions:

- `constellationfilenode.objectkind` is aligned with `metric_recommendations.object_kind`.
- `constellationvmnode.tier` uses a string like `T1-core`, `T2-vault`, etc.

```sql
CREATE VIEW IF NOT EXISTS v_chunk_metric_context AS
SELECT
    c.chunk_id,
    c.repo,
    c.path,
    c.start_line,
    c.end_line,
    c.approx_token_cost,
    c.chunk_kind,
    fn.objectkind       AS objectKind,
    v.tier              AS tier,
    mr.metric_code,
    mr.recommended_min,
    mr.recommended_max
FROM chunks AS c
JOIN constellation_file_manifest AS f
    ON f.repository = c.repo AND f.filepath = c.path
JOIN constellation_file_node AS fn
    ON fn.file_id = f.file_id
JOIN constellation_vmnode AS v
    ON v.vmnodeid = f.vmnodeid
LEFT JOIN metric_recommendations AS mr
    ON mr.object_kind = fn.objectkind
   AND mr.tier = v.tier;
```

Agents can use this to:

- Highlight chunks that are part of high‑DET or high‑UEC contracts.
- Suggest code comments or scaffolding based on target bands.

Example query:

```sql
SELECT chunk_id, path, metric_code, recommended_min, recommended_max
FROM v_chunk_metric_context
WHERE objectKind = 'regionContractCard'
  AND metric_code = 'UEC';
```

## 3. Cross-repo schema navigation

### 3.1 View: schema consumers by repo and object kind

This view answers: “where is this schema used, and in what role?” for a given schema URI.

```sql
CREATE VIEW IF NOT EXISTS v_schema_consumers AS
SELECT
    s.schemauri,
    s.schemakind,
    v.vmname,
    f.repository,
    f.filepath,
    f.objectkind
FROM constellationschemaregistry AS s
JOIN constellation_file_manifest AS f
    ON f.schemauri = s.schemauri
JOIN constellation_vmnode AS v
    ON v.vmnodeid = f.vmnodeid;
```

AI-Chat can use this to find:

- All `region-contract-card` schema consumers across Atrocity-Seeds and Horror.Place.
- The correct repo+path when asked to “extend the region contract to support humor hints.”

Example query:

```sql
SELECT vmname, repository, filepath, objectkind
FROM v_schema_consumers
WHERE schemauri = 'region-contract-card.v1.json';
```

## 4. Code-oriented views for agents

### 4.1 View: language-oriented file index

If you track language in `constellationfilemanifest.language`, create a view that AI-Chat can query when it needs “all Rust files touching BCI” or “Lua APIs in Horror.Place”.

```sql
CREATE VIEW IF NOT EXISTS v_code_files AS
SELECT
    f.file_id,
    f.repository,
    f.filepath,
    f.language,
    f.objectkind,
    f.schemauri,
    v.vmname,
    v.tier
FROM constellation_file_manifest AS f
JOIN constellation_vmnode AS v
    ON v.vmnodeid = f.vmnodeid
WHERE f.language IN ('lua', 'gdscript', 'rust', 'cpp');
```

Paired with `constellation_file_tag`, this gives coding agents a clean substrate:

```sql
SELECT repository, filepath
FROM v_code_files
WHERE vmname = 'HorrorPlace-Codebase-of-Death'
  AND language = 'lua';
```

### 4.2 View: “hotspots” by token cost and metrics

Bind `chunks` to invariants/metrics to find high‑impact areas:

```sql
CREATE VIEW IF NOT EXISTS v_chunk_hotspots AS
SELECT
    c.chunk_id,
    c.repo,
    c.path,
    c.approx_token_cost,
    fn.objectkind,
    fn.det,
    fn.cic,
    fn.aos,
    fn.uec,
    fn.arr,
    fn.cdl
FROM chunks AS c
JOIN constellation_file_manifest AS f
    ON f.repository = c.repo AND f.filepath = c.path
JOIN constellation_file_node AS fn
    ON fn.file_id = f.file_id;
```

An agent can then ask:

- “Show me the top 10 highest‑DET chunks in Atrocity‑Seeds”:

```sql
SELECT chunk_id, path, det, approx_token_cost
FROM v_chunk_hotspots
WHERE repo = 'HorrorPlace-Atrocity-Seeds'
ORDER BY det DESC
LIMIT 10;
```

And choose which segments to open or refactor first.

## 5. How agents should use these patterns

- **Discovery:** Use `v_files_by_tag` and `v_code_files` to locate the right Lua, GDScript, Rust, and schema files instead of guessing paths.
- **Context:** Use `v_chunk_metric_context` and `v_chunk_hotspots` to pick safe, high‑signal chunks when generating or modifying code.
- **Routing:** Use `v_schema_consumers` to resolve from an `objectKind` or schema URI to the correct repo and path for new artifacts.
- **Planning:** Combine views with wiring patterns (`wiring-patterns/*.json`) so that each AI‑Chat plan can:
  - Find the relevant files.
  - Read only a bounded number of chunks.
  - Emit new artifacts and SQLite row manifests that align with invariants and metric recommendations.

These patterns make the SQLite database a first-class navigation and planning substrate for AI‑Chat and coding agents, not just a passive log. With the views in place, agents can issue small, deterministic SQL queries instead of attempting to infer the repo’s structure from scratch.
