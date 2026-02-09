# forge-obsidian — Installation

> **For AI agents**: This guide covers installation of forge-obsidian. Follow the steps for your deployment mode.

## As part of forge-core

Already included as a submodule. No additional setup needed.

## Standalone (Claude Code plugin)

```bash
claude plugin install forge-obsidian
```

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
