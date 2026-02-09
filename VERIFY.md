# forge-obsidian — Verification

> **For AI agents**: Complete this checklist after installation. Every check must pass before declaring the module installed.

## Quick check

```bash
bash Modules/forge-obsidian/tests/test.sh
```

## Manual checks

### SKILL.md structure
```bash
head -5 Modules/forge-obsidian/skills/ObsidianConventions/SKILL.md
# Should show frontmatter with name: and description:
```

### SessionStart hook
```bash
bash Modules/forge-obsidian/hooks/session-start.sh
# Should emit metadata (name: ObsidianConventions)
```

### steer tool (requires config.yaml with steering: paths)
```bash
Modules/forge-steering/bin/steer Modules/forge-obsidian
# With steering: paths configured → tree output
# Without config.yaml → no output
```

## Expected test results

- Tests covering structure, session-start.sh, steer integration, User.md, DCI expansion, config override
- All tests PASS
