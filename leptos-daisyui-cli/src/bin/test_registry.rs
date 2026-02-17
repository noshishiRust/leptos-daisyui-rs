use leptos_daisyui_cli::component::ComponentRegistry;

fn main() -> anyhow::Result<()> {
    let registry = ComponentRegistry::load()?;

    println!("Loaded {} components", registry.all().len());
    println!("\nComponents:");
    for component in registry.all() {
        println!("  - {} ({})", component.display_name, component.name);
    }

    println!("\nCategories:");
    for category in registry.categories() {
        println!("  - {}", category);
    }

    Ok(())
}
