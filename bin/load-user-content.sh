#!/usr/bin/env bash
# Load user content extensions from config.
# Called by SKILL.md via !`command` (Dynamic Context Injection).
set -euo pipefail

MODULE_ROOT="${CLAUDE_PLUGIN_ROOT:-$(builtin cd "$(dirname "$0")/.." && pwd)}"
PROJECT_ROOT="${CLAUDE_PROJECT_ROOT:-$(pwd)}"

if [ -n "${FORGE_LIB:-}" ] && [ -f "$FORGE_LIB/load.sh" ]; then
  source "$FORGE_LIB/load.sh"
elif [ -f "$MODULE_ROOT/lib/load.sh" ]; then
  source "$MODULE_ROOT/lib/load.sh"
else
  exit 0
fi

load_user_content "$MODULE_ROOT" "$PROJECT_ROOT" || true
