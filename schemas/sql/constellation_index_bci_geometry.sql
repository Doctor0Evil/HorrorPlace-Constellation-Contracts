CREATE TABLE IF NOT EXISTS bci_variables (
    var_id INTEGER PRIMARY KEY,
    symbol TEXT NOT NULL UNIQUE,
    full_name TEXT NOT NULL,
    source TEXT,
    range_min REAL,
    range_max REAL
);

INSERT INTO bci_variables (var_id, symbol, full_name, source, range_min, range_max) VALUES
(1, 'S', 'stressScore', 'bci-metrics-envelope', 0.0, 1.0),
(2, 'V', 'visualOverloadIndex', 'bci-metrics-envelope', 0.0, 1.0),
(3, 'K', 'startleSpike', 'bci-metrics-envelope', 0.0, 1.0),
(4, 'A', 'attentionBand', 'bci-metrics-envelope', 0.0, 1.0),
(5, 'L', 'LSG', 'invariants', 0.0, 1.0);

CREATE TABLE IF NOT EXISTS patterns (
    pattern_id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    kind TEXT NOT NULL,
    shci_weight REAL
);

INSERT INTO patterns (pattern_id, name, kind, shci_weight) VALUES
(0, 'zombie-vomit', 'visual', 0.92),
(1, 'toxic-smear', 'visual', 0.90),
(2, 'face-drips', 'visual', 0.93),
(3, 'skin-worms', 'visual', 0.91),
(4, 'hanging-skin', 'visual', 0.93);

CREATE TABLE IF NOT EXISTS formula_catalog (
    formula_id INTEGER PRIMARY KEY AUTOINCREMENT,
    pattern_id INTEGER NOT NULL,
    param_code TEXT NOT NULL,
    symbolic_expr TEXT NOT NULL,
    formula_type TEXT NOT NULL,
    r2 REAL,
    coef_vector TEXT,
    depends_on TEXT,
    spr_contribution REAL,
    shci_contribution REAL,
    git_commit TEXT,
    derived_by TEXT,
    status TEXT,
    FOREIGN KEY (pattern_id) REFERENCES patterns(pattern_id)
);

INSERT INTO formula_catalog (pattern_id, param_code, symbolic_expr, formula_type, r2, depends_on, spr_contribution, shci_contribution, status) VALUES
(0, 'maskRadius', '0.95 - 0.55*S - 0.35*V', 'linear', 0.95, '["S","V"]', 0.0, 0.92, 'verified'),
(0, 'decayGrain', '0.60 + 0.15*S + 0.10*V + 0.05*K', 'linear', 0.95, '["S","V","K"]', 0.0, 0.92, 'verified'),
(0, 'motionSmear', '0.40 + 0.25*K + 0.15*V + 0.02*S', 'linear', 0.95, '["K","V","S"]', 0.0, 0.92, 'verified'),
(0, 'breathGain', '0.40 + 0.35*S + 0.09*K', 'linear', 0.95, '["S","K"]', 0.0, 0.92, 'verified');
