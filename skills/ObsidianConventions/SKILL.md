---
name: ObsidianConventions
version: 0.1.0
description: Vault conventions for wikilinks, frontmatter and tags. USE WHEN working with Obsidian vault files.
---

## Obsidian Conventions

- Use [[wikilinks]] liberally — people, projects, organizations, topics, locations. Anything that could be a note.
- Do NOT use tags for topics or categories — use keywords with [[wikilinks]] instead.
- Tags are reserved for system, structural and inline use only.
- Always verify file paths exist before claiming something is inaccessible.
- Frontmatter properties must be flat — Obsidian's Properties panel cannot display nested YAML (arrays of objects, deeply nested keys). Use strings or lists of strings only.

### Canon + Sidecar in Obsidian

Skills use two files: `SKILL.md` (canon — Claude Code frontmatter + body) and `SKILL.yaml` (sidecar — Obsidian metadata). The Obsidian Linter reformats frontmatter on save, stripping unrecognized keys like `name:`. Keeping them separate prevents cross-contamination. See `/CreateSkill` for the full pattern. The `forge-promote` / `forge-draft` scripts handle the split and merge automatically.

!`dispatch skill-load forge-obsidian`
