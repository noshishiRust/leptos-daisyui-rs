use anyhow::Result;

pub fn execute(all: bool, category: Option<String>) -> Result<()> {
    println!("list command - all: {}, category: {:?}", all, category);
    Ok(())
}
