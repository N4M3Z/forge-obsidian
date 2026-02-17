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
| [safety-net](https://github.com/kenryu42/claude-code-safety-net) | Recommended | — | Blocks destructive commands — see [root INSTALL.md](../../INSTALL.md#recommended-security-tools) |
| shellcheck | Recommended | `brew install shellcheck` | Shell script linting |
| semgrep | Recommended | `brew install semgrep` | OWASP static analysis for Rust |

## Recommended Obsidian Plugins

Two plugins extend AI vault operations. Both are optional — all skills fall back to file-system operations when unavailable. See `/ObsidianREST` and `/ObsidianActions` skills for full API reference.

| Plugin | Install | Purpose |
|--------|---------|---------|
| [Local REST API](https://github.com/coddingtonbear/obsidian-local-rest-api) | Community plugins → "Local REST API" | Read operations — list files, search content, check note existence via HTTPS |
| [Actions URI](https://github.com/czottmann/obsidian-actions-uri) | Community plugins → "Actions URI" | Write operations — atomic `props-set`, rename with backlink updates |

### REST API setup

1. Install and enable in Obsidian
2. Copy the API key from Settings → Local REST API
3. Create `.env` in the module root (gitignored):

```
OBSIDIAN_REST_API_KEY=<your-api-key>
```

HTTPS on port **27124** (insecure HTTP on 27123 is disabled by default).

### Actions URI setup

Install and enable — no API key needed. Vault name defaults to `Personal` (override with `OBSIDIAN_VAULT` env var).

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
