---
name: WikiLink
description: Add [[wikilinks]] to a markdown document by matching terms against existing vault notes — body links and frontmatter keywords/related enrichment. USE WHEN a document needs wikilinks, terms should link to vault notes, knowledge graph needs enrichment, or enrich keywords.
argument-hint: "[path to markdown file]"
---

# WikiLink

Add `[[wikilinks]]` to a markdown document by matching terms against existing note titles in the Obsidian vault. Links the first occurrence of each matching term in the body and enriches frontmatter `keywords:` and `related:` fields.

## Instructions

### Step 1: Resolve vault root and read the target file

```bash
eval "$(bash Core/bin/paths.sh)"
echo "VAULT: $FORGE_USER_ROOT"
```

If an argument was provided, use it as the file path. Otherwise, ask which file to enrich.

Check TLP before reading:
- GREEN/CLEAR: Read directly
- AMBER: Use `safe-read` via Bash
- RED: Refuse

### Step 2: Build the note title index

Use the best available method (try in order):

**Option A — Local REST API** (fastest, if running):
```bash
curl -s http://localhost:27123/vault/ -H "Authorization: Bearer $(cat ~/.obsidian-rest-api-key 2>/dev/null)" 2>/dev/null
```

**Option B — Glob** (always works):
```bash
find "$FORGE_USER_ROOT" -name '*.md' -not -path '*/.obsidian/*' -not -path '*/.trash/*' -not -path '*/.git/*' | sed 's|.*/||; s|\.md$//' | sort -u
```

**Option C — Actions URI spot-check** (for individual validation):
```bash
Hooks/obsidian-uri.sh exists "<note-name>"
```

Obsidian resolves wikilinks by filename, not directory path — index by stem only.

### Step 3: Filter the index

#### 3a: Exclude RED paths

Read the vault `.tlp` file. Remove stems from RED directories — their existence is protected information.

#### 3b: Remove stopwords

Remove common short terms that create noise if linked:

Single-character stems, and words like: Data, Home, Table, View, List, Type, Item, Note, Page, Link, Text, File, Name, Date, Time, Tags, Status, All, New, Index, About, Help

Also exclude the document's own filename (no self-links).

#### 3c: Sort by length

Longest first. "Forge Framework" matches before "Forge". "Claude Code" before "Claude".

### Step 4: Identify protected zones

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

### Step 5: Match and link terms in the body

For each note title in the index (longest first):

1. Search for the term in unprotected text spans
2. Match case-insensitively at word boundaries
3. On first unlinked occurrence:
   - Exact case match: `[[Term]]`
   - Different case: `[[NoteTitle|matched text]]`
4. Mark the term as linked — skip subsequent occurrences
5. Mark the matched span as protected (prevent shorter terms from matching within it)

#### Word boundaries

A match must be at a word boundary on both sides: start/end of line, space, or punctuation (except `[`, `]`, `|`). This prevents matching "Forge" inside "ForgeCore" or "rust" inside "frustrated".

#### Multi-word terms

Multi-word note titles ("Forge Framework", "Claude Code") match as complete phrases. Never split into individual words.

### Step 6: Enrich frontmatter keywords and related

Scan the document body for topic-relevant terms that match vault notes. Classify each into:

**`keywords:`** — abstract Topics only. Test: "Could this be a Space that groups many Resources?"
- Examples: `[[Security]]`, `[[Project Management]]`, `[[Rust]]`, `[[Automation]]`

**`related:`** — concrete entities (tools, organizations, projects, people, specific notes)
- Examples: `[[forge-tlp]]`, `[[Claude Code]]`, `[[Obsidian]]`, `[[Proton AG]]`

Rules:
- **Append only** — never remove or overwrite existing entries in either field
- Format as `- "[[Term]]"` (quoted wikilink in YAML list)
- Skip terms already present in either field
- If `Hooks/obsidian-uri.sh` is available, prefer `props-set` for frontmatter updates (avoids Obsidian Linter race conditions)

### Step 7: Review and confirm

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

### Step 8: Write the enriched file

- AMBER: use `safe-write write` via Bash
- GREEN/CLEAR: use Write tool directly
- If Actions URI available for frontmatter: use `Hooks/obsidian-uri.sh props-set` instead

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
- After enriching, suggest `/MarkdownLint` if the document also has formatting issues
