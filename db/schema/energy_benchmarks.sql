-- File: db/schema/energy_benchmarks.sql

CREATE TABLE energy_benchmarks (
    benchmark_id INTEGER PRIMARY KEY,
    pattern_id INTEGER,
    stress_score REAL,
    visual_overload REAL,
    cpu_cycles INTEGER,
    cache_misses INTEGER,
    energy_joules REAL, -- Measured via RAPL or hardware counters
    timestamp INTEGER
);

-- Aggregate statistics
CREATE VIEW energy_cost_summary AS
SELECT 
    pattern_id,
    AVG(cpu_cycles) AS avg_cycles,
    AVG(cache_misses) AS avg_cache_misses,
    AVG(energy_joules) AS avg_energy
FROM energy_benchmarks
GROUP BY pattern_id;

-- Find cheapest pattern that meets visual intensity requirements
SELECT pattern_id, avg_energy
FROM energy_cost_summary
WHERE pattern_id IN (
    SELECT pattern_id FROM pattern_catalog 
    WHERE experience_type = 'monster-mode'
)
ORDER BY avg_energy ASC
LIMIT 1;
