# forge-obsidian

Obsidian vault conventions, Base file resolver, and Draft/Promote skill workflow.
Standalone Rust crate (not a workspace). Lives as a submodule under `forge-core`.

## Build & Test

```bash
cargo build --release --manifest-path Cargo.toml
cargo test --manifest-path Cargo.toml                          # all tests (22 unit tests)
cargo test --manifest-path Cargo.toml test_name                # single test by name
cargo test --manifest-path Cargo.toml -- --nocapture            # show println output
cargo clippy --manifest-path Cargo.toml -- -D warnings         # lint (pedantic)
cargo fmt --manifest-path Cargo.toml --check                   # format check
bash tests/test.sh                                             # shell integration tests (~26 assertions)
```

After any code change, run at minimum: `cargo clippy -- -D warnings && cargo test`.

### Binary

| Binary | Path | Purpose |
|--------|------|---------|
| `obsidian-base` | `src/bin/obsidian_base.rs` | Resolve `.base` files to JSONL |

Shell wrapper at `bin/obsidian-base` lazy-builds then execs the Rust binary.

## Project Layout

```
src/
  lib.rs                  # re-exports: pub mod base, eval, note, vault
  bin/obsidian_base.rs    # CLI binary entry point
  base/  mod.rs tests.rs  # .base YAML parser (BaseSpec, ViewSpec, FilterNode)
  eval/  mod.rs tests.rs  # expression tokenizer, parser, AST, evaluator
  note/  mod.rs tests.rs  # NoteContext, frontmatter parsing, vault walking
  vault/ mod.rs tests.rs  # vault root discovery (find .obsidian/)
tests/test.sh             # shell integration tests
skills/                   # 11 skill definitions (SKILL.md files)
hooks/                    # session-start.sh, skill-load.sh, hooks.json
bin/                      # forge-draft, forge-promote, obsidian-base (shell wrappers)
```

## Code Style

### Rust Edition & Lints

- **Edition 2021**. `unsafe_code = "forbid"` at crate level.
- **Clippy**: `all` + `pedantic` at warn. Four allowed exceptions:
  `module_name_repetitions`, `must_use_candidate`, `missing_errors_doc`, `missing_panics_doc`.
- No `rustfmt.toml` -- uses default `cargo fmt` settings.

### Imports

Order: crate-internal, external crates, std. No blank lines between groups.

```rust
use crate::note::NoteContext;           // 1. internal
use serde_yaml::Value;                  // 2. external
use std::collections::HashMap;          // 3. std
use std::path::{Path, PathBuf};         // curly braces for multiple items
```

- Each `std` sub-module gets its own `use` line.
- `self` import for namespace access: `use forge_obsidian::base::{self, FilterEntry}`.
- No wildcard `use` in production code (only `use super::*` in tests).

### Error Handling

`Result<T, String>` everywhere. No `anyhow`, `thiserror`, or custom error types.

```rust
// map_err + format! is the standard pattern
let content = fs::read_to_string(path)
    .map_err(|e| format!("Cannot read {}: {e}", path.display()))?;
```

- `?` for propagation within Result-returning functions.
- `eprintln!` for user-facing errors in the binary.
- `Option` where absence is not an error; `unwrap_or_default()` for non-critical fallbacks.
- `let Some(x) = expr else { return ... };` for early returns from Option.
- Binary `main() -> ExitCode`, not `Result`.

### Naming

| Kind | Convention | Examples |
|------|-----------|----------|
| Functions | `snake_case`, verb prefixes | `parse_file`, `eval_filter`, `extract_tags`, `find_vault` |
| Types | `PascalCase`, domain nouns | `BaseSpec`, `NoteContext`, `Val`, `FilterNode` |
| Modules | `snake_case`, single word | `base`, `eval`, `note`, `vault` |
| Constants | `SCREAMING_SNAKE_CASE` | `WIKILINK_RE` |
| Tests | `snake_case`, no `test_` prefix | `parse_simple_base`, `eval_contains_file_path` |

### Types & Traits

- No type aliases. Concrete types throughout, no generics on custom types.
- Only `derive` traits: `Debug`, `Clone`, `PartialEq`, `Eq`, `Copy` as needed.
- No `#[derive(Serialize, Deserialize)]` -- all serde is manual via `Value` tree traversal.
- `Box<Expr>` for recursive AST nodes.
- Struct fields are `pub` on public structs.

### Serde Patterns

- YAML input: `serde_yaml::from_str::<Value>` then manual `.get()` / `.as_str()` traversal.
- JSON output: `serde_json::json!()` macro, `serde_json::Map::new()` for dynamic objects.
- `serde_yaml::Value` stored directly as frontmatter property type in `NoteContext`.

### String Handling

- `.to_owned()` for `&str` -> `String` (preferred over `.to_string()` for str).
- `.display()` for paths in error messages (not `.to_str()`).
- `.to_string_lossy().into_owned()` for path-to-String storage.
- Inline format strings: `format!("Cannot read {}: {e}", path.display())`.
- `"literal".into()` when inserting into maps.

### Documentation

- `///` doc comments on all `pub` items. Single-line, sentence case, period at end.
- Section dividers in large files: `// --- Section Name -------...`
- Inline `//` comments only for non-obvious logic.

### Module Organization

Each module follows `mod.rs` + sibling `tests.rs`:

```rust
// In mod.rs, at top:
#[cfg(test)]
mod tests;
```

`lib.rs` is minimal -- only `pub mod` declarations.

### Testing

- Test files: `use super::*;` to import parent module.
- Factory helpers: `make_note()`, `make_this()`, `make_vault()` for test data setup.
- `tempfile::tempdir()` for filesystem isolation.
- `assert_eq!` for equality, `assert!(matches!(...))` for patterns.
- Raw strings `r#"..."#` for expressions containing quotes.
- `.unwrap()` freely in tests (never in production except `Regex::new` on constants).
- Dev deps: `tempfile`, `assert_cmd`, `predicates`.

### Regex

- `std::sync::LazyLock<Regex>` for static compiled patterns (not `lazy_static!`).
- Hand-written tokenizer in `eval` (no regex for parsing).

## Shell Conventions

All shell scripts use `set -euo pipefail` (test.sh uses `set -uo pipefail`).
- `command cd/cp/mv/rm` instead of bare (aliases may intercept).
- Pre-commit hook runs `shellcheck -S warning` on staged `.sh` files.

## Shell Tools

- `bin/forge-draft` -- pull a module skill into the vault for editing
- `bin/forge-promote` -- push a vault skill back to its module
