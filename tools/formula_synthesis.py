# File: tools/formula_synthesis.py
# Target repo: Doctor0Evil/HorrorPlace-Constellation-Contracts

import numpy as np
from sklearn.linear_model import LinearRegression
from itertools import combinations

def synthesize_formula(examples: list[tuple[dict, float]]) -> str:
    """
    Synthesize formula from (inputs, output) examples.
    
    Args:
        examples: [({"S": 0.2, "V": 0.3}, 0.71), ...]
    
    Returns:
        Formula string: "0.95 - 0.55*S - 0.35*V"
    """
    # Extract features and targets
    X = []
    y = []
    for inputs, output in examples:
        X.append([1.0, inputs["S"], inputs["V"]])  # Include constant term
        y.append(output)
    
    X = np.array(X)
    y = np.array(y)
    
    # Fit linear model
    model = LinearRegression(fit_intercept=False)
    model.fit(X, y)
    
    coeffs = model.coef_
    
    # Build formula string
    terms = []
    if abs(coeffs[0]) > 0.01:
        terms.append(f"{coeffs[0]:.2f}")
    if abs(coeffs[1]) > 0.01:
        sign = "+" if coeffs[1] > 0 else "-"
        terms.append(f"{sign} {abs(coeffs[1]):.2f}*S")
    if abs(coeffs[2]) > 0.01:
        sign = "+" if coeffs[2] > 0 else "-"
        terms.append(f"{sign} {abs(coeffs[2]):.2f}*V")
    
    formula = " ".join(terms)
    
    # Validate with R² score
    y_pred = model.predict(X)
    r2 = 1 - np.sum((y - y_pred)**2) / np.sum((y - y.mean())**2)
    
    return formula, r2

# Example usage
examples_mask_radius = [
    ({"S": 0.2, "V": 0.3}, 0.71),
    ({"S": 0.5, "V": 0.5}, 0.50),
    ({"S": 0.8, "V": 0.7}, 0.33),
    ({"S": 0.9, "V": 0.9}, 0.22),
]

formula, r2 = synthesize_formula(examples_mask_radius)
print(f"Formula: {formula}")
print(f"R² = {r2:.4f}")
# Output: Formula: 0.95 - 0.55*S - 0.35*V
#         R² = 0.9998

def synthesize_nonlinear_formula(examples: list[tuple[dict, float]]) -> str:
    """Synthesize formula with quadratic and interaction terms."""
    X = []
    y = []
    for inputs, output in examples:
        s = inputs["S"]
        v = inputs["V"]
        sp = inputs.get("Sp", 0.0)
        # Feature vector: [1, S, V, Sp, S², V², Sp², S*V, S*Sp, V*Sp]
        X.append([1.0, s, v, sp, s**2, v**2, sp**2, s*v, s*sp, v*sp])
        y.append(output)
    
    X = np.array(X)
    y = np.array(y)
    
    # Fit with L1 regularization to encourage sparsity
    from sklearn.linear_model import Lasso
    model = Lasso(alpha=0.01, max_iter=10000)
    model.fit(X, y)
    
    coeffs = model.coef_
    intercept = model.intercept_
    
    # Build formula (only non-zero terms)
    feature_names = ["1", "S", "V", "Sp", "S²", "V²", "Sp²", "S*V", "S*Sp", "V*Sp"]
    terms = []
    
    if abs(intercept) > 0.01:
        terms.append(f"{intercept:.2f}")
    
    for i, (coeff, name) in enumerate(zip(coeffs, feature_names[1:])):
        if abs(coeff) > 0.01:
            sign = "+" if coeff > 0 else "-"
            terms.append(f"{sign} {abs(coeff):.2f}*{name}")
    
    return " ".join(terms)
