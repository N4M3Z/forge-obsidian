# forge-obsidian

Obsidian vault conventions for AI sessions. SKILL.md is the single source of truth — system content inline, user extensions via `!`command`` Dynamic Context Injection.

## Layout

```
forge-obsidian/
├── module.yaml              # Module metadata (name, version, events, metadata map)
├── skills/
│   └── ObsidianConventions/
│       └── SKILL.md         # Source of truth: inline conventions + 2 !commands
├── hooks/
│   ├── hooks.json           # Claude Code hook registration (standalone mode)
│   └── session-start.sh     # Hook script — forge-load or awk fallback
├── test.sh                  # Module tests
├── INSTALL.md               # Installation guide
├── VERIFY.md                # Verification guide
├── .claude-plugin/
│   └── plugin.json          # Plugin discovery (standalone mode)
├── .gitignore
└── README.md
```

## Quick Start

```bash
# As a Claude Code plugin (standalone)
claude plugin install forge-obsidian

# Or as part of forge-core (submodule, already included)
git clone forge-obsidian
```

Once active, invoke the skill when working with vault files. Claude Code discovers it automatically.

## User Extensions

Two mechanisms (additive — both can be active):

### 1. External vault steering (via forge-steering)

Create `config.yaml` (gitignored) with paths to external convention directories:

```yaml
steering:
  - /path/to/vault/Orchestration/Steering/
```

The SKILL.md `!`command`` invokes forge-steering's `bin/steer` tool, which `tree`s the directory. Claude reads specific files on demand.

### 2. Inline overrides (User.md)

Create `skills/ObsidianConventions/User.md` (gitignored) with your conventions:

```markdown
## My Overrides

- Always use ISO 8601 dates in frontmatter
- Link people with [[FirstName LastName]] format
```

## How It Works

**Claude Code** discovers `skills/ObsidianConventions/SKILL.md` via native skill discovery. At session start, Claude reads the skill's frontmatter (~30 tokens). When working with vault files, Claude invokes the skill — the body contains inline conventions and two `!`command`` blocks that inject external steering content and User.md overrides.

**Other providers** use the SessionStart hook, which tries forge-load for metadata emission (convention mode reads module.yaml for field mapping), falling back to awk frontmatter extraction.

### Content Delivery

| Provider | Discovery | Body loading | Mechanism |
|----------|-----------|-------------|-----------|
| **Claude Code** | SKILL.md frontmatter (native) | `!`command`` preprocessing | Lazy — skill invoked on demand |
| **OpenCode** | SessionStart hook (metadata) | forge-load `load_context` | Lazy (~30 tokens at start) |
| **Cursor / Copilot** | Baked into static config | Already inline | Eager (all tokens at start) |

## Configuration

**module.yaml** — checked into git, contains metadata field mapping:

```yaml
name: forge-obsidian
version: 0.4.0
description: Obsidian vault conventions. USE WHEN working with Obsidian vault files.
events:
  - SessionStart
metadata:
  name: [name, title]
  description: description
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
| **forge-load** | Optional | Lazy loading for non-Claude-Code providers |
| **forge-steering** | Optional | External steering via `bin/steer` tool |

Both degrade gracefully when absent — awk fallback for session-start, silent no-op for steer.

## Testing

```bash
bash Modules/forge-obsidian/test.sh
```
