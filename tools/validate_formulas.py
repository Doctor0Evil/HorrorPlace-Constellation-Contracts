# File: tools/validate_formulas.py
# Target repo: Doctor0Evil/HorrorPlace-Constellation-Contracts

import sqlite3
import numpy as np
import pandas as pd
from typing import List, Dict

class FormulaValidator:
    def __init__(self, db_path: str):
        self.conn = sqlite3.connect(db_path)
        self.cursor = self.conn.cursor()
    
    def check_range_violations(self, pattern_id: int, parameter: str) -> Dict:
        """Check if formula produces values outside [0, 1] for typical inputs."""
        formula = self.cursor.execute(
            "SELECT formula_simplified FROM formula_catalog WHERE pattern_id = ? AND parameter_name = ?",
            (pattern_id, parameter)
        ).fetchone()
        
        # Test on grid of BCI states
        violations = []
        for s in np.linspace(0, 1, 20):
            for v in np.linspace(0, 1, 20):
                for sp in [0.0, 0.5, 1.0]:
                    for q in [0.5, 1.0]:
                        # Evaluate formula (simplified: just linear terms)
                        # In production, use actual formula evaluator
                        value = eval(formula.replace('S', str(s))
                                            .replace('V', str(v))
                                            .replace('Sp', str(sp))
                                            .replace('Q', str(q)))
                        
                        if value < 0 or value > 1:
                            violations.append({
                                'S': s, 'V': v, 'Sp': sp, 'Q': q,
                                'value': value
                            })
        
        return {
            'total_tests': 20 * 20 * 3 * 2,
            'violations': len(violations),
            'violation_rate': len(violations) / (20 * 20 * 3 * 2),
            'examples': violations[:5]
        }
    
    def check_monotonicity(self, pattern_id: int, parameter: str) -> Dict:
        """Check if parameter increases with stress (expected for most params)."""
        formula = self.cursor.execute(
            "SELECT formula_simplified FROM formula_catalog WHERE pattern_id = ? AND parameter_name = ?",
            (pattern_id, parameter)
        ).fetchone()
        
        # Sample along stress axis
        stress_values = np.linspace(0, 1, 50)
        outputs = []
        
        for s in stress_values:
            value = eval(formula.replace('S', str(s))
                                .replace('V', '0.5')
                                .replace('Sp', '0.0')
                                .replace('Q', '1.0'))
            outputs.append(value)
        
        # Check if mostly increasing
        diffs = np.diff(outputs)
        increasing_ratio = np.sum(diffs > 0) / len(diffs)
        
        return {
            'is_monotonic_increasing': increasing_ratio > 0.9,
            'increasing_ratio': increasing_ratio,
            'non_monotonic_points': np.sum(diffs <= 0)
        }
    
    def check_coefficient_magnitudes(self, pattern_id: int, parameter: str) -> Dict:
        """Warn if coefficients are unusually large (likely typo)."""
        coeffs_str = self.cursor.execute(
            "SELECT coefficient_vector FROM formula_catalog WHERE pattern_id = ? AND parameter_name = ?",
            (pattern_id, parameter)
        ).fetchone()
        
        coeffs = eval(coeffs_str)  # List of floats
        
        large_coeffs = [c for c in coeffs if abs(c) > 2.0]
        
        return {
            'has_large_coefficients': len(large_coeffs) > 0,
            'large_coefficients': large_coeffs,
            'max_coefficient': max([abs(c) for c in coeffs])
        }
    
    def run_full_validation(self) -> pd.DataFrame:
        """Run all validation checks on all formulas."""
        results = []
        
        formulas = self.cursor.execute(
            "SELECT pattern_id, parameter_name FROM formula_catalog"
        ).fetchall()
        
        for pattern_id, param in formulas:
            range_check = self.check_range_violations(pattern_id, param)
            mono_check = self.check_monotonicity(pattern_id, param)
            coeff_check = self.check_coefficient_magnitudes(pattern_id, param)
            
            results.append({
                'pattern_id': pattern_id,
                'parameter': param,
                'range_violation_rate': range_check['violation_rate'],
                'is_monotonic': mono_check['is_monotonic_increasing'],
                'has_large_coeffs': coeff_check['has_large_coefficients'],
                'max_coefficient': coeff_check['max_coefficient'],
                'passed': (
                    range_check['violation_rate'] < 0.05 and
                    not coeff_check['has_large_coefficients']
                )
            })
        
        return pd.DataFrame(results)

if __name__ == '__main__':
    validator = FormulaValidator('constellation_index.db')
    report = validator.run_full_validation()
    
    print("\n=== Formula Validation Report ===")
    print(f"Total formulas: {len(report)}")
    print(f"Passed: {report['passed'].sum()}")
    print(f"Failed: {(~report['passed']).sum()}")
    
    print("\nFailed formulas:")
    print(report[~report['passed']][['pattern_id', 'parameter', 'range_violation_rate', 'max_coefficient']])
    
    report.to_csv('validation_report.csv', index=False)
