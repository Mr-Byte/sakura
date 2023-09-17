mod err;
mod generators;
mod model;
mod workspace;

use anyhow::Context;
use clap::Parser;
use model::{lower, GrammarSrc, SYNTAX_KINDS_SRC};

use self::err::CodegenError;

const GRAMMAR_SRC: &str = include_str!("../../../sakura.ungram");

#[derive(Debug, Parser)]
pub(crate) struct Codegen;

impl Codegen {
    pub(crate) fn run(&self) -> anyhow::Result<()> {
        let root_path = workspace::root_path()?;
        let grammar = GRAMMAR_SRC.parse().context("failed to parse grammar")?;
        let grammar = lower(grammar);
        let results = [
            generate_syntax_kinds(&root_path),
            generate_syntax_nodes(&root_path, &grammar),
            generate_syntax_tokens(&root_path, &grammar),
        ];

        print_results(results)?;

        Ok(())
    }
}

fn print_results(
    results: impl IntoIterator<Item = Result<(), CodegenError>>,
) -> Result<(), anyhow::Error> {
    let mut code_updated = false;
    for result in results.into_iter() {
        let Err(CodegenError::CodeUpdated(err)) = result else {
            result.context("code generator failed")?;

            continue;
        };

        code_updated = true;
        eprintln!("❌ {}", err);
    }

    if code_updated {
        anyhow::bail!("code updated");
    }

    Ok(())
}

fn generate_syntax_kinds(root_path: &std::path::Path) -> anyhow::Result<(), CodegenError> {
    let src = generators::syntax_kinds::generate(SYNTAX_KINDS_SRC)?;
    let path = root_path.join("crates/libsakura/src/syntax/kinds/generated_kinds.rs");

    workspace::ensure_file_contents(&path, &src)
}

fn generate_syntax_nodes(
    root_path: &std::path::PathBuf,
    grammar: &GrammarSrc,
) -> anyhow::Result<(), CodegenError> {
    let src = generators::syntax_nodes::generate(SYNTAX_KINDS_SRC, grammar)?;
    let path = root_path.join("crates/libsakura/src/syntax/ast/generated_nodes.rs");

    workspace::ensure_file_contents(&path, &src)
}

fn generate_syntax_tokens(
    root_path: &std::path::PathBuf,
    grammar: &GrammarSrc,
) -> anyhow::Result<(), CodegenError> {
    let src = generators::syntax_tokens::generate(grammar)?;
    let path = root_path.join("crates/libsakura/src/syntax/ast/generated_tokens.rs");

    workspace::ensure_file_contents(&path, &src)
}
