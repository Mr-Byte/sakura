use miette::{bail, miette, IntoDiagnostic, Result};
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
    .ok_or(miette!("Failed to find root path"))
    .map(Path::to_path_buf)
}

pub(crate) fn ensure_file_contents(file: &Path, contents: &str) -> Result<()> {
    if let Ok(old_contents) = fs::read_to_string(file) {
        if normalize_newlines(&old_contents) == normalize_newlines(contents) {
            return Ok(());
        }
    }

    let display_path = file.strip_prefix(root_path()?).unwrap_or(file);

    eprintln!(
        "\n\x1b[31;1merror\x1b[0m: {} was not up-to-date, updating\n",
        display_path.display()
    );

    if let Some(parent) = file.parent() {
        fs::create_dir_all(parent).into_diagnostic()?;
    }

    fs::write(file, contents).into_diagnostic()?;

    bail!("{} has been updated, please re-run `cargo test`", file.display());
}

fn normalize_newlines(s: &str) -> String {
    s.replace("\r\n", "\n")
}
