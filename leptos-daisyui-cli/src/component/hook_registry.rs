use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Metadata for a single hook.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookMetadata {
    /// Hook identifier (e.g. "modal").
    pub name: String,
    /// Human-readable name (e.g. "use_modal").
    pub display_name: String,
    /// Brief description.
    pub description: String,
    /// Public exports from this hook.
    pub exports: Vec<String>,
    /// Component names this hook depends on.
    pub depends_on: Vec<String>,
}

/// Registry of all available hooks.
#[derive(Debug, Serialize, Deserialize)]
pub struct HookRegistry {
    /// Schema version.
    pub version: String,
    /// All hooks.
    pub hooks: Vec<HookMetadata>,
}

impl HookRegistry {
    /// Load hook metadata from embedded JSON.
    pub fn load() -> Result<Self> {
        let json_data = include_str!("../../assets/hook_metadata.json");
        let registry: HookRegistry = serde_json::from_str(json_data)?;
        Ok(registry)
    }

    /// Get all hooks.
    pub fn all(&self) -> &[HookMetadata] {
        &self.hooks
    }

    /// Get hook by name.
    pub fn get(&self, name: &str) -> Option<&HookMetadata> {
        self.hooks.iter().find(|h| h.name == name)
    }
}
