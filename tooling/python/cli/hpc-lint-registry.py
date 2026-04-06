#!/usr/bin/env python3
"""
hpc-lint-registry.py

Thin CLI wrapper to orchestrate linting of all NDJSON registry files.

Responsibilities:
- Discover registry/**/*.ndjson files by default (or take explicit paths).
- Delegate structural and referential checks to schemaspine.registrylinter.
- Aggregate diagnostics and maintain per-file status.
- Exit non-zero on any error so CI/local hooks can fail fast.
- Optionally emit machine-readable JSONL diagnostics for AI/CI tools.
"""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import asdict
from pathlib import Path
from typing import List, Optional

from schemaspine import registrylinter


REPO_ROOT_SENTINELS = {".git", ".github"}


def _find_repo_root(start: Path) -> Path:
    current = start
    while current != current.parent:
        if any((current / s).exists() for s in REPO_ROOT_SENTINELS):
            return current
        current = current.parent
    return start


def _discover_registry_paths(repo_root: Path) -> List[Path]:
    """
    Discover NDJSON registry files under a conventional layout.

    Default patterns:
        registry/*.ndjson
        registry/**/*.ndjson
    """
    paths: List[Path] = []
    for pattern in ("registry/*.ndjson", "registry/**/*.ndjson"):
        paths.extend(sorted((repo_root / pattern).glob()))
    # De-duplicate
    seen = set()
    unique: List[Path] = []
    for p in paths:
        if p not in seen and p.is_file():
            seen.add(p)
            unique.append(p)
    return unique


def _parse_args(argv: Optional[List[str]] = None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Lint NDJSON registries using schemaspine.registrylinter."
    )
    parser.add_argument(
        "paths",
        nargs="*",
        help="Optional registry .ndjson paths. If omitted, discovers registry/**/*.ndjson.",
    )
    parser.add_argument(
        "--json",
        action="store_true",
        help="Emit diagnostics as JSON objects (one per line) for AI/CI consumption.",
    )
    parser.add_argument(
        "--summary",
        action="store_true",
        help="Print a brief human-readable summary after linting.",
    )
    return parser.parse_args(argv)


def _print_diagnostics_json(result: registrylinter.LintResult) -> None:
    for d in result.diagnostics:
        obj = asdict(d)
        compact = {k: v for k, v in obj.items() if v is not None}
        sys.stdout.write(json.dumps(compact, ensure_ascii=False) + "\n")


def _print_diagnostics_text(result: registrylinter.LintResult) -> None:
    for d in result.diagnostics:
        location = f"{d.registry_file}:{d.line_number}"
        if d.json_pointer:
            location += f"{d.json_pointer}"
        sys.stdout.write(
            f"[{d.severity.upper()}] {d.code} at {location}: {d.message}\n"
        )


def _print_summary(result: registrylinter.LintResult, registry_paths: List[Path]) -> None:
    total_files = len(registry_paths)
    total_diags = len(result.diagnostics)
    error_count = sum(1 for d in result.diagnostics if d.severity == "error")
    warning_count = sum(1 for d in result.diagnostics if d.severity == "warning")

    sys.stdout.write(
        f"Registry lint summary: {total_files} file(s) checked, "
        f"{error_count} error(s), {warning_count} warning(s), "
        f"{total_diags} diagnostic(s) total.\n"
    )


def main(argv: Optional[List[str]] = None) -> int:
    args = _parse_args(argv)

    repo_root = _find_repo_root(Path.cwd())

    if args.paths:
        registry_paths = [Path(p) for p in args.paths if Path(p).is_file()]
    else:
        registry_paths = _discover_registry_paths(repo_root)

    if not registry_paths:
        sys.stderr.write("No registry .ndjson files found to lint.\n")
        return 0

    result = registrylinter.lint_registries(
        repo_root=repo_root,
        registry_paths=registry_paths,
    )

    if args.json:
        _print_diagnostics_json(result)
    else:
        _print_diagnostics_text(result)

    if args.summary:
        _print_summary(result, registry_paths)

    has_errors = any(d.severity == "error" for d in result.diagnostics)
    return 1 if has_errors else 0


if __name__ == "__main__":
    raise SystemExit(main())
