---
name: ObsidianActions
version: 0.1.0
description: "[DEPRECATED] Use /ObsidianCLI instead. Obsidian Actions URI — atomic frontmatter props-set, rename with backlink updates, create notes, open notes."
argument-hint: ""
disable-model-invocation: true
---

# ObsidianActions

> **Deprecated** — The official Obsidian CLI (1.12+) replaces this functionality with `property:set`, `rename`, `create`. See `/ObsidianCLI` for the current reference. This skill remains for users on Obsidian < 1.12.

Reference for the Obsidian Actions URI plugin. Communicates with Obsidian via URL schemes (`obsidian://actions-uri/...`). Fire-and-forget — sends commands, no response to the shell.

Best for atomic writes that Obsidian processes internally (property updates, renames with backlink tracking). Falls back to `safe-write` / Write tool when unavailable.

## Setup

1. Install: Obsidian Settings → Community Plugins → search "Actions URI"
2. Enable the plugin — no API key needed

## Wrapper Script

`Hooks/obsidian-uri.sh` wraps common operations. The vault name defaults to `Personal` (override with `OBSIDIAN_VAULT` env var).

### Set frontmatter properties

```bash
Hooks/obsidian-uri.sh props-set "vault/relative/path.md" '{"keywords": ["[[Security]]"], "related": ["[[Obsidian]]"]}'
```

Uses `mode=update` (merge) — existing properties are preserved, new ones are added/updated. Safe for append-only fields like `keywords:` and `related:`.

### Remove frontmatter keys

```bash
Hooks/obsidian-uri.sh props-remove "vault/relative/path.md" '["obsolete_key", "deprecated_field"]'
```

### Rename with backlink updates

```bash
Hooks/obsidian-uri.sh rename "old/path.md" "new/path.md"
```

Obsidian updates all `[[wikilinks]]` across the vault that pointed to the old name. This is the only way to rename with automatic backlink tracking — file-system `mv` breaks links.

### Create a note

```bash
Hooks/obsidian-uri.sh create "path/to/note.md" "Optional initial content"
```

### Open a note in Obsidian

```bash
Hooks/obsidian-uri.sh open "path/to/note.md"
```

### Check note existence

```bash
Hooks/obsidian-uri.sh exists "path/to/note.md"
```

**Warning**: This is unreliable — `open` on macOS always returns exit 0 regardless of whether the note exists. Use the REST API `GET /vault/{path}` (200 vs 404) for reliable existence checks, or the Glob tool as fallback.

## Raw URI Format

If the wrapper script is unavailable, construct URIs directly:

```bash
# URL-encode the path
FILE=$(python3 -c "import urllib.parse,sys; print(urllib.parse.quote(sys.argv[1], safe=''))" "path/to/note.md")
VAULT="${OBSIDIAN_VAULT:-Personal}"

# Set properties
open "obsidian://actions-uri/note-properties/set?vault=${VAULT}&file=${FILE}&properties=$(python3 -c "import urllib.parse,sys; print(urllib.parse.quote(sys.argv[1], safe=''))" '{"key":"value"}')&mode=update&silent=true"

# Rename
NEW=$(python3 -c "import urllib.parse,sys; print(urllib.parse.quote(sys.argv[1], safe=''))" "new/path.md")
open "obsidian://actions-uri/note/rename?vault=${VAULT}&file=${FILE}&new-filename=${NEW}&silent=true"
```

## When to Prefer Actions URI Over File I/O

| Scenario | Why Actions URI wins |
|----------|---------------------|
| Setting frontmatter on AMBER files | Atomic update — no read-modify-write cycle, no Linter race |
| Renaming notes | Obsidian updates all backlinks vault-wide |
| Creating notes that need Linter formatting | Obsidian applies Linter rules on creation |
| Rapid successive property updates | Each `props-set` is independent — no stale-file conflicts |

## Detecting Availability

```bash
if [ -f "${FORGE_USER_ROOT}/.obsidian/plugins/actions-uri/main.js" ]; then
  echo "Actions URI available"
else
  echo "Actions URI unavailable — falling back to safe-write"
fi
```

## Fallback

| Actions URI operation | File-system fallback |
|----------------------|---------------------|
| `props-set` | `safe-write edit` (read → modify YAML → write) |
| `props-remove` | `safe-write edit` |
| `rename` | `command mv` + manual backlink updates (lossy) |
| `create` | Write tool or `safe-write write` |
| `open` | No equivalent |

## Constraints

- Fire-and-forget — no return value, no error feedback to the shell
- Requires Obsidian desktop app to be running
- `exists` command is unreliable (always exit 0) — use REST API instead
- `mode=update` merges properties — use `mode=overwrite` only when intentionally replacing all properties
- URL encoding is required for all path and property arguments
- Does not work in headless/CI environments
