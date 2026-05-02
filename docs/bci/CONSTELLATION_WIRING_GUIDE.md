# VM-Constellation Wiring Guide for AI-Chat and Coding Agents

This document provides AI-Chat agents and coding assistants with the query patterns and reference tables needed to interpret advanced wiring logic across the HorrorPlace VM-Constellation.

## Purpose

The `constellation_wiring_v1` SQLite schema provides a machine-queryable registry of:
- All VM nodes and their tier/role assignments
- Chain-reaction families with invariant/metric weights
- Event contracts with cross-repository routing maps
- Intervention rules with validation metrics
- Generated files with dependency tracking
- 12-month investigation progress milestones

## Core Tables for AI-Agent Queries

### 1. Query VM Node Registry

```sql
SELECT vm_name, tier, roles, subscribed_chains
FROM constellation_vm_node
WHERE status = 'active'
ORDER BY tier, vm_name;
```

### 2. Query Chain Family Weights

```sql
SELECT chain_family_key, description, tier,
       json_extract(invariant_weights, '$.AOS') AS aos_weight,
       json_extract(metric_weights, '$.UEC') AS uec_weight
FROM v_chain_family_weights
WHERE status = 'live';
```

### 3. Query Intervention Rules by Type

```sql
SELECT rule_key, rule_name, intervention_type, tier, enabled,
       json_extract(trigger_conditions, '$.stressBand') AS stress_trigger
FROM v_intervention_rule_summary
WHERE intervention_type = 'HINT_SYSTEM'
ORDER BY rule_key;
```

### 4. Query File Generation Status by Repository

```sql
SELECT repository, tier, file_type,
       SUM(CASE WHEN status = 'generated' THEN 1 ELSE 0 END) AS generated_count,
       SUM(CASE WHEN status = 'pending' THEN 1 ELSE 0 END) AS pending_count
FROM v_file_manifest_by_repository
GROUP BY repository, tier
ORDER BY tier, repository;
```

### 5. Query 12-Month Investigation Progress

```sql
SELECT phase_id, total_milestones, total_completed_files,
       completion_percentage, validated_milestones
FROM v_phase_progress_summary
ORDER BY phase_id;
```

### 6. Query Schema Registry by Tier

```sql
SELECT schema_key, schema_version, repository, path, status
FROM constellation_schema_registry
WHERE tier = 1 AND status = 'active'
ORDER BY schema_key;
```

### 7. Query Transition Channels for Event Routing

```sql
SELECT tc.channel_name, src.vm_name AS source, tgt.vm_name AS target,
       tc.data_schema_id, tc.direction
FROM constellation_transition_channel tc
JOIN constellation_vm_node src ON tc.source_vm_node_id = src.vm_node_id
JOIN constellation_vm_node tgt ON tc.target_vm_node_id = tgt.vm_node_id
WHERE tc.enabled = 1
ORDER BY tc.priority DESC;
```

## AI-Chat Integration Patterns

### Pattern 1: File Generation Tracking

When an AI-Chat agent generates a new file, it should:
1. Insert a record into `constellation_file_manifest`
2. Update `constellation_progress_tracker` completed_files count
3. Log the change in `constellation_wiring_audit_log`

```sql
-- Example: Log new file generation
INSERT INTO constellation_file_manifest
    (file_path, file_name, repository, tier, file_type, schema_ref, status, generated_at, updated_at)
VALUES
    ('schemas/bci/new-schema-v1.json', 'new-schema-v1.json', 'HorrorPlace-Constellation-Contracts', 1, 'schema', 'new-schema-v1', 'generated', datetime('now'), datetime('now'));

-- Update progress tracker
UPDATE constellation_progress_tracker
SET completed_files = completed_files + 1,
    updated_at = datetime('now')
WHERE phase_id = 'phase-1' AND milestone_name = 'BCI Schema Spine Complete';

-- Audit log entry
INSERT INTO constellation_wiring_audit_log
    (change_type, entity_type, entity_id, new_value, changed_by, change_reason, changed_at)
VALUES
    ('CREATE', 'file_manifest', last_insert_rowid(), '{"status": "generated"}', 'AI-Chat-Agent', 'New schema file generated', datetime('now'));
```

### Pattern 2: Cross-Repository Dependency Check

Before generating a file that depends on another schema:

```sql
-- Check if dependency exists and is active
SELECT schema_key, schema_version, repository, path, status
FROM constellation_schema_registry
WHERE schema_key = 'bci-mapping-request-v1' AND status = 'active';
```

### Pattern 3: Validate VM Routing Before Event Contract Creation

```sql
-- Verify target VM nodes exist and are active
SELECT vm_node_id, vm_name, tier, roles
FROM constellation_vm_node
WHERE vm_name IN ('HorrorPlace-Codebase-of-Death', 'HorrorPlace-Spectral-Foundry')
  AND status = 'active';
```

## Wiring Doctrine for AI-Agents

1. **Tier-1 Schemas Are Immutable**: AI agents must never propose edits to Tier-1 schemas. Changes require formal schema versioning.

2. **File Manifest Must Be Updated**: Every generated file must have a corresponding `constellation_file_manifest` entry with checksum.

3. **Progress Tracker Is Authoritative**: The `v_phase_progress_summary` view is the single source of truth for 12-month investigation status.

4. **Audit All Changes**: Every CREATE, UPDATE, DELETE on wiring tables must be logged in `constellation_wiring_audit_log`.

5. **Validate Before Generation**: AI agents must query dependency tables before proposing new files to ensure schema compatibility.

## Quick Reference: Repository Tier Assignments

| Repository | Tier | Primary Role |
| :--- | :--- | :--- |
| HorrorPlace-Constellation-Contracts | 1 | Public schemas, canonical contracts |
| HorrorPlace-Codebase-of-Death | 2 | Private implementations, mature content |
| HorrorPlace-Neural-Resonance-Lab | 3 | Lab tooling, BCI experiments |
| HorrorPlace-Black-Archivum | 2 | Historical archives, AOS data |
| HorrorPlace-Spectral-Foundry | 2 | Persona contracts, agent artifacts |
| HorrorPlace-Atrocity-Seeds | 2 | Event seeds, region data |
| HorrorPlace-Obscura-Nexus | 2 | Style contracts, visual/audio profiles |
| HorrorPlace-Liminal-Continuum | 2 | Agent routing, ledger entries |
| HorrorPlace-Dead-Ledger-Network | 2 | ZKP proofs, age-gating circuits |
| HorrorPlace-Redacted-Chronicles | 3 | BCI telemetry, baseline tests |
| HorrorPlace-Process-Gods-Research | 3 | Process god personas, experiments |

## Support Queries for AI-Chat Sessions

### "Which files are pending generation for Phase 1?"

```sql
SELECT file_path, file_name, repository, file_type
FROM constellation_file_manifest
WHERE status = 'pending'
  AND repository IN (
    SELECT repository FROM constellation_progress_tracker
    WHERE phase_id = 'phase-1'
  );
```

### "What is the completion percentage for Phase 2?"

```sql
SELECT completion_percentage, total_completed_files, total_target_files
FROM v_phase_progress_summary
WHERE phase_id = 'phase-2';
```

### "List all active intervention rules for HINT_SYSTEM"

```sql
SELECT rule_key, rule_name, json_extract(trigger_conditions, '$.metricConditions') AS triggers
FROM constellation_intervention_rule
WHERE intervention_type = 'HINT_SYSTEM' AND enabled = 1;
```

---

This guide enables AI-Chat agents to query the constellation wiring database for context-aware file generation, progress tracking, and cross-repository dependency validation.
