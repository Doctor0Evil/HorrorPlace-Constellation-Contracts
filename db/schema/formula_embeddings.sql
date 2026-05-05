-- File: db/schema/formula_embeddings.sql

CREATE TABLE formula_embeddings (
    embedding_id INTEGER PRIMARY KEY,
    formula_id INTEGER REFERENCES formula_catalog(formula_id),
    description_text TEXT,
    embedding_vector BLOB, -- Serialized float array (e.g., 384-dim from MiniLM)
    embedding_model TEXT -- 'sentence-transformers/all-MiniLM-L6-v2'
);

-- Virtual table for similarity search (requires sqlite-vss extension)
CREATE VIRTUAL TABLE formula_search USING vss0(
    embedding(384)
);
