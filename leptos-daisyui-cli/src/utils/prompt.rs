use anyhow::Result;
use inquire::Confirm;

pub struct Prompt;

impl Prompt {
    pub fn confirm(message: &str, default: bool) -> Result<bool> {
        Confirm::new(message)
            .with_default(default)
            .prompt()
            .map_err(|e| anyhow::anyhow!("Prompt error: {}", e))
    }
}
