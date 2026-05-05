-- File: db/schema_bci_pipeline_seed_rottingvisuals.sql

-- Stages
INSERT INTO bci_pipeline_stage (
    repo, stage_key, name, layer,
    input_type, output_type, primary_file, aux_files, note
) VALUES
('Rotting-Visuals-BCI', 'rvbci_request_in', 'Rotting-Visuals BCI Request Ingest',
 'ingest',
 'ai-bci-geometry-request-v1',
 'BciSummary+Invariants',
 'schemas/ai-bci-geometry-request-v1.json',
 'docs/ai-metadata.json',
 'Validates and normalizes BCI request into internal structs'),

('Rotting-Visuals-BCI', 'rvviz_geometry_compute', 'Rotting-Visuals Geometry and Audio Compute',
 'compute',
 'BciSummary+Invariants',
 'AiBciGeometryResponseV1',
 'src/monstermode/mod.rs',
 'src/geometry/monstermode_mappings.hpp',
 'Computes visual/audio params from BCI and invariants'),

('Rotting-Visuals-BCI', 'rvsql_log_geometry', 'Rotting-Visuals SQLite Geometry Log',
 'log',
 'AiBciGeometryResponseV1',
 'BindingRows+PaletteRows',
 'db/schema_rottingvisuals_monstermode.sql',
 'crates/rottingvisuals_log/src/lib.rs',
 'Persists frames, bindings, and palette swatches to SQLite'),

('Rotting-Visuals-BCI', 'rvsql_can_registry', 'Rotting-Visuals CAN Token Registry Index',
 'persistence',
 'CanTokenRegistryJson',
 'CanTokenRows',
 'db/schema_can_tokens.sql',
 'crates/rottingvisuals_log/src/can_registry_loader.rs',
 'Indexes CAN token registry JSON into can_token and can_token_registry');

-- Edges (use SELECT to get stage_ids in a real script)
-- Below assumes stage_ids 1..4 in order of insertion for brevity.

INSERT INTO bci_pipeline_edge (from_stage_id, to_stage_id, protocol, description) VALUES
(1, 2, 'in-process',
 'Request ingest passes BciSummary and Invariants into Rotting-Visuals compute functions'),
(2, 3, 'sqlite',
 'Geometry compute inserts bcirequestframe, bcibindinggeometry, and bcipalette rows'),
(4, 2, 'in-process',
 'CAN token registry constrains and documents CAN-exposed parameter ranges for compute');
