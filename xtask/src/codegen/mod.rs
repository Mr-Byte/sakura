mod err;
mod generators;
mod model;

use self::err::CodegenError;
use crate::workspace;
use clap::Parser;
use miette::{Context, IntoDiagnostic};
use model::{lower, GrammarSrc, SYNTAX_KINDS_SRC};

const GRAMMAR_SRC: &str = include_str!("../../../sakura.ungram");

#[derive(Debug, Parser)]
pub(crate) struct Codegen;

impl Codegen {
    pub(crate) fn run(&self) -> miette::Result<()> {
        let root_path = workspace::root_path()?;
        let grammar = GRAMMAR_SRC.parse().into_diagnostic().context("failed to parse grammar")?;
        let grammar = lower(grammar);

        [
            generate_syntax_kinds(&root_path),
            generate_syntax_nodes(&root_path, &grammar),
            generate_syntax_tokens(&root_path, &grammar),
        ]
        .into_iter()
        .collect::<CodegenError>()
        .into()
    }
}

fn generate_syntax_kinds(root_path: &std::path::Path) -> miette::Result<()> {
    let src = generators::syntax_kinds::generate(SYNTAX_KINDS_SRC)?;
    let path = root_path.join("crates/libsakura/src/syntax/kinds/generated_kinds.rs");

    workspace::ensure_file_contents(&path, &src)
}

fn generate_syntax_nodes(
    root_path: &std::path::PathBuf,
    grammar: &GrammarSrc,
) -> miette::Result<()> {
    let src = generators::syntax_nodes::generate(SYNTAX_KINDS_SRC, grammar)?;
    let path = root_path.join("crates/libsakura/src/syntax/ast/generated_nodes.rs");

    workspace::ensure_file_contents(&path, &src)
}

fn generate_syntax_tokens(
    root_path: &std::path::PathBuf,
    grammar: &GrammarSrc,
) -> miette::Result<()> {
    let src = generators::syntax_tokens::generate(SYNTAX_KINDS_SRC, grammar)?;
    let path = root_path.join("crates/libsakura/src/syntax/ast/generated_tokens.rs");

    workspace::ensure_file_contents(&path, &src)
}
