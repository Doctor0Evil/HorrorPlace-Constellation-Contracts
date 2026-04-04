---
name: "📦 Registry Entry Request"
about: "Request addition, modification, or deprecation of an NDJSON registry entry (region, event, style, persona)."
title: "[Registry Request] <type> <proposed-id>"
labels: ["registry", "ndjson", "governance", "request"]
---

## Registry Type
<!-- Select one -->
- [ ] `registry-regions.v1.json`
- [ ] `registry-events.v1.json`
- [ ] `registry-styles.v1.json`
- [ ] `registry-personas.v1.json`

## Proposed Entry Data
<!-- Provide the NDJSON line or JSON object. Must match schema conventions. -->
```json
{
  "id": "",
  "schemaref": "",
  "deadledgerref": "",
  "artifactid": "",
  "createdAt": "",
  "status": ""
}
```

## Validation Checklist
- [ ] `id` follows naming convention (e.g., `REG-<CODE>-0001`)
- [ ] `schemaref` points to a canonical `$id` in `schemas/`
- [ ] `deadledgerref` contains a valid cryptographic or opaque proof identifier
- [ ] `artifactid` uses content-addressable or implementation-agnostic reference
- [ ] No raw content, URLs, or engine-specific paths embedded

## Rationale / Usage Context
<!-- Why this entry is needed, which constellation repo(s) will consume it, and what systems depend on it. -->
- Consuming repos: 
- Expected telemetry/metric bindings: 
- AI authoring impact: 

## Attachments
<!-- Link to validation reports, spine index queries, or draft contract cards. -->
