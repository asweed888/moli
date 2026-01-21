use std::fs;
use std::path::Path;
use anyhow::{Context, Result};
use crate::project_management::config::models::{Project, Module};
use super::package_generator::GoPackageGenerator;

/// Go module handler for generating Go project structure
pub struct GoModuleHandler;

impl GoModuleHandler {
    /// Check if a file is a Go code file (ends with .go)
    fn is_go_code_file(filename: &str) -> bool {
        filename.ends_with(".go")
    }

    /// Generate complete Go project structure
    pub fn generate_project<P: AsRef<Path>>(
        project_path: P,
        project: &Project,
    ) -> Result<()> {
        let project_path = project_path.as_ref();
        
        // Create project directory
        fs::create_dir_all(project_path)
            .with_context(|| format!("Failed to create project directory: {}", project_path.display()))?;

        // Generate go.mod
        GoPackageGenerator::generate_go_mod(project_path, project.name())?;
        
        // Generate go.sum
        GoPackageGenerator::generate_go_sum(project_path)?;
        
        // Generate main.go if needed
        if GoPackageGenerator::should_generate_main_go(project) {
            GoPackageGenerator::generate_main_go(project_path)?;
        }
        
        // Generate project-level code files
        for codefile in project.files() {
            let filename = codefile.filename_with_extension("go");
            let file_path = project_path.join(&filename);

            // Only create file if it doesn't already exist
            if !file_path.exists() {
                // Only add package declaration for Go code files
                let content = if Self::is_go_code_file(&filename) {
                    // Use main package for root level files (Go convention)
                    let package_name = if codefile.name() == "main" || filename == "main.go" {
                        "main".to_string()
                    } else {
                        "main".to_string() // Most Go projects use main package for root level files
                    };
                    format!("package {}\n\n", package_name)
                } else {
                    String::new() // Non-Go files get no content
                };

                fs::write(&file_path, content)
                    .with_context(|| format!("Failed to create file: {}", file_path.display()))?;
            }
        }
        
        // Generate module structure
        for module in project.tree() {
            GoPackageGenerator::generate_module(project_path, module, &[])?;
        }

        Ok(())
    }
    
    /// Sanitize package name for Go (convert hyphens to underscores, make lowercase)
    fn sanitize_package_name(name: &str) -> String {
        name.replace('-', "_").to_lowercase()
    }
}