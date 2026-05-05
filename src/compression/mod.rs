// File: src/compression/mod.rs
// Target repo: Doctor0Evil/HorrorPlace-Constellation-Contracts

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Compression dictionary for BCI geometry schemas
pub struct CompressionDict {
    field_map: HashMap<String, String>,
    reverse_map: HashMap<String, String>,
}

impl CompressionDict {
    pub fn bci_standard() -> Self {
        let pairs = vec![
            // BciSummary fields
            ("stressScore", "S"),
            ("visualOverloadIndex", "V"),
            ("startleSpike", "Sp"),
            ("signalQuality", "Q"),
            ("stressBand", "SB"),
            ("attentionBand", "AB"),
            // VisualParams
            ("maskRadius", "mR"),
            ("maskFeather", "mF"),
            ("decayGrain", "dG"),
            ("colorDesat", "cD"),
            ("veinOverlay", "vO"),
            ("motionSmear", "mS"),
            ("paletteHex", "pH"),
            // AudioParams
            ("infectedChannelGain", "iG"),
            ("squadMuffle", "sM"),
            ("heartbeatGain", "hG"),
            ("breathGain", "bG"),
            ("ringingLevel", "rL"),
            ("direct", "dL"),
            // Invariants
            ("CIC", "C"),
            ("AOS", "A"),
            ("DET", "D"),
            ("LSG", "L"),
            ("UEC", "U"),
            ("EMD", "E"),
            ("STCI", "T"),
            ("CDL", "K"),
            ("ARR", "R"),
        ];
        
        let field_map: HashMap<_, _> = pairs.iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        
        let reverse_map: HashMap<_, _> = pairs.iter()
            .map(|(k, v)| (v.to_string(), k.to_string()))
            .collect();
        
        Self { field_map, reverse_map }
    }
    
    /// Compress JSON to compact notation
    pub fn compress_json(&self, json: &serde_json::Value) -> Result<String, String> {
        match json {
            serde_json::Value::Object(map) => {
                let compressed_pairs: Vec<String> = map.iter()
                    .map(|(k, v)| {
                        let short_key = self.field_map.get(k)
                            .ok_or_else(|| format!("Unknown field: {}", k))?;
                        let compressed_value = match v {
                            serde_json::Value::Number(n) => format!("{:.2}", n.as_f64().unwrap()),
                            serde_json::Value::String(s) => format!("'{}'", s),
                            serde_json::Value::Array(arr) => {
                                let items: Vec<String> = arr.iter()
                                    .map(|x| x.as_str().unwrap_or("").to_string())
                                    .collect();
                                format!("[{}]", items.join(","))
                            }
                            _ => return Err("Unsupported JSON type".to_string()),
                        };
                        Ok(format!("{}:{}", short_key, compressed_value))
                    })
                    .collect::<Result<Vec<_>, String>>()?;
                
                Ok(format!("{{{}}}", compressed_pairs.join(",")))
            }
            _ => Err("Expected JSON object".to_string()),
        }
    }
    
    /// Decompress back to JSON
    pub fn decompress_to_json(&self, compressed: &str) -> Result<serde_json::Value, String> {
        // Parser implementation
        let stripped = compressed.trim_start_matches('{').trim_end_matches('}');
        let mut map = serde_json::Map::new();
        
        for pair in stripped.split(',') {
            let parts: Vec<&str> = pair.split(':').collect();
            if parts.len() != 2 {
                return Err(format!("Invalid pair: {}", pair));
            }
            
            let short_key = parts.trim();
            let long_key = self.reverse_map.get(short_key)
                .ok_or_else(|| format!("Unknown short key: {}", short_key))?;
            
            let value_str = parts.trim();
            let value = if value_str.starts_with('\'') {
                serde_json::Value::String(value_str.trim_matches('\'').to_string())
            } else if value_str.starts_with('[') {
                // Parse array
                let items_str = value_str.trim_start_matches('[').trim_end_matches(']');
                let items: Vec<serde_json::Value> = items_str.split(',')
                    .map(|s| serde_json::Value::String(s.trim().to_string()))
                    .collect();
                serde_json::Value::Array(items)
            } else {
                // Parse number
                serde_json::json!(value_str.parse::<f64>().unwrap())
            };
            
            map.insert(long_key.clone(), value);
        }
        
        Ok(serde_json::Value::Object(map))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_roundtrip() {
        let dict = CompressionDict::bci_standard();
        
        let original = serde_json::json!({
            "maskRadius": 0.55,
            "maskFeather": 0.30,
            "decayGrain": 0.90
        });
        
        let compressed = dict.compress_json(&original).unwrap();
        assert_eq!(compressed, "{mR:0.55,mF:0.30,dG:0.90}");
        
        let decompressed = dict.decompress_to_json(&compressed).unwrap();
        assert_eq!(decompressed, original);
    }
}
