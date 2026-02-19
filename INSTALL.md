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
| Obsidian 1.12+ | Recommended | [obsidian.md](https://obsidian.md) | CLI for vault operations (files, properties, search, links, bases) |
| Rust + cargo | Yes | `curl https://sh.rustup.rs -sSf \| sh` | Build `obsidian-base` binary |
| `jq` | Recommended | `brew install jq` | Filter JSONL output from `obsidian-base` |
| `yt-dlp` | Recommended | `brew install yt-dlp` | YouTube metadata, subtitles, audio for vault content |
| [safety-net](https://github.com/kenryu42/claude-code-safety-net) | Recommended | — | Blocks destructive commands — see [root INSTALL.md](../../INSTALL.md#recommended-security-tools) |
| shellcheck | Recommended | `brew install shellcheck` | Shell script linting |
| semgrep | Recommended | `brew install semgrep` | OWASP static analysis for Rust |

### Obsidian CLI setup

The CLI ships inside Obsidian 1.12+. Add to PATH (one-time):

```bash
export PATH="$PATH:/Applications/Obsidian.app/Contents/MacOS"
```

Or use the full path directly. Requires a running Obsidian desktop app (communicates via named pipe). Catalyst license ($25) required during Early Access. See `/ObsidianCLI` for the full command reference.

## Legacy Obsidian Plugins (deprecated)

> **Deprecated** — The Obsidian CLI (1.12+) replaces both plugins below. They remain for users on Obsidian < 1.12 or headless/remote environments.

| Plugin | Install | Purpose |
|--------|---------|---------|
| [Local REST API](https://github.com/coddingtonbear/obsidian-local-rest-api) | Community plugins → "Local REST API" | Read operations — list files, search content, check note existence via HTTPS |
| [Actions URI](https://github.com/czottmann/obsidian-actions-uri) | Community plugins → "Actions URI" | Write operations — atomic `props-set`, rename with backlink updates |

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
