-- File: db/analytics/impact_metrics.sql
-- Target repo: Doctor0Evil/HorrorPlace-Constellation-Contracts

-- Overall token efficiency
CREATE VIEW token_efficiency_metrics AS
SELECT 
    'Overall' AS scope,
    SUM(tokens_without_index) AS total_baseline_tokens,
    SUM(tokens_with_index) AS total_optimized_tokens,
    (1.0 - CAST(SUM(tokens_with_index) AS REAL) / SUM(tokens_without_index)) * 100 AS savings_percentage,
    SUM(tokens_without_index - tokens_with_index) AS total_tokens_saved
FROM ai_chat_queries
UNION ALL
SELECT 
    query_type AS scope,
    SUM(tokens_without_index),
    SUM(tokens_with_index),
    (1.0 - CAST(SUM(tokens_with_index) AS REAL) / SUM(tokens_without_index)) * 100,
    SUM(tokens_without_index - tokens_with_index)
FROM ai_chat_queries
GROUP BY query_type
ORDER BY total_tokens_saved DESC;

-- Developer productivity
CREATE VIEW developer_productivity_metrics AS
SELECT 
    DATE(query_timestamp / 1000, 'unixepoch') AS date,
    COUNT(*) AS queries_per_day,
    AVG(latency_ms) AS avg_latency_ms,
    SUM(tokens_without_index - tokens_with_index) AS tokens_saved_per_day
FROM ai_chat_queries
GROUP BY date
ORDER BY date DESC;

-- Formula quality
CREATE VIEW formula_quality_metrics AS
SELECT 
    AVG(r_squared) AS avg_r_squared,
    MIN(r_squared) AS min_r_squared,
    COUNT(*) AS total_formulas,
    SUM(CASE WHEN r_squared > 0.95 THEN 1 ELSE 0 END) AS high_quality_formulas,
    CAST(SUM(CASE WHEN r_squared > 0.95 THEN 1 ELSE 0 END) AS REAL) / COUNT(*) * 100 AS high_quality_percentage
FROM formula_catalog;

-- Cross-repo navigation efficiency
CREATE VIEW cross_repo_navigation_metrics AS
SELECT 
    caller_repo.repo_name AS from_repo,
    COUNT(DISTINCT callee_repo.repo_name) AS dependencies_count,
    COUNT(*) AS total_cross_repo_calls,
    SUM(fc.call_count_estimate) AS estimated_daily_calls
FROM function_calls fc
JOIN functions caller_func ON fc.caller_function_id = caller_func.function_id
JOIN functions callee_func ON fc.callee_function_id = callee_func.function_id
JOIN source_files caller_file ON caller_func.file_id = caller_file.file_id
JOIN source_files callee_file ON callee_func.file_id = callee_file.file_id
JOIN repositories caller_repo ON caller_file.repo_id = caller_repo.repo_id
JOIN repositories callee_repo ON callee_file.repo_id = callee_repo.repo_id
WHERE fc.is_cross_repo = TRUE
GROUP BY caller_repo.repo_name;
