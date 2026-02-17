use crate::component::ComponentRegistry;
use crate::project::ProjectDetector;
use anyhow::Result;
use colored::Colorize;
use std::collections::HashSet;
use std::env;
use std::fs;

pub fn execute(all: bool, category: Option<String>) -> Result<()> {
    let registry = ComponentRegistry::load()?;

    // Detect installed components
    let installed = detect_installed_components()?;

    if all {
        display_all_components(&registry, &installed, category.as_deref())?;
    } else {
        display_installed_components(&registry, &installed)?;
    }

    Ok(())
}

fn detect_installed_components() -> Result<HashSet<String>> {
    let mut installed = HashSet::new();

    // Try to detect project and check src/generated
    if let Ok(current_dir) = env::current_dir() {
        if let Ok(project) = ProjectDetector::detect(&current_dir) {
            if let Ok(src_dir) = ProjectDetector::find_main_binary_crate(&project) {
                let generated_dir = src_dir.join("generated");

                if generated_dir.exists() {
                    // Read all subdirectories in generated/
                    if let Ok(entries) = fs::read_dir(&generated_dir) {
                        for entry in entries.flatten() {
                            if entry.path().is_dir() {
                                if let Some(name) = entry.file_name().to_str() {
                                    installed.insert(name.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(installed)
}

fn display_all_components(
    registry: &ComponentRegistry,
    installed: &HashSet<String>,
    category_filter: Option<&str>,
) -> Result<()> {
    let components = if let Some(cat) = category_filter {
        registry.get_by_category(cat)
    } else {
        registry.all().iter().collect()
    };

    println!();
    println!("{}", "Available Components:".bold());
    println!("{}", "=".repeat(80));
    println!();

    // Group by category
    let mut by_category: std::collections::HashMap<&str, Vec<_>> =
        std::collections::HashMap::new();
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

    let total = components.len();
    let installed_count = components
        .iter()
        .filter(|c| installed.contains(&c.name))
        .count();

    println!("{}", "─".repeat(80).dimmed());
    println!(
        "{} total components, {} installed",
        total.to_string().bold(),
        installed_count.to_string().green().bold()
    );
    println!();

    Ok(())
}

fn display_installed_components(
    registry: &ComponentRegistry,
    installed: &HashSet<String>,
) -> Result<()> {
    if installed.is_empty() {
        println!();
        println!("{}", "No components installed yet.".yellow());
        println!();
        println!("Run {} to see available components", "leptos-daisyui list --all".cyan());
        println!("Run {} to add components", "leptos-daisyui add <component>".cyan());
        println!();
        return Ok(());
    }

    println!();
    println!("{}", "Installed Components:".bold());
    println!("{}", "=".repeat(80));
    println!();

    for component_name in installed.iter() {
        if let Some(component) = registry.get(component_name) {
            println!(
                "  {:<20} {}",
                component.display_name.bold(),
                component.description.dimmed()
            );
        }
    }

    println!();
    println!("{} components installed", installed.len().to_string().bold());
    println!();

    Ok(())
}

