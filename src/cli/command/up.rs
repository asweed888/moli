use clap::Command;
use anyhow::{bail, Context, Result};
use std::fs;
use std::path::Path;
use crate::project_management::config::{ConfigParser, ConfigValidator};
use crate::code_generation::core::generator::CodeGenerator;

pub fn spec() -> Command {
    Command::new("up")
        .about("Generate project structure from moli.yml configuration")
        .long_about(
            "Generate complete project structures from moli.yml specification. \
            Supports both single-project and multi-project configurations.\n\
            \n\
            Single Project Mode:\n  \
            Projects with 'root: true' generate directly in the current directory\n\
            \n\
            Multi-Project Mode:\n  \
            Creates separate directories for each project with workspace support\n\
            \n\
            Features:\n\
            • Multi-language support: Rust, Go, Python, TypeScript, JavaScript\n\
            • Language-specific project files (Cargo.toml, package.json, go.mod, etc.)\n\
            • Intelligent module structure with barrel exports/imports\n\
            • Workspace management for Rust multi-project setups\n\
            • File extension preservation (.tsx, .jsx, .vue, etc.)\n\
            \n\
            Prerequisites:\n\
            • moli.yml must exist (create with 'moli new')\n\
            • Valid YAML configuration with supported languages"
        )
        .subcommand(
            Command::new("claude-skill")
                .about("Generate Claude Code skill for moli-based system development")
                .long_about(
                    "Creates .claude/skills/moli/SKILL.md file that provides a Claude Code skill \
                    for system development using moli declarative framework"
                )
        )
}

pub fn action(sub_matches: &clap::ArgMatches) -> Result<()> {
    match sub_matches.subcommand() {
        Some(("claude-skill", _)) => action_claude_skill(),
        _ => action_generate(),
    }
}

fn action_generate() -> Result<()> {
    // Check if moli.yml exists
    if !ConfigParser::config_exists() {
        bail!("moli.yml not found. Run 'moli new' to create a new project configuration.");
    }

    // Parse configuration
    let config = ConfigParser::parse_default()
        .context("Failed to parse moli.yml")?;

    // Validate configuration
    ConfigValidator::validate(&config)
        .context("Configuration validation failed")?;

    // Print generating message for each project
    for project in config.projects() {
        println!("Generating project: {}", project.name());
    }

    // Generate structure using the new CodeGenerator
    CodeGenerator::generate_from_config(".", &config)
        .context("Failed to generate project structure")?;

    // Print success message for each project
    for project in config.projects() {
        println!("  ✓ Generated {} ({}) structure", project.name(), project.language());
    }

    println!("[Success] generate of moli has been completed.");
    Ok(())
}

fn action_claude_skill() -> Result<()> {
    // .claude/skills/moliディレクトリを作成
    let skills_dir = Path::new(".claude/skills/moli");
    fs::create_dir_all(skills_dir)?;

    // SKILL.mdファイルのパスを作成
    let file_path = skills_dir.join("SKILL.md");

    // スキルの内容を生成
    let content = generate_skill_content();

    // ファイルに書き込み
    fs::write(&file_path, content)?;

    println!("✓ Created Claude Code skill: .claude/skills/moli/SKILL.md");

    Ok(())
}

fn generate_skill_content() -> String {
    let version = env!("CARGO_PKG_VERSION");
    format!(r##"---
name: moli
description: moliフレームワークを使った宣言的システム開発スキル。moli.ymlの設計・編集・コード生成を支援します。
---

# moli - 宣言的システム開発スキル

このスキルは、moliフレームワーク（v{version}）を使ったシステム開発をClaude Codeで効率的に行うためのものです。

## moliとは

moliは、YAML仕様（`moli.yml`）からコードを生成する宣言的開発フレームワークです。
「moli」は日本語の「森」を表しており、tree:として階層を深くしていく構造を表現しています。

### 対応言語
rust, go, python, typescript, javascript, markdown

## 基本コマンド

```bash
moli new              # プロジェクト初期化（対話モード）
moli new --lang rust  # プロジェクト初期化（言語指定）
moli up               # moli.ymlに基づいてコード生成
moli rm               # moli.ymlから削除されたモジュールを削除
moli scan             # 既存プロジェクト構造からmoli.ymlを生成
```

## moli.yml設計ガイド

### 基本構造

```yaml
- name: プロジェクト名
  root: true           # ルートプロジェクト（カレントディレクトリに生成）
  lang: rust            # 対象言語
  tree:                 # ディレクトリ構造（再帰的）
    - name: src
      file:             # 生成するファイル
        - name: main
      tree:
        - name: domain
          file:
            - name: user
```

### フィールド詳細

| フィールド | 必須 | 説明 |
|-----------|------|------|
| `name` | ✓ | プロジェクト名またはディレクトリ名（kebab-case推奨） |
| `root` | - | `true`でカレントディレクトリに直接生成 |
| `lang` | ✓ | プログラミング言語の指定 |
| `tree` | - | サブディレクトリの階層構造（再帰的に使用） |
| `file` | - | 生成するコードファイルの定義 |

### ファイル拡張子の処理

- **拡張子あり**（`Button.tsx`, `Modal.vue`）：そのまま保持
- **拡張子なし**（`utils`）：言語固有の拡張子を自動付与

### 言語別テンプレート

#### Rust
```yaml
- name: my-app
  root: true
  lang: rust
  tree:
    - name: src
      file:
        - name: main
      tree:
        - name: handlers
          file:
            - name: user
        - name: models
          file:
            - name: user
```

#### TypeScript
```yaml
- name: my-app
  root: true
  lang: typescript
  tree:
    - name: src
      tree:
        - name: components
          file:
            - name: Button.tsx
            - name: utils
      file:
        - name: index
```

#### Go
```yaml
- name: my-app
  root: true
  lang: go
  tree:
    - name: pkg
      tree:
        - name: models
          file:
            - name: user
        - name: handlers
          file:
            - name: api
  file:
    - name: main
```

#### Python
```yaml
- name: my-app
  root: true
  lang: python
  tree:
    - name: src
      tree:
        - name: domain
          file:
            - name: user
        - name: api
          file:
            - name: routes
```

### マルチプロジェクト構成

複数言語のプロジェクトを1つのmoli.ymlで管理できます:

```yaml
- name: frontend
  lang: typescript
  tree:
    - name: src
      tree:
        - name: components
          file:
            - name: App.tsx

- name: backend
  lang: rust
  tree:
    - name: src
      file:
        - name: main
      tree:
        - name: handlers
          file:
            - name: user
```

- 最初のプロジェクトには`root: true`が自動設定
- Rustの場合、複数プロジェクトでワークスペース構成を自動生成

## ファイル保護システム（3層）

### 1. コードファイル（完全保護）
- `.rs`, `.go`, `.py`, `.js`, `.ts`, `.tsx`, `.vue`等
- 一度作成されたファイルは決して上書きされない
- 開発者の実装コードを保護

### 2. 管理ファイル（部分更新）
- `mod.rs`, `lib.rs`, `main.rs`, `__init__.py`, `index.js`, `index.ts`
- マーカー間のmoli管理セクションのみを更新
- マーカー例: `// start auto exported by moli.` ... `// end auto exported by moli.`

### 3. 設定ファイル（初回のみ）
- `Cargo.toml`, `package.json`, `go.mod`等
- 存在しない場合のみ作成

## 開発ワークフロー

### 新規プロジェクト開始時

1. `moli new --lang <言語>` でプロジェクトを初期化
2. `moli.yml` を編集してモジュール構造を設計
3. `moli up` でコード生成
4. 生成されたファイルに実装コードを記述

### モジュール追加時

1. `moli.yml` の `tree` / `file` にモジュールを追加
2. `moli up` で新しいモジュールのみ生成（既存コードは保護）

### モジュール削除時

1. `moli.yml` から該当モジュールを削除
2. `moli rm` で不要なファイル/ディレクトリを削除

### 既存プロジェクトへの導入

1. `moli scan` で既存の構造から `moli.yml` を自動生成
2. 必要に応じて `moli.yml` を調整

## ベストプラクティス

- **構造設計を先に行う**: コードを書く前に`moli.yml`でモジュール構造を設計する
- **kebab-caseで命名**: プロジェクト名やモジュール名はkebab-caseを使用
- **適切な粒度**: 1ファイル1責任を意識してモジュールを分割
- **moli.ymlで管理しないもの**: アセットファイル、設定ファイル、ビルド成果物
"##, version = version)
}
