use std::fs;
use std::path::Path;
use anyhow::{Context, Result};
use crate::project_management::config::models::Project;

/// Rust Cargo handler for generating individual project Cargo.toml
pub struct CargoHandler;

impl CargoHandler {
    /// Generate Cargo.toml for individual Rust project
    pub fn generate_cargo_toml<P: AsRef<Path>>(
        project_path: P,
        project: &Project,
    ) -> Result<()> {
        let cargo_toml_path = project_path.as_ref().join("Cargo.toml");
        let cargo_content = Self::generate_cargo_toml_content(project);
        
        // Only create Cargo.toml if it doesn't already exist
        if !cargo_toml_path.exists() {
            fs::write(&cargo_toml_path, cargo_content)
                .with_context(|| format!("Failed to create Cargo.toml: {}", cargo_toml_path.display()))?;
        }

        Ok(())
    }

    /// Generate Cargo.toml content
    fn generate_cargo_toml_content(project: &Project) -> String {
        let package_name = Self::extract_package_name(project.name());
        format!(
            r#"[package]
name = "{}"
version = "0.1.0"
edition = "2024"

[dependencies]

"#,
            package_name
        )
    }

    /// Extract package name from project name (get part after last '/')
    fn extract_package_name(project_name: &str) -> String {
        project_name
            .rfind('/')
            .map(|index| &project_name[index + 1..])
            .unwrap_or(project_name)
            .to_string()
    }

    /// Determine if project should be a library based on presence of lib.rs
    fn is_library_project(project: &Project) -> bool {
        // Check if there's a lib.rs file defined in the project
        let has_lib_in_project = project.files().iter()
            .any(|f| f.name() == "lib" || f.filename_with_extension("rust") == "lib.rs");
        
        let has_lib_in_src = project.tree().iter()
            .filter(|m| m.name() == "src")
            .flat_map(|m| m.files())
            .any(|f| f.name() == "lib" || f.filename_with_extension("rust") == "lib.rs");
        
        has_lib_in_project || has_lib_in_src
    }

    /// Sanitize crate name for Rust (replace hyphens with underscores)
    fn sanitize_crate_name(name: &str) -> String {
        name.replace('-', "_")
    }
}