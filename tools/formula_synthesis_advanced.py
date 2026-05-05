# File: tools/formula_synthesis_advanced.py
# Target repo: Doctor0Evil/HorrorPlace-Constellation-Contracts

import numpy as np
import pandas as pd
from sklearn.linear_model import ElasticNet, LassoCV
from sklearn.preprocessing import PolynomialFeatures
from sklearn.pipeline import Pipeline
import sympy as sp
import sqlite3

def synthesize_formula_advanced(
    df: pd.DataFrame,
    pattern_id: int,
    parameter_name: str,
    feature_cols: list[str],
    max_degree: int = 2,
    alpha_range: tuple = (0.001, 1.0)
) -> dict:
    """
    Synthesize formula with automatic feature selection and regularization.
    
    Args:
        df: Benchmark data
        pattern_id: Pattern to synthesize for
        parameter_name: Output parameter (e.g., 'maskRadius')
        feature_cols: Input features (e.g., ['stressScore', 'visualOverloadIndex'])
        max_degree: Maximum polynomial degree
        alpha_range: L1/L2 regularization strength range
    
    Returns:
        {
            'formula_symbolic': str,
            'formula_simplified': str,
            'r_squared': float,
            'coefficients': list,
            'feature_names': list,
            'rmse': float
        }
    """
    # Filter data for this pattern
    pattern_df = df[df['pattern_id'] == pattern_id].copy()
    
    X = pattern_df[feature_cols].values
    y = pattern_df[parameter_name].values
    
    # Try multiple modeling approaches
    models = []
    
    # 1. Linear model
    from sklearn.linear_model import LinearRegression
    lr = LinearRegression()
    lr.fit(X, y)
    y_pred = lr.predict(X)
    r2_linear = 1 - np.sum((y - y_pred)**2) / np.sum((y - y.mean())**2)
    models.append(('linear', lr, r2_linear, feature_cols))
    
    # 2. Polynomial features with Lasso regularization
    if max_degree > 1:
        poly = PolynomialFeatures(degree=max_degree, include_bias=False)
        X_poly = poly.fit_transform(X)
        
        lasso = LassoCV(alphas=np.logspace(*np.log10(alpha_range), 20), cv=5)
        lasso.fit(X_poly, y)
        y_pred_poly = lasso.predict(X_poly)
        r2_poly = 1 - np.sum((y - y_pred_poly)**2) / np.sum((y - y.mean())**2)
        
        poly_feature_names = poly.get_feature_names_out(feature_cols)
        models.append(('polynomial', lasso, r2_poly, poly_feature_names, X_poly))
    
    # Select best model
    best_model = max(models, key=lambda x: x)
    model_type, model, r2, feature_names = best_model[:4]
    
    # Extract formula
    if model_type == 'linear':
        coeffs = model.coef_
        intercept = model.intercept_
        X_used = X
    else:
        coeffs = model.coef_
        intercept = model.intercept_
        X_used = best_model
    
    # Build symbolic formula
    symbols_map = {
        'stressScore': 'S',
        'visualOverloadIndex': 'V',
        'startleSpike': 'Sp',
        'signalQuality': 'Q',
        'stressBand': 'SB',
        'attentionBand': 'AB'
    }
    
    terms = []
    if abs(intercept) > 0.01:
        terms.append(f"{intercept:.3f}")
    
    for coeff, fname in zip(coeffs, feature_names):
        if abs(coeff) < 0.01:
            continue  # Skip near-zero coefficients
        
        # Map feature name to symbol
        symbol = fname
        for long_name, short_name in symbols_map.items():
            symbol = symbol.replace(long_name, short_name)
        
        sign = '+' if coeff > 0 else '-'
        terms.append(f"{sign} {abs(coeff):.3f}*{symbol}")
    
    formula_raw = ' '.join(terms)
    
    # Simplify using sympy
    try:
        S, V, Sp, Q, SB, AB = sp.symbols('S V Sp Q SB AB')
        expr = sp.sympify(formula_raw.replace('^', '**'))
        formula_simplified = str(sp.simplify(expr))
    except:
        formula_simplified = formula_raw
    
    # Calculate RMSE
    y_pred_final = model.predict(X_used)
    rmse = np.sqrt(np.mean((y - y_pred_final)**2))
    
    return {
        'formula_symbolic': formula_raw,
        'formula_simplified': formula_simplified,
        'r_squared': r2,
        'coefficients': coeffs.tolist(),
        'feature_names': list(feature_names),
        'rmse': rmse,
        'model_type': model_type
    }

def populate_formula_catalog(csv_path: str, db_path: str):
    """Synthesize all formulas and populate SQL catalog."""
    df = pd.read_csv(csv_path)
    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()
    
    parameters = [
        'maskRadius', 'maskFeather', 'decayGrain', 'colorDesat', 
        'veinOverlay', 'motionSmear',
        'infectedChannelGain', 'squadMuffle', 'heartbeatGain',
        'breathGain', 'ringingLevel', 'direct'
    ]
    
    feature_cols = ['stressScore', 'visualOverloadIndex', 'startleSpike', 'signalQuality']
    
    for pattern_id in range(5):
        print(f"\nSynthesizing formulas for pattern {pattern_id}...")
        
        for param in parameters:
            result = synthesize_formula_advanced(
                df, pattern_id, param, feature_cols, max_degree=2
            )
            
            print(f"  {param}: R²={result['r_squared']:.4f}, RMSE={result['rmse']:.4f}")
            
            # Insert into database
            cursor.execute("""
                INSERT INTO formula_catalog (
                    pattern_id, parameter_name, formula_symbolic, formula_simplified,
                    formula_type, r_squared, input_variables, coefficient_vector,
                    created_timestamp
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            """, (
                pattern_id,
                param,
                result['formula_symbolic'],
                result['formula_simplified'],
                result['model_type'],
                result['r_squared'],
                str(result['feature_names']),
                str(result['coefficients']),
                int(pd.Timestamp.now().timestamp() * 1000)
            ))
    
    conn.commit()
    conn.close()
    print(f"\nPopulated formula catalog with {5 * len(parameters)} formulas.")

if __name__ == '__main__':
    populate_formula_catalog(
        'benchmarks/bci_patterns_full.csv',
        'constellation_index.db'
    )
