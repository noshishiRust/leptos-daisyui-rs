use anyhow::Result;

pub struct ComponentRegistry;

impl ComponentRegistry {
    pub fn load() -> Result<Self> {
        // TODO: Load from component_metadata.json
        Ok(ComponentRegistry)
    }
}
