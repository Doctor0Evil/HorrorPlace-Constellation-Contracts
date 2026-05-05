// File: src/pattern_composition/mod.rs

pub enum CompositionMode {
    LinearBlend { weights: Vec<f32> },
    Max,
    Gated { conditions: Vec<Condition> },
}

pub struct CompositePattern {
    pub patterns: Vec<PatternId>,
    pub mode: CompositionMode,
}

impl CompositePattern {
    pub fn compute_visual(&self, bci: &BciSummary, inv: &Invariants) -> VisualParams {
        match &self.mode {
            CompositionMode::LinearBlend { weights } => {
                let mut result = VisualParams::default();
                for (i, &pattern) in self.patterns.iter().enumerate() {
                    let params = compute_visual_by_id(pattern, bci, inv);
                    result.blend_add(&params, weights[i]);
                }
                result
            }
            CompositionMode::Max => {
                self.patterns.iter()
                    .map(|&p| compute_visual_by_id(p, bci, inv))
                    .max_by(|a, b| a.decay_grain.partial_cmp(&b.decay_grain).unwrap())
                    .unwrap()
            }
            CompositionMode::Gated { conditions } => {
                for (i, condition) in conditions.iter().enumerate() {
                    if condition.evaluate(bci) {
                        return compute_visual_by_id(self.patterns[i], bci, inv);
                    }
                }
                compute_visual_by_id(self.patterns.last().unwrap(), bci, inv)
            }
        }
    }
}
