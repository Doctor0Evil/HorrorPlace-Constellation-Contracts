# AI Chat Flow: Audit a Pull Request with CHAT_DIRECTOR

This walkthrough describes how to wire `hpc-chat-director` and the Python tooling into a pull request (PR) review flow, so AI chats and CI can automatically reject non-compliant artifacts before merge.

---

## 1. Goals of PR auditing

The PR audit flow aims to:

- Ensure new or modified schemas remain valid and aligned with the spine.
- Catch registry regressions (broken references, bad IDs).
- Verify AI-generated envelopes and artifacts satisfy structural and band constraints.
- Provide machine-readable diagnostics that AI agents can use to auto-fix PRs.

---

## 2. Detect changed files in a PR

In CI (e.g., GitHub Actions), identify the set of files changed in the PR:

- `schemas/**/*.json`
- `registry/**/*.ndjson`
- `.ai/**/*.json` (ai-authoring requests/responses)
- Any files referenced by manifests as targets.

GitHub Actions example:

```yaml
- name: List changed files
  id: changed
  run: |
    git fetch origin ${{ github.base_ref }} --depth=1
    git diff --name-only origin/${{ github.base_ref }}...HEAD > changed_files.txt
```

AI agents can read `changed_files.txt` to decide which checks to run.

---

## 3. Run schema validation for touched schemas

If any files under `schemas/` changed, validate schemas:

```bash
python tooling/python/cli/hpc-generate-spine-index.py --check
python tooling/python/cli/hpc-validate-schema.py --json
```

- `--check` ensures the spine index matches the current schemas.
- The JSON output of `hpc-validate-schema.py` gives per-file, per-pointer diagnostics.

If any errors are reported, the PR should fail, and an AI assistant can propose fixes.

---

## 4. Lint registries impacted by the PR

If any files under `registry/` changed, lint registries:

```bash
python tooling/python/cli/hpc-lint-registry.py --json --summary
```

The linter:

- Validates NDJSON structure line-by-line.
- Enforces ID patterns and cross-registry references.
- Respects trust and phase boundaries defined in the manifests (via the Rust crate rules mirrored in Python).

CI should fail the PR if:

- Any registry entry is structurally invalid.
- Any reference IDs are missing or forbidden.

---

## 5. Validate AI envelopes before invoking Rust

For any changed files under `.ai/` representing `ai-authoring-request` or `ai-authoring-response` envelopes, run the Python authoring validator first:

```bash
# Requests
find .ai -name "*request*.json" -print0 | xargs -0 -I {} \
  python tooling/python/schemaspine/aiauthoringvalidator.py request {}

# Responses
find .ai -name "*response*.json" -print0 | xargs -0 -I {} \
  python tooling/python/schemaspine/aiauthoringvalidator.py response {}
```

These checks are fast and catch:

- Missing required fields.
- Broken envelopes (no primary artifact, missing provenance).
- Out-of-band invariants and metrics.

Only envelopes that pass here should be forwarded to `hpc-chat-director` in later steps.

---

## 6. Run CHAT_DIRECTOR on AI responses

For each AI response that passed the Python checks, CI can run full validation:

```bash
for resp in $(find .ai -name "*response*.json"); do
  ./target/debug/hpc-chat-director validate-response \
    --from-file "$resp" \
    --format json \
    > "${resp%.json}.validated.json" || exit 1
done
```

This enforces:

- Full JSON Schema validation.
- Invariant and metric enforcement.
- Manifest and tier policies.
- Envelope integrity.

If any validation fails, the job exits with non-zero and the PR is blocked.

---

## 7. Surfacing diagnostics to AI chats

To support AI-assisted PR fixing:

1. Ensure all tooling emits machine-readable diagnostics (`--json` flags, JSONL output).
2. Configure the CI system to attach diagnostics to the PR (e.g., as check annotations, comments, or artifact files).
3. Allow AI agents to read these diagnostics and propose commits that:

   - Fix schema definitions.
   - Correct registry references or IDs.
   - Adjust invariant and metric values into acceptable bands.
   - Repair envelopes and provenance metadata.

---

## 8. Putting it together in CI

A GitHub Actions sketch:

```yaml
name: PR Audit with CHAT_DIRECTOR

on:
  pull_request:
    paths:
      - "schemas/**"
      - "registry/**"
      - ".ai/**"
      - "tooling/python/**"
      - "tooling/lua/**"

jobs:
  audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: actions/setup-python@v5
        with:
          python-version: "3.11"

      - name: Install Python deps
        run: |
          python -m pip install --upgrade pip
          pip install jsonschema

      - uses: dtolnay/rust-toolchain@stable

      - name: Build hpc-chat-director
        run: |
          cargo build --bin hpc-chat-director

      - name: Generate and check spine index
        run: |
          python tooling/python/cli/hpc-generate-spine-index.py --check

      - name: Validate schemas
        run: |
          python tooling/python/cli/hpc-validate-schema.py --json

      - name: Lint registries
        run: |
          python tooling/python/cli/hpc-lint-registry.py --json --summary

      - name: Validate AI authoring envelopes
        run: |
          find .ai -name "*request*.json" -print0 | xargs -0 -I {} \
            python tooling/python/schemaspine/aiauthoringvalidator.py request {}
          find .ai -name "*response*.json" -print0 | xargs -0 -I {} \
            python tooling/python/schemaspine/aiauthoringvalidator.py response {}

      - name: Full CHAT_DIRECTOR validation (responses)
        run: |
          for resp in $(find .ai -name "*response*.json"); do
            ./target/debug/hpc-chat-director validate-response \
              --from-file "$resp" \
              --format json \
              > "${resp%.json}.validated.json"
          done
```

This pattern lets AI chats treat the repo as a machine-checked contract space. Any PR must pass schema validation, registry linting, AI envelope pre-flight checks, and full CHAT_DIRECTOR validation before merge.
