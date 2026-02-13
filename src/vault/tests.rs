use super::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn finds_vault_with_obsidian_dir() {
    let dir = tempdir().unwrap();
    fs::create_dir_all(dir.path().join(".obsidian")).unwrap();
    let nested = dir.path().join("sub/deep");
    fs::create_dir_all(&nested).unwrap();

    let file = nested.join("note.md");
    fs::write(&file, "# Hello").unwrap();

    let vault = find_vault(file.to_str().unwrap());
    assert_eq!(vault, Some(dir.path().to_path_buf()));
}

#[test]
fn returns_none_without_obsidian_dir() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("note.md");
    fs::write(&file, "# Hello").unwrap();

    assert_eq!(find_vault(file.to_str().unwrap()), None);
}
