# File: tools/generate_docs.py
# Target repo: Doctor0Evil/HorrorPlace-Constellation-Contracts

from ai_chat_interface import ConstellationQueryInterface
import os

def generate_pattern_docs(output_dir: str = 'docs/patterns'):
    """Generate markdown documentation for all patterns."""
    api = ConstellationQueryInterface()
    
    os.makedirs(output_dir, exist_ok=True)
    
    patterns = ['zombie-vomit', 'toxic-smear', 'face-drips', 'skin-worms', 'hanging-skin']
    
    # Generate individual pattern pages
    for pattern in patterns:
        summary = api.get_pattern_summary(pattern)
        energy = api.estimate_energy_cost(pattern)
        
        # Generate markdown
        md = f"""# {pattern.title().replace('-', ' ')}

## Overview
{summary['desc']}

## Configuration
- **Default Region**: `{summary['region']}`
- **Palette Groups**: {', '.join([f'`{p}`' for p in summary['palettes']])}

## Formulas

### Visual Parameters
"""
        # Add formula table
        visual_params = ['maskRadius', 'maskFeather', 'decayGrain', 'colorDesat', 'veinOverlay', 'motionSmear']
        md += "| Parameter | Formula |\n|-----------|--------|\n"
        for param in visual_params:
            formula_data = api.get_formula(pattern, param)
            if formula_data:
                md += f"| {param} | `{formula_data['formula']}` (R²={formula_data['R²']}) |\n"
        
        md += "\n### Audio Parameters\n"
        md += "| Parameter | Formula |\n|-----------|--------|\n"
        audio_params = ['infectedChannelGain', 'squadMuffle', 'heartbeatGain', 'breathGain', 'ringingLevel', 'direct']
        for param in audio_params:
            formula_data = api.get_formula(pattern, param)
            if formula_data:
                md += f"| {param} | `{formula_data['formula']}` (R²={formula_data['R²']}) |\n"
        
        md += f"""
## Energy Profile
- **Average Energy**: {energy.get('energy', 'N/A')} units
- **Average CPU Cycles**: {energy.get('cycles', 'N/A')}

## Usage
See [Integration Guide](../integration.md) for usage examples.

***
*Auto-generated from constellation_index.db on {pd.Timestamp.now().strftime('%Y-%m-%d')}*
"""
        
        with open(f"{output_dir}/{pattern}.md", 'w') as f:
            f.write(md)
    
    print(f"Generated documentation for {len(patterns)} patterns in {output_dir}/")
    
    # Generate index page
    with open(f"{output_dir}/README.md", 'w') as f:
        f.write("# BCI Geometry Patterns\n\n")
        for pattern in patterns:
            f.write(f"- [{pattern.title().replace('-', ' ')}]({pattern}.md)\n")

if __name__ == '__main__':
    generate_pattern_docs()
