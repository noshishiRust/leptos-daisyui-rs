use anyhow::Result;

pub struct TailwindInstaller;

impl TailwindInstaller {
    pub fn check_installed() -> bool {
        // TODO: Check for package.json with tailwindcss
        false
    }

    pub fn setup() -> Result<()> {
        // TODO: Create package.json and install tailwindcss
        Ok(())
    }
}
