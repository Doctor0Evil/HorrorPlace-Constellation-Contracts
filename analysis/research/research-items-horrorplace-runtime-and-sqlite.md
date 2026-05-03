Below is a set of **50** high‑impact **research‑questions**, **definition‑requests**, **detail‑queries**, and **objection‑identifiers** derived from the provided Godot script, the companion SQLite / Lua / workflow additions, and the surrounding Horror.Place‑constellation architecture.  
They are designed to push all of those components toward AI‑safe, token‑efficient, and rigorously‑specified completion.

**Filename:** `research-items-horrorplace-runtime-and-sqlite.md`  
**Destination:** `analysis/research/` inside the `HorrorPlace‑Constellation‑Contracts` repository

---

## Research‑Questions

These explore open design spaces, algorithmic choices, and future capabilities.

### RQ‑01
**How should the humor module be evolved into a procedurally‑driven horror‑comedy system that adjusts joke intensity based on live invariants (DET, UEC, CDL) while remaining within safe audience‑comfort bands?**

### RQ‑02
**What is the optimal token‑budget per chunk for a GDScript file in the Horror.Place codebase such that a single AI agent call can read, reason about, and emit a complete prism‑validated change without context overflow?**

### RQ‑03
**How can the iptables‑style invariant rule chains be integrated with Godot’s node‑tree events so that a rule’s action (ACCEPT/DROP/ESCALATE) directly modifies game‑object behaviour (e.g., spawn denial, weather intensity, kill‑feed filters) without hard‑coding the mapping?**

### RQ‑04
**What is the smallest set of `chunk_kind` values (schema, contract, runtime‑lua, registry, doc, test) that can cover all repository content while maximising reusability across the constellation, and how should each be token‑priced?**

### RQ‑05
**How can the `hex_arrays` table be used as a pre‑computed cache for pathfinding or region‑accessibility masks inside a Godot‑based horror game, and what is the acceptable staleness (in ticks) before a mask must be re‑evaluated?**

### RQ‑06
**What is the most effective algorithm to derive a single aggregated threat/entertainment “score” from the 11 core invariants and 5 entertainment metrics, suitable for powering dynamic Levelution events in Godot?**

### RQ‑07
**How should the SQLite schema be extended to fully represent “prism contract dependencies” across repositories, enabling a CHATDIRECTOR to automatically determine which prism ID must be invalidated when a schema field’s allowed range changes?**

### RQ‑08
**What minimal Lua/Rust API layer on top of `constellation.db` would allow an AI agent to ask “which files define the DET invariant for region X” and receive a ranked list of actionable chunks, without ever executing raw file‑system access?**

### RQ‑09
**How can the `invariant_rules` table be augmented with a “cool‑down” or rate‑limit field to prevent knowledge‑driven ESCALATE actions from oscillating in tight loops within a multiplayer horror game?**

### RQ‑10
**What telemetry schema additions are required to measure and reduce token waste per AI authoring session, distinguishing between reading overhead, writing overhead, and validation re‑tries?**

### RQ‑11
**How can the “wiring patterns” concept (Add‑Region, Add‑Persona, etc.) be formalised into a JSON Schema so that CHATDIRECTOR can automatically instantiate a pattern into a concrete `chunkPlan` given only a goal description and the `constellation.db`?**

### RQ‑12
**What concurrency model should the Rust‑based `hpc‑constellation‑indexer` use to safely and quickly update the SQLite DB when multiple Git repositories are being scanned in parallel, given that the DB may also be read by AI agents simultaneously?**

### RQ‑13
**How should the `horrorplace_store.gd` stub evolve into a real GDScript singleton that batches, compresses, or filters high‑volume events before insertion into SQLite, to avoid flooding the game thread during intense horror sequences?**

### RQ‑14
**What is the theoretical minimum number of `invariant_rules` per chain required to achieve a guaranteed “safe spawn” decision for any combination of CIC, DET, and LSG values, and can that set be auto‑generated from formal specifications rather than hand‑tuned?**

### RQ‑15
**How can the hex‑mask compression method (32‑bit encoding of rule decisions) be generalised to handle more than 32 rules per snapshot without losing determinism, e.g., by using LuaJIT 64‑bit or multiple 32‑bit chunks with bit packing?**

---

## Definition‑Requests

These ask for precise, machine‑readable specifications that are currently missing.

### DR‑01
**Provide a formal definition (in JSON Schema) of a “humor‑activation profile” that maps game event types, audience‑state invariants, and permissible joke intensity ranges into an `impact` field used by the Godot humor module.**

### DR‑02
**Define the exact set of fields and their allowed values for the “prism‑dependency” sub‑object inside a prism envelope, including how a dependency references a Dead‑Ledger proof when required by the target repo’s authoring rules.**

### DR‑03
**Define the “agent‑profile structural limits” JSON block that explicitly caps the number of chunks an agent may read and write per prism execution, and specify whether those limits are absolute or advisory.**

### DR‑04
**Produce a normative JSON Schema for `chunk-manifest.v1.json` that includes `chunkId`, `repo`, `path`, `startLine`, `endLine`, `approxTokenCost`, `chunkKind`, `invariantsUsed`, `metricsUsed`, and `prismRefs`.**

### DR‑05
**Define a domain‑specific language (or JSON‑structure) for describing a chain‑of‑rules decision flow within `invariant_rules` that supports not just wildcards (`NULL` meaning any) but also range‑only matching without explicit `min_`/`max_` columns for each invariant.**

### DR‑06
**Formalise the “game‑event trigger” vocabulary used by `trigger_game_event` in the Godot script into a controlled taxonomy (e.g., `dynamic_weather`, `levelution_breach`, `spawn_blocked`, `humor_event`) with explicit parameter schemas.**

### DR‑07
**Request a formal definition of “token cost estimation model” for GDScript chunks, including whether line count, byte size, or a heuristic based on function nesting depth is the recommended method and how ±25% accuracy is measured.**

### DR‑08
**Define the “schema‑spine index” SQLite views or queries that must exist to support a CHATDIRECTOR plan‑call, detailing every join needed to resolve an `objectKind`+`tier` into a single repo, schema file, and invariant bands.**

### DR‑09
**Specify the exact message format and delivery guarantees for `notify_kill_feed` in the Godot script, distinguishing between local HUD display, networked broadcast, and SQLite persistence, so that a replacement implementation can be validated.**

### DR‑10
**Define the “metric‑recommendation” JSON structure that binds an `objectKind`+`tier` pair to recommended and forbidden ranges for each entertainment metric, and state whether out‑of‑range values should trigger a warning or a hard rejection in CHATDIRECTOR.**

---

## Detail‑Queries

These request specific implementation facts, parameters, or missing pieces.

### DQ‑01
**What are the exact SQLite `PRAGMA` settings (journal mode, synchronous, cache size, busy timeout) that balance crash‑safety and write throughput for a Godot game that logs 50–200 events per second during horror peaks?**

### DQ‑02
**In the Lua `H.Store.Rules` module, how should `bit.tohex` handle edge cases where the numeric mask is exactly zero or exceeds the width of the bit library’s `tobit` function, and what fallback must be used for non‑LuaJIT runtimes?**

### DQ‑03
**What is the recommended way to extend the `compute_hex_array_for_snapshot` function to produce masks per chain rather than a single aggregate mask, and how should the resulting array IDs be disambiguated?**

### DQ‑04
**Should the `hex_arrays.hex_payload` column store the mask as a hexadecimal string (e.g., `“00FF0A3C”`) or as a SQLite BLOB to reduce overhead when the array is thousands of bits long, and what is the encoding standard for multi‑byte endianness?**

### DQ‑05
**What exact SQL query must the `v_invariant_effective_action` view use when more than one rule in the same chain has the same priority, to ensure a deterministic winner?**

### DQ‑06
**How should the `horrorplace_runtime_events.sql` pragmas be adjusted when the database is hosted on a RAM‑disk or `tmpfs` (for CI testing) versus persistent storage, and which pragmas can be omitted?**

### DQ‑07
**What is the precise lifecycle of a `HorrorPlaceStore` autoload singleton in Godot—should it be instantiated in `_ready` of a root node or as an autoload with `_enter_tree`, and how must shutdown flushes be handled to avoid losing the last event batch?**

### DQ‑08
**What are the exact character limits and JSON‑escaping rules for `invariants_snapshot.meta_json` to guarantee that arbitrary engine metadata (including user‑generated strings) does not break SQLite insertion?**

### DQ‑09
**What is the concrete mechanism for the Godot `GameManager` to receive results from the Lua `H.Store.Rules` module when running in a separate Lua VM? Should it use Unix sockets, shared memory, or a lightweight network protocol, and how is the connection kept alive across scene changes?**

### DQ‑10
**In the CI workflow `lua-rules-tests.yml`, how should the SQLite database file be initialised if multiple tests need to run in parallel? Should each test create an in‑memory copy, or is file‑based with `WAL` mode sufficient for serial test runners?**

### DQ‑11
**What is the exact mapping between the 32‑bit mask bits in `hex_arrays` and the order of rules in `v_invariant_effective_action` when multiple chains produce parallel decisions? Is there a guaranteed deterministic ordering across all SQLite versions?**

### DQ‑12
**What are the required Godot‑side error‑handling steps when `HStore.upsert_flag` fails because the SQLite database is locked? Should it retry with exponential backoff, queue the update for the next frame, or log and drop the flag update?**

---

## Objection‑Identifiers

These highlight risks, inconsistencies, or open problems that could derail the project if left unaddressed.

### OI‑01
**The current Godot `GameManager` script contains only stubs for persistence (`sync_to_backend`); if the real implementation uses a blocking SQLite call on the game thread, frame hitching will occur during high‑event bursts. Identify the threading or asynchronous model needed to avoid this.**

### OI‑02
**The `invariant_rules` table relies on `NULL` to mean “any value”, but SQLite index usage may be suboptimal for nullable range predicates. This could cause full table scans on every snapshot if the table grows beyond a few thousand rules. Objection: the schema must be adapted to use sentinel extreme values or separate “match_any” boolean flags.**

### OI‑03
**The hex‑mask compaction in `compute_hex_array_for_snapshot` silently truncates rules beyond the 32nd. If more than 32 rules are common, this will produce silently incorrect masks, leading to game‑logic bugs. Objection: the function must either raise an error or signal overflow explicitly.**

### OI‑04
**The Lua `H.Store.Rules` module assumes a synchronous `sqlite3` binding, but Horror.Place’s VM‑constellation may need to keep the Lua runtime non‑blocking. Objection: a synchronous call inside a game‑critical path contradicts the event‑loop architecture and must be replaced or wrapped with coroutines.**

### OI‑05
**The schema migration `migrate_001_invariants_and_rules.sql` does not enforce `CHECK` constraints on invariant value ranges (e.g., 0.0–1.0), meaning invalid snapshots from a buggy engine will silently corrupt the analysis. Objection: either add `CHECK` constraints or require an application‑layer validator before every INSERT.**

### OI‑06
**The test script `test_rules_mask.lua` wipes the entire `hex_arrays` table, which could collide with other tests or real telemetry if run against a shared DB. Objection: tests should use an in‑memory SQLite database or a unique test‑only file to prevent data loss.**

### OI‑07
**The Godot `upgrade_upload_artifact_action` function attempts to read and write YAML using stubs, but Godot has no built‑in YAML parser. Objection: the current design expects a hybrid tool to be invoked from Godot, which complicates CI and may introduce shell‑injection vulnerabilities if paths are unsanitised.**

### OI‑08
**The iptables‑style concept in `invariant_rules` uses the action name `ESCALATE`, but there is no corresponding handler in the Godot script; it only triggers `trigger_game_event(“dynamic_weather”)` unconditionally. Objection: the rule engine outputs actions that have no consumer, leading to silent misconfiguration. All action values must be mapped to known game‑event identifiers.**

### OI‑09
**The chunk‑manifest plan relies on a “constellation indexer” that clones or reads every repository, but no access control mechanism is specified for private tiers (T2‑vault, T3‑lab). Objection: the indexer could leak secret content into a debug‑accessible SQLite file. The schema must include visibility‑aware access tokens and the indexer must omit or redact sensitive rows.**

### OI‑10
**The `horrorplace_runtime_events` schema stores all event `payload_json` as text, but the AI agents may need to search inside that JSON. SQLite’s `json1` extension can be missing on some builds. Objection: the design must state whether `json1` is a hard requirement and handle its absence gracefully in Godot and Lua.**

### OI‑11
**The research plan proposes a single `constellation.db` file, but if multiple AI agents and the indexer write to it concurrently, file‑level locking may trigger `SQLITE_BUSY` exceptions, disrupting the entire CI pipeline. Objection: a single‑file SQLite database cannot scale to multiple readers/writers without a WAL‑mode tuning preset and a connection‑pooling layer outside SQLite itself.**

### OI‑12
**The definition of “wiring patterns” assumes AI will always generate a chunk plan before writing files, but the current Godot script directly mutates internal state (`new_platform_features` array) without any contract or prism envelope. Objection: this breaks the one‑file‑per‑request promise and makes the pattern unenforceable. All state‑mutating actions inside the game must likewise be gated by prism‑style authoring contracts, even if that means rewriting the bootstrap flow.**

### OI‑13
**The `v_invariant_effective_action` view filters by `active = 1`, but there is no mechanism in the migrations to prevent accidental deactivation of all rules in a chain, which would leave spawn decisions uncontrolled. Objection: a safety interlock (e.g., a minimum number of active rules per chain) must be enforced at the application level or by a trigger.**

---

Each of the items above is phrased to be directly actionable by a developer, architect, or AI‑assisted tool.  
They map to the specific files and patterns already present in the Horror.Place constellation and are designed to generate the next set of code, schemas, and policies that will make the system production‑ready and AI‑orchestration‑compatible.

**Suggested immediate next steps**:  
- Turn the `Definition‑Requests` into formal JSON Schemas in `HorrorPlace‑Constellation‑Contracts/schemas/`.  
- Answer the `Detail‑Queries` in the repository’s `docs/` and incorporate the results into implementation tickets.  
- Mitigate the `Objection‑Identifiers` before closing any “stable” milestone.
