#!/usr/bin/env python3
"""
CLI: Lint NDJSON registry files for governance and format compliance.
Usage: python hpc-lint-registry.py --mode file --file <path> [--report out.json]
"""

import argparse
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from schema_spine import RegistryLinter


def main() -> int:
    parser = argparse.ArgumentParser(description="Lint NDJSON registry files")
    parser.add_argument("--mode", choices=["file", "batch"], default="file")
    parser.add_argument("--file", type=str, help="Path to registry file")
    parser.add_argument("--root", type=str, default="registry/", help="Root for batch scanning")
    parser.add_argument("--pattern", type=str, default="*.example", help="Glob pattern for batch")
    parser.add_argument("--report", type=str, help="Output report JSON")
    parser.add_argument("--strict", action="store_true", help="Fail on warnings")
    args = parser.parse_args()

    linter = RegistryLinter()
    report = {"valid": True, "files_checked": 0, "errors": [], "warnings": []}

    try:
        if args.mode == "file" and args.file:
            path = Path(args.file)
            if path.is_file():
                linter.lint_file(path)
                report["files_checked"] = 1
        elif args.mode == "batch":
            root = Path(args.root)
            for p in root.rglob(args.pattern):
                linter.lint_file(p)
                report["files_checked"] += 1

        r = linter.get_report()
        report.update(r)

        if args.report:
            Path(args.report).parent.mkdir(parents=True, exist_ok=True)
            with open(args.report, "w") as f:
                json.dump(report, f, indent=2)

        if not report["valid"]:
            print("[FAIL] Registry linting failed.", file=sys.stderr)
            return 1
        if args.strict and report.get("warnings"):
            print("[FAIL] Strict mode enabled. Warnings treated as failures.", file=sys.stderr)
            return 1
        print(f"[OK] Linted {report['files_checked']} files successfully.")
        return 0

    except Exception as e:
        print(f"[FATAL] {e}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    sys.exit(main())
