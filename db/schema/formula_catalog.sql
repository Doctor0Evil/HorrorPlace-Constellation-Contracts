-- File: db/schema/formula_catalog.sql
-- Target repo: Doctor0Evil/HorrorPlace-Constellation-Contracts

CREATE TABLE formula_catalog (
    formula_id INTEGER PRIMARY KEY,
    pattern_id INTEGER,
    parameter_name TEXT, -- 'maskRadius', 'decayGrain', etc.
    formula_symbolic TEXT, -- '0.95 - 0.55*S - 0.35*V'
    formula_type TEXT, -- 'linear', 'quadratic', 'piecewise'
    r_squared REAL, -- Goodness of fit
    input_variables TEXT, -- JSON array: ["S", "V", "Sp"]
    coefficient_vector TEXT, -- JSON array for machine parsing
    created_timestamp INTEGER
);

-- Insert zombie-vomit formulas
INSERT INTO formula_catalog VALUES
    (1, 0, 'maskRadius', '0.95 - 0.55*S - 0.35*V + 0.15*S', 'linear', 0.9998, 
     '["S", "V"]', '[0.95, -0.55, -0.35, 0.15]', strftime('%s', 'now') * 1000),
    (2, 0, 'decayGrain', '0.2 + 0.7*S + 0.3*SB + 0.15*S + 0.10*V', 'linear', 0.9996,
     '["S", "SB", "V"]', '[0.2, 0.85, 0.3, 0.10]', strftime('%s', 'now') * 1000),
    (3, 0, 'motionSmear', '0.15 + 0.4*V + 0.25*(1-AF) + 0.25*Sp + 0.15*V', 'linear', 0.9994,
     '["V", "AF", "Sp"]', '[0.15, 0.55, -0.25, 0.25]', strftime('%s', 'now') * 1000);
