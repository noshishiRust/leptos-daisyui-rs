use crate::project::ProjectStructure;
use crate::utils::Prompt;
use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;

pub struct LeptosInstaller;

impl LeptosInstaller {
    /// Check if leptos is installed in project dependencies
    pub fn check_installed(project: &ProjectStructure) -> Result<bool> {
        let cargo_toml_path = match project {
            ProjectStructure::Monorepo { cargo_toml, .. } => cargo_toml,
            ProjectStructure::Workspace { workspace_toml, .. } => {
                // For workspace, check the workspace root
                workspace_toml
            }
        };

        let content = fs::read_to_string(cargo_toml_path)
            .with_context(|| format!("Failed to read {}", cargo_toml_path.display()))?;

        Ok(content.contains("leptos"))
    }

    /// Install leptos dependency (prompts user for manual installation)
    pub fn install() -> Result<()> {
        println!("{} leptos not found in dependencies", "!".yellow());

        if Prompt::confirm("Add leptos to dependencies?", true)? {
            println!("{}", "  Please add leptos manually to your Cargo.toml:".dimmed());
            println!("{}", r#"    leptos = { version = "0.8", features = ["csr"] }"#.cyan());
        }

        Ok(())
    }
}
