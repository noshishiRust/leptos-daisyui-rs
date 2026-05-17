use crate::component::{ComponentGenerator, ComponentRegistry, HookGenerator, HookRegistry};
use crate::css::CssManager;
use crate::project::ProjectDetector;
use anyhow::{Context, Result};
use colored::Colorize;
use std::env;
use std::fs;

pub fn execute(components: Vec<String>, force: bool, _interactive: bool) -> Result<()> {
    let registry = ComponentRegistry::load()?;
    let hook_registry = HookRegistry::load()?;
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

    // Process each name (could be component or hook)
    for name in &components {
        // Try component first, then hook
        if registry.get(name).is_some() {
            add_component(&registry, &generated_dir, &project, name, force)?;
        } else if hook_registry.get(name).is_some() {
            add_hook(
                &hook_registry,
                &registry,
                &generated_dir,
                &project,
                name,
                force,
            )?;
        } else {
            anyhow::bail!(
                "'{}' not found as a component or hook. Run '{}' to see available options.",
                name,
                "leptos-daisyui list --all".cyan()
            );
        }
    }

    println!();
    println!("{}", "✓ Added successfully!".bold().green());
    println!();
    println!("Next steps:");
    println!(
        "  1. Add to your lib.rs or main.rs: {}",
        "pub mod generated;".cyan()
    );
    println!(
        "  2. Import: {}",
        "use crate::generated::Button; // component".cyan()
    );
    println!(
        "  3. Run {} to install CSS dependencies",
        "npm install".cyan()
    );

    Ok(())
}

fn add_component(
    registry: &ComponentRegistry,
    generated_dir: &std::path::Path,
    project: &crate::project::ProjectStructure,
    component_name: &str,
    force: bool,
) -> Result<()> {
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

fn add_hook(
    hook_registry: &HookRegistry,
    component_registry: &ComponentRegistry,
    generated_dir: &std::path::Path,
    project: &crate::project::ProjectStructure,
    hook_name: &str,
    force: bool,
) -> Result<()> {
    let metadata = hook_registry
        .get(hook_name)
        .ok_or_else(|| anyhow::anyhow!("Hook '{}' not found", hook_name))?;

    println!("Adding hook: {}", metadata.display_name.bold());

    // Check if hook file already exists
    let hook_file = generated_dir.join("hooks").join(format!("{}.rs", metadata.name));
    if hook_file.exists() && !force {
        println!(
            "  {} Hook already exists. Use {} to overwrite.",
            "!".yellow(),
            "--force".bold()
        );
        return Ok(());
    }

    // Auto-add dependent components
    for dep in &metadata.depends_on {
        if !generated_dir.join(dep).exists() {
            println!(
                "  {} Auto-adding dependency: {}",
                "→".cyan(),
                dep.bold()
            );
            add_component(component_registry, generated_dir, project, dep, false)?;
        } else {
            println!("  {} Dependency already installed: {}", "✓".green(), dep);
        }
    }

    // Generate hook file
    HookGenerator::write_hook(&metadata.name, generated_dir)
        .with_context(|| format!("Failed to generate hook '{}'", metadata.name))?;

    println!("  {} Created hook file", "✓".green());

    // Update generated/hooks/mod.rs
    update_hooks_mod(generated_dir, &metadata.name)?;
    // Ensure generated/mod.rs includes hooks module
    ensure_hooks_in_mod(generated_dir)?;
    println!("  {} Updated module exports", "✓".green());

    Ok(())
}

fn update_hooks_mod(generated_dir: &std::path::Path, hook_name: &str) -> Result<()> {
    let hooks_dir = generated_dir.join("hooks");
    let mod_rs = hooks_dir.join("mod.rs");

    let mut content = if mod_rs.exists() {
        fs::read_to_string(&mod_rs).with_context(|| format!("Failed to read {}", mod_rs.display()))?
    } else {
        "//! Generated daisyUI hooks\n".to_string()
    };

    let mod_line = format!("pub mod {};", hook_name);
    let use_line = format!("pub use {}::*;", hook_name);

    if content.contains(&mod_line) {
        return Ok(());
    }

    content.push_str(&format!("\n{}\n{}\n", mod_line, use_line));

    fs::write(&mod_rs, content).with_context(|| format!("Failed to write {}", mod_rs.display()))?;

    Ok(())
}

fn ensure_hooks_in_mod(generated_dir: &std::path::Path) -> Result<()> {
    let mod_rs = generated_dir.join("mod.rs");
    let mut content = fs::read_to_string(&mod_rs)
        .with_context(|| format!("Failed to read {}", mod_rs.display()))?;

    let hooks_mod = "pub mod hooks;";
    let hooks_use = "pub use hooks::*;";

    if content.contains(hooks_mod) {
        return Ok(());
    }

    content.push_str(&format!("\n{}\n{}\n", hooks_mod, hooks_use));

    fs::write(&mod_rs, content).with_context(|| format!("Failed to write {}", mod_rs.display()))?;

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

    fs::write(&mod_rs, content).with_context(|| format!("Failed to write {}", mod_rs.display()))?;

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
