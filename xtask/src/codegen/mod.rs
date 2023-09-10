use anyhow::{Context, Result};
use clap::Parser;
use ungrammar::Grammar;

use crate::workspace;

use self::ast_src::KINDS_SOURCE;

mod ast_src;
mod fmt;
mod generate_syntax_kinds;

#[derive(Debug, Parser)]
pub(crate) struct Codegen;

impl Codegen {
    pub(crate) fn run(&self) -> Result<()> {
        let root_path = workspace::root_path()?;

        let syntax_kinds_src = generate_syntax_kinds::generate_kinds(KINDS_SOURCE)?;
        let syntax_kinds_path =
            root_path.clone().join("crates/libsakura/src/syntax/kinds/generated.rs");
        workspace::ensure_file_contents(&syntax_kinds_path, &syntax_kinds_src)?;

        // const GRAMMAR_SRC: &str = include_str!("../../../sakura.ungram");
        // let grammar: Grammar = GRAMMAR_SRC.parse().context("failed to parse grammar")?;

        Ok(())
    }
}
