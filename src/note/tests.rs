use super::*;
use std::fs;
use tempfile::tempdir;

fn make_vault() -> (tempfile::TempDir, PathBuf) {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join(".obsidian")).unwrap();
    fs::create_dir_all(dir.path().join("Notes")).unwrap();
    let root = dir.path().to_path_buf();
    (dir, root)
}

#[test]
fn parses_frontmatter_and_body() {
    let content = "---\ntitle: Hello\ntags:\n  - type/note\n  - topic/rust\n---\n# Hello\n\nBody text with [[Link One]] and [[Link Two|alias]].\n";
    let (props, body) = parse_frontmatter(content);
    assert_eq!(props.get("title").and_then(Value::as_str), Some("Hello"));
    assert!(body.contains("# Hello"));
    assert!(body.contains("[[Link One]]"));
}

#[test]
fn extracts_tags_from_sequence() {
    let mut props = HashMap::new();
    props.insert(
        "tags".to_owned(),
        Value::Sequence(vec![
            Value::String("type/note".into()),
            Value::String("#topic/rust".into()),
        ]),
    );
    let tags = extract_tags(&props);
    assert_eq!(tags, vec!["type/note", "topic/rust"]);
}

#[test]
fn extracts_wikilinks() {
    let body = "See [[Note A]] and [[Note B|display]] but not [regular](link).";
    let links = extract_wikilinks(body);
    assert_eq!(links, vec!["Note A", "Note B"]);
}

#[test]
fn note_context_from_file() {
    let (dir, vault) = make_vault();
    let file = dir.path().join("Notes/test.md");
    fs::write(
        &file,
        "---\ntitle: Test\ntags:\n  - type/item\ncreated: 2026-01-15\n---\n# Test\n\nLinks to [[Other Note]].\n",
    )
    .unwrap();

    let ctx = NoteContext::from_file(&vault, &file).unwrap();
    assert_eq!(ctx.name, "test");
    assert_eq!(ctx.ext, "md");
    assert_eq!(ctx.folder, "Notes");
    assert_eq!(ctx.rel_path, "Notes/test.md");
    assert_eq!(ctx.tags, vec!["type/item"]);
    assert_eq!(ctx.links, vec!["Other Note"]);
    assert!(ctx.has_tag("type/item"));
    assert!(!ctx.has_tag("type/project"));
    assert!(ctx.has_link("Other Note"));
}

#[test]
fn walk_vault_skips_hidden_dirs() {
    let (dir, vault) = make_vault();
    fs::write(dir.path().join("visible.md"), "---\ntitle: V\n---\n").unwrap();
    fs::create_dir_all(dir.path().join(".hidden")).unwrap();
    fs::write(dir.path().join(".hidden/secret.md"), "---\ntitle: S\n---\n").unwrap();

    let notes = walk_vault(&vault);
    assert_eq!(notes.len(), 1);
    assert_eq!(notes[0].name, "visible");
}
