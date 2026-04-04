---
name: "🔄 Contract Change Proposal"
about: "Propose structural or logical changes to contract cards, prismMeta, or agentProfile schemas."
title: "[Contract Proposal] <contract-type> <change-summary>"
labels: ["contract", "proposal", "review"]
---

## Contract Target
<!-- Select applicable -->
- [ ] `policyEnvelope.v1.json`
- [ ] `regionContractCard.v1.json`
- [ ] `seedContractCard.v1.json`
- [ ] `prismMeta.v1.json`
- [ ] `agentProfile.v1.json`
- [ ] Other: 

## Proposed Change
<!-- Clearly describe the added/removed/modified fields, constraints, or logic. -->
- **Current structure:** 
- **Proposed structure:** 
```json
<!-- Diff or full snippet -->
```

## Backward Compatibility
- [ ] Non-breaking (additive only)
- [ ] Deprecation window required
- [ ] Breaking (requires migration plan)
- **Migration strategy:** 

## Validation & Testing Steps
- [ ] Passes `hpc-validate-schema.py --strict`
- [ ] CI `schema-validate.yml` green
- [ ] AI authoring validator accepts new shape
- [ ] Spine index generator handles updated schema
- **Test payloads/artifacts:** 

## Impact on Constellation
<!-- How this change affects registries, telemetry, AI pipelines, or cross-repo dependencies. -->
- Affected consumers: 
- Required registry updates: 
- AI-chat prompt adjustments: 

## References
<!-- Links to related research notes, open questions, or implementation branches. -->
