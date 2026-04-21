#!/usr/bin/env python3
"""
hp-authoring-plan: CLI for validating and summarizing AI authoring session plans.
Part of HorrorPlace-Constellation-Contracts tooling.

Usage:
  hp-authoring-plan validate path/to/plan.json
  hp-authoring-plan summarize path/to/plan.json
  hp-authoring-plan check-artifact plan.json objectKind targetRepo schemaRef
"""

import argparse
import json
import sys
from pathlib import Path
from jsonschema import validate, ValidationError, Draft7Validator

# Canonical lists (loaded from spine schemas)
CANONICAL_INVARIANTS = {
    "CIC", "MDI", "AOS", "RRM", "FCF", "SPR", "RWF", "DET", "HVF", "HVF_mag", "LSG", "SHCI"
}
CANONICAL_METRICS = {"UEC", "EMD", "STCI", "CDL", "ARR"}
VALID_TIERS = {"Tier1Public", "Tier2Internal", "Tier3Vault", "Tier4Underground"}
VALID_REPOS = {
    "Horror.Place", "Horror.Place-Orchestrator", "HorrorPlace-Constellation-Contracts",
    "HorrorPlace-Codebase-of-Death", "HorrorPlace-Black-Archivum", "HorrorPlace-Spectral-Foundry",
    "HorrorPlace-Atrocity-Seeds", "HorrorPlace-Obscura-Nexus", "HorrorPlace-Liminal-Continuum",
    "HorrorPlace-Process-Gods-Research", "HorrorPlace-Redacted-Chronicles",
    "HorrorPlace-Neural-Resonance-Lab", "HorrorPlace-Dead-Ledger-Network"
}
VALID_OBJECT_KINDS = {
    "dungeonNodeContract", "dungeonRunContract", "historySelectorPattern",
    "directorPersonaContract", "directorComfortPolicy", "moodContract",
    "eventContract", "regionContract", "styleContract", "aiChatHorrorProfile"
}

def load_schema(schema_path: str) -> dict:
    """Load a JSON Schema file."""
    with open(schema_path, "r", encoding="utf-8") as f:
        return json.load(f)

def validate_plan(plan: dict, schema: dict) -> list[str]:
    """Validate plan against ai-safe-authoring-contract schema + custom rules."""
    errors = []

    # Schema validation
    try:
        validate(instance=plan, schema=schema)
    except ValidationError as e:
        errors.append(f"Schema validation error: {e.message} at {'/'.join(str(p) for p in e.path)}")
        return errors  # Stop early if schema fails

    # Custom governance rules
    if plan.get("mode") not in ("MapDesignWizard", "DungeonFlowTuning", "ContractAuthor", "TelemetryReview"):
        errors.append(f"Unknown mode: {plan.get('mode')}")

    for i, artifact in enumerate(plan.get("artifacts", [])):
        # Repo check
        if artifact.get("targetRepo") not in VALID_REPOS:
            errors.append(f"artifacts[{i}]: invalid targetRepo '{artifact.get('targetRepo')}'")

        # Tier check
        if artifact.get("tier") not in VALID_TIERS:
            errors.append(f"artifacts[{i}]: invalid tier '{artifact.get('tier')}'")

        # Object kind check
        if artifact.get("objectKind") not in VALID_OBJECT_KINDS:
            errors.append(f"artifacts[{i}]: unknown objectKind '{artifact.get('objectKind')}'")

        # Invariant/metric subset check
        for inv in artifact.get("invariants", []):
            if inv not in CANONICAL_INVARIANTS:
                errors.append(f"artifacts[{i}]: unknown invariant '{inv}'")
        for met in artifact.get("metrics", []):
            if met not in CANONICAL_METRICS:
                errors.append(f"artifacts[{i}]: unknown metric '{met}'")

        # SchemaRef format check
        schemaref = artifact.get("schemaRef")
        if schemaref and not schemaref.endswith(".v1"):
            errors.append(f"artifacts[{i}]: schemaRef should end with version, e.g., '...v1': '{schemaref}'")

    return errors

def summarize_plan(plan: dict) -> str:
    """Generate human-readable summary of a validated plan."""
    lines = [
        f"Session Plan Summary",
        f"==================",
        f"Session ID: {plan.get('sessionId', 'N/A')}",
        f"Mode: {plan.get('mode', 'N/A')}",
        f"Agent Profile: {plan.get('agentProfileId', 'N/A')}",
        f"AI-Chat Profile: {plan.get('aiChatProfileId', 'N/A')}",
        f"",
        f"Artifacts ({len(plan.get('artifacts', []))} total):"
    ]
    for i, art in enumerate(plan.get("artifacts", []), 1):
        invs = ", ".join(art.get("invariants", [])[:5])
        if len(art.get("invariants", [])) > 5:
            invs += f" (+{len(art['invariants'])-5} more)"
        mets = ", ".join(art.get("metrics", []))
        lines.append(
            f"  {i}. {art.get('objectKind')} @ {art.get('targetRepo')}:{art.get('path')}\n"
            f"     Tier: {art.get('tier')}, Schema: {art.get('schemaRef')}\n"
            f"     Invariants: {invs or 'none'} | Metrics: {mets or 'none'}"
        )
    return "\n".join(lines)

def check_artifact(plan: dict, object_kind: str, target_repo: str, schema_ref: str) -> bool:
    """Check if a proposed artifact is permitted by the plan."""
    for art in plan.get("artifacts", []):
        if (art.get("objectKind") == object_kind and
            art.get("targetRepo") == target_repo and
            art.get("schemaRef") == schema_ref):
            return True
    return False

def main():
    parser = argparse.ArgumentParser(description="HorrorPlace Authoring Plan CLI")
    subparsers = parser.add_subparsers(dest="command", required=True)

    # validate subcommand
    validate_parser = subparsers.add_parser("validate", help="Validate a session plan")
    validate_parser.add_argument("plan_path", type=str, help="Path to plan JSON file")
    validate_parser.add_argument("--schema", type=str, default="schemas/ai-safe-authoring-contract.v1.json",
                                help="Path to ai-safe-authoring-contract schema")

    # summarize subcommand
    summarize_parser = subparsers.add_parser("summarize", help="Print human-readable plan summary")
    summarize_parser.add_argument("plan_path", type=str, help="Path to plan JSON file")

    # check-artifact subcommand
    check_parser = subparsers.add_parser("check-artifact", help="Check if artifact is permitted by plan")
    check_parser.add_argument("plan_path", type=str, help="Path to plan JSON file")
    check_parser.add_argument("--object-kind", required=True, help="objectKind to check")
    check_parser.add_argument("--target-repo", required=True, help="targetRepo to check")
    check_parser.add_argument("--schema-ref", required=True, help="schemaRef to check")

    args = parser.parse_args()

    # Load plan
    plan_path = Path(args.plan_path)
    if not plan_path.exists():
        print(f"ERROR: Plan file not found: {plan_path}", file=sys.stderr)
        sys.exit(1)

    with open(plan_path, "r", encoding="utf-8") as f:
        plan = json.load(f)

    if args.command == "validate":
        schema_path = Path(args.schema)
        if not schema_path.exists():
            print(f"ERROR: Schema file not found: {schema_path}", file=sys.stderr)
            sys.exit(1)
        schema = load_schema(schema_path)
        errors = validate_plan(plan, schema)
        if errors:
            print("PLAN VALIDATION FAILED:", file=sys.stderr)
            for err in errors:
                print(f"  - {err}", file=sys.stderr)
            sys.exit(1)
        else:
            print(f"OK: Plan {plan_path} is valid")
            sys.exit(0)

    elif args.command == "summarize":
        print(summarize_plan(plan))
        sys.exit(0)

    elif args.command == "check-artifact":
        if check_artifact(plan, args.object_kind, args.target_repo, args.schema_ref):
            print(f"OK: Artifact {args.object_kind} @ {args.target_repo} with schema {args.schema_ref} is permitted")
            sys.exit(0)
        else:
            print(f"ERROR: Artifact not permitted by plan", file=sys.stderr)
            sys.exit(1)

if __name__ == "__main__":
    main()
