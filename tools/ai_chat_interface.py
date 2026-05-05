# File: tools/ai_chat_interface.py
# Target repo: Doctor0Evil/HorrorPlace-Constellation-Contracts

import sqlite3
from typing import List, Dict, Optional
import json

class ConstellationQueryInterface:
    """High-level API for AI-chat to query constellation knowledge."""
    
    def __init__(self, db_path: str = 'constellation_index.db'):
        self.conn = sqlite3.connect(db_path)
        self.conn.row_factory = sqlite3.Row  # Return dicts instead of tuples
    
    def get_formula(self, pattern_name: str, parameter: str) -> Optional[Dict]:
        """
        Get formula for a pattern parameter.
        
        Returns compact representation optimized for token efficiency.
        """
        cursor = self.conn.cursor()
        result = cursor.execute("""
            SELECT 
                fc.formula_simplified,
                fc.r_squared,
                fc.input_variables
            FROM formula_catalog fc
            JOIN pattern_catalog pc ON fc.pattern_id = pc.pattern_id
            WHERE pc.pattern_name = ? AND fc.parameter_name = ?
        """, (pattern_name, parameter)).fetchone()
        
        if not result:
            return None
        
        return {
            'formula': result['formula_simplified'],
            'R²': round(result['r_squared'], 4),
            'inputs': json.loads(result['input_variables'])
        }
    
    def get_pattern_summary(self, pattern_name: str) -> Dict:
        """Get complete summary of a pattern (formulas + metadata)."""
        cursor = self.conn.cursor()
        
        # Get pattern metadata
        pattern = cursor.execute("""
            SELECT pattern_id, description, default_region, palette_groups
            FROM pattern_catalog
            WHERE pattern_name = ?
        """, (pattern_name,)).fetchone()
        
        if not pattern:
            return {'error': f'Pattern {pattern_name} not found'}
        
        # Get all formulas
        formulas = cursor.execute("""
            SELECT parameter_name, formula_simplified, r_squared
            FROM formula_catalog
            WHERE pattern_id = ?
            ORDER BY parameter_name
        """, (pattern['pattern_id'],)).fetchall()
        
        # Compact representation
        formula_compact = {
            row['parameter_name'][:2]: row['formula_simplified']  # 2-char param names
            for row in formulas
        }
        
        return {
            'desc': pattern['description'][:100] + '...',  # Truncate description
            'region': pattern['default_region'],
            'palettes': json.loads(pattern['palette_groups']),
            'formulas': formula_compact
        }
    
    def find_similar_formulas(self, query: str, limit: int = 5) -> List[Dict]:
        """Semantic search for formulas (requires embeddings)."""
        # Placeholder: would use vector similarity search
        cursor = self.conn.cursor()
        
        # Simple keyword search fallback
        results = cursor.execute("""
            SELECT 
                pc.pattern_name,
                fc.parameter_name,
                fc.formula_simplified
            FROM formula_catalog fc
            JOIN pattern_catalog pc ON fc.pattern_id = pc.pattern_id
            WHERE fc.formula_simplified LIKE ?
            LIMIT ?
        """, (f'%{query}%', limit)).fetchall()
        
        return [dict(row) for row in results]
    
    def get_cross_repo_dependencies(self, repo_name: str) -> Dict:
        """Find all repos that this repo depends on."""
        cursor = self.conn.cursor()
        
        deps = cursor.execute("""
            SELECT DISTINCT
                callee_repo.repo_name AS dependency,
                COUNT(*) AS call_count
            FROM function_calls fc
            JOIN functions caller_func ON fc.caller_function_id = caller_func.function_id
            JOIN functions callee_func ON fc.callee_function_id = callee_func.function_id
            JOIN source_files caller_file ON caller_func.file_id = caller_file.file_id
            JOIN source_files callee_file ON callee_func.file_id = callee_file.file_id
            JOIN repositories caller_repo ON caller_file.repo_id = caller_repo.repo_id
            JOIN repositories callee_repo ON callee_file.repo_id = callee_repo.repo_id
            WHERE fc.is_cross_repo = TRUE
              AND caller_repo.repo_name = ?
            GROUP BY callee_repo.repo_name
            ORDER BY call_count DESC
        """, (repo_name,)).fetchall()
        
        return {
            'repo': repo_name,
            'dependencies': [{'name': row['dependency'], 'calls': row['call_count']} for row in deps]
        }
    
    def estimate_energy_cost(self, pattern_name: str) -> Dict:
        """Get energy cost estimate for a pattern."""
        cursor = self.conn.cursor()
        
        result = cursor.execute("""
            SELECT AVG(ec.avg_energy) AS energy, AVG(ec.avg_cycles) AS cycles
            FROM energy_cost_summary ec
            JOIN pattern_catalog pc ON ec.pattern_id = pc.pattern_id
            WHERE pc.pattern_name = ?
        """, (pattern_name,)).fetchone()
        
        if not result or result['energy'] is None:
            return {'error': 'No energy data available'}
        
        return {
            'energy': round(result['energy'], 2),
            'cycles': int(result['cycles']),
            'unit': 'arbitrary'
        }
    
    def generate_compact_report(self, pattern_name: str) -> str:
        """
        Generate ultra-compact report for AI-chat (target: <200 tokens).
        
        Format:
        pattern_name: description
        Region: default_region | Palettes: [p1, p2]
        Formulas: {mR: formula1, dG: formula2, ...}
        Energy: X units (Yth/5)
        """
        summary = self.get_pattern_summary(pattern_name)
        energy = self.estimate_energy_cost(pattern_name)
        
        if 'error' in summary:
            return f"Error: {summary['error']}"
        
        report_lines = [
            f"{pattern_name}: {summary['desc']}",
            f"Region: {summary['region']} | Palettes: {summary['palettes']}",
            f"Formulas: {summary['formulas']}",
            f"Energy: {energy.get('energy', 'N/A')} units"
        ]
        
        return '\n'.join(report_lines)

# Example usage for AI-chat
if __name__ == '__main__':
    api = ConstellationQueryInterface()
    
    # Ultra-compact query: Get formula
    formula = api.get_formula('zombie-vomit', 'maskRadius')
    print(f"Formula: {formula}")  # ~30 tokens
    
    # Compact pattern summary
    summary = api.get_pattern_summary('zombie-vomit')
    print(f"Summary: {summary}")  # ~80 tokens
    
    # Full compact report
    report = api.generate_compact_report('zombie-vomit')
    print(report)  # ~150 tokens vs. ~3500 reading source
