---
description: Resolve Obsidian Base files (.base) to see what the user sees in Obsidian. USE WHEN the user asks about Base views, queries vault notes, wants to see what a .base file shows, or needs filtered/sorted vault data. Wraps the obsidian-base CLI.
arguments: "[path to .base file or description of what to query]"
name: ObsidianBase
---

## ObsidianBase

Resolve `.base` files against the vault — the same declarative queries [[Obsidian]] renders internally. Returns matching notes as [[JSONL]] or file paths, enabling [[Claude]] to see exactly what the user sees in their [[Obsidian Base]] views.

### When to Use

- User asks "what does this [[Base]] show?" or "what books are on my reading list?"
- You need to query vault notes by [[frontmatter]] properties, [[tags]], paths, or [[wikilinks]]
- A `.base` file is referenced in conversation and you want to resolve it
- You need filtered, sorted note lists for any vault operation

### Tool

**Binary:** `Modules/forge-obsidian/bin/obsidian-base`

The binary auto-discovers the vault root by walking up from the `.base` file looking for `.obsidian/`. It reads the [[YAML]] query, walks all `.md` files, evaluates filters against [[frontmatter]] + file metadata, applies sorts, and emits results.

### Dependencies

| Dependency | Required | Purpose |
| ---------- | -------- | ------- |
| [[Rust]] (`cargo`) | Yes | Builds the `obsidian-base` binary on first run (lazy compilation) |
| [[jq]] | No | Post-filter [[JSONL]] output (optional) |

The binary is self-contained after compilation — no runtime dependencies beyond the standard library.

### Usage

```bash
# Default: JSONL output (one JSON object per matched note, per view)
Modules/forge-obsidian/bin/obsidian-base "/path/to/File.base"

# Resolve only a specific view
Modules/forge-obsidian/bin/obsidian-base "/path/to/File.base" --view "Table"

# File paths only (one per line) — for piping to other tools
Modules/forge-obsidian/bin/obsidian-base "/path/to/File.base" --paths

# Count matches
Modules/forge-obsidian/bin/obsidian-base "/path/to/File.base" --paths | wc -l

# Filter JSONL with jq
Modules/forge-obsidian/bin/obsidian-base "/path/to/File.base" | jq 'select(.tags | contains(["type/item"]))'
```

### Intent-to-Flag Mapping

| User Says                           | Flag               | Effect                                    |
| ----------------------------------- | ------------------ | ----------------------------------------- |
| "show me what this Base has"        | *(default)*        | [[JSONL]] output with all view columns    |
| "which view", "only the Table view" | `--view "Name"`    | Resolve a single named view               |
| "just the files", "list the paths"  | `--paths`          | One file path per line                    |
| "how many notes match"              | `--paths \| wc -l` | Count of matching notes                   |
| "filter by tag/property"            | pipe to `[[jq]]`   | Post-filter [[JSONL]] with [[jq]] expressions |

### Output Format

#### JSONL (default)

Each line is a self-contained [[JSON]] object:

```json
{"view":"Table","file":"Resources/Books/The Pragmatic Programmer.md","name":"The Pragmatic Programmer","tags":["type/item/book"],"item.read":true}
```

Fields include:
- `view` — which view matched this note
- `file` — vault-relative path
- `name` — note name (stem, no extension)
- Plus any columns defined in the view's `order` list ([[frontmatter]] properties, file metadata)

#### Paths mode (`--paths`)

```
Resources/Books/The Pragmatic Programmer.md
Resources/Books/Designing Data-Intensive Applications.md
```

### Finding .base Files

Base files live alongside the content they query. Common locations:

```bash
# Find all .base files in a vault
find /path/to/vault -name "*.base" -type f
```

### Expression Surface

The `.base` filter [[DSL]] supports:

| Category        | Examples                                                                       |
| --------------- | ------------------------------------------------------------------------------ |
| File properties | `file.name`, `file.path`, `file.ext`, `file.folder`, `file.tags`, `file.links` |
| Frontmatter     | `property.key` or bare `key`                                                   |
| Context         | `this.file.name`, `this.file.path`, `this.file.folder`                         |
| String methods  | `.startsWith()`, `.endsWith()`, `.contains()`, `.toString()`, `.slice(n,m)`    |
| Functions       | `contains(collection, value)`, `file.hasTag("tag")`, `file.hasLink("link")`    |
| Operators       | `!=`, `!` (prefix negation)                                                    |
| Combinators     | `and: […]`, `or: […]` ([[YAML]]-level boolean logic)                            |

### Limitations

- **`this` context** — `this.file.*` refers to the note embedding the [[Base]], not the `.base` file itself. When resolving template [[Bases]] (e.g., `Daily.base`) from CLI, `this` references the `.base` file's location. Pass a note context mentally when interpreting results from template Bases.
- **Performance** — walks the entire vault (~1s for large vaults). Results are not cached.
- **Formulas** — `.unique()`, `.filter()`, `.asFile()` are not yet supported.
- **Rendering** — no view layout (cards, list, board). Output is data only.

### Constraints

- Always use the full path to the `.base` file (absolute or project-relative)
- Check [[TLP]] before reading vault files referenced in output
- [[JSONL]] output can be large — pipe through [[jq]], `head`, or `wc -l` for summaries
- Do not modify `.base` files without the user's explicit request — they define the user's [[Obsidian]] views
