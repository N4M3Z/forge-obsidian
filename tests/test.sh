#!/usr/bin/env bash
# forge-obsidian module tests.
# Run: bash Modules/forge-obsidian/tests/test.sh
set -uo pipefail

MODULE_ROOT="$(command cd "$(dirname "$0")/.." && pwd)"
PROJECT_ROOT="$(command cd "$MODULE_ROOT/../.." && pwd)"
PASS=0 FAIL=0

# --- Helpers ---

_tmpdirs=()
setup() {
  _tmpdir=$(mktemp -d)
  _tmpdirs+=("$_tmpdir")
}
cleanup_all() {
  command rm -f "$MODULE_ROOT/config.yaml"
  for d in "${_tmpdirs[@]}"; do
    [ -d "$d" ] && command rm -rf "$d"
  done
}
trap cleanup_all EXIT

assert_eq() {
  local label="$1" expected="$2" actual="$3"
  if [ "$expected" = "$actual" ]; then
    printf '  PASS  %s\n' "$label"
    PASS=$((PASS + 1))
  else
    printf '  FAIL  %s\n' "$label"
    printf '    expected: %s\n' "$(echo "$expected" | head -5)"
    printf '    actual:   %s\n' "$(echo "$actual" | head -5)"
    FAIL=$((FAIL + 1))
  fi
}

assert_contains() {
  local label="$1" needle="$2" haystack="$3"
  if echo "$haystack" | grep -qF "$needle"; then
    printf '  PASS  %s\n' "$label"
    PASS=$((PASS + 1))
  else
    printf '  FAIL  %s\n' "$label"
    printf '    expected to contain: %s\n' "$needle"
    printf '    actual: %s\n' "$(echo "$haystack" | head -5)"
    FAIL=$((FAIL + 1))
  fi
}

assert_not_contains() {
  local label="$1" needle="$2" haystack="$3"
  if echo "$haystack" | grep -qF "$needle"; then
    printf '  FAIL  %s\n' "$label"
    printf '    should not contain: %s\n' "$needle"
    FAIL=$((FAIL + 1))
  else
    printf '  PASS  %s\n' "$label"
    PASS=$((PASS + 1))
  fi
}

assert_empty() {
  local label="$1" actual="$2"
  if [ -z "$actual" ]; then
    printf '  PASS  %s\n' "$label"
    PASS=$((PASS + 1))
  else
    printf '  FAIL  %s\n' "$label"
    printf '    expected empty, got: %s\n' "$(echo "$actual" | head -3)"
    FAIL=$((FAIL + 1))
  fi
}

echo "=== forge-obsidian tests ==="

# ============================================================
# Structure tests
# ============================================================
printf '\n--- Structure ---\n'

[ -f "$MODULE_ROOT/skills/ObsidianConventions/SKILL.md" ] \
  && { printf '  PASS  SKILL.md exists\n'; PASS=$((PASS + 1)); } \
  || { printf '  FAIL  SKILL.md missing\n'; FAIL=$((FAIL + 1)); }

# SKILL.md has name: in frontmatter
result=$(awk '/^---$/{if(n++)exit;next} n && /^name:/{print; exit}' "$MODULE_ROOT/skills/ObsidianConventions/SKILL.md")
[ -n "$result" ] \
  && { printf '  PASS  SKILL.md has name: frontmatter\n'; PASS=$((PASS + 1)); } \
  || { printf '  FAIL  SKILL.md missing name: frontmatter\n'; FAIL=$((FAIL + 1)); }

# SKILL.md has two !` lines
bang_count=$(grep -c '^!\`' "$MODULE_ROOT/skills/ObsidianConventions/SKILL.md" || true)
assert_eq "SKILL.md has two !command blocks" "2" "$bang_count"

# module.yaml has required fields
[ -f "$MODULE_ROOT/module.yaml" ] \
  && { printf '  PASS  module.yaml exists\n'; PASS=$((PASS + 1)); } \
  || { printf '  FAIL  module.yaml missing\n'; FAIL=$((FAIL + 1)); }

mod_yaml=$(cat "$MODULE_ROOT/module.yaml")
assert_contains "module.yaml has name" "name:" "$mod_yaml"
assert_contains "module.yaml has events" "events:" "$mod_yaml"
assert_contains "module.yaml has metadata" "metadata:" "$mod_yaml"

# hooks.json is valid JSON
[ -f "$MODULE_ROOT/hooks/hooks.json" ] && python3 -c "import json; json.load(open('$MODULE_ROOT/hooks/hooks.json'))" 2>/dev/null \
  && { printf '  PASS  hooks.json is valid JSON\n'; PASS=$((PASS + 1)); } \
  || { printf '  FAIL  hooks.json invalid or missing\n'; FAIL=$((FAIL + 1)); }

# session-start.sh exists and is readable
[ -f "$MODULE_ROOT/hooks/session-start.sh" ] \
  && { printf '  PASS  session-start.sh exists\n'; PASS=$((PASS + 1)); } \
  || { printf '  FAIL  session-start.sh missing\n'; FAIL=$((FAIL + 1)); }

# skill-load.sh exists and is executable
[ -x "$MODULE_ROOT/hooks/skill-load.sh" ] \
  && { printf '  PASS  skill-load.sh exists and is executable\n'; PASS=$((PASS + 1)); } \
  || { printf '  FAIL  skill-load.sh missing or not executable\n'; FAIL=$((FAIL + 1)); }

# No defaults.yaml (deleted)
[ ! -f "$MODULE_ROOT/defaults.yaml" ] \
  && { printf '  PASS  defaults.yaml deleted\n'; PASS=$((PASS + 1)); } \
  || { printf '  FAIL  defaults.yaml still exists\n'; FAIL=$((FAIL + 1)); }

# bin/ directory has forge-draft and forge-promote scripts
[ -x "$MODULE_ROOT/bin/forge-draft" ] \
  && { printf '  PASS  bin/forge-draft exists and is executable\n'; PASS=$((PASS + 1)); } \
  || { printf '  FAIL  bin/forge-draft missing or not executable\n'; FAIL=$((FAIL + 1)); }
[ -x "$MODULE_ROOT/bin/forge-promote" ] \
  && { printf '  PASS  bin/forge-promote exists and is executable\n'; PASS=$((PASS + 1)); } \
  || { printf '  FAIL  bin/forge-promote missing or not executable\n'; FAIL=$((FAIL + 1)); }

# No lib/ directory (deleted)
[ ! -d "$MODULE_ROOT/lib" ] \
  && { printf '  PASS  lib/ directory deleted\n'; PASS=$((PASS + 1)); } \
  || { printf '  FAIL  lib/ directory still exists\n'; FAIL=$((FAIL + 1)); }

# ============================================================
# session-start.sh tests
# ============================================================
printf '\n--- session-start.sh ---\n'

# With forge-load available (convention mode)
FORGE_LOAD="$PROJECT_ROOT/Modules/forge-load/src"
if [ -f "$FORGE_LOAD/load.sh" ]; then
  result=$(FORGE_ROOT="$PROJECT_ROOT" bash "$MODULE_ROOT/hooks/session-start.sh" 2>/dev/null) || true
  assert_contains "session-start (forge-load): emits name" "name: ObsidianConventions" "$result"
  assert_contains "session-start (forge-load): emits description" "description:" "$result"
  # --index-only should NOT emit body
  assert_not_contains "session-start (forge-load): no body" "## Obsidian Conventions" "$result"

  # Test awk fallback: hide forge-load temporarily
  setup
  result=$(FORGE_ROOT="$_tmpdir" bash "$MODULE_ROOT/hooks/session-start.sh" 2>/dev/null) || true
  assert_contains "session-start (awk fallback): has name:" "name:" "$result"
else
  printf '  SKIP  forge-load not available\n'
fi

# Both paths exit 0
exit_code=0
bash "$MODULE_ROOT/hooks/session-start.sh" >/dev/null 2>&1 || exit_code=$?
assert_eq "session-start.sh exits 0" "0" "$exit_code"

# ============================================================
# steer integration tests
# ============================================================
printf '\n--- steer tool ---\n'

STEER="$PROJECT_ROOT/Modules/forge-steering/bin/steer"

if [ -x "$STEER" ]; then
  # No config.yaml → no output
  setup
  mkdir -p "$_tmpdir/mod"
  result=$("$STEER" "$_tmpdir/mod" 2>/dev/null)
  assert_empty "steer: no config.yaml → no output" "$result"

  # config.yaml with existing dir
  setup
  mkdir -p "$_tmpdir/mod" "$_tmpdir/steering"
  printf 'test-file\n' > "$_tmpdir/steering/conventions.md"
  printf 'steering:\n  - %s\n' "$_tmpdir/steering" > "$_tmpdir/mod/config.yaml"
  result=$("$STEER" "$_tmpdir/mod" 2>/dev/null)
  assert_contains "steer: existing dir → tree output" "conventions.md" "$result"

  # config.yaml with non-existent dir
  setup
  mkdir -p "$_tmpdir/mod"
  printf 'steering:\n  - /nonexistent/path/1234\n' > "$_tmpdir/mod/config.yaml"
  result=$("$STEER" "$_tmpdir/mod" 2>/dev/null)
  assert_empty "steer: non-existent dir → no output" "$result"

  # steer exits 0 in all cases
  exit_code=0
  "$STEER" "$_tmpdir/mod" >/dev/null 2>&1 || exit_code=$?
  assert_eq "steer exits 0" "0" "$exit_code"
else
  printf '  SKIP  forge-steering/bin/steer not available\n'
fi

# ============================================================
# User.md tests
# ============================================================
printf '\n--- User.md ---\n'

# No User.md → !command produces nothing
SKILL_DIR="$MODULE_ROOT/skills/ObsidianConventions"
[ ! -f "$SKILL_DIR/User.md" ] \
  && { printf '  PASS  User.md does not exist by default\n'; PASS=$((PASS + 1)); } \
  || { printf '  FAIL  User.md should not exist by default\n'; FAIL=$((FAIL + 1)); }

# Create temp User.md and verify cat works
setup
USER_MD="$_tmpdir/User.md"
printf '## My Overrides\n\n- Custom rule\n' > "$USER_MD"
result=$(F="$USER_MD"; [ -f "$F" ] && cat "$F")
assert_contains "User.md cat: content emitted" "Custom rule" "$result"

# ============================================================
# Config override
# ============================================================
printf '\n--- Config override ---\n'

if [ -x "$PROJECT_ROOT/Core/bin/dispatch" ]; then
  # events: [] disables module
  printf 'events: []\n' > "$MODULE_ROOT/config.yaml"
  result=$(CLAUDE_PLUGIN_ROOT="$PROJECT_ROOT" "$PROJECT_ROOT/Core/bin/dispatch" SessionStart < /dev/null 2>/dev/null) || true
  assert_not_contains "config events: [] disables module" "ObsidianConventions" "$result"
  command rm -f "$MODULE_ROOT/config.yaml"
else
  printf '  SKIP  dispatch binary not available\n'
fi

# ============================================================
# DCI expansion tests
# ============================================================
printf '\n--- DCI expansion ---\n'

# DCI line 1: standalone path (module root = plugin root)
exit_code=0
"$MODULE_ROOT/hooks/skill-load.sh" >/dev/null 2>&1 || exit_code=$?
assert_eq "DCI standalone: skill-load.sh exits 0" "0" "$exit_code"

# DCI line 2: forge-core path (project root + Modules/...)
exit_code=0
"$PROJECT_ROOT/Modules/forge-obsidian/hooks/skill-load.sh" >/dev/null 2>&1 || exit_code=$?
assert_eq "DCI forge-core: skill-load.sh exits 0" "0" "$exit_code"

# skill-load.sh with no User.md produces no user content
result=$("$MODULE_ROOT/hooks/skill-load.sh" 2>/dev/null) || true
assert_not_contains "skill-load.sh: no User.md → no user content" "My Overrides" "$result"

# ============================================================
# Summary
# ============================================================
printf '\n=== Results ===\n'
printf '  %d passed, %d failed\n\n' "$PASS" "$FAIL"
[ "$FAIL" -eq 0 ] && exit 0 || exit 1
