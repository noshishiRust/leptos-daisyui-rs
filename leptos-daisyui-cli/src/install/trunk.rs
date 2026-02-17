use anyhow::Result;

pub struct TrunkInstaller;

impl TrunkInstaller {
    pub fn check_installed() -> bool {
        which::which("trunk").is_ok()
    }

    pub fn install() -> Result<()> {
        // TODO: Install trunk via cargo install
        Ok(())
    }
}
