use clap::{Arg, ArgMatches, Command};
use anyhow::{bail, Context, Result};
use inquire::Select;
use std::fs;

pub fn spec() -> Command {
    Command::new("new")
        .about("Initialize a new project with moli.yml configuration")
        .long_about(
            "Initialize a new project by creating or updating moli.yml. \
            Supports two modes:\n\
            \n\
            Human Mode (Interactive):\n  \
            moli new  # Prompts for language selection\n\
            \n\
            AI Mode (Direct):\n  \
            moli new --lang rust  # Direct language specification\n\
            \n\
            Features:\n\
            • Auto-generates sequential project names (app_1, app_2, etc.)\n\
            • Smart multi-project handling (removes root: true from existing projects)\n\
            • Language-specific directory structures (Rust uses src/, others use root-level)\n\
            • Supports: rust, go, python, typescript, javascript, any"
        )
        .arg(
            Arg::new("lang")
                .short('l')
                .long("lang")
                .help("Programming language for direct specification (AI mode)")
                .long_help(
                    "Specify the programming language directly without interactive prompts. \
                    Supported languages: rust, go, python, typescript, javascript, any. \
                    When omitted, enters interactive mode for human users."
                )
                .value_name("LANGUAGE")
                .required(false)
        )
}

pub fn action(matches: &ArgMatches) -> Result<()> {
    let language = if let Some(lang) = matches.get_one::<String>("lang") {
        // AI mode - language specified via --lang option
        let supported_languages = ["rust", "go", "python", "typescript", "javascript", "any"];
        if !supported_languages.contains(&lang.as_str()) {
            bail!("Unsupported language: {}. Supported languages: {}", lang, supported_languages.join(", "));
        }
        lang.clone()
    } else {
        // Human mode - interactive language selection
        let languages = vec!["rust", "go", "python", "typescript", "javascript", "any"];
        Select::new("Programming language:", languages)
            .prompt()
            .context("Failed to get programming language")?.
            to_string()
    };

    // Determine project name
    let project_name = if language == "any" && fs::metadata("moli.yml").is_err() {
        // New moli.yml with any language: use "docs"
        "docs".to_string()
    } else {
        // All other cases: use sequential naming
        generate_sequential_project_name()?
    };

    // Check if moli.yml already exists
    if fs::metadata("moli.yml").is_ok() {
        // Existing moli.yml - append new project
        append_to_existing_moli_yml(&project_name, &language)?;
        println!("✓ Added {} ({}) project to existing moli.yml", project_name, language);
    } else {
        // No existing moli.yml - create new one with root: true
        let moli_content = generate_new_moli_yml(&project_name, &language)?;
        fs::write("moli.yml", moli_content)
            .context("Failed to write moli.yml")?;
        println!("✓ Generated new moli.yml for {} ({}) project", project_name, language);
    }

    println!("[Success] moli.yml initialization completed.");
    println!("Run 'moli up' to generate your project structure.");

    Ok(())
}

fn generate_new_moli_yml(project_name: &str, language: &str) -> Result<String> {
    let main_file = get_main_file_name(language, true); // root: true

    match language {
        "rust" => {
            // Rust standard: src/main.rs or src/lib.rs
            Ok(format!(
                r#"- name: {}
  root: true
  lang: {}
  tree:
    - name: src
      file:
        - name: {}
"#,
                project_name, language, main_file
            ))
        },
        "go" => {
            // Go standard: main.go at project root for simple projects
            Ok(format!(
                r#"- name: {}
  root: true
  lang: {}
  file:
    - name: {}
"#,
                project_name, language, main_file
            ))
        },
        "python" | "typescript" | "javascript" => {
            // Modern standard: src/ directory structure
            Ok(format!(
                r#"- name: {}
  root: true
  lang: {}
  tree:
    - name: src
      file:
        - name: {}
"#,
                project_name, language, main_file
            ))
        },
        "any" => {
            // Any language: root-level files with specified extensions
            Ok(format!(
                r#"- name: {}
  root: true
  lang: {}
  file:
    - name: {}
"#,
                project_name, language, main_file
            ))
        },
        _ => {
            // Default: src/ directory structure
            Ok(format!(
                r#"- name: {}
  root: true
  lang: {}
  tree:
    - name: src
      file:
        - name: {}
"#,
                project_name, language, main_file
            ))
        }
    }
}

fn append_to_existing_moli_yml(project_name: &str, language: &str) -> Result<()> {
    // Read existing moli.yml
    let existing_content = fs::read_to_string("moli.yml")
        .context("Failed to read existing moli.yml")?;

    // Replace first project's name with "." and remove root: true
    let updated_content = replace_first_project_name_with_current_dir(&existing_content)?;

    // Generate new project YAML
    let main_file = get_main_file_name(language, false); // not root: true
    let new_project_yaml = match language {
        "rust" => {
            // Rust standard: src/main.rs or src/lib.rs
            format!(
                r#"

- name: {}
  lang: {}
  tree:
    - name: src
      file:
        - name: {}
"#,
                project_name, language, main_file
            )
        },
        "go" => {
            // Go standard: main.go at project root for simple projects
            format!(
                r#"

- name: {}
  lang: {}
  file:
    - name: {}
"#,
                project_name, language, main_file
            )
        },
        "python" | "typescript" | "javascript" => {
            // Modern standard: src/ directory structure
            format!(
                r#"

- name: {}
  lang: {}
  tree:
    - name: src
      file:
        - name: {}
"#,
                project_name, language, main_file
            )
        },
        "any" => {
            // Any language: root-level files with specified extensions
            format!(
                r#"

- name: {}
  lang: {}
  file:
    - name: {}
"#,
                project_name, language, main_file
            )
        },
        _ => {
            // Default: src/ directory structure
            format!(
                r#"

- name: {}
  lang: {}
  tree:
    - name: src
      file:
        - name: {}
"#,
                project_name, language, main_file
            )
        }
    };

    // Combine and write back
    let final_content = format!("{}{}", updated_content, new_project_yaml);
    fs::write("moli.yml", final_content)
        .context("Failed to write updated moli.yml")?;

    Ok(())
}

fn generate_sequential_project_name() -> Result<String> {
    let mut counter = 1;

    // If moli.yml exists, check for existing app_X projects
    if fs::metadata("moli.yml").is_ok() {
        let content = fs::read_to_string("moli.yml")
            .context("Failed to read existing moli.yml")?;

        // Find the highest app_X number
        for line in content.lines() {
            if let Some(name_part) = line.strip_prefix("- name: app_") {
                if let Ok(num) = name_part.parse::<i32>() {
                    if num >= counter {
                        counter = num + 1;
                    }
                }
            }
        }
    }

    Ok(format!("app_{}", counter))
}

fn replace_first_project_name_with_current_dir(content: &str) -> Result<String> {
    use regex::Regex;

    // Pattern to match: "- name: <project_name>\n  root: true\n"
    // We need to:
    // 1. Replace the name with "."
    // 2. Remove the "root: true" line
    let pattern = r"(?m)^- name: ([^\n]+)\n  root: true\n";
    let re = Regex::new(pattern)
        .context("Failed to create regex for first project detection")?;

    // Replace first occurrence
    let result = re.replace(content, "- name: .\n");

    Ok(result.to_string())
}

fn get_main_file_name(language: &str, is_root: bool) -> &str {
    match language {
        "rust" => if is_root { "main" } else { "lib" },
        "go" => "main",
        "python" => "main",
        "typescript" => "index",
        "javascript" => "index",
        "any" => "README.md",
        _ => "main",
    }
}
