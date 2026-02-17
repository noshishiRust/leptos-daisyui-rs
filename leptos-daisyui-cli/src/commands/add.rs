use crate::component::{ComponentGenerator, ComponentRegistry};
use crate::css::CssManager;
use crate::project::ProjectDetector;
use anyhow::{Context, Result};
use colored::Colorize;
use std::env;
use std::fs;

pub fn execute(components: Vec<String>, force: bool, _interactive: bool) -> Result<()> {
    let registry = ComponentRegistry::load()?;
    let current_dir = env::current_dir()?;
    let project = ProjectDetector::detect(&current_dir)
        .context("Failed to detect project. Run 'leptos-daisyui init' first.")?;

    let src_dir = ProjectDetector::find_main_binary_crate(&project)?;
    let generated_dir = src_dir.join("generated");

    // Ensure generated directory exists
    if !generated_dir.exists() {
        anyhow::bail!(
            "src/generated/ directory not found. Run '{}' first.",
            "leptos-daisyui init".cyan()
        );
    }

    // Process each component
    for component_name in &components {
        add_component(&registry, &generated_dir, &project, component_name, force)?;
    }

    println!();
    println!("{}", "✓ Components added successfully!".bold().green());
    println!();
    println!("Next steps:");
    println!("  1. Add to your lib.rs or main.rs: {}", "pub mod generated;".cyan());
    println!("  2. Import components: {}", "use crate::generated::Button;".cyan());
    println!("  3. Run {} to install CSS dependencies", "npm install".cyan());

    Ok(())
}

fn add_component(
    registry: &ComponentRegistry,
    generated_dir: &std::path::Path,
    project: &crate::project::ProjectStructure,
    component_name: &str,
    force: bool,
) -> Result<()> {
    // Get component metadata
    let metadata = registry
        .get(component_name)
        .ok_or_else(|| anyhow::anyhow!("Component '{}' not found", component_name))?;

    println!("Adding component: {}", metadata.display_name.bold());

    let component_dir = generated_dir.join(&metadata.name);

    // Check if already exists
    if component_dir.exists() && !force {
        println!(
            "  {} Component already exists. Use {} to overwrite.",
            "!".yellow(),
            "--force".bold()
        );
        return Ok(());
    }

    // Generate component files
    ComponentGenerator::write_component(&metadata.name, generated_dir)
        .with_context(|| format!("Failed to generate component '{}'", metadata.name))?;

    println!("  {} Created component files", "✓".green());

    // Update generated/mod.rs
    update_generated_mod(generated_dir, &metadata.name)?;
    println!("  {} Updated generated/mod.rs", "✓".green());

    // Add CSS classes to input.css
    add_css_classes(project, metadata)?;
    println!("  {} Added CSS classes to input.css", "✓".green());

    Ok(())
}

fn update_generated_mod(generated_dir: &std::path::Path, component_name: &str) -> Result<()> {
    let mod_rs = generated_dir.join("mod.rs");
    let mut content = fs::read_to_string(&mod_rs)
        .with_context(|| format!("Failed to read {}", mod_rs.display()))?;

    let mod_line = format!("pub mod {};", component_name);
    let use_line = format!("pub use {}::*;", component_name);

    // Check if already present
    if content.contains(&mod_line) {
        return Ok(());
    }

    // Add mod and use declarations
    content.push_str(&format!("\n{}\n{}\n", mod_line, use_line));

    fs::write(&mod_rs, content)
        .with_context(|| format!("Failed to write {}", mod_rs.display()))?;

    Ok(())
}

fn add_css_classes(
    project: &crate::project::ProjectStructure,
    metadata: &crate::component::ComponentMetadata,
) -> Result<()> {
    let root = project.root();
    let input_css = find_input_css(root)?;

    let mut content = fs::read_to_string(&input_css)
        .with_context(|| format!("Failed to read {}", input_css.display()))?;

    // Define markers
    let marker_start = "/* === leptos-daisyui-cli managed - do not edit manually === */";
    let marker_end = "/* === end leptos-daisyui-cli managed === */";

    // Component CSS entry
    let component_css = format!(
        "/* {} */\n@source inline(\"{}\");",
        metadata.display_name, metadata.css_classes
    );

    // Check if component already added
    if content.contains(&format!("/* {} */", metadata.display_name)) {
        return Ok(());
    }

    // Find managed section
    if let Some(start_idx) = content.find(marker_start) {
        if let Some(end_idx) = content.find(marker_end) {
            // Insert before end marker
            let insert_pos = end_idx;
            content.insert_str(insert_pos, &format!("{}\n", component_css));
        } else {
            // Start marker exists but no end marker - append after start
            let insert_pos = start_idx + marker_start.len();
            content.insert_str(insert_pos, &format!("\n{}\n{}", component_css, marker_end));
        }
    } else {
        // No managed section - create one at the end
        content.push_str(&format!(
            "\n{}\n{}\n{}\n",
            marker_start, component_css, marker_end
        ));
    }

    fs::write(&input_css, content)
        .with_context(|| format!("Failed to write {}", input_css.display()))?;

    Ok(())
}

fn find_input_css(root: &std::path::Path) -> Result<std::path::PathBuf> {
    CssManager::find_input_css_in(root)
}

