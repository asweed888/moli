use std::fs;
use std::path::Path;
use anyhow::{Context, Result};
use crate::project_management::config::models::{MoliConfig, Project};
use crate::shared::utils::content_updater::ContentUpdater;

/// Rust workspace handler for generating workspace Cargo.toml and Makefile.toml
pub struct WorkspaceHandler;

impl WorkspaceHandler {
    /// Generate workspace Cargo.toml for multi-project configuration with Rust projects
    pub fn generate_workspace_cargo_toml<P: AsRef<Path>>(
        workspace_path: P,
        config: &MoliConfig,
    ) -> Result<()> {
        let rust_projects: Vec<&Project> = config.projects()
            .iter()
            .filter(|p| p.language() == "rust")
            .collect();

        if rust_projects.is_empty() {
            return Ok(()); // No Rust projects, no workspace needed
        }

        let cargo_toml_path = workspace_path.as_ref().join("Cargo.toml");

        // Check if the FIRST project (第一階層目) has name: "." and is a Rust project
        let first_project_is_root = config.projects()
            .first()
            .map(|p| p.name() == "." && p.language() == "rust")
            .unwrap_or(false);

        // Build members list (include all Rust projects, including "." if it exists)
        let members: Vec<String> = rust_projects
            .iter()
            .map(|p| p.name().to_string())
            .collect();

        // Extract package info if first project is root package
        let package_info = if first_project_is_root {
            config.projects()
                .first()
                .map(|p| {
                    (
                        Self::extract_package_name_from_path(workspace_path.as_ref()),
                        p
                    )
                })
        } else {
            None
        };

        ContentUpdater::update_workspace_cargo_toml(&cargo_toml_path, &members, package_info)
            .with_context(|| format!("Failed to update workspace Cargo.toml: {}", cargo_toml_path.display()))?;

        Ok(())
    }

    /// Extract package name from workspace path (use directory name)
    /// Sanitizes the name to be a valid Rust crate name
    fn extract_package_name_from_path(path: &Path) -> String {
        // Convert to absolute path to get the actual directory name
        let absolute_path = path.canonicalize()
            .unwrap_or_else(|_| path.to_path_buf());

        let dir_name = absolute_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("workspace");

        Self::sanitize_crate_name(dir_name)
    }

    /// Sanitize name to be a valid Rust crate name
    /// - Replace hyphens with underscores
    /// - Convert to lowercase
    /// - Ensure it doesn't start with a number
    /// - Remove invalid characters
    fn sanitize_crate_name(name: &str) -> String {
        let mut result = name
            .replace('-', "_")
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_')
            .collect::<String>();

        // If name starts with a number, prepend underscore
        if result.chars().next().map(|c| c.is_numeric()).unwrap_or(false) {
            result = format!("_{}", result);
        }

        // If result is empty, use default
        if result.is_empty() {
            result = "workspace".to_string();
        }

        result
    }

    /// Generate Makefile.toml for cargo-make
    pub fn generate_makefile_toml<P: AsRef<Path>>(
        workspace_path: P,
        config: &MoliConfig,
    ) -> Result<()> {
        let makefile_toml_path = workspace_path.as_ref().join("Makefile.toml");
        
        // Only create if file doesn't exist (protect existing configuration)
        if !makefile_toml_path.exists() {
            let makefile_content = Self::generate_makefile_toml_content(config);
            fs::write(&makefile_toml_path, makefile_content)
                .with_context(|| format!("Failed to create Makefile.toml: {}", makefile_toml_path.display()))?;
        }

        Ok(())
    }


    /// Generate Makefile.toml content for cargo-make
    fn generate_makefile_toml_content(config: &MoliConfig) -> String {
        let rust_projects: Vec<&Project> = config.projects()
            .iter()
            .filter(|p| p.language() == "rust")
            .collect();

        // Use the first Rust project as default for dev task
        let default_project = rust_projects.first().map(|p| p.name()).unwrap_or("app_1");

        format!(
            r#"[env]
CARGO_MAKE_EXTEND_WORKSPACE_MAKEFILE = true
# CARGO_MAKE_LOAD_SCRIPT = ".env"

[tasks.dev]
description = "Start development environment."
workspace = false
cwd = "{}"
command = "cargo"
args = ["watch", "-x", "run"]
"#,
            default_project
        )
    }

    /// Check if workspace generation is needed (multi-project with Rust)
    pub fn should_generate_workspace(config: &MoliConfig) -> bool {
        let rust_project_count = config.projects()
            .iter()
            .filter(|p| p.language() == "rust")
            .count();

        // Generate workspace if there are multiple projects with at least one Rust project
        // OR if there's a single Rust project but other language projects exist
        rust_project_count > 1 || 
        (rust_project_count > 0 && config.projects().len() > rust_project_count)
    }
}