use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Not a Rust project")]
    NotRustProject,

    #[error("Component '{0}' not found")]
    ComponentNotFound(String),

    #[error("Failed to detect project structure: {0}")]
    ProjectDetectionFailed(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML parsing error: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("{0}")]
    Custom(String),
}

pub type Result<T> = std::result::Result<T, CliError>;
