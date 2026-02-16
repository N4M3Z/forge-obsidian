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

## Obsidian plugins (optional)

### Local REST API
```bash
export $(cat Modules/forge-obsidian/.env 2>/dev/null | xargs 2>/dev/null)
curl -sk --max-time 2 "https://localhost:27124/" \
  -H "Authorization: Bearer ${OBSIDIAN_REST_API_KEY:-none}" 2>/dev/null \
  | grep -q '"authenticated":true' && echo "ok REST API" || echo "-- REST API (optional)"
```

### Actions URI
```bash
ls "${FORGE_USER_ROOT:-.}/.obsidian/plugins/actions-uri/main.js" 2>/dev/null \
  && echo "ok Actions URI" || echo "-- Actions URI (optional)"
```

## Expected results

- `obsidian-base` binary compiles and is available
- All 26 Rust tests + shell tests pass
- SessionStart hook emits metadata index
- `jq`, `yt-dlp` available for vault operations (recommended, not required)
- REST API responds on `https://localhost:27124/` (optional, enhances WikiLink)
- Actions URI plugin installed (optional, enhances frontmatter writes)
