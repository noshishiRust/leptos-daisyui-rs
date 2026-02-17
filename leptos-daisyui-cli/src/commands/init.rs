use crate::project::ProjectDetector;
use crate::utils::Prompt;
use anyhow::{Context, Result};
use colored::Colorize;
use std::env;
use std::fs;
use std::path::PathBuf;

pub fn execute() -> Result<()> {
    println!("{}", "Initializing leptos-daisyui-rs...".bold().green());
    println!();

    // 1. Detect project structure
    let current_dir = env::current_dir()?;
    let project = ProjectDetector::detect(&current_dir)
        .context("Failed to detect project structure. Are you in a Rust project?")?;

    println!("{} Detected project at: {}", "✓".green(), project.root().display());
    println!();

    // 2. Check leptos dependency
    check_leptos_dependency(&project)?;

    // 3. Check trunk installation
    check_trunk_installation()?;

    // 4. Check tailwindcss setup
    check_tailwindcss_setup(&project)?;

    // 5. Initialize src/generated directory
    initialize_generated_directory(&project)?;

    println!();
    println!("{}", "✓ Initialization complete!".bold().green());
    println!();
    println!("Next steps:");
    println!("  1. Run {} to see available components", "leptos-daisyui list --all".cyan());
    println!("  2. Run {} to add components", "leptos-daisyui add <component>".cyan());

    Ok(())
}

fn check_leptos_dependency(project: &crate::project::ProjectStructure) -> Result<()> {
    let cargo_toml_path = match project {
        crate::project::ProjectStructure::Monorepo { cargo_toml, .. } => cargo_toml,
        crate::project::ProjectStructure::Workspace { workspace_toml, .. } => {
            // For workspace, we need to check the binary crate
            // For now, just check the workspace root
            workspace_toml
        }
    };

    let content = fs::read_to_string(cargo_toml_path)
        .with_context(|| format!("Failed to read {}", cargo_toml_path.display()))?;

    if content.contains("leptos") {
        println!("{} leptos dependency found", "✓".green());
    } else {
        println!("{} leptos not found in dependencies", "!".yellow());

        if Prompt::confirm("Add leptos to dependencies?", true)? {
            println!("{}", "  Please add leptos manually to your Cargo.toml:".dimmed());
            println!("{}", r#"    leptos = { version = "0.8", features = ["csr"] }"#.cyan());
        }
    }

    Ok(())
}

fn check_trunk_installation() -> Result<()> {
    if which::which("trunk").is_ok() {
        println!("{} trunk is installed", "✓".green());
    } else {
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
    }

    Ok(())
}

fn check_tailwindcss_setup(project: &crate::project::ProjectStructure) -> Result<()> {
    let root = project.root();
    let package_json = root.join("package.json");

    if package_json.exists() {
        let content = fs::read_to_string(&package_json)?;
        if content.contains("tailwindcss") && content.contains("daisyui") {
            println!("{} Tailwind CSS and daisyUI found in package.json", "✓".green());
            return Ok(());
        }
    }

    println!("{} Tailwind CSS not fully configured", "!".yellow());

    if Prompt::confirm("Set up Tailwind CSS and daisyUI?", true)? {
        setup_tailwindcss(root)?;
    }

    Ok(())
}

fn setup_tailwindcss(root: &std::path::Path) -> Result<()> {
    // Create package.json if it doesn't exist
    let package_json = root.join("package.json");
    if !package_json.exists() {
        let content = r#"{
  "devDependencies": {
    "@tailwindcss/cli": "^4.1.18",
    "tailwindcss": "^4.1.18",
    "daisyui": "^5.5.18"
  }
}
"#;
        fs::write(&package_json, content)?;
        println!("{} Created package.json", "✓".green());
    }

    // Create input.css if it doesn't exist
    let input_css = root.join("input.css");
    if !input_css.exists() {
        let content = r#"@import "tailwindcss";
@plugin "daisyui";
@source "../src/**/*.rs";

/* === leptos-daisyui-cli managed - do not edit manually === */
/* === end leptos-daisyui-cli managed === */
"#;
        fs::write(&input_css, content)?;
        println!("{} Created input.css", "✓".green());
    }

    println!();
    println!("Please run: {}", "npm install".cyan());

    Ok(())
}

fn initialize_generated_directory(project: &crate::project::ProjectStructure) -> Result<()> {
    let src_dir = ProjectDetector::find_main_binary_crate(project)?;
    let generated_dir = src_dir.join("generated");

    if generated_dir.exists() {
        println!("{} src/generated/ directory already exists", "✓".green());
        return Ok(());
    }

    // Create directory
    fs::create_dir_all(&generated_dir)
        .with_context(|| format!("Failed to create {}", generated_dir.display()))?;

    // Create mod.rs
    let mod_rs = generated_dir.join("mod.rs");
    let content = r#"//! Generated daisyUI components
//!
//! This module contains components generated by leptos-daisyui-cli.
//! Components can be added using: leptos-daisyui add <component>

"#;
    fs::write(&mod_rs, content)?;

    println!("{} Created src/generated/ directory", "✓".green());
    println!("{}", "  Don't forget to add to your lib.rs or main.rs:".dimmed());
    println!("{}", "    pub mod generated;".cyan());

    Ok(())
}

