use std::fs;
use std::path::Path;
use anyhow::{Context, Result};
use crate::project_management::config::models::MoliConfig;

/// Config parser for v2 moli.yml format
pub struct ConfigParser;

impl ConfigParser {
    /// Parse moli.yml from file path
    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<MoliConfig> {
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file: {}", path.as_ref().display()))?;

        Self::parse_string(&content)
    }

    /// Parse moli.yml from string content
    pub fn parse_string(content: &str) -> Result<MoliConfig> {
        let projects: Vec<crate::project_management::config::models::Project> =
            serde_yaml::from_str(content)
                .with_context(|| "Failed to parse YAML content")?;

        Ok(MoliConfig { projects })
    }

    /// Parse default moli.yml in current directory
    pub fn parse_default() -> Result<MoliConfig> {
        Self::parse_file("moli.yml")
    }

    /// Check if moli.yml exists in current directory
    pub fn config_exists() -> bool {
        Path::new("moli.yml").exists()
    }

    /// Get default config file path
    pub fn default_config_path() -> &'static str {
        "moli.yml"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_project() {
        let yaml_content = r#"
- name: app
  root: true
  lang: rust
  spec:
    - name: src
      tree:
        - name: domain
          file:
            - name: model
            - name: repository
"#;

        let config = ConfigParser::parse_string(yaml_content).unwrap();

        assert_eq!(config.projects().len(), 1);
        assert!(config.is_single_project());

        let project = config.root_project().unwrap();
        assert_eq!(project.name(), "app");
        assert!(project.is_root());
        assert_eq!(project.language(), "rust");
        assert_eq!(project.spec().len(), 1);
    }

    #[test]
    fn test_parse_multi_project() {
        let yaml_content = r#"
- name: backend
  lang: rust
  spec:
    - name: src
      file:
        - name: main

- name: frontend
  lang: javascript
  spec:
    - name: src
      file:
        - name: index.js
"#;

        let config = ConfigParser::parse_string(yaml_content).unwrap();

        assert_eq!(config.projects().len(), 2);
        assert!(!config.is_single_project());
        assert_eq!(config.sub_projects().len(), 2);

        let backend = &config.projects()[0];
        assert_eq!(backend.name(), "backend");
        assert!(!backend.is_root());
        assert_eq!(backend.language(), "rust");

        let frontend = &config.projects()[1];
        assert_eq!(frontend.name(), "frontend");
        assert_eq!(frontend.language(), "javascript");
    }

    #[test]
    fn test_file_extension_handling() {
        let yaml_content = r#"
- name: app
  lang: rust
  spec:
    - name: src
      file:
        - name: model
        - name: component.vue
"#;

        let config = ConfigParser::parse_string(yaml_content).unwrap();
        let project = &config.projects()[0];
        let files = &project.spec()[0].files();

        // Test auto extension
        assert_eq!(files[0].filename_with_extension("rust"), "model.rs");

        // Test explicit extension
        assert_eq!(files[1].filename_with_extension("rust"), "component.vue");
    }
}
