use std::collections::HashMap;

pub struct AdaptationProfile {
    pub total_sessions: u32,
    pub pattern_exposures: HashMap<String, u32>,
    pub base_rate: f32,
}

impl AdaptationProfile {
    pub fn compute_pattern_adaptation(&self, pattern_name: &str) -> f32 {
        let exposure_count = self.pattern_exposures
            .get(pattern_name)
            .copied()
            .unwrap_or(0);
        
        1.0 + (exposure_count as f32 * 0.003).min(0.6)  // Cap at 60%
    }
    
    pub fn apply_adaptation_boost(
        &self,
        base_value: f32,
        pattern_name: &str,
        boost_sensitivity: f32,
    ) -> f32 {
        let adaptation = self.compute_pattern_adaptation(pattern_name);
        let boost_factor = 1.0 + (adaptation - 1.0) * boost_sensitivity;
        
        (base_value * boost_factor).clamp(0.0, 1.0)
    }
}
