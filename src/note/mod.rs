#[cfg(test)]
mod tests;

use regex::Regex;
use serde_yaml::Value;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use walkdir::WalkDir;

static WIKILINK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[\[([^\]|]+)(?:\|[^\]]+)?\]\]").unwrap());

/// A vault note enriched with file metadata and parsed frontmatter.
#[derive(Debug, Clone)]
pub struct NoteContext {
    pub path: PathBuf,
    pub rel_path: String,
    pub name: String,
    pub ext: String,
    pub folder: String,
    pub tags: Vec<String>,
    pub links: Vec<String>,
    pub properties: HashMap<String, Value>,
    pub content: String,
}

impl NoteContext {
    /// Build a `NoteContext` from a file path relative to the vault root.
    pub fn from_file(vault_root: &Path, abs_path: &Path) -> Option<Self> {
        let content = fs::read_to_string(abs_path).ok()?;
        let rel_path = abs_path.strip_prefix(vault_root).ok()?;

        let name = abs_path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();
        let ext = abs_path
            .extension()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();
        let folder = rel_path
            .parent()
            .unwrap_or(Path::new(""))
            .to_string_lossy()
            .into_owned();

        let (properties, body) = parse_frontmatter(&content);

        let tags = extract_tags(&properties);
        let links = extract_wikilinks(body);

        Some(Self {
            path: abs_path.to_path_buf(),
            rel_path: rel_path.to_string_lossy().into_owned(),
            name,
            ext,
            folder,
            tags,
            links,
            properties,
            content,
        })
    }

    /// Get a frontmatter property value by key.
    pub fn get_property(&self, key: &str) -> Option<&Value> {
        self.properties.get(key)
    }

    /// Check if the note has a specific tag (prefix match: "type/project" matches "type/project/foo").
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag || t.starts_with(&format!("{tag}/")))
    }

    /// Check if the note links to a given target (by name, case-insensitive).
    pub fn has_link(&self, target: &str) -> bool {
        let target_lower = target.to_lowercase();
        self.links.iter().any(|l| l.to_lowercase() == target_lower)
    }
}

/// Walk a vault directory and build `NoteContext` for every `.md` file.
pub fn walk_vault(vault_root: &Path) -> Vec<NoteContext> {
    // Canonicalize to resolve symlinks (macOS /var → /private/var)
    let vault_root = vault_root.canonicalize().unwrap_or_else(|_| vault_root.to_path_buf());
    let mut notes = Vec::new();
    for entry in WalkDir::new(&vault_root)
        .into_iter()
        .filter_entry(|e| {
            // Skip hidden directories (like .obsidian, .trash) but not the root
            e.depth() == 0 || !e.file_name().to_string_lossy().starts_with('.')
        })
        .filter_map(Result::ok)
    {
        let path = entry.into_path();
        let path = path.canonicalize().unwrap_or(path);
        if path.is_file() && path.extension().is_some_and(|e| e == "md") {
            if let Some(ctx) = NoteContext::from_file(&vault_root, &path) {
                notes.push(ctx);
            }
        }
    }
    notes
}

/// Split frontmatter from content. Returns (properties, body).
fn parse_frontmatter(content: &str) -> (HashMap<String, Value>, &str) {
    let empty = (HashMap::new(), content);

    if !content.starts_with("---") {
        return empty;
    }

    let after_first = &content[3..];
    let after_first = after_first.strip_prefix('\n').unwrap_or(after_first);

    let Some(end) = after_first.find("\n---") else {
        return empty;
    };

    let yaml = &after_first[..end];
    let rest = &after_first[end + 4..];
    let body = rest.strip_prefix('\n').unwrap_or(rest);

    let Ok(Value::Mapping(mapping)) = serde_yaml::from_str::<Value>(yaml) else {
        return (HashMap::new(), body);
    };

    let props: HashMap<String, Value> = mapping
        .into_iter()
        .filter_map(|(k, v)| k.as_str().map(|s| (s.to_owned(), v)))
        .collect();

    (props, body)
}

/// Extract tags from frontmatter `tags` field (handles string, list, null).
fn extract_tags(props: &HashMap<String, Value>) -> Vec<String> {
    let Some(val) = props.get("tags") else {
        return Vec::new();
    };
    match val {
        Value::Sequence(seq) => seq
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.trim_start_matches('#').to_owned()))
            .collect(),
        Value::String(s) => s
            .split(',')
            .map(|t| t.trim().trim_start_matches('#').to_owned())
            .filter(|t| !t.is_empty())
            .collect(),
        _ => Vec::new(),
    }
}

/// Extract `[[wikilinks]]` from note body.
fn extract_wikilinks(body: &str) -> Vec<String> {
    WIKILINK_RE
        .captures_iter(body)
        .map(|cap| cap[1].to_owned())
        .collect()
}
