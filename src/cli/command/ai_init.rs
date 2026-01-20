use clap::Command;
use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn spec() -> Command {
    Command::new("ai-init")
        .about("Generate Claude Code custom command for moli initialization")
        .long_about(
            "Creates .claude/commands/moli_init.md file that provides a custom command \
            for Claude Code to automatically document moli.yml specification in CLAUDE.md"
        )
}

pub fn action() -> Result<()> {
    // .claude/commandsディレクトリを作成
    let commands_dir = Path::new(".claude/commands");
    fs::create_dir_all(commands_dir)?;

    // moli_init.mdファイルのパスを作成
    let file_path = commands_dir.join("moli_init.md");

    // カスタムコマンドの内容を生成
    let content = generate_custom_command_content();

    // ファイルに書き込み
    fs::write(&file_path, content)?;

    println!("✓ Created Claude Code custom command: .claude/commands/moli_init.md");
    println!();
    println!("Usage:");
    println!("  Run '/moli_init' in Claude Code to update CLAUDE.md with moli.yml documentation");

    Ok(())
}

fn generate_custom_command_content() -> String {
    let version = env!("CARGO_PKG_VERSION");
    format!(r##"# moli_init - moli.yml Documentation Generator

このカスタムコマンドは、プロジェクトのCLAUDE.mdファイルに「## moli.ymlの書き方」セクションを追加します。

## 指示

Claude Code (claude.ai/code) へ:

プロジェクトのCLAUDE.mdファイルに「## moli.ymlの書き方」という見出しで、
moli.yml設定ファイルの書き方に関する詳細なドキュメントを追加してください。

## moliとは

moliは、YAML仕様に基づいてコードを生成する宣言的開発フレームワークです。
「moli」は日本語の「森」を表しており、tree:として階層を深くしていく構造を表現しています。

### 主な機能
- **マルチ言語対応**: Rust、Go、Python、TypeScript、JavaScript、Markdownの6言語をサポート
- **プロジェクト構造の自動生成**: moli.ymlに記述したモジュール構造を元に、ディレクトリとコードファイルを自動生成
- **設定ファイルの自動生成**: 各言語に応じた設定ファイル（Cargo.toml、package.json、go.modなど）を自動作成
- **マルチプロジェクト対応**: 複数の言語のプロジェクトを一つのワークスペースで管理可能

### moliを使うメリット
1. **構造設計に集中**: ファイル作成の手間を削減し、アーキテクチャ設計に集中できる
2. **一貫性の確保**: 命名規則やモジュール構成を統一的に管理
3. **迅速なプロトタイピング**: 新規プロジェクトやモジュールの追加が高速
4. **リファクタリングの容易さ**: moli.ymlを編集するだけで構造を再編成可能

### 基本的な使い方
```bash
# プロジェクト初期化（対話モード）
moli new

# プロジェクト初期化（言語指定）
moli new --lang rust

# moli.ymlに基づいてコードを生成
moli up
```

## 指示内容

1. **既存セクションの確認**
   - CLAUDE.mdに既に「moli.ymlの書き方」セクションが存在するか確認
   - 存在する場合は、内容が最新の仕様（v{version}）と一致するか確認

2. **セクションの追加/更新**
   - セクションが存在しない場合：新規に追加
   - セクションが古い場合：最新の仕様に基づいて更新
   - セクションが最新の場合：更新不要

3. **記載すべき内容**

   以下の項目を含めてください：

   ### 基本構造
   - moli.ymlの基本的なYAML配列構造
   - 必須フィールド（name, lang）
   - オプションフィールド（root, spec, file）

   ### サポート言語
   - rust, go, python, typescript, javascript, markdown
   - 各言語の特徴と生成されるファイル

   ### フィールド詳細
   - **name**: プロジェクト名（kebab-case推奨）
   - **root**: 単一プロジェクトモードのフラグ（true/false）
   - **lang**: プログラミング言語の指定
   - **spec**: プロジェクトのトップレベル構造（モジュール/ディレクトリの階層）
   - **tree**: サブモジュール/サブディレクトリの階層構造（再帰的に使用）
   - **file**: 各レベルのコードファイル定義

   ### ファイル拡張子の処理
   - 拡張子あり（Button.tsx, Modal.vue）：そのまま保持
   - 拡張子なし（utils）：言語固有の拡張子を自動付与

   ### 言語別のプロジェクト例
   各サポート言語について、実際のmoli.yml例を記載：
   - **Rust**: src/ディレクトリ構造、Cargo.toml、ワークスペース対応
   - **TypeScript**: package.json、tsconfig.json、バレルエクスポート
   - **Go**: go.mod、パッケージ命名規則
   - **Python**: setup.py、__init__.py、バレルインポート
   - **JavaScript**: package.json、ESモジュール対応
   - **Markdown**: ドキュメント構造、README.md

   ### マルチプロジェクト構成
   - 複数言語の混在例
   - ワークスペース自動生成の仕組み
   - root: trueフラグの管理

   ### ファイル保護システム
   moliは3段階のファイル保護システムを実装しています：

   #### 1. コードファイル（完全保護）
   - **ファイル**: .rs, .go, .py, .js, .ts, .jsx, .tsx, .vue, など
   - **動作**: 一度作成されると、これらのファイルは決して上書きされない
   - **目的**: 開発者の実装コードを保護

   #### 2. 管理ファイル（部分更新）
   - **ファイル**: mod.rs, lib.rs, main.rs, __init__.py, index.js, index.ts
   - **動作**: ヘッダー/フッターマーカー間のmoli管理セクションのみを更新
   - **マーカー例**:
     - Rust: `// start auto exported by moli.` ... `// end auto exported by moli.`
     - Python: `# start auto exported by moli.` ... `# end auto exported by moli.`
     - JS/TS: `// start auto exported by moli.` ... `// end auto exported by moli.`
   - **目的**: moliがインポート/エクスポートを管理しながらカスタムコードを保持

   #### 3. プロジェクト設定ファイル（初回作成のみ）
   - **ファイル**: package.json, tsconfig.json, Cargo.toml, go.mod, go.sum, requirements.txt, setup.py
   - **動作**: 存在しない場合のみ作成
   - **目的**: ユーザーのプロジェクト設定変更の上書きを防ぐ

   ### ベストプラクティス
   - プロジェクト命名規則
   - モジュール構成の推奨パターン
   - ファイル拡張子の指定方針

   ### moli.yml管理対象
   - **推奨対象**: コードファイル、ドキュメントファイル、モジュールディレクトリ
   - **非推奨対象**: アセットファイル、設定ファイル、ビルド成果物

## 参考情報

現在のmoliバージョン: v{version}

## 実行後の確認

- CLAUDE.mdに「moli.ymlの書き方」セクションが正しく追加/更新されたか確認
- 日本語で記載されているか確認（プロジェクトのCLAUDE.mdは日本語で記載）
- コード例が適切に記載されているか確認

---

上記の指示に従って、CLAUDE.mdを更新してください。
"##, version = version)
}
