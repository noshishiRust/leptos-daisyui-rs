use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Managed section markers for leptos-daisyui-cli CSS directives
const MARKER_START: &str = "/* === leptos-daisyui-cli managed - do not edit manually === */";
const MARKER_END: &str = "/* === end leptos-daisyui-cli managed === */";

#[allow(dead_code)]
pub struct CssManager {
    project_root: PathBuf,
}

impl CssManager {
    #[allow(dead_code)]
    pub fn new(project_root: PathBuf) -> Self {
        Self { project_root }
    }

    /// Find input.css file in common locations
    #[allow(dead_code)]
    pub fn find_input_css(&self) -> Result<PathBuf> {
        Self::find_input_css_in(&self.project_root)
    }

    /// Find input.css file in common locations (static method)
    pub fn find_input_css_in(root: &Path) -> Result<PathBuf> {
        let candidates = vec![
            root.join("input.css"),
            root.join("src/input.css"),
            root.join("styles/input.css"),
            root.join("assets/input.css"),
        ];

        for candidate in candidates {
            if candidate.exists() {
                return Ok(candidate);
            }
        }

        // If not found, use default location
        Ok(root.join("input.css"))
    }

    /// Check if a specific component's CSS is already in the managed section.
    pub fn has_component(input_css: &Path, display_name: &str) -> Result<bool> {
        let content = fs::read_to_string(input_css)
            .with_context(|| format!("Failed to read {}", input_css.display()))?;
        Ok(content.contains(&format!("/* {} */", display_name)))
    }

    /// Append a CSS directive for a component to the managed section of input.css.
    ///
    /// Creates the managed section if it doesn't exist. Skips if the component
    /// is already present.
    pub fn append_directive(input_css: &Path, display_name: &str, css_classes: &str) -> Result<()> {
        // Skip if already present
        if Self::has_component(input_css, display_name)? {
            return Ok(());
        }

        let mut content = fs::read_to_string(input_css)
            .with_context(|| format!("Failed to read {}", input_css.display()))?;

        let component_css = format!(
            "/* {} */\n@source inline(\"{}\");",
            display_name, css_classes
        );

        if let Some(start_idx) = content.find(MARKER_START) {
            if let Some(end_idx) = content.find(MARKER_END) {
                // Insert before end marker
                content.insert_str(end_idx, &format!("{}\n", component_css));
            } else {
                // Start marker exists but no end marker - append after start
                let insert_pos = start_idx + MARKER_START.len();
                content.insert_str(insert_pos, &format!("\n{}\n{}", component_css, MARKER_END));
            }
        } else {
            // No managed section - create one at the end
            content.push_str(&format!(
                "\n{}\n{}\n{}\n",
                MARKER_START, component_css, MARKER_END
            ));
        }

        fs::write(input_css, content)
            .with_context(|| format!("Failed to write {}", input_css.display()))?;

        Ok(())
    }
}
