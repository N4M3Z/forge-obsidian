---
name: ProjectManagement
version: 0.1.0
description: Project lifecycle management — create, update, close, backlog rules, safe file operations. USE WHEN creating projects, closing projects, moving project files, managing project backlogs, or checking project status.
---

# ProjectManagement

Project lifecycle management — creating, updating, closing projects, backlog rules, and safe file operations.

## Creating Projects

- Use the canonical template at `Orchestration/Templates/Project.md`.
- Every project MUST have a folder: `Projects/<Name>/<Name>.md` (Folder Notes convention).
- Copy the template, fill frontmatter, replace the description placeholder.
- Set `project.status: planned` for new projects, `project.priority` to appropriate level.
- Add to `Orchestration/Backlog.md` if the project has actionable tasks.

## Project Frontmatter

```yaml
project.status: planned   # planned | active | on-hold | completed
project.priority: Medium   # Critical | High | Medium | Low
project.deadline:
project.owner:
project.team: []
project.objectives: []
```

Status and priority are **scalars** (single value), never arrays.

## Work Logs

- Each project file has a `## Work log` section at the bottom.
- Entries are date-headed: `### [[YYYY-MM-DD]]` with sub-bullets for detail.
- Daily journal gets a one-liner; project file gets the full detail.
- Wikilink people, technologies, and related projects liberally.

## Closing Projects

1. Set `project.status: completed` and `updated:` date.
2. Open backlog items in the project file must be handled:
   - Promote to `Orchestration/Backlog.md` (use `[[Project#Section]]` wikilinks to reference source)
   - Move to a successor project
   - Or explicitly dismiss with rationale
3. Closed projects CANNOT have an open backlog. If improvements remain, they belong in Backlog.md or a new project.

## Backlog Cross-References

- Keep suggested improvements in the project file itself (under `## Backlog` or similar heading).
- For repo-backed projects, also mention them in the repo README.
- Link from `Orchestration/Backlog.md` using `[[Project Name#Backlog]]` section wikilinks.
- Resolve `[[File#Heading]]` links by globbing for the file and reading the heading section.

## Safe File Operations

When moving, renaming, or reorganizing project files:

1. `mkdir -p` the target directory
2. `command cp -R` source to target
3. `command diff` source and target (for directories, verify file count)
4. `command rm` the original only after verification
5. **Never** use bare `mv`, `cp`, or `rm` — always prefix with `command`
6. **Never** overwrite without reading first

See imperative: [[Safe file operations in vault — copy verify remove never bare mv]]
