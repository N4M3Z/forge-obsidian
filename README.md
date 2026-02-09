# forge-obsidian

Obsidian vault conventions for AI sessions. Emits metadata index at SessionStart; body is lazy-loaded by the provider on demand.

## Layout

```
forge-obsidian/
├── module.yaml              # Module metadata (name, version, events)
├── config.yaml              # Content paths + metadata mapping
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

At SessionStart, the hook emits **metadata only** via `load_context --index-only`:

```
---
name: Obsidian Conventions
description: Vault conventions for wikilinks, frontmatter and tags. USE WHEN working with Obsidian vault files.
---
```

This tells the AI what content exists and when to use it. When the AI encounters a matching situation (working with vault files), it loads the full body through its native file-reading capability.

### Content Delivery by Provider

Every provider gets Tier 1 metadata at session start. Tier 2+3 body loading differs:

| Provider | Session start | Body loading | Mechanism |
|----------|--------------|--------------|-----------|
| **Claude Code** | Metadata (hook or native skill discovery) | Skill tool invocation | Lazy (~30 tokens at start) |
| **OpenCode** | Metadata (hook via `forge-plugin.ts`) | Read tool | Lazy (~30 tokens at start) |
| **Cursor / Copilot** | Baked into static config | Already inline | Eager (all tokens at start) |

Claude Code also supports native skill discovery — it auto-loads skill frontmatter without needing the SessionStart hook. If registered as a skill, the hook becomes redundant but harmless.

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

## System Content

`src/SYSTEM.md` covers:
- Wikilink usage conventions
- Tag vs keyword conventions
- Path verification rules
