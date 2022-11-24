use std::path::{Path, PathBuf};

pub mod syntax;

pub fn format_output(source: String) -> String {
    let rustfmt_toml = workspace_root().join("rustfmt.toml");
    let shell = xshell::Shell::new().expect("unable to open shell");
    let mut stdout = xshell::cmd!(shell, "rustfmt --config-path {rustfmt_toml}")
        .stdin(source)
        .read()
        .expect("failed to call rustfmt");

    stdout.insert_str(0, "//! Generated code, do not edit by hand.\n\n");
    stdout.push('\n');

    stdout
}

pub fn workspace_root() -> PathBuf {
    manifest_dir()
        .parent()
        .and_then(Path::parent)
        .expect("could not find workspace root")
        .to_owned()
}

pub fn syntax_src_dir() -> PathBuf {
    manifest_dir()
        .parent()
        .map(|parent| parent.join("syntax/src"))
        .expect("unable to find syntax dir")
}

pub fn parser_src_dir() -> PathBuf {
    manifest_dir()
        .parent()
        .map(|parent| parent.join("parser/src"))
        .expect("unable to find parser dir")
}

fn manifest_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_owned()
}
