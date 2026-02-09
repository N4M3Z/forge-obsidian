#!/usr/bin/env bash
# SessionStart hook: emit skill metadata for non-Claude-Code providers.
# Uses forge-load for lazy loading if available, awk fallback otherwise.
set -euo pipefail

MODULE_ROOT="${FORGE_MODULE_ROOT:-${CLAUDE_PLUGIN_ROOT:-$(builtin cd "$(dirname "$0")/.." && pwd)}}"
PROJECT_ROOT="${CLAUDE_PROJECT_ROOT:-$(builtin cd "$MODULE_ROOT/../.." && pwd)}"

FORGE_LOAD="${FORGE_ROOT:-$PROJECT_ROOT}/Modules/forge-load/src"
if [ -f "$FORGE_LOAD/load.sh" ]; then
  source "$FORGE_LOAD/load.sh"
  load_context "$MODULE_ROOT" "$PROJECT_ROOT" --index-only
else
  awk '/^---$/{if(n++)exit;next} n{print}' "$MODULE_ROOT/skills/ObsidianConventions/SKILL.md"
fi
