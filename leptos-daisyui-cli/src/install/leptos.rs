use anyhow::Result;

pub struct LeptosInstaller;

impl LeptosInstaller {
    pub fn check_installed() -> bool {
        // TODO: Check if leptos is in dependencies
        false
    }

    pub fn install() -> Result<()> {
        // TODO: Add leptos to Cargo.toml
        Ok(())
    }
}
