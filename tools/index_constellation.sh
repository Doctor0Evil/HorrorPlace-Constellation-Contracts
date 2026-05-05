# File: tools/index_constellation.sh
# Target repo: Doctor0Evil/HorrorPlace-Constellation-Contracts

#!/bin/bash
set -euo pipefail

REPOS=(
    "../Rotting-Visuals-BCI"
    "../HorrorPlace-Constellation-Contracts"
    "../Death-Engine"
    "../HorrorPlace-Dead-Ledger-Network"
    "../HorrorPlace-Neural-Resonance-Lab"
    "../HorrorPlace-RotCave"
    "../HorrorPlace-Spectral-Foundry"
)

echo "Building constellation index..."
python3 tools/build_dependency_graph.py "${REPOS[@]}" \
    --output constellation_index.db \
    --workers 8 \
    --languages rust,cpp,sql

echo "Synthesizing formulas..."
python3 tools/formula_synthesis.py \
    --benchmark-data benchmarks/bci_patterns.csv \
    --db constellation_index.db \
    --min-r2 0.95

echo "Generating embeddings..."
python3 tools/index_formulas.py \
    --db constellation_index.db \
    --model sentence-transformers/all-MiniLM-L6-v2

echo "Validating schema..."
sqlite3 constellation_index.db < db/validation/check_constraints.sql

echo "Index complete. Size: $(du -h constellation_index.db | cut -f1)"
