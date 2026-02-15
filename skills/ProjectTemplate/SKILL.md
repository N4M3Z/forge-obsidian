---
name: ProjectTemplate
description: Obsidian project template conventions — base files, embeds, Dataview, frontmatter fields. USE WHEN creating project notes, checking project template compliance, or working with Project.base views.
---

# ProjectTemplate

Obsidian project template conventions — base files, embeds, Dataview integration, and frontmatter field definitions.

## Template Location

`Orchestration/Templates/Project.md` — the canonical project template.
`Orchestration/Templates/Project.base` — Obsidian Base file for project views.

## Required Structure

Every project note must include these embeds in order:

```markdown
![[Assets.base]]

---

![[Project.base#Resources]]

---
![[Project.base#Events]]

> [!tasks]- Tasks
>
> ```dataviewjs
> await dv.view("Scripts/Dataview/views/logs");
> await dv.view("Scripts/Dataview/views/tasks");
> ```

## Work log

### [[YYYY-MM-DD]]
- {One-liner per work item. Wikilink people, projects, technologies.}
```

## Project.base Views

The Base file provides three filtered views of backlinked notes:

| View | Shows | Filter |
|------|-------|--------|
| **Resources** | All linked notes except projects, journals, events | Grouped by `root`, sorted by name |
| **Events** | Notes tagged `type/event` | Sorted by name |
| **Journals** | Notes tagged `type/journal` with backlink | List format, comma-separated |

## Frontmatter Conventions

- `project.status` — scalar: `planned`, `active`, `on-hold`, `completed`
- `project.priority` — scalar: `Critical`, `High`, `Medium`, `Low`
- `collection: "[[Projects]]"` — standard collection link
- `keywords` — wikilinks to topics, technologies, systems (never tags)
- `related` — wikilinks to related projects or notes
- `project.team` — wikilinks to people involved
- `project.objectives` — short text objectives for the project

## Folder Notes

Every project lives in `Projects/<Name>/<Name>.md`. The folder note IS the project note. Associated files (plans, designs, references) live alongside in the same folder. The Folder Notes plugin renders the note when clicking the folder in the sidebar.
