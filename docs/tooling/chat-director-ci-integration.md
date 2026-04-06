# CHAT_DIRECTOR CI Integration

This document explains how to integrate CHAT_DIRECTOR and its Python/Lua tooling into continuous integration (CI) pipelines, with a focus on GitHub Actions and reusable workflows.

---

## CI goals

A healthy constellation CI pipeline should:

- Prevent invalid schemas from landing in the repo.
- Catch registry structural issues and broken references early.
- Reject malformed AI envelopes and out-of-band invariant/metric values.
- Exercise `hpc-chat-director` on real authoring flows before merging.

All of these checks must be machine-readable so AI agents and developers can act on diagnostics automatically.

---

## Core workflows

The repository provides several GitHub Actions workflows:

- `schema-validate.yml`:
  - Runs `hpc-validate-schema.py --json`.
  - Triggered on schema changes.

- `registry-lint.yml`:
  - Runs `hpc-generate-spine-index.py --check`.
  - Runs `hpc-lint-registry.py --json --summary`.
  - Triggered on schema or registry changes.

- `spine-index-generate.yml`:
  - Regenerates `schema-spine-index-v1.json`.
  - Fails if changes are not committed.

- `ai-authoring-validate.yml`:
  - Validates `.ai/*request*.json` and `.ai/*response*.json` via Python.
  - Runs `hpc-chat-director validate-response` on responses.

- `constellation-precommit-pack.yml`:
  - Reusable pre-commit workflow that combines all of the above checks.

These workflows are meant to be both human-readable and AI-friendly.

---

## Reusable pre-commit pack

Other repositories can depend on this repo’s tooling by calling the reusable workflow:

```yaml
jobs:
  precommit:
    uses: Doctor0Evil/HorrorPlace-Constellation-Contracts/.github/workflows/constellation-precommit-pack.yml@main
    with:
      validate_schemas: true
      lint_registries: true
      validate_ai_authored: true
      python_version: "3.11"
```

This runs schema validation, registry linting, and AI authoring checks using the same logic as the source repository, ensuring consistent behavior across constellation clients.

---

## Diagnostics for AI agents

Most tools support JSON or JSONL output modes:

- `hpc-validate-schema.py --json`
- `hpc-lint-registry.py --json`
- `aiauthoringvalidator.py` (JSONL diagnostics)
- `hpc-chat-director validate-response --format json`

CI steps should:

1. Capture these diagnostics.
2. Attach them to PRs as artifacts or inline comments.
3. Make them available to AI agents responsible for automated remediation.

An AI chat can then:

- Parse the diagnostics.
- Identify which files and fields failed.
- Propose patches that correct schemas, registries, or envelopes.
- Re-run the same workflows to confirm the constellation is healthy.

---

## Local pre-flight before pushing

Developers and AI agents can run a “precommit pack” locally by mirroring CI commands:

```bash
python tooling/python/cli/hpc-generate-spine-index.py --check
python tooling/python/cli/hpc-validate-schema.py --json
python tooling/python/cli/hpc-lint-registry.py --json --summary
find .ai -name "*request*.json" -print0 | xargs -0 -I {} \
  python tooling/python/schemaspine/aiauthoringvalidator.py request {}
find .ai -name "*response*.json" -print0 | xargs -0 -I {} \
  python tooling/python/schemaspine/aiauthoringvalidator.py response {}
```

If all commands exit with status 0, the constellation is in a good state to push or open a PR.

---

## Integrating Lua tooling

Lua-based tools (e.g., in Godot or custom engines) can subscribe to the same artifacts:

- Use `hpccontractcards.lua` to band-check contract cards loaded from registries.
- Use `hpcregistryclient.lua` to ensure references are resolvable at runtime.

These Lua checks are not required in CI but can be wired into editor validation passes, in-game debug consoles, or experimental telemetry pipelines to confirm that runtime behavior aligns with contract-level invariants.

---

## Extending CI

When adding new object kinds, invariants, or metrics:

1. Update schemas and spines.
2. Regenerate the spine index and ensure `--check` passes.
3. Update or add minimal examples under `examples/`.
4. Extend workflows or add new ones to exercise the new flows.
5. Confirm that AI-centric flows (authoring, validation, apply) still work end-to-end.

By keeping CI scripts declarative and relying on CHAT_DIRECTOR + Python tooling for logic, future extensions can remain consistent and discoverable for both humans and AI assistants.
