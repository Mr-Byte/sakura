use anyhow::{anyhow, bail, Result};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub(crate) fn root_path() -> Result<PathBuf> {
    Path::new(
        &env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned()),
    )
    .ancestors()
    .nth(1)
    .ok_or(anyhow!("Failed to find root path"))
    .map(Path::to_path_buf)
}

pub(crate) fn ensure_file_contents(file: &Path, contents: &str) -> Result<()> {
    if let Ok(old_contents) = fs::read_to_string(file) {
        if normalize_newlines(&old_contents) == normalize_newlines(contents) {
            // File is already up to date.
            return Ok(());
        }
    }

    let display_path = file.strip_prefix(root_path()?).unwrap_or(file);

    eprintln!(
        "\n\x1b[31;1merror\x1b[0m: {} was not up-to-date, updating\n",
        display_path.display()
    );

    if std::env::var("CI").is_ok() {
        eprintln!("    NOTE: run `cargo test` locally and commit the updated files\n");
    }

    if let Some(parent) = file.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(file, contents)?;

    bail!("A code generated file has been updated, please re-run `cargo test`");
}

fn normalize_newlines(s: &str) -> String {
    s.replace("\r\n", "\n")
}
