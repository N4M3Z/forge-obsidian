---
paths:
  - "**/Vaults/**"
---

Frontmatter properties must be flat — Obsidian's Properties panel cannot display nested YAML. Use strings or lists of strings only, never nested objects.

YAML values containing `:` must be quoted. Bare colons break parsing — always wrap in double quotes.
