use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Tier as used in manifests and routing spine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Tier {
    #[serde(rename = "T1-core")]
    T1Core,
    #[serde(rename = "T2-vault")]
    T2Vault,
    #[serde(rename = "T3-lab")]
    T3Lab,
}

/// Canonical repository names.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RepoName {
    #[serde(rename = "Horror.Place")]
    HorrorPlace,
    #[serde(rename = "Horror.Place-Orchestrator")]
    HorrorPlaceOrchestrator,
    #[serde(rename = "HorrorPlace-Constellation-Contracts")]
    HorrorPlaceConstellationContracts,
    #[serde(rename = "HorrorPlace-Codebase-of-Death")]
    HorrorPlaceCodebaseOfDeath,
    #[serde(rename = "HorrorPlace-Black-Archivum")]
    HorrorPlaceBlackArchivum,
    #[serde(rename = "HorrorPlace-Spectral-Foundry")]
    HorrorPlaceSpectralFoundry,
    #[serde(rename = "HorrorPlace-Atrocity-Seeds")]
    HorrorPlaceAtrocitySeeds,
    #[serde(rename = "HorrorPlace-Obscura-Nexus")]
    HorrorPlaceObscuraNexus,
    #[serde(rename = "HorrorPlace-Liminal-Continuum")]
    HorrorPlaceLiminalContinuum,
    #[serde(rename = "HorrorPlace-Process-Gods-Research")]
    HorrorPlaceProcessGodsResearch,
    #[serde(rename = "HorrorPlace-Redacted-Chronicles")]
    HorrorPlaceRedactedChronicles,
    #[serde(rename = "HorrorPlace-Neural-Resonance-Lab")]
    HorrorPlaceNeuralResonanceLab,
    #[serde(rename = "HorrorPlace-Dead-Ledger-Network")]
    HorrorPlaceDeadLedgerNetwork,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoPolicyOneFilePerRequest {
    pub enabled: bool,
    pub maxFilesPerRequest: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoPolicyNoRawNarrativeInTier {
    pub enabled: bool,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoPolicyRequireSchemaRef {
    pub enabled: bool,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoPolicyRequireTargetMetadata {
    pub enabled: bool,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoPolicyMaxFileSizeBytes {
    pub enabled: bool,
    pub maxFileSizeBytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoPolicyRequireRoutingSpineRef {
    pub enabled: bool,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RepoPolicies {
    #[serde(default)]
    pub POL001_OneFilePerRequest: Option<RepoPolicyOneFilePerRequest>,
    #[serde(default)]
    pub POL002_NoRawNarrativeInTier: Option<RepoPolicyNoRawNarrativeInTier>,
    #[serde(default)]
    pub POL003_RequireSchemaRef: Option<RepoPolicyRequireSchemaRef>,
    #[serde(default)]
    pub POL004_RequireTargetMetadata: Option<RepoPolicyRequireTargetMetadata>,
    #[serde(default)]
    pub POL005_MaxFileSizeBytes: Option<RepoPolicyMaxFileSizeBytes>,
    #[serde(default)]
    pub POL006_RequireRoutingSpineRef: Option<RepoPolicyRequireRoutingSpineRef>,
}

/// Manifest model (subset) matching repo-manifest-v1.json.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoManifest {
    pub schemaVersion: String,
    pub schemaRef: String,
    pub repoName: RepoName,
    pub tier: Tier,
    pub kind: String,
    pub description: String,
    pub visibility: String,
    pub implicitDeny: bool,
    #[serde(default)]
    pub routingSpineRef: Option<String>,
    #[serde(default)]
    pub allowedObjectKinds: Vec<String>,
    #[serde(default)]
    pub defaultPaths: HashMap<String, String>,
    #[serde(default)]
    pub schemaWhitelist: Vec<String>,
    #[serde(default)]
    pub policies: RepoPolicies,
    // ciChecks and authoringHints omitted here for brevity; they are not needed by the validator core
}

/// Routing spine math constraint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathConstraint {
    pub id: String,
    pub description: String,
    #[serde(default)]
    pub appliesTo: Vec<String>,
}

/// Single route entry from the routing spine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteEntry {
    pub id: String,
    pub tier: Tier,
    pub repo: RepoName,
    pub schemaRef: String,
    pub aiAuthoringKind: String,
    pub defaultTargetPath: String,
    pub registryRef: String,
    #[serde(default)]
    pub invariants: Vec<String>,
    #[serde(default)]
    pub metrics: Vec<String>,
    #[serde(default)]
    pub constraints: Vec<String>,
    pub phase: String,
    pub additionalPropertiesRequired: bool,
}

/// Routing rules for a single objectKind.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectKindEntry {
    pub name: String,
    pub description: String,
    pub allowedTiers: Vec<Tier>,
    pub routes: Vec<RouteEntry>,
}

/// Top-level routing spine structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingSpine {
    pub routingSpineVersion: String,
    pub schemaRef: String,
    pub updatedAt: String,
    #[serde(default)]
    pub mathConstraints: Vec<MathConstraint>,
    pub objectKinds: Vec<ObjectKindEntry>,
}

/// Error codes for routing validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoutingErrorCode {
    ERR_ROUTE_CONFLICT,
    ERR_ROUTE_NOT_FOUND,
    ERR_ROUTE_TIER_NOT_ALLOWED,
    ERR_MANIFEST_IMPLICIT_DENY,
    ERR_MANIFEST_OBJECT_KIND_DENY,
    ERR_SOVEREIGNTY_VIOLATION,
    ERR_CONSTRAINT_MISSING,
}

/// Structured error returned by validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingValidationError {
    pub code: RoutingErrorCode,
    pub message: String,
    pub objectKind: Option<String>,
    pub tier: Option<Tier>,
    pub repo: Option<RepoName>,
    pub routeId: Option<String>,
}

/// Decision returned by resolve_route.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteDecision {
    pub objectKind: String,
    pub tier: Tier,
    pub repo: RepoName,
    pub schemaRef: String,
    pub defaultTargetPath: String,
    pub invariants: Vec<String>,
    pub metrics: Vec<String>,
    pub constraints: Vec<String>,
    pub phase: String,
}

impl RoutingSpine {
    /// Resolve a unique route for (objectKind, tier).
    pub fn resolve_route(
        &self,
        object_kind: &str,
        tier: Tier,
    ) -> Result<RouteDecision, RoutingValidationError> {
        let mut matches: Vec<(&ObjectKindEntry, &RouteEntry)> = Vec::new();

        for ok in &self.objectKinds {
            if ok.name != object_kind {
                continue;
            }
            if !ok.allowedTiers.contains(&tier) {
                continue;
            }
            for route in &ok.routes {
                if route.tier == tier {
                    matches.push((ok, route));
                }
            }
        }

        if matches.is_empty() {
            return Err(RoutingValidationError {
                code: RoutingErrorCode::ERR_ROUTE_NOT_FOUND,
                message: format!("No route found for objectKind={} tier={:?}", object_kind, tier),
                objectKind: Some(object_kind.to_string()),
                tier: Some(tier),
                repo: None,
                routeId: None,
            });
        }

        if matches.len() > 1 {
            return Err(RoutingValidationError {
                code: RoutingErrorCode::ERR_ROUTE_CONFLICT,
                message: format!(
                    "Multiple routes found for objectKind={} tier={:?}",
                    object_kind, tier
                ),
                objectKind: Some(object_kind.to_string()),
                tier: Some(tier),
                repo: None,
                routeId: None,
            });
        }

        let (_ok, route) = matches[0];

        Ok(RouteDecision {
            objectKind: object_kind.to_string(),
            tier,
            repo: route.repo.clone(),
            schemaRef: route.schemaRef.clone(),
            defaultTargetPath: route.defaultTargetPath.clone(),
            invariants: route.invariants.clone(),
            metrics: route.metrics.clone(),
            constraints: route.constraints.clone(),
            phase: route.phase.clone(),
        })
    }
}

/// Validate the entire routing system: manifests + routing spine.
///
/// This is a pure function suitable for WASM: feed in JSON strings, get back a JSON array of errors.
pub fn validate_routing_system(
    manifests: &[RepoManifest],
    spine: &RoutingSpine,
) -> Vec<RoutingValidationError> {
    let mut errors: Vec<RoutingValidationError> = Vec::new();

    // Index manifests by repoName for quick lookup.
    let mut manifest_by_repo: HashMap<RepoName, &RepoManifest> = HashMap::new();
    for m in manifests {
        manifest_by_repo.insert(m.repoName.clone(), m);
    }

    // 1. Check uniqueness: for each (objectKind, tier) there must be at most one route.
    let mut route_keys: HashMap<(String, Tier), Vec<String>> = HashMap::new();

    for ok in &spine.objectKinds {
        for route in &ok.routes {
            let key = (ok.name.clone(), route.tier);
            route_keys.entry(key).or_default().push(route.id.clone());
        }
    }

    for ((object_kind, tier), ids) in route_keys.iter() {
        if ids.len() > 1 {
            errors.push(RoutingValidationError {
                code: RoutingErrorCode::ERR_ROUTE_CONFLICT,
                message: format!(
                    "Route conflict for objectKind={} tier={:?}, routeIds={:?}",
                    object_kind, tier, ids
                ),
                objectKind: Some(object_kind.clone()),
                tier: Some(*tier),
                repo: None,
                routeId: None,
            });
        }
    }

    // 2. Check that each route's repo has a manifest and that manifests allow that objectKind.
    //    Also enforce implicitDeny semantics and allowedObjectKinds.
    for ok in &spine.objectKinds {
        for route in &ok.routes {
            let repo = &route.repo;
            let manifest = match manifest_by_repo.get(repo) {
                Some(m) => *m,
                None => {
                    errors.push(RoutingValidationError {
                        code: RoutingErrorCode::ERR_SOVEREIGNTY_VIOLATION,
                        message: format!(
                            "Route {} targets repo {:?} but no manifest was provided.",
                            route.id, repo
                        ),
                        objectKind: Some(ok.name.clone()),
                        tier: Some(route.tier),
                        repo: Some(repo.clone()),
                        routeId: Some(route.id.clone()),
                    });
                    continue;
                }
            };

            // Implicit deny: if manifest.implicitDeny == true, then repo must explicitly list this objectKind.
            if manifest.implicitDeny {
                if !manifest.allowedObjectKinds.contains(&ok.name) {
                    errors.push(RoutingValidationError {
                        code: RoutingErrorCode::ERR_MANIFEST_IMPLICIT_DENY,
                        message: format!(
                            "Route {} assigns objectKind={} to repo {:?} but manifest has implicitDeny=true and does not list this object kind.",
                            route.id, ok.name, repo
                        ),
                        objectKind: Some(ok.name.clone()),
                        tier: Some(route.tier),
                        repo: Some(repo.clone()),
                        routeId: Some(route.id.clone()),
                    });
                }
            } else {
                // Even without implicitDeny, we still prefer explicit allowedObjectKinds.
                if !manifest.allowedObjectKinds.is_empty()
                    && !manifest.allowedObjectKinds.contains(&ok.name)
                {
                    errors.push(RoutingValidationError {
                        code: RoutingErrorCode::ERR_MANIFEST_OBJECT_KIND_DENY,
                        message: format!(
                            "Route {} assigns objectKind={} to repo {:?} but manifest.allowedObjectKinds does not include it.",
                            route.id, ok.name, repo
                        ),
                        objectKind: Some(ok.name.clone()),
                        tier: Some(route.tier),
                        repo: Some(repo.clone()),
                        routeId: Some(route.id.clone()),
                    });
                }
            }

            // Tiers must match: route tier and manifest tier must be compatible.
            if manifest.tier != route.tier {
                errors.push(RoutingValidationError {
                    code: RoutingErrorCode::ERR_ROUTE_TIER_NOT_ALLOWED,
                    message: format!(
                        "Route {} uses tier={:?} but manifest for repo {:?} is tier={:?}.",
                        route.id, route.tier, repo, manifest.tier
                    ),
                    objectKind: Some(ok.name.clone()),
                    tier: Some(route.tier),
                    repo: Some(repo.clone()),
                    routeId: Some(route.id.clone()),
                });
            }
        }
    }

    // 3. Check that every constraint id referenced in routes exists in mathConstraints.
    let known_constraints: HashSet<String> = spine
        .mathConstraints
        .iter()
        .map(|c| c.id.clone())
        .collect();

    for ok in &spine.objectKinds {
        for route in &ok.routes {
            for c_id in &route.constraints {
                if !known_constraints.contains(c_id) {
                    errors.push(RoutingValidationError {
                        code: RoutingErrorCode::ERR_CONSTRAINT_MISSING,
                        message: format!(
                            "Route {} for objectKind={} tier={:?} references unknown constraint id={}",
                            route.id, ok.name, route.tier, c_id
                        ),
                        objectKind: Some(ok.name.clone()),
                        tier: Some(route.tier),
                        repo: Some(route.repo.clone()),
                        routeId: Some(route.id.clone()),
                    });
                }
            }
        }
    }

    errors
}
