---
name: Draft
description: Pull a module skill into the vault for editing in Obsidian. USE WHEN the user wants to edit, iterate on, or author a skill that will eventually live in a module. Creates a working copy in Orchestration/Skills/ with source tracking.
argument-hint: "[SkillName] [optional: module-name]"
---

# Draft

Pull a skill from a module into the vault workspace for editing in Obsidian.

## Tool

**Binary:** `Modules/forge-obsidian/bin/forge-draft`

Auto-resolves `FORGE_USER_ROOT` from `forge.yaml`. Copies the skill to `Orchestration/Skills/`, adds `source_module:` provenance to frontmatter. If an upstream symlink exists (from a previous promote), removes it first.

## Usage

```bash
# Draft a skill (auto-finds the module)
Modules/forge-obsidian/bin/forge-draft BehavioralSteering

# Draft from a specific module
Modules/forge-obsidian/bin/forge-draft BehavioralSteering forge-steering
```

## Intent-to-Flag Mapping

| User Says | Args | Effect |
|-----------|------|--------|
| "draft BehavioralSteering" | `BehavioralSteering` | Search all modules for skill, copy to workspace |
| "draft from forge-steering" | `BehavioralSteering forge-steering` | Copy from specific module |

## New Skill (no source)

To author a brand-new skill that doesn't exist in any module yet:

1. Create `$FORGE_USER_ROOT/Orchestration/Skills/<SkillName>/SKILL.md` manually
2. Write the skill content with standard SKILL.md frontmatter (`name:`, `description:`)
3. Add `source_module: <target-module>` to indicate where it should eventually be promoted
4. Iterate in Obsidian — it's immediately usable as a skill
5. When ready, run `/Promote`

## Vault Layout

| Directory | Purpose |
|-----------|---------|
| `Orchestration/Skills/` | Draft workspace — editable copies for authoring |
| `Orchestration/Upstream/` | Symlinks to promoted module skills (Linter-excluded, read-only view) |

## Constraints

- The Skills/ directory is registered in plugin.json, so vault versions override module versions during development
- If an upstream symlink exists, draft removes it and creates a real copy
- Warns and exits if a real draft already exists (prevents overwriting in-progress edits)
- Does not commit — handle git operations after the script runs
