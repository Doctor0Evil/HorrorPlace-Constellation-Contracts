#[cfg(feature = "wasm")]
mod wasm {
    use super::*;
    use wasm_bindgen::prelude::*;

    // Optional: smaller allocator for wasm.
    #[global_allocator]
    static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

    /// Validate routing system given JSON strings.
    ///
    /// `manifests_json` is a JSON array of repo-manifest objects.
    /// `spine_json` is a single routing spine object.
    #[wasm_bindgen]
    pub fn validate_routing_system_json(
        manifests_json: &str,
        spine_json: &str,
    ) -> Result<String, JsValue> {
        let manifests: Vec<RepoManifest> =
            serde_json::from_str(manifests_json).map_err(|e| JsValue::from_str(&e.to_string()))?;
        let spine: RoutingSpine =
            serde_json::from_str(spine_json).map_err(|e| JsValue::from_str(&e.to_string()))?;

        let errors = super::validate_routing_system(&manifests, &spine);
        serde_json::to_string(&errors).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Resolve a single route (objectKind, tier) and return a JSON RouteDecision.
    #[wasm_bindgen]
    pub fn resolve_route_json(object_kind: &str, tier: &str, spine_json: &str) -> Result<String, JsValue> {
        let spine: RoutingSpine =
            serde_json::from_str(spine_json).map_err(|e| JsValue::from_str(&e.to_string()))?;

        let tier_parsed = match tier {
            "T1-core" => Tier::T1Core,
            "T2-vault" => Tier::T2Vault,
            "T3-lab" => Tier::T3Lab,
            other => {
                return Err(JsValue::from_str(&format!(
                    "Unknown tier string: {}",
                    other
                )))
            }
        };

        let decision = spine
            .resolve_route(object_kind, tier_parsed)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;

        serde_json::to_string(&decision).map_err(|e| JsValue::from_str(&e.to_string()))
    }
}
