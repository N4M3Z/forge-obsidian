#[cfg(test)]
mod tests;

use std::path::{Path, PathBuf};

/// Walk up from `start` looking for an `.obsidian/` directory.
fn find_vault_from_dir(start: &Path) -> Option<PathBuf> {
    let mut dir = start;
    loop {
        if dir.join(".obsidian").is_dir() {
            return Some(dir.to_path_buf());
        }
        dir = dir.parent()?;
    }
}

/// Find the vault root for a given file path.
pub fn find_vault(file_path: &str) -> Option<PathBuf> {
    let path = Path::new(file_path);
    let start = if path.is_dir() { path } else { path.parent()? };
    find_vault_from_dir(start)
}

/// Find the vault root from the current working directory.
pub fn find_vault_from_cwd() -> Option<PathBuf> {
    let cwd = std::env::current_dir().ok()?;
    find_vault_from_dir(&cwd)
}
