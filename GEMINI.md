# GEMINI.md

This file provides instructional context for the Gemini AI agent when working with the **forge-obsidian** module.

## Project Overview

**forge-obsidian** is a behavioral-layer module for the **Forge Framework**. It teaches AI agents how to interact with Obsidian vaults using specific conventions (wikilinks, frontmatter, tags) and provides tools for a "Draft/Promote" workflow to author skills directly within an Obsidian vault.

### Core Components
- **Rust Core (`obsidian-base`):** A Rust-based utility for resolving Obsidian Base files (`.base`) to JSONL and performing vault operations.
- **Skills:** Markdown-based behavioral rules located in `skills/`.
    - `ObsidianConventions`: Rules for wikilinks, tags, and frontmatter.
    - `VaultOperations`: Operational rules, TLP integration, and preferences.
    - `Draft` & `Promote`: Workflow for moving skills between modules and the vault workspace.
- **Hooks:** Integration points for the Forge framework (e.g., `SessionStart`).

## Building and Running

### Key Commands
- **Build:** `cargo build --release` (compiles `obsidian-base` binary).
- **Test (Rust):** `cargo test` (runs unit tests in `src/*/tests.rs`).
- **Test (Integration):** `bash tests/test.sh` (runs shell-based integration tests).
- **Lint:** `cargo clippy -- -D warnings` (strict linting).
- **Format:** `cargo fmt --check` (verifies code formatting).
- **Verification:** `bash VERIFY.md` contains a checklist and commands to verify a complete installation.

### Binaries
| Binary | Source | Purpose |
| :--- | :--- | :--- |
| `obsidian-base` | `src/bin/obsidian_base.rs` | Resolves Obsidian Base files to JSONL. |

## Development Conventions

### Rust Coding Style
- **Edition:** Rust 2021.
- **Unsafe:** Strictly forbidden (`#![forbid(unsafe_code)]`).
- **Error Handling:** Use `Result<T, String>`. Do NOT use `anyhow` or `thiserror`.
- **Module Structure:** Follows the pattern of `mod.rs` with a sibling `tests.rs` for each module (e.g., `src/base/mod.rs` and `src/base/tests.rs`).
- **Lints:** Pedantic clippy lints are enabled.

### Obsidian & Vault Conventions
- **Wikilinks:** Use `[[wikilinks]]` for notes, people, projects, and topics.
- **Tags:** Reserved for system, structural, or status use. Do NOT use tags for topics or categories; use wikilinks instead.
- **TLP (Traffic Light Protocol):** Respect file access levels (RED, AMBER, GREEN) as defined by the Forge framework.
- **Draft/Promote Workflow:** Skills can be pulled to the vault for editing (`/Draft`) and pushed back to the module (`/Promote`).

### Shell Usage
- **Bypass Aliases:** Use `command` for external programs and `builtin` for shell builtins to ensure compatibility across different environments.
- **Path Resolution:** Use `Core/bin/paths.sh` (from the Forge root) to resolve shared paths like `FORGE_ROOT`, `SAFE_READ_CMD`, etc.

## Key Files
- `Cargo.toml`: Rust project configuration and dependencies.
- `module.yaml`: Module metadata and event triggers.
- `skills/`: Directory containing all behavioral instructions.
- `hooks/hooks.json`: Definitions for Forge framework hooks.
- `bin/forge-draft` / `bin/forge-promote`: Scripts for the skill authoring workflow.
- `AGENTS.md`: Specific instructions and command references for AI agents.
