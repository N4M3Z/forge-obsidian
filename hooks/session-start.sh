#!/usr/bin/env bash
# SessionStart hook: emit Obsidian conventions.
# Loads SYSTEM/ defaults, then any configured convention_dirs.
#
# Dual-mode: works standalone (CLAUDE_PLUGIN_ROOT) or as forge-core module (FORGE_MODULE_ROOT).
set -euo pipefail

MODULE_ROOT="${FORGE_MODULE_ROOT:-${CLAUDE_PLUGIN_ROOT:-$(builtin cd "$(dirname "$0")/.." && pwd)}}"
PROJECT_ROOT="${CLAUDE_PROJECT_ROOT:-$(builtin cd "$MODULE_ROOT/../.." && pwd)}"

# Source strip-front: forge-core shared lib > local lib > inline fallback
if [ -n "${FORGE_LIB:-}" ] && [ -f "$FORGE_LIB/strip-front.sh" ]; then
  source "$FORGE_LIB/strip-front.sh"
elif [ -f "$MODULE_ROOT/lib/strip-front.sh" ]; then
  source "$MODULE_ROOT/lib/strip-front.sh"
else
  strip_front() {
    awk '
      /^---$/ && !started { started=1; skip=1; next }
      /^---$/ && skip      { skip=0; next }
      skip                 { next }
      !body && /^# /       { body=1; next }
      { body=1; print }
    ' "$1"
  }
fi

# Parse convention_dirs from config.yaml (no yq dependency)
CONFIG="$MODULE_ROOT/config.yaml"
DIRS=()
if [ -f "$CONFIG" ]; then
  while IFS= read -r line; do
    dir=$(echo "$line" | sed 's/^[[:space:]]*-[[:space:]]*//' | sed 's/^["'"'"']//;s/["'"'"']$//')
    [ -n "$dir" ] && DIRS+=("$dir")
  done < <(grep -A 100 '^convention_dirs:' "$CONFIG" | tail -n +2 | grep '^[[:space:]]*-' || true)
fi

# Emit SYSTEM defaults (pre-stripped, just cat)
output=""
for f in "$MODULE_ROOT"/SYSTEM/*.md; do
  [ -f "$f" ] || continue
  output+="$(cat "$f")"$'\n'
done

# Emit configured convention dirs (strip frontmatter)
for dir in "${DIRS[@]}"; do
  abs_dir="$PROJECT_ROOT/$dir"
  [ -d "$abs_dir" ] || continue
  for f in "$abs_dir"/*.md; do
    [ -f "$f" ] || continue
    output+="$(strip_front "$f")"$'\n'
  done
done

[ -n "$output" ] && printf '%s\n' "$output"
exit 0
