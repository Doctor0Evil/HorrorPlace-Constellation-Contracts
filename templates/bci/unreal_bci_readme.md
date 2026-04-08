# Unreal BCI Subsystem Template Usage

This directory contains Unreal templates for integrating Horror$Place BCI features:

- `unreal_bci_subsystem_template.h`
- `unreal_bci_subsystem_template.cpp`

The templates implement:

- `UEEGFeatureSubsystem`: An engine subsystem that:
  - Reads EEGFeatureContract-compatible JSON (live feature server or replay file).
  - Maintains a latest `FEEGFeatures` snapshot.
  - Avoids any direct hardware access.

- `UBCIHorrorDirectorSubsystem`: An engine subsystem that:
  - Reads `FEEGFeatures` from `UEEGFeatureSubsystem`.
  - Applies a small mapping from stress and CIC to tension.
  - Computes an enemy spawn multiplier under BCI intensity policy caps.

To use these templates:

1. Create a `UBCIConnectionConfig` data asset and set:
   - Data source (live or replay).
   - Host and port for the feature server, or the replay NDJSON file path.
   - Policy caps for tension and spawn multiplier.

2. Register both subsystems in your module and ensure:
   - `UEEGFeatureSubsystem` is configured with the connection asset.
   - `UBCIHorrorDirectorSubsystem` is configured with the same asset.

3. In game code or Blueprints:
   - Read `Tension` and `EnemySpawnMultiplier` via the HorrorDirector subsystem.
   - Drive post-process volumes, AI parameters, and encounter pacing using these values.
   - Do not introduce direct hardware or BrainFlow references into gameplay classes.

These templates mirror the Unity `EEGFeatureService` and `HorrorDirector` patterns and must remain aligned with `BCI-Dev-Guide.md` and the BCI schemas in Horror$Place.
