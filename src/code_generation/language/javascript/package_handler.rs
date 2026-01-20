use std::fs;
use std::path::Path;
use anyhow::{Context, Result};
use crate::project_management::config::models::{Project, Module};
use super::module_generator::JavaScriptModuleGenerator;

/// JavaScript package handler for generating JavaScript project structure
pub struct JavaScriptPackageHandler;

impl JavaScriptPackageHandler {
    /// Generate complete JavaScript project structure
    pub fn generate_project<P: AsRef<Path>>(
        project_path: P,
        project: &Project,
    ) -> Result<()> {
        let project_path = project_path.as_ref();
        
        // Create project directory
        fs::create_dir_all(project_path)
            .with_context(|| format!("Failed to create project directory: {}", project_path.display()))?;

        // Generate package.json
        JavaScriptModuleGenerator::generate_package_json(project_path, project.name())?;
        
        // Generate main index.js if needed
        if JavaScriptModuleGenerator::should_generate_main_index_js(project) {
            JavaScriptModuleGenerator::generate_main_index_js(project_path)?;
        }
        
        // Generate project-level code files
        for codefile in project.files() {
            let filename = codefile.filename_with_extension("javascript");
            let file_path = project_path.join(&filename);
            
            // Create empty JavaScript file (only if it doesn't exist)
            if !file_path.exists() {
                fs::write(&file_path, "")
                    .with_context(|| format!("Failed to create file: {}", file_path.display()))?;
            }
        }
        
        // Generate module structure
        for module in project.spec() {
            JavaScriptModuleGenerator::generate_module(project_path, module, &[])?;
        }

        Ok(())
    }
}