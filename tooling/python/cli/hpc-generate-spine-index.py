#!/usr/bin/env python3
"""
hpc-generate-spine-index.py

Single source-of-truth generator for schema-spine-index-v1.json.

Responsibilities:
- Invoke schemaspine.spineindexbuilder to rebuild the schema spine index
  from the current schemas and invariants/metrics spines.
- Write schema-spine-index-v1.json under schemas/core/.
- Optionally support a --check mode that compares the regenerated index
  with the committed one and exits non-zero if there is drift.
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import Optional

from schemaspine import spineindexbuilder


REPO_ROOT_SENTINELS = {".git", ".github"}
CORE_SCHEMAS_DIR = Path("schemas") / "core"
SPINE_OUTPUT_NAME = "schema-spine-index-v1.json"


def _find_repo_root(start: Path) -> Path:
    current = start
    while current != current.parent:
        if any((current / s).exists() for s in REPO_ROOT_SENTINELS):
            return current
        current = current.parent
    return start


def _load_json(path: Path) -> Optional[object]:
    if not path.is_file():
        return None
    with path.open("r", encoding="utf-8") as f:
        return json.load(f)


def _parse_args(argv: Optional[list[str]] = None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Regenerate schemas/core/schema-spine-index-v1.json from current schemas."
    )
    parser.add_argument(
        "--check",
        action="store_true",
        help="Do not write; regenerate in-memory and exit non-zero if it differs from the committed index.",
    )
    parser.add_argument(
        "--json",
        action="store_true",
        help="Emit a small JSON status object to stdout for AI/CI consumption.",
    )
    return parser.parse_args(argv)


def main(argv: Optional[list[str]] = None) -> int:
    args = _parse_args(argv)

    repo_root = _find_repo_root(Path.cwd())
    existing_path = repo_root / CORE_SCHEMAS_DIR / SPINE_OUTPUT_NAME
    existing_index = _load_json(existing_path)

    if args.check:
        # Regenerate in memory but do not write.
        new_index = spineindexbuilder.build_index(repo_root=repo_root)

        # Convert dataclasses to plain dicts for comparison.
        def _convert(obj):
            if hasattr(obj, "__dataclass_fields__"):
                from dataclasses import asdict
                return {k: _convert(v) for k, v in asdict(obj).items()}
            if isinstance(obj, list):
                return [_convert(x) for x in obj]
            if isinstance(obj, dict):
                return {k: _convert(v) for k, v in obj.items()}
            return obj

        new_index_dict = _convert(new_index)

        drift = existing_index is None or existing_index != new_index_dict

        if args.json:
            status = {
                "mode": "check",
                "drift": drift,
                "existingPath": str(existing_path),
            }
            sys.stdout.write(json.dumps(status, ensure_ascii=False) + "\n")
        else:
            if drift:
                sys.stdout.write(
                    "schema-spine-index-v1.json is out of date. Run hpc-generate-spine-index.py to regenerate.\n"
                )
            else:
                sys.stdout.write("schema-spine-index-v1.json is up to date.\n")

        return 1 if drift else 0

    # Normal mode: regenerate and write.
    try:
        output_path = spineindexbuilder.build_and_write_index(repo_root=repo_root)
    except Exception as exc:
        if args.json:
            status = {
                "mode": "write",
                "success": False,
                "error": str(exc),
            }
            sys.stdout.write(json.dumps(status, ensure_ascii=False) + "\n")
        else:
            sys.stderr.write(f"Error generating spine index: {exc}\n")
        return 1

    if args.json:
        status = {
            "mode": "write",
            "success": True,
            "outputPath": str(output_path),
        }
        sys.stdout.write(json.dumps(status, ensure_ascii=False) + "\n")
    else:
        sys.stdout.write(f"Generated schema spine index at {output_path}\n")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
