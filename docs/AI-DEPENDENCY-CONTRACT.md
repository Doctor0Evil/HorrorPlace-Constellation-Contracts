# AI Dependency Contract for This Repository

This document is a binding contract for all coding agents (AI or human‑assisted) working in this repository. It governs how dependencies, Cargo configuration, and build commands must be handled, especially in constrained CI or sandbox environments.

The high‑level rule is:

> **Do not change dependencies or perform networked cargo operations unless a human has explicitly requested a dependency update task.**

This repo is designed to run in low‑disk, low‑network environments. Breaking these rules will cause repeated build failures and noisy tool output.

---

## 1. Cargo.lock is law

1. `Cargo.lock` is the canonical source of dependency versions.
2. Agents must treat the existing combination of:
   - `Cargo.toml` files
   - `Cargo.lock`
   as **frozen**, unless a human explicitly assigns a “dependency update” task.
3. Agents must not:
   - Add new crates to `[dependencies]`, `[dev-dependencies]`, or `[build-dependencies]`.
   - Bump existing version numbers.
   - Remove dependencies to “clean up” or “simplify” builds.
4. If you believe a new dependency is strictly necessary:
   - Do **not** modify `Cargo.toml` directly.
   - Instead, add a short note under `docs/BCI-DEV-TODO.md` (or the relevant TODO backlog) describing:
     - The crate you want to add.
     - The crate and module where it would be used.
     - Why std / existing crates are insufficient.
   - Leave the current build configuration untouched.

---

## 2. Network‑free builds in constrained environments

Many CI and sandbox environments used with this repository have:

- Very limited disk space.
- No guarantee that network traffic to `crates.io` can succeed.
- A pre‑populated Cargo cache with a fixed subset of crates.

To respect this:

1. Agents must assume that **no new crates can be downloaded** during normal development and CI runs.
2. When suggesting or scripting build commands, prefer:
   - `cargo check --offline`
   - `cargo test --offline`
   - `cargo build --offline`
   whenever reasonable.
3. Agents must not include these commands in CI snippets or shell examples:
   - `cargo update`
   - `cargo install <crate>`
   - `cargo install --path .`
   - `cargo fetch` (except in explicit provisioning jobs with their own instructions)
4. If you see errors like:
   - `failed to update registry 'crates-io'`
   - `cannot extend packfile ... No space left on device`
   then:
   - Do **not** propose dependency changes as a fix.
   - Do **not** propose deleting the entire cargo registry or other global caches.
   - Treat the error as an environment limitation, and continue working only on code, docs, or schemas.

---

## 3. Dependency behavior per crate

Some crates are especially sensitive to dependency bloat (for example, small FFI crates or engine‑facing runtime modules).

When working inside any crate under `crates/`, agents must:

1. Avoid adding “ergonomics” dependencies (e.g., new argument parsers, logging frameworks, or JSON libraries) unless explicitly requested.
2. Prefer:
   - Standard library.
   - Existing dependencies already listed in that crate’s `Cargo.toml`.
3. If a crate has its own `AI-AGENT-RULES.md` file:
   - That file takes precedence for that crate.
   - Follow both this contract and the crate‑specific rules.

---

## 4. Registry and index rules

The repository may include `.cargo/config.toml` to configure registries and index behavior.

Agents must obey the following:

1. Do not modify `.cargo/config.toml` without an explicit “Cargo config update” task.
2. Do not add or change registry URLs (for example, to point to a new mirror or a personal index).
3. Do not assume you can use alternative registries; always target whatever is already configured.
4. Do not attempt to “fix” index errors by:
   - Editing registry settings.
   - Resetting or deleting `~/.cargo/registry/index` in shared CI.

If the index is too large or encounters disk errors, that is an environment or infrastructure issue, not something agents should repair by changing this repository.

---

## 5. Patterns to prefer

When you need to use a dependency:

1. First, look at the top‑level `Cargo.toml` and any workspace‑member `Cargo.toml` to see which crates are already available.
2. Prefer reusing those crates rather than adding new ones. For example:
   - If serde is already present, prefer `serde_json` or typed structs over introducing new JSON parsing crates.
   - If there is already a CLI or logging crate in use, reuse those patterns rather than adding alternatives.
3. Always keep Rust numerics and safety kernels (especially BCI‑related code) aligned with the existing crate architecture for the repository.
4. If you need functionality that appears to be missing:
   - Try to implement it with std or existing crates.
   - If that is not feasible, write a short design note in a TODO or design doc instead of changing dependencies.

---

## 6. How to handle dependency‑related errors

When Cargo emits errors about:

- Missing crates due to network issues.
- Registry update failures.
- “No space left on device” while updating the crates.io index.

Agents must:

1. Treat these as **non‑actionable** at the code level.
2. Continue with:
   - Schema design.
   - Rust / Lua / C++ code changes that only rely on existing dependencies.
   - Test planning, doc writing, and offline analysis.
3. Optionally, add a small note to `docs/CI-SANDBOX-CONSTRAINTS.md` summarizing the observed error so humans can adjust CI or disk allocations later.

Agents should never attempt to “fix” these by altering dependencies.

---

## 7. Human override

A human maintainer can override this contract by:

- Creating an issue or task explicitly titled “Dependency update” or similar.
- Providing concrete instructions specifying which dependencies can be changed.

When such an override exists, agents may:

- Modify `Cargo.toml` according to the override.
- Run or suggest online Cargo commands necessary for that specific update.

Absent that explicit instruction, this contract remains binding.

---
