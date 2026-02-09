# forge-obsidian — Verification

## Quick check

```bash
bash Modules/forge-obsidian/test.sh
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

- ~20 tests covering structure, session-start.sh, steer integration, User.md, config override
- All tests PASS
