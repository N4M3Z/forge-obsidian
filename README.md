# forge-obsidian

Obsidian vault conventions for AI sessions. SKILL.md is the single source of truth — system content inline, user extensions via `!`command`` Dynamic Context Injection.

## Layout

```
forge-obsidian/
├── module.yaml              # Module metadata (name, version, events)
├── defaults.yaml            # Default config (checked into git)
├── bin/
│   └── load-user-content.sh # User content loader (called by SKILL.md !`command`)
├── skills/
│   └── ObsidianConventions/
│       └── SKILL.md         # Source of truth: inline content + !`command` for user extensions
├── hooks/
│   ├── hooks.json           # Claude Code hook registration (standalone mode)
│   └── session-start.sh     # Hook script — emits metadata index
├── lib/
│   └── load.sh              # Context loader (standalone fallback, synced from Core)
├── .claude-plugin/
│   └── plugin.json          # Plugin discovery (standalone mode)
└── README.md
```

## Quick Start

```bash
# As a Claude Code plugin (standalone)
claude plugin install forge-obsidian

# Or as part of forge-core (submodule, already included)
git clone forge-obsidian
```

Once active, invoke the skill when working with vault files. Claude Code discovers it automatically. To add your own conventions, create `config.yaml`:

```yaml
user:
  - /path/to/your/ObsidianConventions/
```

All `*.md` files in that directory are loaded as user extensions when the skill runs.

## How It Works

**Claude Code** discovers `skills/ObsidianConventions/SKILL.md` via native skill discovery. At session start, Claude reads the skill's frontmatter (~30 tokens). When working with vault files, Claude invokes the skill — the body contains inline conventions and a `!`command`` block that loads user extensions from config.

**Other providers** use the SessionStart hook, which emits metadata only via `load_context --index-only`. On demand, `load_context` renders the full SKILL.md content including `!`command`` execution.

The SessionStart hook is enabled by default. To disable it (e.g. for Claude Code where skill discovery makes it redundant), set `events: []` in `config.yaml`.

### Content Delivery

| Provider | Discovery | Body loading | Mechanism |
|----------|-----------|-------------|-----------|
| **Claude Code** | SKILL.md frontmatter (native) | `!`command`` preprocessing | Lazy — skill invoked on demand |
| **OpenCode** | SessionStart hook (metadata) | `load_context` with `!`command`` | Lazy (~30 tokens at start) |
| **Cursor / Copilot** | Baked into static config | Already inline | Eager (all tokens at start) |

## Configuration

Config follows `.env.example` / `.env` pattern:

- **`defaults.yaml`** — checked into git, ships with module
- **`config.yaml`** — gitignored, user creates to override (typically adding `user:` paths)

Loader reads `config.yaml` if it exists, else falls back to `defaults.yaml`.

```yaml
# defaults.yaml
system:                           # Used by SessionStart hook for metadata extraction
  - skills/ObsidianConventions/SKILL.md

user: []                          # User extensions loaded by SKILL.md !`command`

metadata:
  name: [name, title]
  description: description
```

To add custom vault conventions, create `config.yaml`:

```yaml
user:
  - /path/to/vault/Conventions/   # directory — all *.md files loaded
```

Entries can be files or directories. Paths resolve module-local first, then project root, then absolute.

## Loading Modes

This module works in multiple configurations depending on your setup:

| Mode | How it loads | When |
|------|-------------|------|
| **forge-core** | Dispatcher sets `FORGE_LIB` — shared `Core/lib/load.sh` | Part of full framework |
| **Claude Code plugin** | `CLAUDE_PLUGIN_ROOT` — local `lib/load.sh` | `claude plugin install` |
| **Other providers** | SessionStart hook via `load_context` | OpenCode, Cursor, etc. |

The local `lib/load.sh` is an identical copy of the Core version, synced for standalone operation.
