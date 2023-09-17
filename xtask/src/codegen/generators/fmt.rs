use anyhow::bail;
use xshell::{cmd, Shell};

pub(crate) fn format_src(text: &str) -> Result<String, anyhow::Error> {
    ensure_rustfmt()?;

    let shell = Shell::new()?;
    let _env = shell.push_env("RUSTUP_TOOLCHAIN", "stable");

    let cmd_output = cmd!(shell, "rustfmt --config blank_lines_lower_bound=1")
        .stdin(text)
        .read()?
        .replace("]\n", "]")
        .replace(" !", "!");

    // NOTE: Run rustfmt a second time to remove extraneous newlines.
    let cmd_output = cmd!(shell, "rustfmt").stdin(cmd_output).read()?;

    Ok(format!(
        "{}\n\n{}\n",
        "//! Code generated by `cargo xtask codegen`; DO NOT EDIT.", cmd_output
    ))
}

fn ensure_rustfmt() -> Result<(), anyhow::Error> {
    let shell = Shell::new()?;

    let cmd_output = cmd!(shell, "rustfmt --version").read()?;
    if !cmd_output.contains("stable") {
        bail!(
            "Failed to run rustfmt from toolchain 'stable'. \
             Please run `rustup component add rustfmt --toolchain stable` to install it.",
        )
    }
    Ok(())
}
