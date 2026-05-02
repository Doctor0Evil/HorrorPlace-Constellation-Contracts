-- Seed file for bci_phase1_default.db
-- Run after bci_phase1_schema.sql to install a tiny, empty lab database.

-- Example tileset row (safe corridor) for smoke tests.
INSERT INTO bci_tileset (
    tileset_id, name, region_class,
    cic, aos, mdi, det_baseline, lsg, spr, shci
) VALUES (
    1,
    'corridor_safe_test',
    'safe-lowDET-cooldown',
    0.5, 0.4, 0.3,
    2.0,
    0.2, 0.3, 0.4
);

-- Example geometry profile pointing at the test tileset.
INSERT INTO bci_geometry_profile (
    profile_id,
    profile_key,
    tileset_id,
    tier,
    status,
    safety_profile_id,
    notes
) VALUES (
    1,
    'corridor_safe_test_profile',
    1,
    'lab',
    'experimental',
    'bci-safety-profile-v1.monster-mode-standard',
    'Smoke-test profile for Phase 1 lab runs.'
);
