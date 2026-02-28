---
name: ValidateVault
description: "Validate vault files against .mdschema files — structural checks on frontmatter, headings, and required sections. USE WHEN validate vault, check vault, vault lint, schema check, validate journals, validate memory."
---

# ValidateVault

Validate vault markdown files against `.mdschema` schema files using the `mdschema` CLI. Reports structural violations: missing frontmatter fields, wrong field types, unexpected headings, missing required sections.

## Workflow Routing

| Workflow | Trigger | Section |
|----------|---------|---------|
| **Check** | "validate vault", "check journals", "validate memory" | [Check](#check) |
| **Report** | "how bad is it", "vault health", "show violations" | [Report](#report) |

## Tool Reference

Binary: `mdschema` (Go, installed via `brew install jackchuka/tap/mdschema`).

```bash
mdschema check "glob/pattern/**/*.md" --schema path/to/.mdschema
```

Output: per-file list of `✗` violations with line numbers, categories (`[frontmatter]`, `[structure]`), and descriptions.

## Schema Locations

Schemas ship with their convention-owning module, not in the vault. The vault contains instances; modules contain the rules.

| Content | Schema Location | Vault Glob |
|---------|----------------|------------|
| Daily journals | `$VAULT/Templates/Journals/.mdschema` | `$VAULT/Resources/Journals/Daily/**/*.md` |
| Weekly journals | `$VAULT/Templates/Journals/.mdschema` | `$VAULT/Resources/Journals/Weekly/**/*.md` |
| Memory Insights | `$MODULES/forge-reflect/skills/MemoryInsights/Templates/.mdschema` | `$VAULT/Orchestration/Memory/Insights/*.md` |
| Memory Imperatives | `$MODULES/forge-reflect/skills/MemoryInsights/Templates/.mdschema` | `$VAULT/Orchestration/Memory/Imperatives/*.md` |
| Memory Ideas | `$MODULES/forge-reflect/skills/MemoryInsights/Templates/.mdschema` | `$VAULT/Orchestration/Memory/Ideas/*.md` |

Where `$VAULT` resolves from `user.root` in `config.yaml` (currently `Vaults/Personal`) relative to `$HOME/Data`, and `$MODULES` is `Modules/`.

---

## Check

1. Resolve the vault root from `config.yaml`:
   ```bash
   FORGE_ROOT="$HOME/Data"
   VAULT="$FORGE_ROOT/$(grep 'root:' "$FORGE_ROOT/config.yaml" | awk '{print $2}')"
   MODULES="$FORGE_ROOT/Modules"
   ```

2. Verify `mdschema` is installed:
   ```bash
   command -v mdschema >/dev/null || { echo "mdschema not installed — brew install jackchuka/tap/mdschema"; exit 1; }
   ```

3. Determine scope from user request. Default to journals if unspecified.

4. Run validation against the relevant schema:
   ```bash
   # Daily journals
   mdschema check "$VAULT/Resources/Journals/Daily/**/*.md" \
       --schema "$VAULT/Templates/Journals/.mdschema"

   # Weekly journals
   mdschema check "$VAULT/Resources/Journals/Weekly/**/*.md" \
       --schema "$VAULT/Templates/Journals/.mdschema"

   # Memory (when requested)
   mdschema check "$VAULT/Orchestration/Memory/Insights/*.md" \
       --schema "$MODULES/forge-reflect/skills/MemoryInsights/Templates/.mdschema"
   ```

5. Present results to the user grouped by violation type.

---

## Report

Summarize validation results as a structured report:

1. Run the Check workflow, capturing full output.
2. Count violations by category:
   - **Frontmatter**: missing fields, wrong types
   - **Structure**: missing required sections, unexpected headings, heading hierarchy
3. Present summary table:

   ```
   | Directory     | Files | Passing | Failing | Top Violation           |
   |---------------|-------|---------|---------|-------------------------|
   | Daily 2026/01 | 31    | 12      | 19      | missing "## Plan"       |
   | Daily 2026/02 | 27    | 25      | 2       | missing 'tlp' field     |
   | Weekly 2026   | 8     | 8       | 0       | —                       |
   ```

4. List the top 5 most common violations with counts.
5. Suggest remediation approach (batch fix vs manual).

---

## Constraints

- Read-only — this skill validates, it does not fix. Remediation is a separate step.
- Skip directories that don't exist (graceful degradation).
- Skip validation entirely if `mdschema` is not installed — report the missing dependency.
- The journal `.mdschema` currently lives in the vault (`Templates/Journals/`). Future: migrate to forge-journals as source of truth.
- `mdschema` validates structure only — it cannot check semantic correctness (enum values, tag formats, date validity).
