# Minimal Constellation Example

This directory provides a small, self-contained constellation that demonstrates how the CHAT_DIRECTOR toolchain fits together: Python schema tooling, NDJSON registries, the schema spine index, and the `hpc-chat-director` CLI.

The goal is to give both humans and AI agents a clear, reproducible CI flow that can be used as a reference when wiring up larger constellations.

---

## Directory layout

- `schemas/` – minimal v1 JSON Schemas for core contract types used in this constellation.
- `registry/` – NDJSON registries with one or two entries per family (regions, moods, events, seeds).
- `.github/workflows/minimal-constellation-ci.yml` – CI workflow that runs the Python CLI tools and `hpc-chat-director` in a single, coherent sequence.

The schemas and registries in this directory are intentionally small and should remain easy to read and reason about.

---

## Prerequisites

Before running the commands below, ensure:

1. The repository is checked out with the usual layout (top-level `schemas/`, `registry/`, `tooling/python`, `tooling/lua`, etc.).
2. Python tooling is installed and available:

   ```bash
   pip install jsonschema
   ```

3. The Rust crate and CLI have been built (or installed):

   ```bash
   cargo build --bin hpc-chat-director
   ```

   or

   ```bash
   cargo install hpc-chat-director
   ```

---

## Step 1: Generate the schema spine index

The spine index aggregates core schemas, invariants, and entertainment metrics into a normalized `schema-spine-index-v1.json` that both Rust and Lua tools can trust.

From the repository root:

```bash
python tooling/python/cli/hpc-generate-spine-index.py
```

This will:

- Walk `schemas/core/` for JSON Schemas.
- Read `invariants-spine.v1.json` and `entertainment-metrics-spine.v1.json`.
- Emit `schemas/core/schema-spine-index-v1.json`.

If you want to ensure the committed file is up to date without rewriting it:

```bash
python tooling/python/cli/hpc-generate-spine-index.py --check
```

A non-zero exit code in `--check` mode indicates drift and should fail CI.

---

## Step 2: Validate core schemas

Next, run schema validation across all relevant schema files, including the minimal constellation’s `schemas/` directory.

From the repository root:

```bash
python tooling/python/cli/hpc-validate-schema.py
```

This command:

- Finds `schemas/**/*.json`.
- Runs JSON Schema Draft 2020-12 validation on each file.
- Emits diagnostics and exits non-zero if any schema is malformed or references are broken.

For machine-readable diagnostics (for AI tools or automated dashboards):

```bash
python tooling/python/cli/hpc-validate-schema.py --json
```

---

## Step 3: Lint NDJSON registries

With schemas valid and the spine index in place, lint the NDJSON registries that make up this minimal constellation.

From the repository root:

```bash
python tooling/python/cli/hpc-lint-registry.py
```

This will:

- Discover `registry/**/*.ndjson` files (including `examples/minimal-constellation/registry/`).
- Validate each line against the appropriate registry schema.
- Enforce ID patterns and cross-reference rules (e.g., region IDs referenced by events must exist).
- Exit non-zero on any structural or referential error.

For JSONL diagnostics suitable for AI parsing:

```bash
python tooling/python/cli/hpc-lint-registry.py --json
```

---

## Step 4: Run CHAT_DIRECTOR on the minimal constellation

Once the spine, schemas, and registries are consistent, you can exercise `hpc-chat-director` using a minimal AI authoring flow.

Example: validate a prepared AI response for a region contract card that belongs to this constellation.

From the repository root:

```bash
hpc-chat-director validate-response \
  --from-file examples/minimal-constellation/registry/region-contract-response.json \
  --json
```

This command:

- Loads the schema spine and manifests.
- Validates the response envelope and underlying artifact against schemas, invariants, and registries.
- Returns structured diagnostics as JSON.

You can add additional commands to:

- `init` a new request,
- `plan` authoring routes,
- `apply` validated artifacts into the minimal constellation’s registries.

---

## Step 5: CI pipeline overview

The `.github/workflows/minimal-constellation-ci.yml` workflow demonstrates the expected end-to-end CI sequence:

1. Checkout repo and install Python + Rust toolchain.
2. Run `hpc-generate-spine-index.py --check` to ensure the spine index is current.
3. Run `hpc-validate-schema.py` to ensure all schemas are structurally valid.
4. Run `hpc-lint-registry.py` to catch registry issues early.
5. Run `hpc-chat-director` commands against the example artifacts to prove CLIs and constellations interact as expected.

Local developers and AI chats can mirror this flow by running the same commands manually before opening or updating a pull request.

---

## Extending the minimal constellation

To extend this example while keeping it pedagogical:

- Prefer adding new schema fields under the existing minimal v1 schemas rather than introducing many new families.
- Keep registries small: one or two entries per family (regions, moods, events, seeds) that form a coherent micro-world.
- Update the README with any new commands or CI steps needed to keep the constellation healthy.

Whenever schemas or invariants change, update the spine index and ensure `--check` passes before committing.
