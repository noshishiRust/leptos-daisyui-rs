use crate::utils::Prompt;
use anyhow::Result;
use colored::Colorize;
use std::fs;
use std::path::Path;

pub struct TailwindInstaller;

impl TailwindInstaller {
    /// Check if Tailwind CSS and daisyUI are installed
    pub fn check_installed(project_root: &Path) -> bool {
        let package_json = project_root.join("package.json");

        if package_json.exists() {
            if let Ok(content) = fs::read_to_string(&package_json) {
                return content.contains("tailwindcss") && content.contains("daisyui");
            }
        }

        false
    }

    /// Set up Tailwind CSS and daisyUI
    pub fn setup(project_root: &Path) -> Result<()> {
        println!("{} Tailwind CSS not fully configured", "!".yellow());

        if !Prompt::confirm("Set up Tailwind CSS and daisyUI?", true)? {
            return Ok(());
        }

        // Create package.json if it doesn't exist
        let package_json = project_root.join("package.json");
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
        let input_css = project_root.join("input.css");
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
}
