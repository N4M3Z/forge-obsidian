# forge-obsidian — Installation

> **For AI agents**: This guide covers installation of forge-obsidian. Follow the steps for your deployment mode.

## As part of forge-core (submodule)

Already included as a submodule. Build with:

```bash
make install    # builds all modules including forge-obsidian
```

Or build individually:

```bash
cargo build --release --manifest-path Modules/forge-obsidian/Cargo.toml
```

## Standalone (Claude Code plugin)

```bash
claude plugin install forge-obsidian
```

## Dependencies

| Dependency | Required | Install | Purpose |
|-----------|----------|---------|---------|
| Rust + cargo | Yes | `curl https://sh.rustup.rs -sSf \| sh` | Build `obsidian-base` binary |
| `jq` | Recommended | `brew install jq` | Filter JSONL output from `obsidian-base` |
| `yt-dlp` | Recommended | `brew install yt-dlp` | YouTube metadata, subtitles, audio for vault content |

## What gets installed

| Binary | Purpose |
|--------|---------|
| `obsidian-base` | Resolve Obsidian Base files (`.base`) to JSONL — filtered, sorted vault queries |

## User Extensions

### Inline overrides (User.md)

Create `skills/ObsidianConventions/User.md` with your vault-specific conventions:

```markdown
## My Overrides

- Always use ISO 8601 dates in frontmatter
- Link people with [[FirstName LastName]] format
```

### External vault steering

Create `config.yaml` (gitignored) with paths to external convention directories:

```yaml
steering:
  - /path/to/vault/Orchestration/Steering/
```

Requires forge-steering's `bin/steer` tool. Claude sees the directory listing and reads files on demand.

## Disable SessionStart hook

To disable the SessionStart hook (e.g., for Claude Code where skill discovery makes it redundant):

```yaml
# config.yaml
events: []
```
