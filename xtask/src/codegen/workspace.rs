use anyhow::anyhow;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

use super::err::CodegenError;

pub(crate) fn root_path() -> Result<PathBuf, CodegenError> {
    Path::new(
        &env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned()),
    )
    .ancestors()
    .nth(1)
    .ok_or(anyhow!("Failed to find root path").into())
    .map(Path::to_path_buf)
}

pub(crate) fn ensure_file_contents(file: &Path, contents: &str) -> Result<(), CodegenError> {
    if let Ok(old_contents) = fs::read_to_string(file) {
        if normalize_newlines(&old_contents) == normalize_newlines(contents) {
            return Ok(());
        }
    }

    if let Some(parent) = file.parent() {
        fs::create_dir_all(parent).map_err(Into::<anyhow::Error>::into)?;
    }

    fs::write(file, contents).map_err(Into::<anyhow::Error>::into)?;

    let display_path = file.strip_prefix(root_path()?).unwrap_or(file);
    let err = format!("{} has been updated, please re-run `cargo test`", display_path.display());

    Err(CodegenError::CodeUpdated(err))
}

fn normalize_newlines(s: &str) -> String {
    s.replace("\r\n", "\n")
}
