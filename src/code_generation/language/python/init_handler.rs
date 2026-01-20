use std::fs;
use std::path::Path;
use anyhow::{Context, Result};
use crate::project_management::config::models::{Project, Module};
use super::package_generator::PythonPackageGenerator;

/// Python package handler for generating Python project structure
pub struct PythonInitHandler;

impl PythonInitHandler {
    /// Generate complete Python project structure
    pub fn generate_project<P: AsRef<Path>>(
        project_path: P,
        project: &Project,
    ) -> Result<()> {
        let project_path = project_path.as_ref();
        
        // Create project directory
        fs::create_dir_all(project_path)
            .with_context(|| format!("Failed to create project directory: {}", project_path.display()))?;

        // Generate requirements.txt
        PythonPackageGenerator::generate_requirements_txt(project_path)?;
        
        // Generate setup.py
        PythonPackageGenerator::generate_setup_py(project_path, project.name())?;
        
        // Generate main.py if needed
        if PythonPackageGenerator::should_generate_main_py(project) {
            PythonPackageGenerator::generate_main_py(project_path)?;
        }
        
        // Generate project-level code files
        for codefile in project.files() {
            let filename = codefile.filename_with_extension("python");
            let file_path = project_path.join(&filename);
            
            // Create empty Python file (only if it doesn't exist)
            if !file_path.exists() {
                fs::write(&file_path, "")
                    .with_context(|| format!("Failed to create file: {}", file_path.display()))?;
            }
        }
        
        // Generate module structure
        for module in project.spec() {
            PythonPackageGenerator::generate_module(project_path, module, &[])?;
        }

        Ok(())
    }
}