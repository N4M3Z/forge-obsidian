---
name: ObsidianCLI
version: 0.1.0
description: Official Obsidian CLI (1.12+) — list files, search content, manage properties, query bases, navigate links, create/read/rename/delete notes. USE WHEN listing vault files, searching vault, managing frontmatter properties, querying bases, checking backlinks, creating notes via Obsidian, renaming with backlink updates, or any vault operation that benefits from Obsidian's internal index.
argument-hint: ""
---

# ObsidianCLI

Reference for the official Obsidian CLI (1.12+). Communicates with the running Obsidian desktop app via named pipe. Returns structured data to stdout.

Falls back to file-system operations (Glob, Grep, safe-read, safe-write) when unavailable.

## Setup

The CLI ships inside Obsidian 1.12+. Add to PATH (one-time):

```bash
export PATH="$PATH:/Applications/Obsidian.app/Contents/MacOS"
```

Or use the full path directly. Verify with `make check` in the module root.

## Parameter Syntax

- Key-value pairs: `key=value`
- Spaces in values: `content="Hello world"`
- File targeting: `file=<name>` (wikilink resolution) or `path=<exact/path.md>`
- Vault targeting: `vault=<name>` or inferred from CWD or active vault

## Output Formats

All commands support: `format=json|csv|tsv|md|paths|text|tree|yaml`

Default is `text`. For programmatic consumption use `format=json` or `format=paths`.

## File Operations

### List files

```bash
obsidian files                              # all files
obsidian files folder=Resources ext=md      # filter by directory and extension
obsidian files format=paths                 # file paths only (one per line)
obsidian files format=json                  # structured JSON
```

### Read file content

```bash
obsidian read file="Security"               # by wikilink name
obsidian read path="Topics/Security.md"     # by exact path
```

### Create a note

```bash
obsidian create name="New Note" content="---\ntitle: New Note\n---\n\nContent here."
obsidian create name="From Template" template=Daily
```

### Append / Prepend

```bash
obsidian append file="Daily" content="- New item"
obsidian prepend path="Topics/Security.md" content="## Update\n\nNew section."
```

`prepend` inserts after frontmatter.

### Move / Rename

```bash
obsidian move file=Recipe to=Archive/
obsidian rename file="Old Name" to="New Name"
```

`rename` updates all wikilinks and markdown links across the vault. This is the only reliable way to rename with backlink tracking — file-system `mv` breaks links.

### Delete

```bash
obsidian delete file=OldNote                # to Obsidian trash
obsidian delete file=OldNote permanent      # permanent
```

## Properties (Frontmatter)

### Set property

```bash
obsidian property:set file="Note" property=status value=active
```

Atomic update — avoids the Obsidian Linter race condition that affects read-modify-write cycles with `safe-write edit`.

### Remove property

```bash
obsidian property:remove file="Note" property=obsolete_key
```

### Read property

```bash
obsidian property:read file="Note" property=status
obsidian property:read file="Note" property=status format=json
```

### Rename property key (vault-wide)

```bash
obsidian property:rename old=old_key new=new_key
```

## Search

### Full-text search

```bash
obsidian search query="forge-tlp"
obsidian search query="forge-tlp" format=json
```

Property filter syntax: `[tag:project]`, `[rating:>4]`, `[status:active]`.

### Search with context

```bash
obsidian search:context query="forge-tlp"
```

## Links & Graph

### Backlinks (incoming links)

```bash
obsidian backlinks file="Security"
obsidian backlinks file="Security" format=json
```

### Outgoing links

```bash
obsidian links file="Security"
```

### Orphans and unresolved

```bash
obsidian orphans                            # notes with no links in or out
obsidian unresolved                         # broken wikilinks
```

## Bases

### Query a Base view

```bash
# Redirect to temp file — stdout race condition on heavy queries (known upstream issue)
obsidian base:query path="Resources/Books.base" format=json > /tmp/base-out.json 2>&1
cat /tmp/base-out.json
command rm -f /tmp/base-out.json

# Path-only output
obsidian base:query path="Resources/Books.base" format=paths
```

When CLI is unavailable, use the standalone `obsidian-base` binary (see `/ObsidianBase`).

## Daily Notes

```bash
obsidian daily                              # open today's daily note (creates if missing)
obsidian daily:read                         # read content
obsidian daily:append content="- [ ] Task"  # append to end
obsidian daily:prepend content="## Morning" # prepend after frontmatter
obsidian daily:path                         # get expected path (no creation)
```

## Developer / Escape Hatch

Execute JavaScript in the running Obsidian context:

```bash
obsidian eval code="app.vault.getFiles().length"
obsidian eval code="app.vault.getMarkdownFiles().map(f => f.path).join('\n')"
```

Has access to `app`, `vault`, `workspace`, `plugins`. Use for operations the CLI doesn't expose natively.

## Fallback Matrix

| CLI operation | When CLI unavailable |
|---|---|
| `files` | Glob tool or `find "$FORGE_USER_ROOT" -name '*.md'` |
| `search` | Grep tool |
| `read` | `safe-read` or Read tool |
| `create` / `append` / `prepend` | `safe-write write` or Write tool |
| `property:set` / `property:remove` | `safe-write edit` (read-modify-write) |
| `move` / `rename` | `command mv` (lossy — no backlink updates) |
| `backlinks` / `links` | Grep for `[[filename]]` patterns |
| `orphans` / `unresolved` | No direct equivalent |
| `base:query` | `obsidian-base` binary (works offline, see `/ObsidianBase`) |
| `daily:*` | Resolve path from `defaults.yaml` journal pattern |
| `delete` | `command rm` (no trash) |

## Constraints

- Requires Obsidian 1.12+ desktop app running (named pipe communication)
- Catalyst license ($25) required during Early Access period
- No headless/remote operation — for that use the Local REST API plugin (see `/ObsidianREST`)
- `base:query` has stdout race condition on heavy queries — use temp file redirect
- TLP access control (`safe-read` / `safe-write`) is a separate concern — CLI bypasses TLP
- 1.12.2 renamed `all` parameter to `active` and `silent` to `open` — pin to 1.12.2+ syntax

!`dispatch skill-load forge-obsidian`
