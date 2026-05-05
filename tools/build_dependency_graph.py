# File: tools/build_dependency_graph.py
# Target repo: Doctor0Evil/HorrorPlace-Constellation-Contracts

import sqlite3
import tree_sitter_rust as tsrust
from tree_sitter import Language, Parser
import os

def parse_rust_file(file_path: str) -> list[dict]:
    """Extract function definitions and calls from Rust file."""
    with open(file_path, 'rb') as f:
        code = f.read()
    
    RUST_LANGUAGE = Language(tsrust.language())
    parser = Parser(RUST_LANGUAGE)
    tree = parser.parse(code)
    
    functions = []
    calls = []
    
    # Query for function definitions
    query_defs = RUST_LANGUAGE.query("""
        (function_item
            name: (identifier) @func_name
            parameters: (parameters) @params
            return_type: (type_identifier)? @return_type
        )
    """)
    
    for match in query_defs.matches(tree.root_node):
        func_name = match.captures['func_name'][0].text.decode('utf8')
        # Extract params and return type...
        functions.append({
            'name': func_name,
            'signature': extract_signature(match),
            'line': match.captures['func_name'][0].start_point[0]
        })
    
    # Query for function calls
    query_calls = RUST_LANGUAGE.query("""
        (call_expression
            function: (identifier) @callee
        )
    """)
    
    for match in query_calls.matches(tree.root_node):
        callee = match.captures['callee'][0].text.decode('utf8')
        calls.append({'callee': callee})
    
    return functions, calls

def build_dependency_graph(repos: list[str], db_path: str):
    """Build complete dependency graph for multiple repos."""
    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()
    
    # Insert repos
    for repo in repos:
        cursor.execute(
            "INSERT OR IGNORE INTO repositories (repo_name, repo_url) VALUES (?, ?)",
            (os.path.basename(repo), repo)
        )
    
    # Walk each repo
    for repo in repos:
        repo_id = cursor.execute(
            "SELECT repo_id FROM repositories WHERE repo_name = ?",
            (os.path.basename(repo),)
        ).fetchone()[0]
        
        for root, dirs, files in os.walk(repo):
            for file in files:
                if file.endswith('.rs'):
                    file_path = os.path.join(root, file)
                    rel_path = os.path.relpath(file_path, repo)
                    
                    # Insert file
                    cursor.execute(
                        "INSERT INTO source_files (repo_id, file_path, file_type) VALUES (?, ?, ?)",
                        (repo_id, rel_path, 'rust')
                    )
                    file_id = cursor.lastrowid
                    
                    # Parse and insert functions
                    functions, calls = parse_rust_file(file_path)
                    for func in functions:
                        cursor.execute(
                            "INSERT INTO functions (file_id, function_name, signature_compressed) VALUES (?, ?, ?)",
                            (file_id, func['name'], func['signature'])
                        )
    
    conn.commit()
    conn.close()

# Usage
build_dependency_graph([
    '../Rotting-Visuals-BCI',
    '../HorrorPlace-Constellation-Contracts',
    '../Death-Engine'
], 'constellation_index.db')
