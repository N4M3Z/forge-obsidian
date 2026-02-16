# forge-obsidian

Obsidian vault conventions, Base file resolver, and Draft/Promote skill workflow. Rust crate (`forge-obsidian`).

## Build & Test

```bash
cargo build --release --manifest-path Cargo.toml
cargo test --manifest-path Cargo.toml                          # all tests
cargo test --manifest-path Cargo.toml test_name                # single test by name
cargo clippy --manifest-path Cargo.toml -- -D warnings         # lint
cargo fmt --manifest-path Cargo.toml --check                   # format check
bash tests/test.sh                                             # shell integration tests
```

### Binary

| Binary | Purpose |
|--------|---------|
| `obsidian-base` | Resolve Obsidian Base files (.base) to JSONL |

## Code Style

- **Edition 2021**, `unsafe` forbidden, clippy pedantic enabled
- **Error handling**: `Result<T, String>` — no `anyhow`/`thiserror`
- **Module pattern**: `mod.rs` + sibling `tests.rs` (base, eval, note, vault)
- **Dependencies**: `regex`, `serde`, `serde_json`, `serde_yaml`, `walkdir`

## Skills (7)

Draft, ObsidianBase, ObsidianConventions, ObsidianTemplates, ProjectConventions, Promote, VaultOperations

## Shell Tools

- `bin/forge-draft` — pull a module skill into the vault for editing
- `bin/forge-promote` — push a vault skill back to its module
