---
name: Promote
description: Push a vault skill to its target module for sharing via GitHub. USE WHEN the user wants to publish a drafted skill, finalize a behavioural rule, or move authored content from the vault workspace to a module.
argument-hint: "[SkillName or blank for all]"
---

# Promote

Push a skill from the vault workspace to its target module, with a review gate to scrub user-specific content before it lands in the module.

## Procedure

### Step 1: Run the review agent

Before running the promote script, spawn a `skill-reviewer` agent (via the Task tool) to analyze the draft skill. The agent MUST:

1. Read the draft at `$FORGE_USER_ROOT/Orchestration/Skills/<SkillName>/SKILL.md`
2. Identify user-specific content that doesn't belong in a shareable module:
   - Hardcoded vault paths (e.g., `Vaults/Personal/...`)
   - Personal names, locations, or references
   - User-specific tool paths or configurations
   - Assumptions about vault structure that aren't portable
3. Check skill quality: frontmatter conventions, description triggers, body structure
4. Return a report with specific suggested edits

### Step 2: Present review to user

Show the review findings. For each issue, present the specific edit. The user can:
- **Accept** — apply the edit
- **Reject** — skip it
- **Bypass** — skip the entire review (user explicitly opts out)

Apply accepted edits to the draft in `Orchestration/Skills/` before promoting.

### Step 3: Run the promote script

```bash
# Promote a single skill
Modules/forge-obsidian/bin/forge-promote BehavioralSteering

# Promote all skills with source_module: in the workspace
Modules/forge-obsidian/bin/forge-promote
```

The script:
1. Reads `source_module:` from frontmatter to find the target module
2. Copies to module, stripping `source_module:` line
3. Removes the draft from `Orchestration/Skills/`
4. Creates a symlink in `Orchestration/Upstream/<SkillName>/` → module skill directory

### Step 4: Post-script

1. **Commit the module**: Stage the promoted skill in the module's git repo and commit
2. **Run `/Update`**: Regenerate plugin.json if skill directories changed
3. **Optionally push**: The user decides when to push to GitHub

## Tool

**Binary:** `Modules/forge-obsidian/bin/forge-promote`

Auto-resolves `FORGE_USER_ROOT` from `forge.yaml`.

## Intent-to-Flag Mapping

| User Says | Args | Effect |
|-----------|------|--------|
| "promote BehavioralSteering" | `BehavioralSteering` | Review + push single skill to its source module |
| "promote all", "promote everything" | _(no args)_ | Review + push all skills with `source_module:` |

## Constraints

- Only promotes skills that have `source_module:` frontmatter — vault-permanent skills are left untouched
- Errors if `source_module:` is missing (prompts you to ask the user which module)
- The module's existing SKILL.md is overwritten — the vault version is the source of truth during development
- After promotion, the draft is replaced with a symlink in `Orchestration/Upstream/` (stays visible in Obsidian, excluded from Linter)
- The review step is mandatory — user must explicitly bypass to skip it
- Does NOT commit or push — handle git operations after the script runs
