# File: tools/simplify_formulas.py

import sympy as sp

def simplify_formula(formula_str: str) -> str:
    """
    Simplify BCI formula by combining like terms.
    
    Example:
        Input: '0.2 + 0.7*S + 0.3*B + 0.15*S + 0.10*V'
        Output: '0.2 + 0.85*S + 0.3*B + 0.1*V'
    """
    # Define symbols
    S, V, Sp, SB, AB, AF, Q = sp.symbols('S V Sp SB AB AF Q')
    
    # Parse formula string to sympy expression
    # (Replace variable names, parse with sympify)
    expr = sp.sympify(formula_str.replace('*', ' * '))
    
    # Simplify
    simplified = sp.simplify(expr)
    
    # Convert back to string
    return str(simplified)

# Example
original = "0.2 + 0.7*S + 0.3*SB + 0.15*S + 0.10*V"
simplified = simplify_formula(original)
print(simplified)
# Output: 0.85*S + 0.3*SB + 0.1*V + 0.2
