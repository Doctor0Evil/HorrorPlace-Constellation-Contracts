-- File: db/schema/constellation_index_v1.sql
-- Target repo: Doctor0Evil/HorrorPlace-Constellation-Contracts

PRAGMA journal_mode=WAL; -- Enable concurrent reads
PRAGMA foreign_keys=ON;
PRAGMA optimize;

-- Core tables (from Part I)
CREATE TABLE repositories (...);
CREATE TABLE source_files (...);
CREATE TABLE functions (...);
CREATE TABLE function_calls (...);
CREATE TABLE formula_catalog (...);
CREATE TABLE pattern_catalog (...);

-- Compression indexes
CREATE INDEX idx_functions_hotpath ON functions(hot_path) WHERE hot_path = TRUE;
CREATE INDEX idx_formula_r2 ON formula_catalog(r_squared) WHERE r_squared > 0.95;

-- Virtual tables for full-text search
CREATE VIRTUAL TABLE functions_fts USING fts5(
    function_name,
    signature_compressed,
    content='functions'
);
