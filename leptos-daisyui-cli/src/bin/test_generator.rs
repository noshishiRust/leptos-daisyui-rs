use leptos_daisyui_cli::component::ComponentGenerator;

fn main() -> anyhow::Result<()> {
    println!("Testing ComponentGenerator...\n");

    // Test reading button component
    println!("Reading button component files...");
    let files = ComponentGenerator::get_component_files("button")?;

    println!("  mod.rs: {} bytes", files.mod_rs.len());
    println!("  component.rs: {} bytes", files.component_rs.len());
    if let Some(style_rs) = &files.style_rs {
        println!("  style.rs: {} bytes", style_rs.len());
    } else {
        println!("  style.rs: not present");
    }

    // Test reading a simple component without style.rs
    println!("\nReading hero component files...");
    let files = ComponentGenerator::get_component_files("hero")?;

    println!("  mod.rs: {} bytes", files.mod_rs.len());
    println!("  component.rs: {} bytes", files.component_rs.len());
    if let Some(style_rs) = &files.style_rs {
        println!("  style.rs: {} bytes", style_rs.len());
    } else {
        println!("  style.rs: not present");
    }

    println!("\n✓ ComponentGenerator working correctly!");

    Ok(())
}
