-- Insert new palette group: Spectral-Pus

INSERT OR IGNORE INTO palettegroups
    (groupname, description, swatches_json, affective_valence, affective_arousal, semantic_tags_json)
VALUES
    (
      'Spectral-Pus',
      'Sickly spectral pus tones in green-yellow rot range, used for infected glow and spectral leakage.',
      '["#1B2A13","#3A4F1A","#667A22","#9FAE3B","#E3E774"]',
      -0.7,
      0.5,
      '["rot","pus","spectral","infected","green-yellow"]'
    );
