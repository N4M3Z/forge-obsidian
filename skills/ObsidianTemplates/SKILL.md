---
name: ObsidianTemplates
version: 0.1.0
description: Obsidian template management — dual-file creation, editing, rendering, promotion from legacy, frontmatter schemas, dynamic query blocks, Templater config updates. USE WHEN creating templates, editing templates, rendering templates, promoting templates, modifying template schemas, updating Templater folder mappings, adding query blocks, or checking template compliance.
---

# ObsidianTemplates

Manages the Obsidian template lifecycle: authoring dual-file template pairs, promoting legacy templates from Assets/Templater/ to the Templates/ submodule, maintaining frontmatter schema conventions, and keeping the Templater plugin config in sync.

## Template Architecture

| Location | Role | Status |
|----------|------|--------|
| `Templates/` | Canonical submodule (`obsidian-templates` repo) | Source of truth |
| `Assets/Templater/` | Active legacy (Templater scripts) | Being promoted |
| `Assets/Templates/` | Deprecated static templates | Archive — do not modify |

## Submodule Directory Layout

```
Templates/               (submodule root)
  Blocks/                Reusable snippets (TODO, FIXME, Key Results)
  Daily Notes/           Core Daily Notes plugin templates (no date math)
  Journals/              Journals plugin templates (Daily, Weekly, Monthly, etc.)
    Legacy/              Archived template versions (v0-v4)
  Orchestration/         Agent, Avatar, Collection, Pattern, Skill
    Memory/              Idea, Imperative, Insight
  PARA/                  Area, Project, Resource
  Resources/             Flat — zettel type conveyed by tags, not subfolders
  Projects/              Event, Key Result, Objective, Quarter
  Zettelkasten/          Fleeting, Literature, Permanent, Publishable
```

## Three-Tier Template Convention

Templates may exist in up to three tiers, targeting different Obsidian plugins:

| Tier | File | Plugin | Date math | Dynamic fields |
|------|------|--------|-----------|---------------|
| **Core** | `Name.md` in `Daily Notes/` | Core Daily Notes + Templates | `{{date:FORMAT}}`, `{{title}}` only | Title, created date |
| **Journals** | `Name.md` in category dir | Journals plugin | `{{date-1d:FORMAT}}` offsets | Title, created, related (prev/next), upstream (parent period) |
| **Templater** | `Name.js.md` in category dir | Templater plugin | Full moment.js + `tp.file.include()` | Everything — computed fields, KR injection, cursor, CSS classes |

Not every template needs all three tiers. The minimum is:
- **Journals `.md`** — the standard version with date offsets for navigation links
- **Templater `.js.md`** — the full-featured version with JavaScript logic

The **Core `.md`** tier is only needed for templates used with the core Daily Notes plugin (no Journals plugin dependency). It lives in a separate `Daily Notes/` directory since core Daily Notes points to a single template file.

### Plugin syntax comparison

| Feature | Core Templates | Journals | Templater |
|---------|---------------|----------|-----------|
| Title | `{{title}}` | `{{title}}` | `<% tp.file.title %>` |
| Date | `{{date:FORMAT}}` | `{{date:FORMAT}}` | `<% tp.date.now("FORMAT") %>` |
| Date offsets | Not supported | `{{date-1d:FORMAT}}`, `{{date+1d:FORMAT}}` | `moment().add()` |
| Week number | Not supported | `{{date:YYYY-[W]ww}}` | `moment().format("YYYY-[W]ww")` |
| JavaScript | Not supported | Not supported | `<%* ... %>` blocks |
| Include | Not supported | Not supported | `<% tp.file.include("[[...]]") %>` |

### Keeping tiers in sync

Body structure (sections, embeds, query blocks) MUST be identical across tiers. Only dynamic field expressions differ. When editing a template, update all existing tiers.

### Templater variants (`.dv.js.md`)

A Templater template may have alternative variants for different plugin dependencies. Name them `Name.dv.js.md` (Dataview variant), `Name.js.md` (Tasks plugin variant). The Templater config folder mapping points to whichever variant is active.

### Cascading with `tp.file.include()`

Reusable logic blocks live in `Templates/Blocks/`. Templates include them via:

```
<% tp.file.include("[[Block Name]]") %>
```

Templater resolves `<%* tR += %>` blocks through includes — cascading works. Use this for:
- Personal content (Key Results injection, habit trackers)
- Query blocks shared across templates
- Any logic that varies per-user

This keeps core templates clean and shareable.

**Pitfalls:**

- **Ambiguous file resolution**: `tp.file.find_tfile("Name")` resolves by basename — fails silently when multiple files share the same name. Use `app.vault.getAbstractFileByPath("full/path/to/File.md")` for explicit resolution.
- **Arrow functions in `<% %>`**: Templater's WASM parser cannot handle `=>` inside `<% %>` tags (`Unexpected token '>'`). Use `<%* %>` execution blocks for multi-line logic instead of `<% (async () => { ... })() %>`.
- **Whitespace from includes**: `tp.file.include()` preserves leading/trailing whitespace from the included file. Use `.trim()` on `tR +=` output to prevent extra blank lines in rendered notes.
- **Linter reformats frontmatter**: After rendering, the Obsidian Linter plugin may reformat quotes, field order, and timestamps. When editing rendered notes programmatically, use Write (full overwrite) instead of Edit to avoid stale-content errors.

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
created:           # YYYY-MM-DD HH:mm Z (empty in .md, Templater in .js.md)
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

#### Journals

```yaml
journal: Daily | Weekly | Monthly | Quarterly | Yearly
timeframe: YYYY-MM-DD
```

#### Contacts

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

#### Literature Sources

```yaml
source.authors:
  -
source.links:
  -
source.published:
```

#### Books (extends Literature)

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

#### Projects

```yaml
project.status: planned | active | on-hold | completed
project.priority: Critical | High | Medium | Low
project.deadline:
project.objectives:
project.owner:
project.team:
  -
```

See ProjectConventions skill for full project runtime conventions (Base files, embeds, Dataview).

#### Organizations

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

#### Geography (City, Country)

```yaml
place.country:
place.region:
```

#### Items

```yaml
item.authors:
item.owned: 0 | 1
item.price:
item.published:
```

#### Events

```yaml
event.time:
event.participants:
  -
```

#### Key Results / Metrics

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

1. **Determine category and name** — Map to a subdirectory in the layout above.
2. **Define frontmatter** — Start from Universal Fields. Add category-specific fields. Use Lucide icon names (`Li*`), never emoji.
3. **Write Journals version** (`Name.md`) — Use `{{title}}`, `{{date:FORMAT}}`, `{{date-1d:FORMAT}}` for dynamic fields. Populate `related:` with prev/next period, `upstream:` with parent period.
4. **Write Templater version** (`Name.js.md`) — Use `<% tp.file.title %>`, `<% tp.date.now() %>`, moment.js for computed fields. Use `tp.file.include()` for reusable blocks. Add `<% tp.file.cursor() %>` at first user-input point.
5. **Write Core version** (optional, in `Daily Notes/`) — Use only `{{title}}`, `{{date}}`, `{{time}}`. No date math. Only needed if the template must work without the Journals plugin.
6. **Place files** — Journals `.md` and Templater `.js.md` in `Templates/<Category>/`. Core `.md` in `Templates/Daily Notes/`.

## Promote Workflow (Legacy to Submodule)

1. **Identify source** in `Assets/Templater/`. If versioned (e.g., Daily.v0-v4), promote ONLY the latest version. The submodule file has no version suffix.
2. **Normalize tags** — Replace `file/*` with `type/*`. Remove `lang/*`, `schema/*`, `process/*`, `project/*`, `book/*`. Move status/metadata to frontmatter fields.
3. **Scrub personal content** — Remove hardcoded file references (Key Results, personal habits). Extract reusable logic into `Blocks/` templates using `tp.file.include()`. Replace vault-specific embeds with inline placeholder text or document as dependencies.
4. **Add missing universal fields** — Ensure `root:`, `image:`, `collection:`, `description:` are present.
5. **Standardize icons** — Replace emoji with Lucide icon names.
6. **Create dual-file pair** — Produce both `.md` (static) and `.js.md` (Templater).
7. **Update Templater config** — See Config Sync below.

## Validate Workflow

1. **Dual-file completeness** — Every `.md` has a matching `.js.md` (and vice versa).
2. **Frontmatter conformance** — Universal fields present. Tags follow `type/*`, `note/*` convention. No deprecated tags.
3. **File sync** — Non-dynamic fields identical between `.md` and `.js.md`.
4. **Icon convention** — All `icon:` values use Lucide names (`Li*`), not emoji.
5. **Timezone format** — All date fields use `Z` (offset), not `z` (timezone name).

## Edit Workflow

1. **Identify all tiers** — Check which tiers exist for the template (Core in `Daily Notes/`, Journals `.md`, Templater `.js.md`, Dataview variant `.dv.js.md`).
2. **Edit the Templater tier first** — It's the most complete version. Make structural changes here.
3. **Sync to other tiers** — Translate dynamic expressions per the plugin syntax comparison table. Non-dynamic content (sections, embeds, static frontmatter) must be identical.
4. **Check YAML validity** — Frontmatter keys must be unique. Duplicate keys (e.g., two `collection:` fields) cause `YAMLParseError: Map keys must be unique` at render time.
5. **Render and verify** — Use the Render Workflow below to create a test note and confirm the output.

## Render Workflow

Templates are rendered by Obsidian, not by Claude. Use the Obsidian CLI (preferred) or Actions URI plugin to trigger rendering.

### Preferred: Obsidian CLI (1.12+)

```bash
# Create a note from a template
obsidian create name="New Note" template=Daily

# Create with explicit path
obsidian create name="2026-02-20" template="Journals/Daily.js.md"
```

Requires Obsidian 1.12+ with Templater `trigger_on_file_creation: true` for Templater templates to auto-execute. See `/ObsidianCLI` for full reference.

### Fallback: Actions URI (Obsidian < 1.12)

> **Deprecated** — Use the CLI above when available.

```bash
# Actions URI with Templater rendering
open "obsidian://actions-uri/note/create?vault=Personal&file=<target-path>&apply=templater&template-file=<template-path>"

# Actions URI with core Templates rendering
open "obsidian://actions-uri/note/create?vault=Personal&file=<target-path>&apply=templates&template-file=<template-path>"
```

### Test workflow

1. Edit the template file(s)
2. Create a test note (use a scratch path like `_Test Render.md`)
3. Open the note in Obsidian and verify rendering
4. Delete the test note when done

## Dynamic Query Blocks

Templates can embed dynamic queries that display task lists, logs, or aggregations. Two query engines are available:

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

```
> [!tasks]- Logs
> ```tasks
> description includes {{query.file.filenameWithoutExtension}}
> tag includes #log
> ```
```

**Pros**: No Dataview dependency. Clean syntax.
**Cons**: Full vault scan per query — slow on large vaults without `limit`. No pagination (only `limit`, no `offset`).

### Dataview queries

Use `dv.view()` with shared view scripts in `Assets/Scripts/Dataview/views/`. Dataview uses an in-memory index — faster for cross-vault aggregation.

```
> [!tasks]- Tasks due this day
> ```dataviewjs
> await dv.view("Scripts/Dataview/views/daily", {flags: 'tasks', container: false});
> ```
```

**For topic pages:**

```
> [!tasks]- Tasks
> ```dataviewjs
> await dv.view("Scripts/Dataview/views/logs");
> await dv.view("Scripts/Dataview/views/tasks");
> ```
```

Available views: `daily` (tasks/created/updated/journals), `tasks` (open/closed by topic), `logs` (effort by topic), `repository` (collection listing), plus type-specific views (`area`, `city`, `contact`, `country`, `event`, `project`, `weekly`, `monthly`, `quarterly`, `yearly`).

**Pros**: Indexed — fast on large vaults. Graph-aware (outlinks, backlinks). Complex filtering.
**Cons**: Dataview plugin dependency. JavaScript view scripts to maintain.

### Choosing between them

| Criterion | Tasks plugin | Dataview |
|-----------|-------------|----------|
| Speed on large vaults | Slow (vault scan) | Fast (indexed) |
| Plugin dependency | Tasks only | Dataview + DataviewJS |
| Dynamic date from frontmatter | `query.file.property()` | `dv.current().file.day` |
| Topic aggregation | `description includes` | `outlinks.includes()` + `text.includes()` |
| Pagination | `limit` only (no offset) | Custom in view scripts |

Use Tasks plugin for simple queries with `limit`. Use Dataview for heavy aggregation (daily notes, topic pages with many references).

## Config Sync Workflow

Templater config: `.obsidian/plugins/templater-obsidian/data.json`

1. **Update `folder_templates`** — Change paths from `Assets/Templater/<path>/<Name>.md` to `Templates/<Category>/<Name>.js.md`. Folder templates MUST point to `.js.md` files.
2. **Update `enabled_templates_hotkeys`** — Update paths for any hotkey-enabled templates that were promoted.
3. **Verify `templates_folder`** — Must remain `"Templates"` (the submodule root).
4. **Fix stale paths** — Correct any references to non-existent directories (e.g., `Plans/` instead of `Journals/`).
