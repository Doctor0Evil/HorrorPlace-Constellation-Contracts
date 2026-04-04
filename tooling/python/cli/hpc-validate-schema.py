#!/usr/bin/env python3
"""
CLI: Validate JSON files against HorrorPlace constellation schemas.
Usage: python hpc-validate-schema.py --mode file --schema <path> --file <path> [--report out.json]
"""

import argparse
import json
import sys
from pathlib import Path

try:
    import jsonschema
except ImportError:
    print("[ERROR] Missing dependency: pip install jsonschema", file=sys.stderr)
    sys.exit(1)

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from schema_spine import AIAuthoringValidator


def main() -> int:
    parser = argparse.ArgumentParser(description="Validate constellation JSON schemas and contracts")
    parser.add_argument("--mode", choices=["schema", "ai-authoring", "batch"], default="schema")
    parser.add_argument("--schema", type=str, help="Path to JSON Schema file")
    parser.add_argument("--file", type=str, help="Path to JSON instance file")
    parser.add_argument("--root", type=str, default="schemas/", help="Root directory for batch scanning")
    parser.add_argument("--report", type=str, help="Output report JSON path")
    parser.add_argument("--strict", action="store_true", help="Treat warnings as errors")
    args = parser.parse_args()

    validator = AIAuthoringValidator()
    report = {"valid": True, "errors": [], "warnings": []}

    try:
        if args.mode == "ai-authoring":
            if not args.file:
                print("[ERROR] --file required for ai-authoring mode", file=sys.stderr)
                return 1
            path = Path(args.file)
            is_valid = validator.validate(path)
            r = validator.get_report()
            report.update(r)

        elif args.mode == "schema" and args.file and args.schema:
            with open(args.schema, "r") as sf, open(args.file, "r") as inst:
                schema = json.load(sf)
                instance = json.load(inst)
            try:
                jsonschema.validate(instance=instance, schema=schema)
            except jsonschema.ValidationError as e:
                report["valid"] = False
                report["errors"].append(str(e.message))

        elif args.mode == "batch":
            # Simplified batch: scan and check parseability
            root = Path(args.root)
            for p in root.rglob("*.json"):
                try:
                    with open(p, "r") as f:
                        json.load(f)
                except Exception as e:
                    report["valid"] = False
                    report["errors"].append(f"{p}: {e}")
        else:
            print("[ERROR] Unsupported mode/args combination", file=sys.stderr)
            return 1

        if args.report:
            Path(args.report).parent.mkdir(parents=True, exist_ok=True)
            with open(args.report, "w") as f:
                json.dump(report, f, indent=2)

        if not report["valid"]:
            print("[FAIL] Validation failed. See report.", file=sys.stderr)
            return 1
        print("[OK] Validation passed.")
        return 0

    except Exception as e:
        print(f"[FATAL] {e}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    sys.exit(main())
