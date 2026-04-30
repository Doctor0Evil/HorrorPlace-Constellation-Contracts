# BCI Binding Promotion Criteria v1

This document explains how `bci-binding-promotion-criteria-v1.json` is used to govern lifecycle promotion of `HorrorMappingConfig.BCI` bindings from `lab` to `standard` to `mature`, and provides a filled-out default criteria artifact that AI-chat and tooling can copy.

## 1. Purpose and wiring

`bci-binding-promotion-criteria-v1.json` defines quantitative thresholds, evidence types, and CI gate requirements that must be met before a binding’s `lifecycle.tier` can move forward.

Each `HorrorMappingConfig.BCI` binding:

- Sets `lifecycle.promotionCriteriaRef` to the `id` of a criteria artifact that conforms to `schemas/bci/bci-binding-promotion-criteria-v1.json`.
- Uses that artifact as the canonical checklist for:
  - Lab → Standard promotion.
  - Standard → Mature promotion.
  - Ongoing maintenance and rollback rules.

The schema lives at:

- `schemas/bci/bci-binding-promotion-criteria-v1.json`

Bindings that target this criteria set should declare:

```json
"lifecycle": {
  "tier": "lab",
  "promotionCriteriaRef": "bci-binding-promotion-criteria-default-v1",
  "createdBy": "human-reviewer-or-tool-id",
  "createdAt": "2026-04-30T00:00:00Z"
}
```

## 2. Default criteria artifact (canonical template)

The JSON example below is a fully populated, default criteria artifact. It is intended as the canonical template for AI-chat and tooling.

- File path: `schemas/bci/bci-binding-promotion-criteria-default-v1.json`
- Schema: `schemas/bci/bci-binding-promotion-criteria-v1.json`

```json
{
  "id": "bci-binding-promotion-criteria-default-v1",
  "version": "1.0.0",
  "schemaref": "schemas/bci/bci-binding-promotion-criteria-v1.json",
  "appliesToSchemaVersion": "HorrorMappingConfig.BCI.v1",
  "descriptionLong": "Default promotion criteria for BCI mapping bindings. Designed for conservative, safety-first promotion from lab to standard to mature tiers under Dead-Ledger governance.",
  "tiers": {
    "lab": {
      "allowedSources": [
        "human-reviewed",
        "ai-assisted"
      ],
      "requiredCiGates": [
        "schema",
        "invariants"
      ],
      "maxIntensityTier": "research",
      "sessionUsageLimits": {
        "allowLiveSessions": false,
        "maxParticipantsPerCohort": 0,
        "requireDeadLedgerExperimentFlag": true
      }
    },
    "standard": {
      "promotionFromLab": {
        "minLiveSessions": 10,
        "minUniqueParticipants": 20,
        "minTotalExposureHours": 15.0,
        "maxAllowedIncidents": 0,
        "requiredCiGates": [
          "schema",
          "invariants",
          "numerical",
          "policy"
        ],
        "telemetryEvidence": {
          "minDetOverloadRate": 0.0,
          "maxOverloadEventsPerHour": 0.5,
          "minTargetBandHitRate": 0.6
        },
        "evidenceSources": [
          "bci-telemetry-summary-v1",
          "bcistate-proof"
        ]
      },
      "maintenance": {
        "maxRegressionWindowDays": 30,
        "rollbackOnIncidentCount": 1,
        "requirePeriodicReviewDays": 90
      }
    },
    "mature": {
      "promotionFromStandard": {
        "minLiveSessions": 50,
        "minUniqueParticipants": 100,
        "minTotalExposureHours": 100.0,
        "maxAllowedIncidents": 1,
        "requiredCiGates": [
          "schema",
          "invariants",
          "numerical",
          "policy",
          "governance"
        ],
        "telemetryEvidence": {
          "maxDetOverloadRate": 0.02,
          "maxOverloadEventsPerHour": 0.2,
          "minTargetBandHitRate": 0.75,
          "maxVarAcrossCohorts": 0.05
        },
        "evidenceSources": [
          "bci-telemetry-summary-v1",
          "bcistate-proof",
          "spectral-seed-attestation",
          "governance-review-record"
        ],
        "requireGovernanceApproval": true
      },
      "maintenance": {
        "maxRegressionWindowDays": 90,
        "rollbackOnIncidentCount": 1,
        "requirePeriodicReviewDays": 180,
        "lockForCriticalUse": false
      }
    }
  }
}
```

## 3. How AI-chat should use this template

When AI-chat generates a new `HorrorMappingConfig.BCI` binding:

1. Set `schemaVersion` to `HorrorMappingConfig.BCI.v1`.
2. Set `lifecycle.tier` to `"lab"`.
3. Set `lifecycle.promotionCriteriaRef` to `"bci-binding-promotion-criteria-default-v1"` unless a more specific criteria artifact is requested.
4. Do not attempt to modify or invent promotion thresholds; instead, reference an existing criteria artifact and let CI and governance apply those rules.

When humans or governance tools promote a binding:

- Update `lifecycle.tier`, `lifecycle.promotedFrom`, `lifecycle.promotedAt`, and append identifiers for telemetry and proof artifacts into `lifecycle.promotionEvidenceRef`.
- Keep `promotionCriteriaRef` stable to preserve the link to the criteria used at the time of promotion.
