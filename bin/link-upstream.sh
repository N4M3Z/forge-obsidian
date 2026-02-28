#!/usr/bin/env bash
# link-upstream — mirror module .md files into Orchestration/Upstream/.
#
# Usage:
#   link-upstream.sh              — link all modules
#   link-upstream.sh --verbose    — show each file linked
#
# Creates a hierarchical symlink tree:
#   Upstream/forge-core/README.md → Modules/forge-core/README.md
#   Upstream/forge-core/skills/BuildSkill/SKILL.md → ...
#   Upstream/forge-core/skills/BuildSkill/SKILL.yaml.md → ...
#
# SKILL.yaml files are symlinked with .md extension for Obsidian display.
# Drafted skills (real directories in Orchestration/Skills/) are skipped.

set -euo pipefail

MODULE_ROOT="$(command cd "$(dirname "$0")/.." && pwd)"
FORGE_ROOT="${FORGE_ROOT:-${CLAUDE_PLUGIN_ROOT:-$(command cd "$MODULE_ROOT/../.." && pwd)}}"

# Auto-resolve FORGE_USER_ROOT if not set
if [ -z "${FORGE_USER_ROOT:-}" ]; then
    _cfg="$FORGE_ROOT/defaults.yaml"
    [ -f "$FORGE_ROOT/config.yaml" ] && _cfg="$FORGE_ROOT/config.yaml"
    if command -v yaml >/dev/null 2>&1; then
        _user_root=$(yaml nested "$_cfg" user root 2>/dev/null)
    else
        _user_root=$(awk '/^  root:/{gsub(/"/,"",$2); if($2!="") {print $2; exit}}' "$_cfg" 2>/dev/null || true)
    fi
    FORGE_USER_ROOT="${_user_root:+$FORGE_ROOT/$_user_root}"
fi
: "${FORGE_USER_ROOT:?Could not resolve FORGE_USER_ROOT from defaults.yaml/config.yaml}"

UPSTREAM="$FORGE_USER_ROOT/Orchestration/Upstream"
SKILLS="$FORGE_USER_ROOT/Orchestration/Skills"
MODULES="$FORGE_ROOT/Modules"
VERBOSE=false
[ "${1:-}" = "--verbose" ] && VERBOSE=true

mkdir -p "$UPSTREAM"

linked=0
skipped=0
modules=0

for mod_dir in "$MODULES"/forge-*; do
    [ -d "$mod_dir" ] || continue
    mod_name=$(basename "$mod_dir")

    # Skip uninitialized submodules (no files beyond .git)
    [ -f "$mod_dir/module.yaml" ] || [ -f "$mod_dir/README.md" ] || continue

    modules=$((modules + 1))
    dst_base="$UPSTREAM/$mod_name"

    # Find all .md files, excluding generated/build directories
    while IFS= read -r rel_path; do
        # Draft skip: if this is a skill file and the skill is drafted locally
        if [[ "$rel_path" =~ ^skills/([^/]+)/ ]]; then
            skill_name="${BASH_REMATCH[1]}"
            if [ -d "$SKILLS/$skill_name" ] && [ ! -L "$SKILLS/$skill_name" ]; then
                skipped=$((skipped + 1))
                continue
            fi
        fi

        src="$mod_dir/$rel_path"
        dst="$dst_base/$rel_path"
        mkdir -p "$(dirname "$dst")"
        ln -sf "$src" "$dst"
        linked=$((linked + 1))
        $VERBOSE && echo "  $mod_name/$rel_path"
    done < <(
        cd "$mod_dir"
        find . \
            \( -name lib -o -name .claude -o -name .gemini -o -name .codex \
               -o -name .opencode -o -name .github -o -name .claude-plugin \
               -o -name target -o -name .git -o -name modules -o -name .githooks \
               -o -name .venv -o -name node_modules \) -prune \
            -o -name '*.md' -type f -print \
        | sed 's|^\./||'
    )

    # Link SKILL.yaml files as SKILL.yaml.md for Obsidian display
    while IFS= read -r rel_path; do
        if [[ "$rel_path" =~ ^skills/([^/]+)/ ]]; then
            skill_name="${BASH_REMATCH[1]}"
            if [ -d "$SKILLS/$skill_name" ] && [ ! -L "$SKILLS/$skill_name" ]; then
                continue
            fi
        fi

        src="$mod_dir/$rel_path"
        dst="$dst_base/${rel_path}.md"
        mkdir -p "$(dirname "$dst")"
        ln -sf "$src" "$dst"
        linked=$((linked + 1))
        $VERBOSE && echo "  $mod_name/${rel_path}.md"
    done < <(
        cd "$mod_dir"
        find . \
            \( -name lib -o -name .claude -o -name .gemini -o -name .codex \
               -o -name .opencode -o -name .github -o -name .claude-plugin \
               -o -name target -o -name .git -o -name modules -o -name .githooks \
               -o -name .venv -o -name node_modules \) -prune \
            -o -name 'SKILL.yaml' -type f -print \
        | sed 's|^\./||'
    )
done

# Cleanup: remove broken symlinks
orphaned=0
while IFS= read -r broken; do
    command rm -f "$broken"
    orphaned=$((orphaned + 1))
    $VERBOSE && echo "  removed: $broken"
done < <(find "$UPSTREAM" -type l ! -exec test -e {} \; -print 2>/dev/null)

# Cleanup: remove empty directories (preserve Upstream/ root and Upstream.md)
find "$UPSTREAM" -mindepth 2 -type d -empty -delete 2>/dev/null || true

echo "  $linked files linked across $modules modules ($skipped drafted, $orphaned orphaned removed)"
