#![warn(missing_docs, rust_2018_idioms)]
#![deny(unsafe_code)]

//! CHATDIRECTOR — Schema-driven authoring compiler for Horror$Place
//!
//! Core Rust crate for AI-chat and tooling to generate, validate, and apply
//! constellation artifacts against the schema spine and repo manifests.

pub mod config;
pub mod errors;
pub mod model;
pub mod spine;
pub mod validate;
pub mod generate;
pub mod cli;

#[cfg(feature = "manifests")]
pub mod manifests;

#[cfg(feature = "http-service")]
pub mod service;

use once_cell::sync::OnceCell;
use std::collections::HashMap;

use crate::config::Config;
use crate::errors::ChatDirectorError;
use crate::model::{
    manifest_types::{PolicyChecklist, RepoManifest, Tier},
    spine_types::{SchemaSpine},
    CapabilityCatalog, InvariantSummary, MetricSummary, PhaseSummary, RepoSummary,
};
use crate::spine::SpineIndex;
use crate::validate::ValidationResult;

/// Core façade for constellation authoring operations.
///
/// Methods consult the schema spine and manifests; governance rules are not
/// hardcoded, only interpreted from data.
#[derive(Clone)]
pub struct ChatDirector {
    config: Config,
    spine: SpineIndex,
    manifests: Vec<RepoManifest>,
    schema_cache: OnceCell<HashMap<String, jsonschema::Validator>>,
}

impl ChatDirector {
    /// Load the constellation environment using an explicit `Config`.
    pub fn load_environment(config: Config) -> Result<Self, ChatDirectorError> {
        let spine = SpineIndex::load_from_root(config.root())?;
        let manifests = config.manifests().to_vec();
        Ok(ChatDirector {
            config,
            spine,
            manifests,
            schema_cache: OnceCell::new(),
        })
    }

    /// Build a director from preloaded spine and manifests.
    pub fn from_spine_and_manifests(
        config: Config,
        spine: SpineIndex,
        manifests: Vec<RepoManifest>,
    ) -> Self {
        ChatDirector {
            config,
            spine,
            manifests,
            schema_cache: OnceCell::new(),
        }
    }

    /// Access the underlying schema spine.
    pub fn spine_schema(&self) -> &SchemaSpine {
        &self.spine.inner()
    }

    /// Access the loaded repo manifests.
    pub fn repo_manifests(&self) -> &[RepoManifest] {
        &self.manifests
    }

    /// Return a capability catalog for AI pre-flight discovery.
    pub fn capability_catalog(&self) -> CapabilityCatalog {
        let invariants = self
            .spine_schema()
            .invariants
            .iter()
            .map(|inv| InvariantSummary {
                name: inv.name.clone(),
                description: inv.description.clone(),
                min: inv.min,
                max: inv.max,
            })
            .collect();

        let metrics = self
            .spine_schema()
            .metrics
            .iter()
            .map(|m| MetricSummary {
                name: m.name.clone(),
                description: m.description.clone(),
                target_min: m.target_min,
                target_max: m.target_max,
            })
            .collect();

        let phases = vec![
            PhaseSummary {
                id: crate::spine::Phase::Schema0.id(),
                name: crate::spine::Phase::Schema0.name().to_string(),
                description: crate::spine::Phase::Schema0.description().to_string(),
            },
            PhaseSummary {
                id: crate::spine::Phase::Registry1.id(),
                name: crate::spine::Phase::Registry1.name().to_string(),
                description: crate::spine::Phase::Registry1.description().to_string(),
            },
            PhaseSummary {
                id: crate::spine::Phase::Bundles2.id(),
                name: crate::spine::Phase::Bundles2.name().to_string(),
                description: crate::spine::Phase::Bundles2.description().to_string(),
            },
            PhaseSummary {
                id: crate::spine::Phase::LuaPolicy3.id(),
                name: crate::spine::Phase::LuaPolicy3.name().to_string(),
                description: crate::spine::Phase::LuaPolicy3.description().to_string(),
            },
            PhaseSummary {
                id: crate::spine::Phase::Adapters4.id(),
                name: crate::spine::Phase::Adapters4.name().to_string(),
                description: crate::spine::Phase::Adapters4.description().to_string(),
            },
        ];

        let available_repos = self
            .manifests
            .iter()
            .map(|m| RepoSummary {
                name: m.repo_name.clone(),
                tier: m.tier.as_str().to_string(),
                path: String::new(),
            })
            .collect();

        let spine_version = Some(self.spine.version().to_string());

        let available_object_kinds = self
            .spine_schema()
            .contract_families
            .iter()
            .map(|f| f.object_kind.clone())
            .collect();

        CapabilityCatalog {
            spine_version,
            available_object_kinds,
            available_repos,
            invariants,
            metrics,
            phases,
        }
    }

    /// Build a policy checklist for a given repo.
    pub fn policy_checklist_for_repo(&self, repo_name: &str) -> Option<PolicyChecklist> {
        let manifest = self.manifests.iter().find(|m| m.repo_name == repo_name)?;
        let mut checklist = PolicyChecklist::new(repo_name, &manifest.tier);
        for rule in &manifest.ai_authoring_rules {
            checklist.add_item(&rule.rule, &rule.description);
        }
        Some(checklist)
    }

    /// Validate a response; high-level wrapper around validate module.
    pub fn validate_response(
        &self,
        request: &crate::model::request_types::AiAuthoringRequest,
        response: &crate::model::response_types::AiAuthoringResponse,
    ) -> Result<ValidationResult, ChatDirectorError> {
        validate::run_full_pipeline(&self.config, &self.spine, &self.manifests, request, response)
            .map_err(ChatDirectorError::from)
    }
}
