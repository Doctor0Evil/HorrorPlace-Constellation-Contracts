#!/usr/bin/env python3
"""Auto-generate Rust tests from BCI contracts."""

import json, sys
from pathlib import Path

def generate_edge_case_test(func_name: str, case: dict) -> str:
    test_name = f"test_{func_name}_{case['name']}"
    
    bci_init = "\n".join([
        f"        {k}: {v}," 
        for k, v in case['input']['bci'].items()
    ])
    
    inv_init = "\n".join([
        f"        {k}: {v}," 
        for k, v in case['input']['invariants'].items()
    ])
    
    assertions = []
    for key, expected in case['expected_output'].items():
        if isinstance(expected, float):
            assertions.append(
                f"    assert!((result.{key} - {expected}).abs() < 0.01);"
            )
        else:
            assertions.append(f"    assert_eq!(result.{key}, {expected});")
    
    return f"""
#[test]
fn {test_name}() {{
    let bci = BciState {{
{bci_init}
    }};
    let invariants = Invariants {{
{inv_init}
    }};
    
    let result = {func_name}(&bci, &invariants, {case['input']['palette_size']}).unwrap();
    
{chr(10).join(assertions)}
}}
"""

def main():
    contract_path = Path(sys.argv[1])
    with open(contract_path) as f:
        contract = json.load(f)
    
    for func in contract['functions']:
        out_path = Path(f"crates/bci_geometry/tests/generated/test_{func['name']}.rs")
        
        with open(out_path, 'w') as out:
            out.write("// AUTO-GENERATED - DO NOT EDIT\n\n")
            for case in func.get('edge_cases', []):
                out.write(generate_edge_case_test(func['name'], case))
        
        print(f"Generated: {out_path}")

if __name__ == '__main__':
    main()
