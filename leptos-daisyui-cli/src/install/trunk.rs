use crate::utils::Prompt;
use anyhow::{Context, Result};
use colored::Colorize;

pub struct TrunkInstaller;

impl TrunkInstaller {
    /// Check if trunk is installed
    pub fn check_installed() -> bool {
        which::which("trunk").is_ok()
    }

    /// Install trunk via cargo install (with user confirmation)
    pub fn install() -> Result<()> {
        println!("{} trunk not installed", "!".yellow());

        if Prompt::confirm("Install trunk via cargo install?", true)? {
            println!("Installing trunk...");
            let output = std::process::Command::new("cargo")
                .args(&["install", "trunk"])
                .output()
                .context("Failed to run cargo install trunk")?;

            if output.status.success() {
                println!("{} trunk installed successfully", "✓".green());
            } else {
                println!("{} Failed to install trunk", "✗".red());
                println!("Please install manually: {}", "cargo install trunk".cyan());
            }
        }

        Ok(())
    }
}
