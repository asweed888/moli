use std::fs;
use std::path::Path;
use anyhow::{Context, Result, anyhow};
use crate::project_management::config::models::Project;
use crate::code_generation::language::{
    rust::module_generator::RustModuleGenerator,
    rust::workspace_handler::WorkspaceHandler,
    rust::cargo_handler::CargoHandler,
    typescript::module_generator::TypeScriptModuleGenerator,
    go::mod_handler::GoModuleHandler,
    python::init_handler::PythonInitHandler,
    javascript::package_handler::JavaScriptPackageHandler,
    any::file_handler::AnyFileHandler,
};
use crate::project_management::config::models::MoliConfig;

/// Core code generator that delegates to language-specific generators
pub struct CodeGenerator;

impl CodeGenerator {
    /// Generate complete project structure from configuration
    pub fn generate_from_config<P: AsRef<Path>>(
        output_path: P,
        config: &MoliConfig,
    ) -> Result<()> {
        let output_path = output_path.as_ref();

        // Check if this is a single project or multi-project configuration
        if config.is_single_project() {
            // Single project mode - generate directly in output path
            if let Some(root_project) = config.root_project() {
                Self::generate_single_project(output_path, root_project)?;
            }
        } else {
            // Multi-project mode - generate each project in its own directory
            Self::generate_multi_project(output_path, config)?;
        }

        Ok(())
    }

    /// Generate single project structure
    pub fn generate_single_project<P: AsRef<Path>>(
        output_path: P,
        project: &Project,
    ) -> Result<()> {
        let output_path = output_path.as_ref();

        // Create output directory
        fs::create_dir_all(output_path)
            .with_context(|| format!("Failed to create output directory: {}", output_path.display()))?;

        // Delegate to language-specific generator
        match project.language() {
            "rust" => Self::generate_rust_project(output_path, project),
            "typescript" => Self::generate_typescript_project(output_path, project),
            "go" => Self::generate_go_project(output_path, project),
            "python" => Self::generate_python_project(output_path, project),
            "javascript" => Self::generate_javascript_project(output_path, project),
            "any" => Self::generate_any_project(output_path, project),
            lang => Err(anyhow!("Unsupported language: {}", lang)),
        }
    }

    /// Generate multi-project structure
    fn generate_multi_project<P: AsRef<Path>>(
        output_path: P,
        config: &MoliConfig,
    ) -> Result<()> {
        let output_path = output_path.as_ref();

        // Create workspace directory
        fs::create_dir_all(output_path)
            .with_context(|| format!("Failed to create workspace directory: {}", output_path.display()))?;

        // Generate workspace-level files if needed
        if WorkspaceHandler::should_generate_workspace(config) {
            WorkspaceHandler::generate_workspace_cargo_toml(output_path, config)?;
            WorkspaceHandler::generate_makefile_toml(output_path, config)?;
        }

        // Generate each project
        for project in config.projects() {
            let project_path = output_path.join(project.name());

            // Create project directory
            fs::create_dir_all(&project_path)
                .with_context(|| format!("Failed to create project directory: {}", project_path.display()))?;

            // Generate project structure
            match project.language() {
                "rust" => Self::generate_rust_project(&project_path, project),
                "typescript" => Self::generate_typescript_project(&project_path, project),
                "go" => Self::generate_go_project(&project_path, project),
                "python" => Self::generate_python_project(&project_path, project),
                "javascript" => Self::generate_javascript_project(&project_path, project),
                "any" => Self::generate_any_project(&project_path, project),
                lang => Err(anyhow!("Unsupported language: {}", lang)),
            }?;
        }

        Ok(())
    }

    /// Generate Rust project
    fn generate_rust_project<P: AsRef<Path>>(
        project_path: P,
        project: &Project,
    ) -> Result<()> {
        let project_path = project_path.as_ref();

        // Generate Cargo.toml
        CargoHandler::generate_cargo_toml(project_path, project)?;

        // Create src directory
        let src_path = project_path.join("src");
        fs::create_dir_all(&src_path)
            .with_context(|| format!("Failed to create src directory: {}", src_path.display()))?;

        // Generate project-level code files (only if they don't exist)
        for codefile in project.files() {
            let filename = codefile.filename_with_extension("rust");
            let file_path = project_path.join(&filename);

            // Only create file if it doesn't already exist
            if !file_path.exists() {
                fs::write(&file_path, "")
                    .with_context(|| format!("Failed to create file: {}", file_path.display()))?;
            }
        }

        // Generate module structure
        for module in project.spec() {
            RustModuleGenerator::generate_module(project_path, module, &[])?;
        }

        // Generate main.rs or lib.rs
        let src_modules: Vec<_> = project.spec().iter()
            .filter(|m| m.name() == "src")
            .cloned()
            .collect();

        if RustModuleGenerator::should_generate_main_rs(project) {
            RustModuleGenerator::generate_main_rs(project_path, &src_modules)?;
        } else if RustModuleGenerator::should_generate_lib_rs(project) {
            RustModuleGenerator::generate_lib_rs(project_path, &src_modules)?;
        }
        // If neither main nor lib is specified, don't generate either (e.g., DDD with src/bin/ structure)

        Ok(())
    }

    /// Generate TypeScript project
    fn generate_typescript_project<P: AsRef<Path>>(
        project_path: P,
        project: &Project,
    ) -> Result<()> {
        let project_path = project_path.as_ref();

        // Generate package.json and tsconfig.json
        TypeScriptModuleGenerator::generate_package_json(project_path, project.name())?;
        TypeScriptModuleGenerator::generate_tsconfig_json(project_path)?;

        // Generate project-level code files (only if they don't exist)
        for codefile in project.files() {
            let filename = codefile.filename_with_extension("typescript");
            let file_path = project_path.join(&filename);

            // Only create file if it doesn't already exist
            if !file_path.exists() {
                fs::write(&file_path, "")
                    .with_context(|| format!("Failed to create file: {}", file_path.display()))?;
            }
        }

        // Generate module structure
        for module in project.spec() {
            TypeScriptModuleGenerator::generate_module(project_path, module, &[])?;
        }

        Ok(())
    }

    /// Generate Go project
    fn generate_go_project<P: AsRef<Path>>(
        project_path: P,
        project: &Project,
    ) -> Result<()> {
        GoModuleHandler::generate_project(project_path, project)
    }

    /// Generate Python project
    fn generate_python_project<P: AsRef<Path>>(
        project_path: P,
        project: &Project,
    ) -> Result<()> {
        PythonInitHandler::generate_project(project_path, project)
    }

    /// Generate JavaScript project
    fn generate_javascript_project<P: AsRef<Path>>(
        project_path: P,
        project: &Project,
    ) -> Result<()> {
        JavaScriptPackageHandler::generate_project(project_path, project)
    }

    /// Generate "any" language project
    fn generate_any_project<P: AsRef<Path>>(
        project_path: P,
        project: &Project,
    ) -> Result<()> {
        AnyFileHandler::generate_project(project_path, project)
    }
}
