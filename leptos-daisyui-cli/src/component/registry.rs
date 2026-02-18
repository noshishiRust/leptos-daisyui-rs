use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentMetadata {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub has_style_enums: bool,
    pub components: Vec<String>,
    pub css_classes: String,
    pub daisyui_url: String,
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComponentRegistry {
    pub version: String,
    pub components: Vec<ComponentMetadata>,
    pub categories: Vec<String>,
}

impl ComponentRegistry {
    /// Load component metadata from embedded JSON
    pub fn load() -> Result<Self> {
        let json_data = include_str!("../../assets/component_metadata.json");
        let registry: ComponentRegistry = serde_json::from_str(json_data)?;
        Ok(registry)
    }

    /// Get all components
    pub fn all(&self) -> &[ComponentMetadata] {
        &self.components
    }

    /// Get component by name
    pub fn get(&self, name: &str) -> Option<&ComponentMetadata> {
        self.components.iter().find(|c| c.name == name)
    }

    /// Get components by category
    pub fn get_by_category(&self, category: &str) -> Vec<&ComponentMetadata> {
        self.components
            .iter()
            .filter(|c| c.category == category)
            .collect()
    }

    /// Get all category names
    #[allow(dead_code)]
    pub fn categories(&self) -> &[String] {
        &self.categories
    }
}
