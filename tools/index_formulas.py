# File: tools/index_formulas.py

from sentence_transformers import SentenceTransformer
import sqlite3
import numpy as np

model = SentenceTransformer('sentence-transformers/all-MiniLM-L6-v2')

conn = sqlite3.connect('constellation_index.db')
cursor = conn.cursor()

# Fetch all formulas
formulas = cursor.execute("""
    SELECT formula_id, formula_symbolic, parameter_name
    FROM formula_catalog
""").fetchall()

for formula_id, symbolic, param_name in formulas:
    # Create descriptive text
    description = f"BCI formula for {param_name}: {symbolic}"
    
    # Generate embedding
    embedding = model.encode(description)
    
    # Store in database
    cursor.execute("""
        INSERT INTO formula_embeddings (formula_id, description_text, embedding_vector, embedding_model)
        VALUES (?, ?, ?, ?)
    """, (formula_id, description, embedding.tobytes(), model.config.name_or_path))

conn.commit()
