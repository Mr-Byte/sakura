use crate::codegen::model::{GrammarSrc, SyntaxKindsSrc};

pub(crate) fn generate(kinds: SyntaxKindsSrc<'_>, grammar: &GrammarSrc) -> miette::Result<String> {
    miette::bail!("TODO: implement")
}
