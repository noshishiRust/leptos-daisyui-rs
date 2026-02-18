use anyhow::{Context, Result};
use rust_embed::Embed;
use std::path::Path;

#[derive(Embed)]
#[folder = "../leptos-daisyui-rs/src/components"]
struct ComponentSources;

pub struct ComponentFiles {
    pub mod_rs: String,
    pub component_rs: String,
    pub style_rs: Option<String>,
}

pub struct ComponentGenerator;

impl ComponentGenerator {
    /// Get component source files from embedded resources
    pub fn get_component_files(component_name: &str) -> Result<ComponentFiles> {
        let mod_path = format!("{}/mod.rs", component_name);
        let component_path = format!("{}/component.rs", component_name);
        let style_path = format!("{}/style.rs", component_name);

        let mod_rs = Self::read_embedded_file(&mod_path)?;
        let component_rs = Self::read_embedded_file(&component_path)?;
        let style_rs = Self::read_embedded_file(&style_path).ok();

        Ok(ComponentFiles {
            mod_rs,
            component_rs,
            style_rs,
        })
    }

    /// Read an embedded file and convert to String
    fn read_embedded_file(path: &str) -> Result<String> {
        let file = ComponentSources::get(path)
            .with_context(|| format!("Component file not found: {}", path))?;

        let content = std::str::from_utf8(file.data.as_ref())
            .with_context(|| format!("Invalid UTF-8 in file: {}", path))?;

        Ok(content.to_string())
    }

    /// Write component files to target directory
    pub fn write_component(
        component_name: &str,
        target_dir: &Path,
    ) -> Result<()> {
        let files = Self::get_component_files(component_name)?;
        let component_dir = target_dir.join(component_name);

        // Create component directory
        std::fs::create_dir_all(&component_dir)
            .with_context(|| format!("Failed to create directory: {}", component_dir.display()))?;

        // Write mod.rs
        std::fs::write(component_dir.join("mod.rs"), files.mod_rs)?;

        // Write component.rs (no path fix needed - macro is exported at crate root)
        std::fs::write(component_dir.join("component.rs"), files.component_rs)?;

        // Write style.rs if present
        if let Some(style_rs) = files.style_rs {
            std::fs::write(component_dir.join("style.rs"), style_rs)?;
        }

        Ok(())
    }
}
