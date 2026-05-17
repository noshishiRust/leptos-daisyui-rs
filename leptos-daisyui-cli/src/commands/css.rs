use crate::component::ComponentRegistry;
use crate::css::CssManager;
use crate::project::ProjectDetector;
use anyhow::{Context, Result};
use colored::Colorize;
use std::env;

/// Manage CSS `@source inline()` directives for daisyUI components.
///
/// This command modifies `input.css` (or equivalent) to include Tailwind CSS v4
/// `@source inline()` directives for the specified components. These directives
/// tell Tailwind CSS which daisyUI classes to generate during build.
///
/// Note: This command only needs the component metadata (CSS class strings),
/// not the full component source files. It is lighter than the `add` command.
pub fn execute(components: Vec<String>, all: bool) -> Result<()> {
    let registry = ComponentRegistry::load()?;
    let current_dir = env::current_dir()?;
    let project = ProjectDetector::detect(&current_dir)
        .context("Failed to detect project. Run 'leptos-daisyui init' first.")?;

    let input_css = CssManager::find_input_css_in(project.root())?;

    if all {
        add_all_components(&registry, &input_css)?;
    } else if components.is_empty() {
        anyhow::bail!(
            "Specify component names or use {} for all components",
            "--all".bold()
        );
    } else {
        for name in &components {
            add_single_component(&registry, &input_css, name)?;
        }
    }

    println!();
    println!(
        "{} CSS directives updated in {}",
        "✓".green(),
        input_css.display()
    );

    Ok(())
}

fn add_all_components(registry: &ComponentRegistry, input_css: &std::path::Path) -> Result<()> {
    let mut added = 0;
    let mut skipped = 0;

    for metadata in registry.all() {
        if CssManager::has_component(input_css, &metadata.display_name)? {
            skipped += 1;
            continue;
        }
        CssManager::append_directive(input_css, &metadata.display_name, &metadata.css_classes)?;
        added += 1;
    }

    if added > 0 {
        println!(
            "{} Added CSS for {} components",
            "✓".green(),
            added.to_string().bold()
        );
    }
    if skipped > 0 {
        println!(
            "{} Skipped {} already present",
            "-".yellow(),
            skipped.to_string().bold()
        );
    }

    Ok(())
}

fn add_single_component(
    registry: &ComponentRegistry,
    input_css: &std::path::Path,
    component_name: &str,
) -> Result<()> {
    let metadata = registry
        .get(component_name)
        .ok_or_else(|| anyhow::anyhow!("Component '{}' not found", component_name))?;

    if CssManager::has_component(input_css, &metadata.display_name)? {
        println!(
            "  {} CSS for {} already present",
            "-".yellow(),
            metadata.display_name.bold()
        );
        return Ok(());
    }

    CssManager::append_directive(input_css, &metadata.display_name, &metadata.css_classes)?;
    println!(
        "  {} Added CSS for {}",
        "✓".green(),
        metadata.display_name.bold()
    );

    Ok(())
}
