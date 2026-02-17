use anyhow::Result;

pub fn execute(components: Vec<String>, force: bool, interactive: bool) -> Result<()> {
    println!(
        "add command - components: {:?}, force: {}, interactive: {}",
        components, force, interactive
    );
    Ok(())
}
