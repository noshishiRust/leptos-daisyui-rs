use anyhow::{Context, Result};
use rust_embed::Embed;
use std::path::Path;

#[derive(Embed)]
#[folder = "../leptos-daisyui-rs/src/hooks"]
struct HookSources;

pub struct HookGenerator;

impl HookGenerator {
    /// Get hook source from embedded resources.
    pub fn get_hook_file(hook_name: &str) -> Result<String> {
        let path = format!("{}.rs", hook_name);
        let file = HookSources::get(&path)
            .with_context(|| format!("Hook file not found: {}", path))?;
        let content = std::str::from_utf8(file.data.as_ref())
            .with_context(|| format!("Invalid UTF-8 in file: {}", path))?;
        Ok(content.to_string())
    }

    /// Write hook file to target directory.
    pub fn write_hook(hook_name: &str, target_dir: &Path) -> Result<()> {
        let content = Self::get_hook_file(hook_name)?;
        let hooks_dir = target_dir.join("hooks");

        std::fs::create_dir_all(&hooks_dir)
            .with_context(|| format!("Failed to create directory: {}", hooks_dir.display()))?;

        std::fs::write(hooks_dir.join(format!("{}.rs", hook_name)), content)?;

        Ok(())
    }
}
