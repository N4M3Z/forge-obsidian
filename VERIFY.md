# forge-obsidian — Verification

> **For AI agents**: Complete this checklist after installation. Every check must pass before declaring the module installed.

## Quick check

```bash
cargo test --manifest-path Modules/forge-obsidian/Cargo.toml
bash Modules/forge-obsidian/tests/test.sh
```

Expected: 26 Rust tests pass (base, eval, note, vault modules) + shell tests pass.

## Binary available

```bash
command -v obsidian-base   # or: Modules/forge-obsidian/bin/obsidian-base --help
```

## Dependencies

```bash
command -v jq      && echo "ok jq"      || echo "-- jq (recommended: brew install jq)"
command -v yt-dlp  && echo "ok yt-dlp"  || echo "-- yt-dlp (recommended: brew install yt-dlp)"
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

### obsidian-base binary
```bash
Modules/forge-obsidian/bin/obsidian-base --help
# Should show usage information
```

### steer tool (requires config.yaml with steering: paths)
```bash
Modules/forge-steering/bin/steer Modules/forge-obsidian
# With steering: paths configured → tree output
# Without config.yaml → no output
```

## Expected results

- `obsidian-base` binary compiles and is available
- All 26 Rust tests + shell tests pass
- SessionStart hook emits metadata index
- `jq`, `yt-dlp` available for vault operations (recommended, not required)
