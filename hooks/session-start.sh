#!/usr/bin/env bash
# SessionStart hook: emit metadata index for Obsidian conventions.
# Emits Tier 1 metadata only (name, description). Body is lazy-loaded
# by the provider (Skill tool on Claude Code, Read tool on others).
#
# Dual-mode: works standalone (CLAUDE_PLUGIN_ROOT) or as forge-core module (FORGE_MODULE_ROOT).
set -euo pipefail

MODULE_ROOT="${FORGE_MODULE_ROOT:-${CLAUDE_PLUGIN_ROOT:-$(builtin cd "$(dirname "$0")/.." && pwd)}}"
PROJECT_ROOT="${CLAUDE_PROJECT_ROOT:-$(builtin cd "$MODULE_ROOT/../.." && pwd)}"

# Context loader: forge-core shared lib → local standalone fallback
if [ -n "${FORGE_LIB:-}" ] && [ -f "$FORGE_LIB/load.sh" ]; then
  source "$FORGE_LIB/load.sh"
elif [ -f "$MODULE_ROOT/lib/load.sh" ]; then
  source "$MODULE_ROOT/lib/load.sh"
else
  echo "Error: load.sh not found" >&2; exit 1
fi

load_context "$MODULE_ROOT" "$PROJECT_ROOT" --index-only
