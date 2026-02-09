# forge-obsidian

Teaches AI coding assistants how to work with your Obsidian vault — wikilinks, frontmatter conventions, tag usage, and file verification. Works with Claude Code, OpenCode, Cursor, and Copilot.

## Quick Start

```bash
# As a Claude Code plugin (standalone)
claude plugin install forge-obsidian

# Or as part of forge-core (submodule, already included)
git submodule update --init Modules/forge-obsidian
```

Once active, Claude Code discovers the skill automatically when you work with vault files.

## Layout

```
forge-obsidian/
├── module.yaml              # Module metadata (name, version, events)
├── skills/
│   └── ObsidianConventions/
│       └── SKILL.md         # Conventions (the actual content AI reads)
├── hooks/
│   ├── hooks.json           # Claude Code hook registration (standalone mode)
│   ├── session-start.sh     # Emits metadata for non-Claude-Code providers
│   └── skill-load.sh        # Injects external steering content
├── tests/
│   └── test.sh              # Module tests
├── .githooks/
│   └── pre-commit           # Shellcheck lint (if available)
├── .claude-plugin/
│   └── plugin.json          # Plugin discovery (standalone mode)
├── .gitignore
├── CONTRIBUTING.md
├── INSTALL.md
├── VERIFY.md
└── README.md
```

## User Extensions

Two ways to customize (additive — both can be active):

### 1. External vault steering

Point the module at directories outside the repo (e.g., your Obsidian vault's convention files). Create `config.yaml` (gitignored):

```yaml
steering:
  - /path/to/vault/Orchestration/Steering/
```

The AI sees a directory listing and reads specific files on demand. Requires the `forge-steering` module.

### 2. Inline overrides (User.md)

Create `skills/ObsidianConventions/User.md` (gitignored) with your personal rules:

```markdown
## My Overrides

- Always use ISO 8601 dates in frontmatter
- Link people with [[FirstName LastName]] format
```

## How It Works

The core content lives in `SKILL.md` — a markdown file with YAML frontmatter and inline conventions. Different AI providers load it differently:

| Provider | How it discovers the skill | How it loads content |
|----------|---------------------------|---------------------|
| **Claude Code** | Reads SKILL.md frontmatter at session start | Loads full skill on demand when relevant |
| **OpenCode** | SessionStart hook emits metadata | forge-load library transforms and emits content |
| **Cursor / Copilot** | Baked into static config via adapters | Content included at session start |

Claude Code also preprocesses shell commands embedded in SKILL.md (written as `` !`command` ``) — these inject external steering content and user overrides at the moment the skill is invoked.

**Note for Claude Code users**: The SessionStart hook (`session-start.sh`) exists for non-Claude-Code providers. Claude Code's skill discovery handles content loading directly, so you can disable it with `events: []` in `config.yaml`.

## Configuration

**module.yaml** — checked into git:

```yaml
name: forge-obsidian
version: 0.5.0
description: Obsidian vault conventions. USE WHEN working with Obsidian vault files.
events:
  - SessionStart
metadata:
  name: [name, title]
  description: description
steering: []
```

**config.yaml** — gitignored, user creates to configure:

```yaml
# Disable SessionStart hook (Claude Code doesn't need it)
events: []

# External vault steering paths
steering:
  - /path/to/vault/Orchestration/Steering/
```

## Dependencies

| Module | Required | Purpose |
|--------|----------|---------|
| **forge-load** | Optional | Content loading for non-Claude-Code providers |
| **forge-steering** | Optional | External steering via `bin/steer` tool |

Both degrade gracefully when absent.

## Testing

```bash
bash tests/test.sh
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for code style and linting requirements.
