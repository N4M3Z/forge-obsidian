# forge-obsidian

Obsidian vault conventions for AI sessions. Emits metadata index at SessionStart; body is lazy-loaded by the provider on demand.

## Layout

```
forge-obsidian/
├── module.yaml              # Module metadata (name, version, events)
├── config.yaml              # Content paths + metadata mapping
├── skills/
│   └── ObsidianConventions/
│       └── SKILL.md         # Claude Code skill (frontmatter + !`command` body)
├── src/
│   └── SYSTEM.md            # System content (ALL CAPS = module-provided)
├── hooks/
│   ├── hooks.json           # Claude Code hook registration (standalone mode)
│   └── session-start.sh     # Hook script — emits metadata index
├── lib/
│   └── load.sh              # Context loader (standalone fallback)
├── .claude-plugin/
│   └── plugin.json          # Plugin discovery (standalone mode)
└── README.md
```

## Three-Tier Loading

Content delivery follows PAI's progressive disclosure model. Only Tier 1 metadata is emitted at session start — the body loads on demand when the provider needs it.

| Tier | What | When | Tokens |
|------|------|------|--------|
| **1 — Metadata** | `name`, `description` (USE WHEN triggers) | Session start | ~30 |
| **2 — System content** | `src/SYSTEM.md` body | On demand (skill or file read) | ~100 |
| **3 — User content** | Vault paths from `user:` config | On demand (skill or file read) | varies |

### How It Works

**Claude Code** uses native skill discovery via `skills/ObsidianConventions/SKILL.md`. Claude reads the skill's `description` frontmatter at session start (~30 tokens) and invokes the skill on demand when working with vault files. The SKILL.md body uses `!`command`` preprocessing to dynamically load content through `load_context --body-only`, which pulls system content from `src/SYSTEM.md` and any user content from `config.yaml` paths — including absolute paths outside the repo.

The SessionStart hook is kept for **other providers** that don't support skill discovery. It emits metadata only via `load_context --index-only`:

```
---
name: Obsidian Conventions
description: Vault conventions for wikilinks, frontmatter and tags. USE WHEN working with Obsidian vault files.
---
```

### Content Delivery by Provider

| Provider | Tier 1 (discovery) | Tier 2+3 (body) | Mechanism |
|----------|-------------------|-----------------|-----------|
| **Claude Code** | SKILL.md frontmatter (native) | `!`command`` preprocessing | Lazy — skill invoked on demand |
| **OpenCode** | SessionStart hook | Read tool | Lazy (~30 tokens at start) |
| **Cursor / Copilot** | Baked into static config | Already inline | Eager (all tokens at start) |

## Configuration

```yaml
# config.yaml
system:                         # Loaded first
  - src/SYSTEM.md

user: []                        # Loaded after system (override with vault paths)

# Metadata mapping (Airbyte/dbt convention): output field = key, source = value.
# Fields not listed are stripped. Omit section to strip all.
# Multi-value sources use first-match fallback: name: [name, title]
metadata:
  name: title
  description: description
```

Entries can be files or directories (all `*.md` files loaded). Paths resolve module-local first, then fall back to the project root.

## Dual-Mode Operation

- **forge-core mode**: `FORGE_LIB` is set — sources shared `Core/load.sh`
- **Standalone mode**: No `FORGE_LIB` — sources local `lib/load.sh` (self-contained with embedded parser and frontmatter stripper)

Install standalone:

```bash
claude plugin install forge-obsidian
```

## Skill Discovery

Claude Code discovers the skill via `skills/ObsidianConventions/SKILL.md`. The frontmatter follows the [Agent Skills](https://agentskills.io) standard:

```yaml
---
name: ObsidianConventions
description: Vault conventions for wikilinks, frontmatter and tags. USE WHEN working with Obsidian vault files.
---
```

The body uses `!`command`` preprocessing to dynamically call `load_context --body-only`, which loads `src/SYSTEM.md` body and any user content paths from `config.yaml`. No build script needed — content is resolved at invocation time, with full filesystem access (user paths can be absolute).

## System Content

`src/SYSTEM.md` covers:
- Wikilink usage conventions
- Tag vs keyword conventions
- Path verification rules
