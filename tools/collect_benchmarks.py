# File: tools/collect_benchmarks.py
# Target repo: Doctor0Evil/HorrorPlace-Constellation-Contracts

import numpy as np
import pandas as pd
from rotting_visuals_bci import *

def generate_bci_states(n_samples: int = 1000) -> pd.DataFrame:
    """Generate diverse BCI states using Sobol sequence for coverage."""
    from scipy.stats import qmc
    
    # 6-dimensional Sobol sampler
    sampler = qmc.Sobol(d=6, scramble=True)
    samples = sampler.random(n=n_samples)
    
    # Map to BCI parameter ranges
    df = pd.DataFrame({
        'stressScore': samples[:, 0],
        'visualOverloadIndex': samples[:, 1],
        'startleSpike': samples[:, 2],
        'signalQuality': samples[:, 3],
        'stressBand_code': (samples[:, 4] * 3).astype(int),  # 0,1,2 for low/mid/high
        'attentionBand_code': (samples[:, 5] * 3).astype(int),
    })
    
    # Map codes to strings
    stress_bands = ['low', 'mid', 'high']
    attention_bands = ['drifting', 'focused', 'locked']
    df['stressBand'] = df['stressBand_code'].apply(lambda x: stress_bands[x])
    df['attentionBand'] = df['attentionBand_code'].apply(lambda x: attention_bands[x])
    
    return df

def evaluate_all_patterns(df: pd.DataFrame) -> pd.DataFrame:
    """Evaluate all 5 patterns on each BCI state."""
    results = []
    
    invariants = Invariants(
        CIC=0.92, AOS=0.81, DET=0.88, LSG=0.94,
        UEC=0.67, EMD=0.74, STCI=0.63, CDL=0.48, ARR=0.71
    )
    
    for idx, row in df.iterrows():
        bci = BciSummary(
            stress_score=row['stressScore'],
            stress_band=row['stressBand'],
            attention_band=row['attentionBand'],
            visual_overload_index=row['visualOverloadIndex'],
            startle_spike=row['startleSpike'],
            signal_quality=row['signalQuality']
        )
        
        for pattern_id in range(5):
            pattern = PatternId(pattern_id)
            visual = compute_visual_by_id(pattern, bci, invariants)
            audio = compute_audio_by_id(pattern, bci, invariants)
            
            results.append({
                'sample_id': idx,
                'pattern_id': pattern_id,
                **row.to_dict(),
                'maskRadius': visual.mask_radius,
                'maskFeather': visual.mask_feather,
                'decayGrain': visual.decay_grain,
                'colorDesat': visual.color_desat,
                'veinOverlay': visual.vein_overlay,
                'motionSmear': visual.motion_smear,
                'infectedChannelGain': audio.infected_channel_gain,
                'squadMuffle': audio.squad_muffle,
                'heartbeatGain': audio.heartbeat_gain,
                'breathGain': audio.breath_gain,
                'ringingLevel': audio.ringing_level,
                'direct': audio.direct,
            })
    
    return pd.DataFrame(results)

if __name__ == '__main__':
    print("Generating BCI states...")
    states = generate_bci_states(n_samples=2000)
    
    print("Evaluating all patterns...")
    results = evaluate_all_patterns(states)
    
    print("Saving to CSV...")
    results.to_csv('benchmarks/bci_patterns_full.csv', index=False)
    
    print(f"Generated {len(results)} samples across 5 patterns.")
    print(f"File size: {results.memory_usage(deep=True).sum() / 1024**2:.2f} MB")
