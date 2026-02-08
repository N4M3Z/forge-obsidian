#!/usr/bin/env bash
# Strip YAML frontmatter and leading H1 from markdown files.
# Source this file to get the strip_front() function.
#
# Usage:
#   strip_front file.md                       # strip all frontmatter + H1
#   strip_front --keep name,description file.md  # keep whitelisted YAML keys only

strip_front() {
  local keep_keys=""
  if [ "$1" = "--keep" ]; then
    keep_keys="$2"
    shift 2
  fi

  local file="$1"
  if [ -z "$file" ] || [ ! -f "$file" ]; then
    return 1
  fi

  if [ -n "$keep_keys" ]; then
    # --keep mode: emit only whitelisted frontmatter keys, strip H1
    awk -v keys="$keep_keys" '
      BEGIN {
        n = split(keys, arr, ",")
        for (i = 1; i <= n; i++) keep[arr[i]] = 1
      }
      /^---$/ && !started { started = 1; in_fm = 1; next }
      /^---$/ && in_fm {
        in_fm = 0
        if (kept > 0) {
          print "---"
          for (i = 1; i <= kept; i++) print kept_lines[i]
          print "---"
        }
        next
      }
      in_fm {
        if (match($0, /^[a-zA-Z_-]+:/)) {
          key = substr($0, RSTART, RLENGTH - 1)
          if (key in keep) {
            kept++
            kept_lines[kept] = $0
          }
        }
        next
      }
      !body && /^# / { body = 1; next }
      { body = 1; print }
    ' "$file"
  else
    # Basic mode: strip all frontmatter + H1
    awk '
      /^---$/ && !started { started = 1; skip = 1; next }
      /^---$/ && skip      { skip = 0; next }
      skip                 { next }
      !body && /^# /       { body = 1; next }
      { body = 1; print }
    ' "$file"
  fi
}
