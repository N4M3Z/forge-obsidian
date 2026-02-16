---
name: ObsidianREST
description: Obsidian Local REST API — list vault files, search content, check note existence, read/write files via HTTPS. USE WHEN listing notes, searching vault, checking if a note exists, querying vault structure, or building note indexes.
argument-hint: ""
---

# ObsidianREST

Reference for the Obsidian Local REST API plugin. Provides an HTTPS server inside Obsidian for vault queries — list files, search content, check existence, read/write. Returns JSON to `curl`.

Falls back to file-system operations (Glob, Grep, `safe-read`) when unavailable.

## Setup

1. Install: Obsidian Settings → Community Plugins → search "Local REST API"
2. Copy the API key from Settings → Local REST API
3. Add to `Modules/forge-obsidian/.env` (gitignored):

```
OBSIDIAN_REST_API_KEY=<your-api-key>
```

HTTPS on port **27124**. Insecure HTTP (27123) is disabled by default. All requests require `-k` for the self-signed certificate.

## Authentication

```bash
export $(cat Modules/forge-obsidian/.env 2>/dev/null | xargs 2>/dev/null)
curl -sk "https://localhost:27124/..." -H "Authorization: Bearer $OBSIDIAN_REST_API_KEY"
```

## Endpoints

### Server info / health check

```bash
curl -sk "https://localhost:27124/" -H "Authorization: Bearer $OBSIDIAN_REST_API_KEY"
# { "authenticated": true, "service": "Obsidian Local REST API", ... }
```

### List files in a directory

```bash
# Vault root (trailing slash = directory listing)
curl -sk "https://localhost:27124/vault/" -H "Authorization: Bearer $OBSIDIAN_REST_API_KEY"
# { "files": ["Topics/", "Resources/", "Orchestration/", "Vault.md", ...] }

# Subdirectory
curl -sk "https://localhost:27124/vault/Topics/" -H "Authorization: Bearer $OBSIDIAN_REST_API_KEY"
# { "files": ["Security.md", "Proton.md", ...] }
```

Trailing slash distinguishes directory listing from file access. Without trailing slash, the path is treated as a filename.

### Check if a note exists

```bash
curl -sk -o /dev/null -w "%{http_code}" \
  "https://localhost:27124/vault/Resources/Applications/Obsidian.md" \
  -H "Authorization: Bearer $OBSIDIAN_REST_API_KEY"
# 200 = exists, 404 = doesn't
```

### Read file content

```bash
# Raw markdown
curl -sk "https://localhost:27124/vault/Topics/Security.md" \
  -H "Authorization: Bearer $OBSIDIAN_REST_API_KEY"

# Structured JSON (frontmatter parsed, tags extracted)
curl -sk "https://localhost:27124/vault/Topics/Security.md" \
  -H "Authorization: Bearer $OBSIDIAN_REST_API_KEY" \
  -H "Accept: application/vnd.olrapi.note+json"
```

### Search vault content

```bash
# Simple text search (POST method, query in URL params)
curl -sk -X POST \
  "https://localhost:27124/search/simple/?query=forge-tlp&contextLength=100" \
  -H "Authorization: Bearer $OBSIDIAN_REST_API_KEY"
# [{ "filename": "...", "score": N, "matches": [{ "match": { "start": N, "end": N }, "context": "..." }] }]

# Dataview DQL query (POST, body is the query)
curl -sk -X POST "https://localhost:27124/search/" \
  -H "Authorization: Bearer $OBSIDIAN_REST_API_KEY" \
  -H "Content-Type: application/vnd.olrapi.dataview.dql+txt" \
  -d 'TABLE file.name FROM "Topics" WHERE contains(file.name, "Security")'

# JsonLogic query (POST, JSON body)
curl -sk -X POST "https://localhost:27124/search/" \
  -H "Authorization: Bearer $OBSIDIAN_REST_API_KEY" \
  -H "Content-Type: application/vnd.olrapi.jsonlogic+json" \
  -d '{"glob": ["*.md", {"var": "path"}]}'
```

### Write / create a file

```bash
curl -sk -X PUT "https://localhost:27124/vault/Topics/NewNote.md" \
  -H "Authorization: Bearer $OBSIDIAN_REST_API_KEY" \
  -H "Content-Type: text/markdown" \
  -d '---
title: New Note
---

Content here.'
```

### Open a note in Obsidian

```bash
curl -sk -X POST "https://localhost:27124/open/Topics/Security.md" \
  -H "Authorization: Bearer $OBSIDIAN_REST_API_KEY"
```

## Detecting Availability

```bash
export $(cat Modules/forge-obsidian/.env 2>/dev/null | xargs 2>/dev/null)
if curl -sk --max-time 2 "https://localhost:27124/" \
     -H "Authorization: Bearer ${OBSIDIAN_REST_API_KEY:-none}" 2>/dev/null \
     | grep -q '"authenticated":true'; then
  echo "REST API available"
else
  echo "REST API unavailable — falling back to file system"
fi
```

## Fallback

| REST API operation | File-system fallback |
|-------------------|---------------------|
| `GET /vault/{dir}/` (list) | `find "$FORGE_USER_ROOT" -name '*.md'` or Glob tool |
| `POST /search/simple/` | Grep tool |
| `GET /vault/{path}` (exists) | Glob `**/{name}.md` |
| `GET /vault/{path}` (read) | `safe-read` or Read tool |
| `PUT /vault/{path}` (write) | `safe-write write` or Write tool |

## Constraints

- Requires Obsidian desktop app to be running
- Self-signed HTTPS certificate — always use `-k` with curl
- API key is local-only (localhost) — not a remote risk, but keep `.env` gitignored
- Does not work in headless/CI environments
- `/search/simple/` is POST, not GET (common mistake)
- Trailing slash on directory paths is required for listings
