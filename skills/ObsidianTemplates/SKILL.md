---
name: ObsidianTemplates
version: 0.2.0
description: Obsidian template management — dual-tree creation, editing, rendering, frontmatter schemas, dynamic query blocks, Templater config updates. USE WHEN creating templates, editing templates, rendering templates, modifying template schemas, updating Templater folder mappings, adding query blocks, or checking template compliance.
---

# ObsidianTemplates

Manages the Obsidian template lifecycle: authoring templates across the two template trees, maintaining frontmatter schema conventions, and keeping the Templater plugin config in sync.

## Template Architecture

Two top-level trees, split by execution model:

| Tree | Role | Syntax |
|------|------|--------|
| `Templates/` | Standard templates — the agent copies schemas from here when creating typed notes | Plain markdown; Journals-plugin placeholders (`{{date}}`, `{{title}}`) allowed |
| `Templater/` | Dynamic templates — executed in-app by the Templater plugin (folder templates, hotkeys, journals) | `<% %>` blocks, moment.js, `tp.*` |

The agent never pastes `<% %>` syntax into a note; `Templater/` files are for in-app execution only. A template may exist in both trees: body structure (sections, embeds, query blocks) must be identical, only dynamic field expressions differ.

## Tree Layout

```
Templates/               standard
  Blocks/  Charts/  Domains/  Journal/  Library/  Maps/  Projects/  Zettelkasten/
  Default.md
Templater/               dynamic (in-app)
  Blocks/  Journal/  Library/  Processors/  Projects/
  Default.md
```

Folder mirrors destination: `Templates/Library/Book.md` → notes in `Library/Books/`, `Templates/Journal/Daily.md` → `Journal/Daily/`.

### Plugin syntax comparison

| Feature | Core Templates | Journals plugin | Templater |
|---------|---------------|-----------------|-----------|
| Title | `{{title}}` | `{{title}}` | `<% tp.file.title %>` |
| Date | `{{date:FORMAT}}` | `{{date:FORMAT}}` | `<% tp.date.now("FORMAT") %>` |
| Date offsets | Not supported | `{{date-1d:FORMAT}}`, `{{date+1d:FORMAT}}` | `moment().add()` |
| Week number | Not supported | `{{date:YYYY-[W]ww}}` | `moment().format("YYYY-[W]ww")` |
| JavaScript | Not supported | Not supported | `<%* ... %>` blocks |
| Include | Not supported | Not supported | `<% tp.file.include("[[...]]") %>` |

### Cascading with `tp.file.include()`

Reusable logic blocks live in `Templater/Blocks/`. Templates include them via:

```
<% tp.file.include("[[Block Name]]") %>
```

Templater resolves `<%* tR += %>` blocks through includes — cascading works. Use this for personal content (Key Results injection, habit trackers), query blocks shared across templates, and any logic that varies per-user.

**Pitfalls:**

- **Ambiguous file resolution**: `tp.file.find_tfile("Name")` resolves by basename — fails silently when multiple files share the same name. Use `app.vault.getAbstractFileByPath("full/path/to/File.md")` for explicit resolution.
- **Arrow functions in `<% %>`**: Templater's parser cannot handle `=>` inside `<% %>` tags (`Unexpected token '>'`). Use `<%* %>` execution blocks for multi-line logic.
- **Whitespace from includes**: `tp.file.include()` preserves leading/trailing whitespace from the included file. Use `.trim()` on `tR +=` output to prevent extra blank lines.
- **Linter reformats frontmatter**: After rendering, the Obsidian Linter may reformat quotes, field order, and timestamps. When editing rendered notes programmatically, use full-overwrite writes instead of incremental edits to avoid stale-content errors.

## Frontmatter Schema Registry

### Universal Fields (all templates)

```yaml
title:
aliases: []
tags:              # system/structural only (type/*, note/*)
keywords:          # wikilinks for topics/categories
  - "[[Topic]]"
description:
icon:              # Lucide icon name (LiCalendarFold, LiUser, etc.) — never emoji
image:
cssclasses:
created:           # YYYY-MM-DD HH:mm Z (empty in standard, Templater fills in dynamic)
updated:
related:
  - "[[Note]]"
collection:        # parent collection wikilink
root:              # parent hierarchy wikilink
```

### Tag Conventions

Primary tag: `type/<noun>` — what the note IS.
Secondary tag: `note/<category>` — zettel provenance (optional, for notes in the knowledge pipeline).

```
type/journal    note/daily | note/weekly | note/monthly | note/quarterly | note/yearly
type/contact    (no secondary — reference data, not zettel)
type/project    (no secondary)
type/book       note/literature
type/event      (no secondary)
type/collection (no secondary)
type/skill      (no secondary)
type/item       note/permanent
type/topic      note/permanent
type/repository note/literature
type/exercise   note/literature
```

DEPRECATED — do not use in new templates:
- `file/*` (use `type/*`)
- `schema/*`, `process/*`, `project/*`, `book/*` (use frontmatter fields for status/metadata)
- `lang/*` (remove — language is obvious from content)

### Category-Specific Schemas

**Journals**

```yaml
journal: Daily | Weekly | Monthly | Quarterly | Yearly
timeframe: YYYY-MM-DD
```

**Contacts**

```yaml
contact.email:
contact.linkedin:
contact.phone:
contact.website:
location.address:
location.city:
location.country:
location.region:
person.birthday:
person.organization:
person.surname:
```

**Literature Sources**

```yaml
source.authors:
  -
source.links:
  -
source.published:
```

**Books (extends Literature)**

```yaml
book.subtitle:
book.publisher:
book.cover:
book.isbn:
book.language:
book.pages:
book.status: unread | reading | read
book.rating:         # 1-5 scale
```

**Projects**

```yaml
project.status: planned | active | ongoing | completed | cancelled
project.priority: Critical | High | Medium | Low | None
project.deadline:
project.owner:
project.team:
  -
```

See the ProjectConventions skill for full project runtime conventions (Base files, embeds, tasks queries).

**Organizations**

```yaml
contact.email:
contact.linkedin:
contact.phone:
contact.website:
location.address:
location.city:
location.country:
location.region:
organization.taxid:
```

**Geography (City, Country)**

```yaml
place.country:
place.region:
```

**Items**

```yaml
item.authors:
item.owned: 0 | 1
item.price:
item.published:
```

**Events**

```yaml
event.time:
event.participants:
  -
```

**Key Results / Metrics**

```yaml
metric.current:
metric.objectives:
metric.projects:
metric.start:
metric.target:
metric.timeframe:
metric.unit:
```

## Create Workflow

1. **Determine category and name** — map to a subdirectory in the tree layout above.
2. **Define frontmatter** — start from Universal Fields, add category-specific fields. Lucide icon names (`Li*`), never emoji.
3. **Write the standard version** (`Templates/<Category>/<Name>.md`) — plain markdown; Journals-plugin placeholders only where the Journals plugin renders the note.
4. **Write the Templater version** (`Templater/<Category>/<Name>.md`) when dynamic fields are needed — `<% tp.* %>`, moment.js, `tp.file.include()` for reusable blocks, `<% tp.file.cursor() %>` at the first user-input point.
5. **Keep bodies in sync** — sections, embeds, and query blocks identical across both trees; only dynamic expressions differ.

## Validate Workflow

1. **Frontmatter conformance** — universal fields present; tags follow `type/*`, `note/*`; no deprecated tags.
2. **Tree sync** — where both versions exist, non-dynamic content is identical.
3. **No exec syntax in `Templates/`** — `<% %>` belongs only in `Templater/`.
4. **Icon convention** — all `icon:` values are Lucide names (`Li*`), not emoji.
5. **Timezone format** — date fields use `Z` (offset), not `z` (timezone name).
6. **YAML validity** — frontmatter keys unique; duplicates cause `YAMLParseError` at render time.

## Render Workflow

Templates are rendered by Obsidian, not by the agent. The Obsidian CLI triggers rendering:

```bash
# Create a note from a template
obsidian create name="New Note" template=Daily

# Create with explicit path
obsidian create name="2026-02-20" template="Journal/Daily.md"
```

Requires Obsidian 1.12+ with Templater `trigger_on_file_creation: true` for Templater templates to auto-execute. See the ObsidianCLI skill for full reference.

### Test workflow

1. Edit the template file(s)
2. Create a test note (use a scratch path like `_Test Render.md`)
3. Open the note in Obsidian and verify rendering
4. Delete the test note when done

## Dynamic Query Blocks

Templates can embed dynamic queries that display task lists, logs, or aggregations. The Tasks plugin is the query engine; Bases views cover structured aggregation.

### Tasks plugin queries

Use `query.file.property()` to read frontmatter values dynamically. Requires the `timeframe` field in frontmatter.

```
> [!tasks]- Tasks due this day
> ```tasks
> not done
> filter by function task.due && task.due.format("YYYY-MM-DD") <= query.file.property('timeframe')
> limit 20
> ```
```

**For topic/collection pages** — aggregate tasks that reference the current note:

```
> [!tasks]- Open Tasks
> ```tasks
> not done
> description includes {{query.file.filenameWithoutExtension}}
> tag does not include #log
> ```
```

Tasks queries are full vault scans — always set `limit` on large vaults. For indexed, structured views (tables over frontmatter, group-bys), embed a Base view instead: `![[Name.base#View]]`.

## Config Sync Workflow

Templater config: `.obsidian/plugins/templater-obsidian/data.json`

1. **`templates_folder`** — must be `"Templater"` (the dynamic tree; Templater never executes from `Templates/`).
2. **`folder_templates`** — folder-to-template mappings point into `Templater/<Category>/<Name>.md`.
3. **`enabled_templates_hotkeys`** — update paths when templates move.
4. **Fix stale paths** — correct any references to directories that no longer exist.
