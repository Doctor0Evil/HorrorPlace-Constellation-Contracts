---
name: "🔍 Schema Drift Report"
about: "Report or investigate divergence between downstream schemas and the canonical schema spine."
title: "[Schema Drift] <affected-schema-name> v<version>"
labels: ["schema", "drift", "validation", "triage"]
---

## Affected Schema(s)
<!-- e.g., invariants-spine.v1.json, policyEnvelope.v1.json -->
- `$id`: 
- Current version: 
- Drift detected in repository/path: 

## Drift Description
<!-- Describe the structural, semantic, or range mismatch. -->
- **Expected behavior (canonical spine):** 
- **Observed behavior (downstream):** 
- **Validation error/log output:** 
  ```text
  <!-- Paste CI or local tool output here -->
  ```

## Impact Scope
- [ ] Breaking change for existing contract cards
- [ ] Registry format incompatibility
- [ ] Telemetry/metric range violation
- [ ] AI authoring pipeline failure
- [ ] Other: 

## Proposed Resolution
<!-- e.g., align downstream to spine, deprecate v1, patch spine with v1.1, or update tooling. -->
- Suggested action: 
- Migration path (if breaking): 
- Required tooling updates: 

## Additional Context
<!-- Attach links to related PRs, CI runs, or affected constellation repos. -->
