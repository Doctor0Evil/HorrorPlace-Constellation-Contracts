use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::errors::ChatDirectorError;
use crate::model::spine_types::{DefaultBands, ObjectKindProfile, SchemaSpine};

/// Phase identifiers for the constellation lifecycle.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Phase {
    Schema0,
    Registry1,
    Bundles2,
    LuaPolicy3,
    Adapters4,
}

impl Phase {
    pub fn id(self) -> u8 {
        match self {
            Phase::Schema0 => 0,
            Phase::Registry1 => 1,
            Phase::Bundles2 => 2,
            Phase::LuaPolicy3 => 3,
            Phase::Adapters4 => 4,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Phase::Schema0 => "Schema0",
            Phase::Registry1 => "Registry1",
            Phase::Bundles2 => "Bundles2",
            Phase::LuaPolicy3 => "LuaPolicy3",
            Phase::Adapters4 => "Adapters4",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Phase::Schema0 => "Core schemas and invariants/metrics spine definition.",
            Phase::Registry1 => "Registry entries and region/seed/mood/event indices.",
            Phase::Bundles2 => "Higher-order bundles and choreography contracts.",
            Phase::LuaPolicy3 => "Lua policy modules bound to contracts and metrics.",
            Phase::Adapters4 => "Engine adapters and external pipeline integration.",
        }
    }
}

/// Wrapper around the loaded schema spine with helpers for AI tooling.
#[derive(Debug, Clone)]
pub struct SpineIndex {
    root: PathBuf,
    schema_spine: SchemaSpine,
}

impl SpineIndex {
    /// Load a schema spine index from the given constellation root directory.
    pub fn load_from_root<P: AsRef<Path>>(root: P) -> Result<Self, ChatDirectorError> {
        let root_path = root.as_ref().to_path_buf();
        let spine_path = root_path
            .join("schemas")
            .join("core")
            .join("schema-spine-index-v1.json");

        let data = fs::read_to_string(&spine_path)
            .map_err(|e| ChatDirectorError::Io(spine_path.clone(), e))?;

        let schema_spine: SchemaSpine =
            serde_json::from_str(&data).map_err(ChatDirectorError::InvalidSpine)?;

        Ok(SpineIndex {
            root: root_path,
            schema_spine,
        })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn version(&self) -> &str {
        &self.schema_spine.version
    }

    pub fn inner(&self) -> &SchemaSpine {
        &self.schema_spine
    }

    pub fn describe_object_kind(&self, kind: &str) -> Option<ObjectKindProfile> {
        self.schema_spine.describe_object_kind(kind)
    }

    pub fn safe_defaults(&self, object_kind: &str, tier: &str) -> DefaultBands {
        self.schema_spine.safe_defaults(object_kind, tier)
    }
}
