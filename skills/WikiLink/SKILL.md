---
name: WikiLink
version: 0.1.0
description: Add [[wikilinks]] to a markdown document by matching terms against existing vault notes — body links and frontmatter keywords/related enrichment. USE WHEN a document needs wikilinks, terms should link to vault notes, knowledge graph needs enrichment, or enrich keywords.
---

# WikiLink

Add `[[wikilinks]]` to a markdown document by matching terms against existing note titles in the Obsidian vault. Links the first occurrence of each matching term in the body and enriches frontmatter `keywords:` and `related:` fields.

## Instructions

### Resolve vault root and read the target file

The vault root is the session cwd when working inside the vault; otherwise target the vault explicitly with `vault=<name>` on every `obsidian` call.

If an argument was provided, use it as the file path. Otherwise, ask which file to enrich.

Check TLP before reading — GREEN/CLEAR: proceed; AMBER: ask first; RED: refuse. Then read through Obsidian:

```bash
obsidian read path=<exact/path.md>
```

The Read tool is the fallback only when the CLI is unavailable (app not running).

### Build the note title index

Through the CLI (see the ObsidianCLI skill for full reference):

```bash
obsidian files ext=md format=paths
```

Fallback when the CLI is unavailable (run from the vault root):
```bash
rg --files --glob '*.md' --glob '!.obsidian/**' --glob '!.trash/**' | sed 's|.*/||; s|\.md$//' | sort -u
```

Obsidian resolves wikilinks by filename, not directory path — index by stem only.

### Filter the index

**3a: Exclude RED paths**

Read the vault `.tlp` file. Remove stems from RED directories — their existence is protected information.

**3b: Remove stopwords**

Remove common short terms that create noise if linked:

Single-character stems, and words like: Data, Home, Table, View, List, Type, Item, Note, Page, Link, Text, File, Name, Date, Time, Tags, Status, All, New, Index, About, Help

Also exclude the document's own filename (no self-links).

**3c: Sort by length**

Longest first. "Forge Framework" matches before "Forge". "Claude Code" before "Claude".

### Identify protected zones

Zones that MUST NOT receive wikilinks:

| Zone | Action |
|------|--------|
| Fenced code blocks | Skip |
| Inline code | Skip |
| Existing wikilinks `[[...]]` | Skip — already linked |
| Markdown links `[text](url)` | Skip |
| URLs `https://...` | Skip |
| Obsidian embeds `![[...]]` | Skip |
| HTML blocks | Skip |

YAML frontmatter IS processed — but only the `keywords:` and `related:` fields (Step 6).

Track which terms are already linked in the document (including pre-existing wikilinks) to enforce first-occurrence-only linking.

### Match and link terms in the body

For each note title in the index (longest first):

1. Search for the term in unprotected text spans
2. Match case-insensitively at word boundaries
3. On first unlinked occurrence:
   - Exact case match: `[[Term]]`
   - Different case: `[[NoteTitle|matched text]]`
4. Mark the term as linked — skip subsequent occurrences
5. Mark the matched span as protected (prevent shorter terms from matching within it)

**Word boundaries**

A match must be at a word boundary on both sides: start/end of line, space, or punctuation (except `[`, `]`, `|`). This prevents matching "Forge" inside "ForgeCore" or "rust" inside "frustrated".

**Multi-word terms**

Multi-word note titles ("Forge Framework", "Claude Code") match as complete phrases. Never split into individual words.

### Enrich frontmatter keywords and related

Scan the document body for topic-relevant terms that match vault notes. Classify each into:

**`keywords:`** — abstract Topics only. Test: "Could this be a Space that groups many Resources?"
- Examples: `[[Security]]`, `[[Project Management]]`, `[[Rust]]`, `[[Automation]]`

**`related:`** — concrete entities (tools, organizations, projects, people, specific notes)
- Examples: `[[forge-tlp]]`, `[[Claude Code]]`, `[[Obsidian]]`, `[[Proton AG]]`

Rules:
- **Append only** — never remove or overwrite existing entries in either field
- Format as `- "[[Term]]"` (quoted wikilink in YAML list)
- Skip terms already present in either field
- If Obsidian CLI is available, prefer `obsidian property:set` for frontmatter updates (atomic — avoids Obsidian Linter race conditions)

### Review and confirm

Present proposed changes:

**Body links** — total count, each with surrounding context (5-10 words)
**Frontmatter** — new `keywords:` and `related:` entries

Group by confidence:
- High: exact case match, unambiguous multi-word term
- Lower: case-insensitive, short terms, potentially ambiguous

Options:
- Apply all
- Apply high-confidence only
- Review individually
- Cancel

### Write the enriched file

Write through Obsidian so its index and the Linter stay coherent:

- Frontmatter changes: `obsidian property:set` (scalar) or `obsidian eval` with `processFrontMatter` (arrays)
- Whole-body rewrite: `obsidian eval` with `app.vault.modify(file, content)` — JSON-encode the new content into the `code` string
- The Write tool is the fallback only when the CLI is unavailable

## Constraints

- **First occurrence only** in body — never link the same title twice
- **Never link inside protected zones** — code, existing links, URLs, embeds
- **Never create links to non-existent notes** — only link to stems found in the index
- **Never self-link** — skip the document's own title
- **Respect TLP** — exclude RED note titles from the index entirely
- **Stopword discipline** — common short words that are note titles create noise, not value
- **Prefer `[[Title]]` over `[[Title|text]]`** — only use aliased form when case genuinely differs
- **keywords vs related** — keywords are abstract topics (Spaces), related are concrete entities. Never mix them.
- **Append-only frontmatter** — never remove existing `keywords:` or `related:` entries
- After enriching, suggest a markdown lint pass (the MarkdownConventions skill) if the document also has formatting issues
