# forge-obsidian

Obsidian vault utilities for the Forge Framework.

## Features

- **strip-front**: Strips YAML frontmatter and H1 headings from markdown files for token-efficient LLM loading. Supports `--keep` whitelist for functional frontmatter fields.
- **Obsidian conventions**: Emits Obsidian-specific behavioral rules at SessionStart (wikilinks, tags, AMBER handling).

## Installation

```bash
claude plugin install forge-obsidian
```

Or clone into your `Plugins/` directory.

## Library: strip-front

Source from other scripts:

```bash
source "$PROJECT_ROOT/Plugins/forge-obsidian/lib/strip-front.sh"

# Strip all frontmatter + H1
strip_front file.md

# Keep specific YAML keys (for skills/commands with functional frontmatter)
strip_front --keep name,description file.md
```

## Configuration

Edit `config.yaml` to add Obsidian convention directories from your vault:

```yaml
convention_dirs:
  - "Vaults/Personal/Orchestration/Conventions"
```

Plugin SYSTEM/ defaults always load first. Configured directories layer on top.

## SYSTEM Defaults

Pre-stripped conventions in `SYSTEM/CONVENTIONS.md` cover:
- Wikilink usage
- Tag vs keyword conventions
- AMBER file handling (Edit vs Write)
- Path verification
