# File: tools/benchmark_tokens.py
# Target repo: Doctor0Evil/HorrorPlace-Constellation-Contracts

from ai_chat_interface import ConstellationQueryInterface
import time

def estimate_tokens(text: str) -> int:
    """Rough token estimation (4 chars per token average)."""
    return len(text) // 4

def benchmark_query_types():
    """Benchmark token savings across different query types."""
    api = ConstellationQueryInterface()
    
    test_cases = [
        {
            'name': 'Get single formula',
            'baseline_tokens': 1500,  # Reading Rust source file
            'query': lambda: api.get_formula('zombie-vomit', 'maskRadius'),
        },
        {
            'name': 'Get pattern summary',
            'baseline_tokens': 3500,  # Reading multiple files
            'query': lambda: api.get_pattern_summary('zombie-vomit'),
        },
        {
            'name': 'Cross-repo dependencies',
            'baseline_tokens': 5000,  # Traversing repos manually
            'query': lambda: api.get_cross_repo_dependencies('Rotting-Visuals-BCI'),
        },
        {
            'name': 'Energy cost estimate',
            'baseline_tokens': 1200,  # Reading benchmark data
            'query': lambda: api.estimate_energy_cost('zombie-vomit'),
        },
    ]
    
    results = []
    
    for case in test_cases:
        start = time.time()
        result = case['query']()
        latency = (time.time() - start) * 1000  # ms
        
        result_text = str(result)
        actual_tokens = estimate_tokens(result_text)
        savings = (case['baseline_tokens'] - actual_tokens) / case['baseline_tokens'] * 100
        
        results.append({
            'name': case['name'],
            'baseline': case['baseline_tokens'],
            'actual': actual_tokens,
            'savings': savings,
            'latency_ms': latency,
        })
    
    # Print results
    print("=== Token Savings Benchmark ===\n")
    for r in results:
        print(f"{r['name']}:")
        print(f"  Baseline: {r['baseline']} tokens")
        print(f"  Actual: {r['actual']} tokens")
        print(f"  Savings: {r['savings']:.1f}%")
        print(f"  Latency: {r['latency_ms']:.1f}ms\n")
    
    avg_savings = sum(r['savings'] for r in results) / len(results)
    print(f"Average savings: {avg_savings:.1f}%")
    print(f"Average latency: {sum(r['latency_ms'] for r in results) / len(results):.1f}ms")
    
    return results

if __name__ == '__main__':
    benchmark_query_types()
