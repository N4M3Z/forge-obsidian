use super::*;
use crate::note::NoteContext;
use std::collections::HashMap;
use std::path::PathBuf;

fn make_note(name: &str, folder: &str, tags: &[&str], links: &[&str]) -> NoteContext {
    let mut properties = HashMap::new();
    let tag_vals: Vec<serde_yaml::Value> = tags.iter().map(|t| serde_yaml::Value::String(t.to_string())).collect();
    properties.insert("tags".to_owned(), serde_yaml::Value::Sequence(tag_vals));
    properties.insert("created".to_owned(), serde_yaml::Value::String("2026-01-15".into()));

    NoteContext {
        path: PathBuf::from(format!("/vault/{folder}/{name}.md")),
        rel_path: format!("{folder}/{name}.md"),
        name: name.to_owned(),
        ext: "md".to_owned(),
        folder: folder.to_owned(),
        tags: tags.iter().map(|s| s.to_string()).collect(),
        links: links.iter().map(|s| s.to_string()).collect(),
        properties,
        content: String::new(),
    }
}

fn make_this(name: &str, folder: &str) -> ThisContext {
    ThisContext {
        name: name.to_owned(),
        folder: folder.to_owned(),
        rel_path: format!("{folder}/{name}.base"),
        properties: HashMap::new(),
    }
}

// ─── Tokenizer ──────────────────────────────────────────────

#[test]
fn tokenize_contains_call() {
    let tokens = tokenize(r#"contains(file.path, "Inventory")"#);
    assert_eq!(tokens[0], Token::Ident("contains".into()));
    assert_eq!(tokens[1], Token::LParen);
    assert_eq!(tokens[2], Token::Ident("file".into()));
    assert_eq!(tokens[3], Token::Dot);
    assert_eq!(tokens[4], Token::Ident("path".into()));
    assert_eq!(tokens[5], Token::Comma);
    assert_eq!(tokens[6], Token::Str("Inventory".into()));
    assert_eq!(tokens[7], Token::RParen);
}

#[test]
fn tokenize_neq() {
    let tokens = tokenize("file.fullname != this.file.fullname");
    assert!(tokens.contains(&Token::Neq));
}

#[test]
fn tokenize_negation() {
    let tokens = tokenize(r#"!file.name.contains("template")"#);
    assert_eq!(tokens[0], Token::Not);
    assert_eq!(tokens[1], Token::Ident("file".into()));
}

// ─── Filter evaluation ─────────────────────────────────────

#[test]
fn eval_contains_file_path() {
    let note = make_note("Paint", "Resources/Inventory/Painting", &["type/item/painting"], &[]);
    let this = make_this("Painting", "Resources/Inventory/Painting");
    assert!(eval_filter(r#"contains(file.path, "Inventory")"#, &note, &this));
    assert!(!eval_filter(r#"contains(file.path, "Contacts")"#, &note, &this));
}

#[test]
fn eval_contains_file_ext() {
    let note = make_note("Paint", "Notes", &[], &[]);
    let this = make_this("Test", "Notes");
    assert!(eval_filter(r#"contains(file.ext, "md")"#, &note, &this));
}

#[test]
fn eval_contains_property_tags() {
    let note = make_note("Paint", "Notes", &["type/item/painting"], &[]);
    let this = make_this("Test", "Notes");
    assert!(eval_filter(r#"contains(property.tags, "type/item/painting")"#, &note, &this));
    assert!(!eval_filter(r#"contains(property.tags, "type/person")"#, &note, &this));
}

#[test]
fn eval_file_fullname_neq_this() {
    let note = make_note("Note", "Notes", &[], &[]);
    let this = make_this("Base", "Notes");
    assert!(eval_filter("file.fullname != this.file.fullname", &note, &this));

    // Same file should fail
    let this_same = make_this("Note", "Notes");
    // rel_path won't match because note is .md and this is .base
    assert!(eval_filter("file.fullname != this.file.fullname", &note, &this_same));
}

#[test]
fn eval_starts_with() {
    let note = make_note("2026-01-15", "Resources/Journals/Daily/2026/01", &[], &[]);
    let this = make_this("Test", "Notes");
    assert!(eval_filter(r#"file.path.startsWith("Resources/Journals")"#, &note, &this));
}

#[test]
fn eval_negation() {
    let note = make_note("My Note", "Notes", &[], &[]);
    let this = make_this("Test", "Notes");
    assert!(eval_filter(r#"!file.name.contains("template")"#, &note, &this));

    let template = make_note("template", "Notes", &[], &[]);
    assert!(!eval_filter(r#"!file.name.contains("template")"#, &template, &this));
}

#[test]
fn eval_has_tag() {
    let note = make_note("Event", "Notes", &["type/event"], &[]);
    let this = make_this("Test", "Notes");
    assert!(eval_filter(r#"file.hasTag("type/event")"#, &note, &this));
    assert!(!eval_filter(r#"file.hasTag("type/project")"#, &note, &this));
}

#[test]
fn eval_has_tag_multi() {
    let note = make_note("Event", "Notes", &["type/event"], &[]);
    let this = make_this("Test", "Notes");
    assert!(eval_filter(r#"file.hasTag("type/project", "type/event")"#, &note, &this));
}

#[test]
fn eval_has_link() {
    let note = make_note("Daily", "Journals", &[], &["Project A", "Project B"]);
    let this = make_this("Project A", "Projects");
    assert!(eval_filter(r#"file.hasLink(this.file)"#, &note, &this));
}

#[test]
fn eval_method_chain_slice() {
    let note = make_note("Note", "Notes", &[], &[]);
    let this = make_this("2026-01-15", "Journals");
    // Test: this.file.name.slice(5, 10) should give "01-15"
    let tokens = tokenize("this.file.name.slice(5, 10)");
    let mut parser = Parser::new(tokens);
    let expr = parser.parse_expr().unwrap();
    let result = eval_expr(&expr, &note, &this);
    assert_eq!(result.to_string_val(), "01-15");
}

#[test]
fn eval_folder_starts_with() {
    let note = make_note("Child", "Projects/MyProject", &[], &[]);
    let this = make_this("MyProject", "Projects/MyProject");
    assert!(eval_filter(r#"file.folder.startsWith(this.file.folder)"#, &note, &this));
}

#[test]
fn eval_file_links_contains() {
    let note = make_note("Daily", "Journals", &[], &["MyProject"]);
    let this = make_this("MyProject", "Projects");
    assert!(eval_filter("file.links.contains(this.file)", &note, &this));
}
