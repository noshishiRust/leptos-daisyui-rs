use crate::component::{ComponentRegistry, HookRegistry};
use crate::project::ProjectDetector;
use anyhow::Result;
use colored::Colorize;
use std::collections::HashSet;
use std::env;
use std::fs;

pub fn execute(all: bool, category: Option<String>) -> Result<()> {
    let registry = ComponentRegistry::load()?;
    let hook_registry = HookRegistry::load()?;

    // Detect installed components and hooks
    let installed = detect_installed_components()?;
    let installed_hooks = detect_installed_hooks()?;

    if all {
        display_all(&registry, &hook_registry, &installed, &installed_hooks, category.as_deref())?;
    } else {
        display_installed(&registry, &hook_registry, &installed, &installed_hooks)?;
    }

    Ok(())
}

fn detect_installed_components() -> Result<HashSet<String>> {
    let mut installed = HashSet::new();

    if let Ok(current_dir) = env::current_dir()
        && let Ok(project) = ProjectDetector::detect(&current_dir)
        && let Ok(src_dir) = ProjectDetector::find_main_binary_crate(&project)
    {
        let generated_dir = src_dir.join("generated");

        if generated_dir.exists() {
            if let Ok(entries) = fs::read_dir(&generated_dir) {
                for entry in entries.flatten() {
                    if entry.path().is_dir()
                        && let Some(name) = entry.file_name().to_str()
                        && name != "hooks"
                        && name != "utils"
                    {
                        installed.insert(name.to_string());
                    }
                }
            }
        }
    }

    Ok(installed)
}

fn detect_installed_hooks() -> Result<HashSet<String>> {
    let mut installed = HashSet::new();

    if let Ok(current_dir) = env::current_dir()
        && let Ok(project) = ProjectDetector::detect(&current_dir)
        && let Ok(src_dir) = ProjectDetector::find_main_binary_crate(&project)
    {
        let hooks_dir = src_dir.join("generated").join("hooks");

        if hooks_dir.exists() {
            if let Ok(entries) = fs::read_dir(&hooks_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file()
                        && path.extension().is_some_and(|ext| ext == "rs")
                        && let Some(name) = path.file_stem().and_then(|s| s.to_str())
                        && name != "mod"
                    {
                        installed.insert(name.to_string());
                    }
                }
            }
        }
    }

    Ok(installed)
}

fn display_all(
    registry: &ComponentRegistry,
    hook_registry: &HookRegistry,
    installed: &HashSet<String>,
    installed_hooks: &HashSet<String>,
    category_filter: Option<&str>,
) -> Result<()> {
    println!();
    println!("{}", "Available Components & Hooks:".bold());
    println!("{}", "=".repeat(80));
    println!();

    // Filter: if "hooks" category, show only hooks
    if category_filter.is_some_and(|c| c == "hooks") {
        display_hooks_section(hook_registry, installed_hooks)?;
        return Ok(());
    }

    // Show components grouped by category
    let components = if let Some(cat) = category_filter {
        registry.get_by_category(cat)
    } else {
        registry.all().iter().collect()
    };

    let mut by_category: std::collections::HashMap<&str, Vec<_>> = std::collections::HashMap::new();
    for component in &components {
        by_category
            .entry(&component.category as &str)
            .or_default()
            .push(component);
    }

    let mut categories: Vec<_> = by_category.keys().copied().collect();
    categories.sort();

    for category in categories {
        println!("{}", category.to_uppercase().bold().cyan());
        println!();

        if let Some(comps) = by_category.get(category) {
            for component in comps {
                let status = if installed.contains(&component.name) {
                    "✓".green().to_string()
                } else {
                    " ".to_string()
                };

                println!(
                    "  {} {:<20} {}",
                    status,
                    component.display_name.bold(),
                    component.description.dimmed()
                );
            }
        }

        println!();
    }

    // Show hooks section (only if no category filter or filter is not "hooks")
    if category_filter.is_none() {
        display_hooks_section(hook_registry, installed_hooks)?;
    }

    let total = components.len();
    let installed_count = components
        .iter()
        .filter(|c| installed.contains(&c.name))
        .count();
    let hook_total = hook_registry.all().len();
    let hook_installed_count = installed_hooks.len();

    println!("{}", "─".repeat(80).dimmed());
    println!(
        "{} components ({} installed), {} hooks ({} installed)",
        total.to_string().bold(),
        installed_count.to_string().green().bold(),
        hook_total.to_string().bold(),
        hook_installed_count.to_string().green().bold()
    );
    println!();

    Ok(())
}

fn display_hooks_section(
    hook_registry: &HookRegistry,
    installed_hooks: &HashSet<String>,
) -> Result<()> {
    println!("{}", "HOOKS".bold().magenta());
    println!();

    for hook in hook_registry.all() {
        let status = if installed_hooks.contains(&hook.name) {
            "✓".green().to_string()
        } else {
            " ".to_string()
        };

        let deps = if hook.depends_on.is_empty() {
            String::new()
        } else {
            format!(" [requires: {}]", hook.depends_on.join(", ").dimmed())
        };

        println!(
            "  {} {:<30} {}{}",
            status,
            hook.display_name.bold(),
            hook.description.dimmed(),
            deps
        );
    }

    println!();
    Ok(())
}

fn display_installed(
    registry: &ComponentRegistry,
    hook_registry: &HookRegistry,
    installed: &HashSet<String>,
    installed_hooks: &HashSet<String>,
) -> Result<()> {
    if installed.is_empty() && installed_hooks.is_empty() {
        println!();
        println!("{}", "No components or hooks installed yet.".yellow());
        println!();
        println!(
            "Run {} to see available options",
            "leptos-daisyui list --all".cyan()
        );
        println!(
            "Run {} to add components or hooks",
            "leptos-daisyui add <name>".cyan()
        );
        println!();
        return Ok(());
    }

    println!();
    println!("{}", "Installed Components & Hooks:".bold());
    println!("{}", "=".repeat(80));
    println!();

    if !installed.is_empty() {
        for component_name in installed.iter() {
            if let Some(component) = registry.get(component_name) {
                println!(
                    "  {:<20} {}",
                    component.display_name.bold(),
                    component.description.dimmed()
                );
            }
        }
    }

    if !installed_hooks.is_empty() {
        println!();
        println!("{}", "Hooks:".bold().magenta());
        for hook_name in installed_hooks.iter() {
            if let Some(hook) = hook_registry.get(hook_name) {
                println!(
                    "  {:<20} {}",
                    hook.display_name.bold(),
                    hook.description.dimmed()
                );
            }
        }
    }

    println!();
    println!(
        "{} components, {} hooks installed",
        installed.len().to_string().bold(),
        installed_hooks.len().to_string().bold()
    );
    println!();

    Ok(())
}
