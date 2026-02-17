use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub enum ProjectStructure {
    /// Single Cargo.toml at root
    Monorepo {
        root: PathBuf,
        cargo_toml: PathBuf,
        src_dir: PathBuf,
    },
    /// Workspace with multiple crates
    Workspace {
        root: PathBuf,
        workspace_toml: PathBuf,
        members: Vec<WorkspaceMember>,
    },
}

#[derive(Debug, Clone)]
pub struct WorkspaceMember {
    pub name: String,
    pub path: PathBuf,
    pub cargo_toml: PathBuf,
    pub src_dir: PathBuf,
    pub is_binary: bool,
}

impl ProjectStructure {
    /// Get the root directory of the project
    pub fn root(&self) -> &Path {
        match self {
            ProjectStructure::Monorepo { root, .. } => root,
            ProjectStructure::Workspace { root, .. } => root,
        }
    }

    /// Get the main Cargo.toml path
    pub fn cargo_toml(&self) -> &Path {
        match self {
            ProjectStructure::Monorepo { cargo_toml, .. } => cargo_toml,
            ProjectStructure::Workspace { workspace_toml, .. } => workspace_toml,
        }
    }
}

pub struct ProjectDetector;

impl ProjectDetector {
    /// Detect project structure starting from the given directory
    pub fn detect(starting_dir: &Path) -> Result<ProjectStructure> {
        // 1. Walk up directory tree to find Cargo.toml
        let root = Self::find_rust_root(starting_dir)?;
        let cargo_toml = root.join("Cargo.toml");

        // 2. Parse Cargo.toml to determine structure
        let content = fs::read_to_string(&cargo_toml)
            .with_context(|| format!("Failed to read {}", cargo_toml.display()))?;
        let manifest: toml::Value = toml::from_str(&content)
            .with_context(|| format!("Failed to parse {}", cargo_toml.display()))?;

        // 3. Check if workspace
        if manifest.get("workspace").is_some() {
            Self::detect_workspace(&root, manifest)
        } else {
            Self::detect_monorepo(&root, cargo_toml)
        }
    }

    /// Find the root directory containing Cargo.toml by walking up the tree
    fn find_rust_root(starting_dir: &Path) -> Result<PathBuf> {
        let mut current = starting_dir.to_path_buf();

        loop {
            let cargo_toml = current.join("Cargo.toml");
            if cargo_toml.exists() {
                return Ok(current);
            }

            if !current.pop() {
                return Err(anyhow!(
                    "Not a Rust project: no Cargo.toml found in {} or parent directories",
                    starting_dir.display()
                ));
            }
        }
    }

    /// Detect workspace structure
    fn detect_workspace(root: &Path, manifest: toml::Value) -> Result<ProjectStructure> {
        let workspace_toml = root.join("Cargo.toml");
        let mut members = Vec::new();

        if let Some(workspace) = manifest.get("workspace") {
            if let Some(member_paths) = workspace.get("members").and_then(|m| m.as_array()) {
                for member_path in member_paths {
                    if let Some(path_str) = member_path.as_str() {
                        let member_root = root.join(path_str);
                        let member_cargo = member_root.join("Cargo.toml");

                        if member_cargo.exists() {
                            let member_content = fs::read_to_string(&member_cargo)?;
                            let member_manifest: toml::Value = toml::from_str(&member_content)?;

                            let name = member_manifest
                                .get("package")
                                .and_then(|p| p.get("name"))
                                .and_then(|n| n.as_str())
                                .unwrap_or(path_str)
                                .to_string();

                            let is_binary = member_manifest
                                .get("bin")
                                .is_some()
                                || member_root.join("src/main.rs").exists();

                            members.push(WorkspaceMember {
                                name,
                                path: member_root.clone(),
                                cargo_toml: member_cargo,
                                src_dir: member_root.join("src"),
                                is_binary,
                            });
                        }
                    }
                }
            }
        }

        Ok(ProjectStructure::Workspace {
            root: root.to_path_buf(),
            workspace_toml,
            members,
        })
    }

    /// Detect monorepo structure
    fn detect_monorepo(root: &Path, cargo_toml: PathBuf) -> Result<ProjectStructure> {
        let src_dir = root.join("src");

        if !src_dir.exists() {
            return Err(anyhow!("src directory not found in {}", root.display()));
        }

        Ok(ProjectStructure::Monorepo {
            root: root.to_path_buf(),
            cargo_toml,
            src_dir,
        })
    }

    /// Find the main binary crate in the project
    pub fn find_main_binary_crate(structure: &ProjectStructure) -> Result<PathBuf> {
        match structure {
            ProjectStructure::Monorepo { src_dir, .. } => Ok(src_dir.clone()),
            ProjectStructure::Workspace { members, .. } => {
                // Find first binary crate
                members
                    .iter()
                    .find(|m| m.is_binary)
                    .map(|m| m.src_dir.clone())
                    .ok_or_else(|| anyhow!("No binary crate found in workspace"))
            }
        }
    }
}
