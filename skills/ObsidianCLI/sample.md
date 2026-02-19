---
title: ObsidianCLI Demo
tags:
  - type/demo
---

# ObsidianCLI Demo

Common operations using the official Obsidian CLI (1.12+).

## List all markdown files

```bash
obsidian files ext=md format=paths
```

## Search for a term

```bash
obsidian search query="forge-tlp" format=json
```

## Set frontmatter property

```bash
obsidian property:set file="Security" property=status value=active
```

## Check backlinks

```bash
obsidian backlinks file="Obsidian" format=json
```

## Rename with backlink updates

```bash
obsidian rename file="Old Name" to="New Name"
```

## Query a Base view

```bash
obsidian base:query path="Resources/Books.base" format=paths
```

## Daily note operations

```bash
obsidian daily:path
obsidian daily:append content="- [#] #log/effort/mid [[Project]] — description"
```
