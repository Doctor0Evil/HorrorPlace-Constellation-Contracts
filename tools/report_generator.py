#!/usr/bin/env python3
"""report_generator.py — Aggregate scan findings into a Markdown compliance report.

Supports two modes:
  1. Single repo:   python report_generator.py /path/to/repo --output report.md
  2. Multi-repo:    python report_generator.py --config repos-manifest.json --output report.md

The report includes executive summary, per-repo findings, patch suggestions,
manual review manifest, invariant coverage matrix, and audit metadata.
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from dataclasses import asdict
from datetime import datetime, timezone
from pathlib import Path

from invariants_spec import CANONICAL_IDENTIFIERS, classify
import schema_drift_detector
import charter_compliance
import ndjson_lint

__version__ = "1.0.0"


def _traffic_light(results: list[dict]) -> str:
    """Return emoji traffic light based on worst severity across all repos."""
    has_charter = any(
        any(f["severity"] == "charter" for f in r.get("charter", []))
        for r in results
    )
    has_critical = any(
        any(f["severity"] == "critical" for f in findings)
        for r in results
        for findings in r.values()
        if isinstance(findings, list)
    )
    if has_charter:
        return "\U0001f534"  # Red circle
    if has_critical:
        return "\U0001f7e1"  # Yellow circle
    return "\U0001f7e2"  # Green circle


def _scan_repo(repo_path: str) -> dict:
    """Run all three scanners on a single repo and return aggregated findings."""
    drift = schema_drift_detector.scan(repo_path)
    charter = charter_compliance.scan(repo_path)
    ndjson = ndjson_lint.scan(repo_path)
    return {
        "drift": [asdict(f) for f in drift.findings],
        "charter": [asdict(f) for f in charter.findings],
        "ndjson": [asdict(f) for f in ndjson.findings],
    }


def _generate_patch_suggestions(findings: list[dict]) -> list[str]:
    """Generate unified diff patch suggestions for critical findings."""
    patches = []
    for f in findings:
        if f.get("suggestion"):
            patch = (
                f"--- a/{f['file']}\n"
                f"+++ b/{f['file']}\n"
                f"@@ -{f['line']},1 +{f['line']},1 @@\n"
                f"-  # {f['message']}\n"
                f"+  # FIXED: {f['suggestion']}\n"
            )
            patches.append(patch)
    return patches


def _coverage_matrix(results: dict[str, dict]) -> str:
    """Build Markdown table showing which canonical IDs are referenced per repo."""
    ids = sorted(CANONICAL_IDENTIFIERS)
    repos = sorted(results.keys())
    header = "| Identifier | Category | " + " | ".join(repos) + " |"
    sep = "|---|---| " + " | ".join(["---"] * len(repos)) + " |"
    rows = [header, sep]

    for ident in ids:
        cat = classify(ident) or "unknown"
        cells = []
        for repo in repos:
            # Check if any drift finding mentions this identifier
            drift_findings = results[repo].get("drift", [])
            mentioned = any(ident in f.get("message", "") for f in drift_findings)
            cells.append("\u2705" if not mentioned else "\u26a0\ufe0f")
        rows.append(f"| {ident} | {cat} | " + " | ".join(cells) + " |")

    return "\n".join(rows)


def generate_report(results: dict[str, dict], output_path: str | None = None) -> str:
    """Build the full Markdown compliance report."""
    now = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M UTC")
    light = _traffic_light(list(results.values()))

    total_findings = sum(
        len(f) for r in results.values() for f in r.values() if isinstance(f, list)
    )
    critical_count = sum(
        1 for r in results.values() for findings in r.values()
        if isinstance(findings, list)
        for f in findings if f.get("severity") in ("critical", "charter")
    )
    review_count = sum(
        1 for r in results.values() for findings in r.values()
        if isinstance(findings, list)
        for f in findings if f.get("requires_review")
    )
    clean_count = sum(
        1 for r in results.values()
        if all(len(f) == 0 for f in r.values() if isinstance(f, list))
    )

    lines: list[str] = []

    # ── Header ───────────────────────────────────────────────────
    lines.append(f"# {light} Horror.Place Schema Drift Audit Report")
    lines.append(f"**Date:** {now}")
    lines.append("")

    # ── Executive summary ────────────────────────────────────────
    lines.append("## Executive Summary")
    lines.append("")
    lines.append("| Metric | Value |")
    lines.append("|--------|-------|")
    lines.append(f"| Repos scanned | {len(results)} |")
    lines.append(f"| Clean repos | {clean_count} |")
    lines.append(f"| Total findings | {total_findings} |")
    lines.append(f"| Critical / Charter | {critical_count} |")
    lines.append(f"| Requires review | {review_count} |")
    lines.append("")

    # ── Per-repo sections ────────────────────────────────────────
    for repo_name, repo_findings in sorted(results.items()):
        lines.append(f"## {repo_name}")
        lines.append("")

        for scanner, findings in sorted(repo_findings.items()):
            if not isinstance(findings, list):
                continue
            if not findings:
                lines.append(f"### {scanner.title()}: \u2705 Clean")
                lines.append("")
                continue

            lines.append(f"### {scanner.title()} ({len(findings)} findings)")
            lines.append("")
            lines.append("| Severity | File | Line | Rule | Message |")
            lines.append("|----------|------|------|------|---------|")
            for f in findings:
                sev = f["severity"].upper()
                lines.append(
                    f"| {sev} | `{f['file']}` | {f['line']} "
                    f"| {f['rule']} | {f['message']} |"
                )
            lines.append("")

        # ── Patch suggestions ────────────────────────────────────
        all_drift = repo_findings.get("drift", [])
        patches = _generate_patch_suggestions(all_drift)
        if patches:
            lines.append("### Patch Suggestions")
            lines.append("")
            for patch in patches:
                lines.append("```diff")
                lines.append(patch)
                lines.append("```")
                lines.append("")

    # ── Manual review manifest ───────────────────────────────────
    review_files = []
    for repo_name, repo_findings in sorted(results.items()):
        for findings in repo_findings.values():
            if not isinstance(findings, list):
                continue
            for f in findings:
                if f.get("requires_review"):
                    review_files.append((repo_name, f["file"], f["rule"], f["message"]))

    if review_files:
        lines.append("## Manual Review Manifest")
        lines.append("")
        lines.append("The following files **MUST** have human review before merge:")
        lines.append("")
        lines.append("| Repo | File | Rule | Reason |")
        lines.append("|------|------|------|--------|")
        for repo, file, rule, msg in review_files:
            lines.append(f"| {repo} | `{file}` | {rule} | {msg} |")
        lines.append("")

    # ── Coverage matrix ──────────────────────────────────────────
    lines.append("## Invariant Coverage Matrix")
    lines.append("")
    lines.append(_coverage_matrix(results))
    lines.append("")

    # ── Audit metadata footer ────────────────────────────────────
    lines.append("---")
    lines.append("")
    lines.append("## Audit Metadata")
    lines.append("")
    lines.append(f"- **Scanner version:** {__version__}")
    lines.append(f"- **Modules:** schema_drift_detector {schema_drift_detector.__version__}, "
                 f"charter_compliance {charter_compliance.__version__}, "
                 f"ndjson_lint {ndjson_lint.__version__}")
    lines.append(f"- **Canonical spec:** invariants_spec (16 identifiers)")
    lines.append(f"- **Auto-merge policy:** NEVER for Charter-flagged files; "
                 f"allowed for info-only findings after 24h hold")
    lines.append(f"- **Generated:** {now}")
    lines.append("")

    report = "\n".join(lines)

    if output_path:
        Path(output_path).write_text(report, encoding="utf-8")

    return report


def main() -> None:
    parser = argparse.ArgumentParser(description="Schema Drift Report Generator")
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("repo", nargs="?", help="Path to a single repository")
    group.add_argument("--config", help="Path to repos-manifest.json")
    parser.add_argument("--output", "-o", help="Output file path", default="report.md")
    args = parser.parse_args()

    results: dict[str, dict] = {}

    if args.config:
        manifest = json.loads(Path(args.config).read_text(encoding="utf-8"))
        for entry in manifest["repos"]:
            name = entry["name"]
            repo_path = entry.get("local_path", name)
            if Path(repo_path).is_dir():
                print(f"Scanning {name} at {repo_path}...")
                results[name] = _scan_repo(repo_path)
            else:
                print(f"Skipping {name}: path {repo_path} not found.")
    else:
        repo_path = args.repo
        name = Path(repo_path).name
        print(f"Scanning {name}...")
        results[name] = _scan_repo(repo_path)

    report = generate_report(results, args.output)
    print(f"\nReport written to {args.output}")

    # Determine exit code
    has_charter = any(
        any(f.get("severity") == "charter" for f in findings)
        for r in results.values()
        for findings in r.values()
        if isinstance(findings, list)
    )
    has_critical = any(
        any(f.get("severity") == "critical" for f in findings)
        for r in results.values()
        for findings in r.values()
        if isinstance(findings, list)
    )
    if has_charter:
        sys.exit(3)
    elif has_critical:
        sys.exit(2)
    elif any(
        len(f) > 0 for r in results.values()
        for f in r.values() if isinstance(f, list)
    ):
        sys.exit(1)
    else:
        sys.exit(0)


if __name__ == "__main__":
    main()
