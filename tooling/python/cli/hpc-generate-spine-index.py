#!/usr/bin/env python3
"""
CLI: Generate schema-spine-index.json from canonical schemas.
Usage: python hpc-generate-spine-index.py --root schemas/ --output index.json
"""

import argparse
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from schema_spine import SpineIndexBuilder


def main() -> int:
    parser = argparse.ArgumentParser(description="Generate constellation schema spine index")
    parser.add_argument("--root", type=str, default="schemas/", help="Schema directory root")
    parser.add_argument("--output", type=str, default="schema-spine-index.json", help="Output file path")
    parser.add_argument("--validate", action="store_true", help="Validate output after generation")
    args = parser.parse_args()

    try:
        builder = SpineIndexBuilder(args.root)
        index = builder.scan()
        builder.save(args.output)

        if args.validate:
            # Basic structural check
            required_keys = {"indexVersion", "generatedAt", "schemas", "invariants", "metrics"}
            if missing := required_keys - index.keys():
                print(f"[FAIL] Generated index missing keys: {missing}", file=sys.stderr)
                return 1
            print("[OK] Spine index validated structurally.")
        return 0

    except Exception as e:
        print(f"[FATAL] Index generation failed: {e}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    sys.exit(main())
