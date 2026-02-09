# forge-obsidian

Obsidian vault conventions for AI sessions. SKILL.md is the single source of truth — system content inline, user extensions via `!`command`` Dynamic Context Injection.

## Layout

```
forge-obsidian/
├── module.yaml              # Module metadata (name, version, events)
├── defaults.yaml            # Default config (checked into git)
├── skills/
│   └── ObsidianConventions/
│       └── SKILL.md         # Source of truth: inline content + !`command` for user extensions
├── hooks/
│   ├── hooks.json           # Claude Code hook registration (standalone mode)
│   └── session-start.sh     # Hook script — emits metadata index (optional for Claude Code)
├── lib/
│   └── load.sh              # Context loader (standalone fallback, synced from Core)
├── .claude-plugin/
│   └── plugin.json          # Plugin discovery (standalone mode)
└── README.md
```

## How It Works

**Claude Code** discovers `skills/ObsidianConventions/SKILL.md` via native skill discovery. At session start, Claude reads the skill's frontmatter (~30 tokens). When working with vault files, Claude invokes the skill — the body contains inline conventions and a `!`command`` block that loads user extensions from config.

**Other providers** use the SessionStart hook, which emits metadata only via `load_context --index-only`. On demand, `load_context` renders the full SKILL.md content including `!`command`` execution.

The SessionStart hook is **optional for Claude Code** — remove `SessionStart` from `module.yaml → events:` to disable it.

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

## Dual-Mode Operation

- **forge-core mode**: `FORGE_LIB` is set — sources shared `Core/lib/load.sh`
- **Standalone mode**: No `FORGE_LIB` — sources local `lib/load.sh` (identical copy, synced from Core)

Install standalone:

```bash
claude plugin install forge-obsidian
```

## Skill Content

`skills/ObsidianConventions/SKILL.md` contains:
- Wikilink usage conventions
- Tag vs keyword conventions
- Path verification rules
- `!`command`` block that loads user content from config
