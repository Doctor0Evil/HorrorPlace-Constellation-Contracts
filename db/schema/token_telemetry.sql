-- File: db/schema/token_telemetry.sql
-- Target repo: Doctor0Evil/HorrorPlace-Constellation-Contracts

CREATE TABLE ai_chat_queries (
    query_id INTEGER PRIMARY KEY AUTOINCREMENT,
    query_timestamp INTEGER NOT NULL,
    query_type TEXT NOT NULL, -- 'get_formula', 'get_summary', 'search', etc.
    query_params TEXT, -- JSON
    tokens_without_index INTEGER, -- Estimated tokens if reading source files
    tokens_with_index INTEGER, -- Actual tokens using index
    token_savings REAL, -- (without - with) / without
    latency_ms INTEGER
);

CREATE INDEX idx_query_type ON ai_chat_queries(query_type);
CREATE INDEX idx_query_timestamp ON ai_chat_queries(query_timestamp);

-- Summary view
CREATE VIEW token_savings_summary AS
SELECT 
    query_type,
    COUNT(*) AS query_count,
    AVG(token_savings) AS avg_savings,
    SUM(tokens_without_index - tokens_with_index) AS total_tokens_saved,
    AVG(latency_ms) AS avg_latency_ms
FROM ai_chat_queries
GROUP BY query_type
ORDER BY total_tokens_saved DESC;
