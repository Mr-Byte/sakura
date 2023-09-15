use convert_case::{Case, Casing};
use quote::{format_ident, quote};

use crate::codegen::model::GrammarSrc;

use super::fmt::format_src;

pub(crate) fn generate(grammar: &GrammarSrc) -> miette::Result<String> {
    let tokens = grammar.tokens.iter().map(|token| {
        let name = format_ident!("{}", token);
        let kind = format_ident!("{}", token.to_case(Case::UpperSnake));
        quote! {
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            pub struct #name {
                pub(crate) syntax: SyntaxToken,
            }

            impl std::fmt::Display for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    std::fmt::Display::fmt(&self.syntax, f)
                }
            }

            impl AstToken for #name {
                fn can_cast(kind: SyntaxKind) -> bool { kind == #kind }
                fn cast(syntax: SyntaxToken) -> Option<Self> {
                    if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
                }
                fn syntax(&self) -> &SyntaxToken { &self.syntax }
            }
        }
    });

    let ast = quote! {
        use crate::syntax::{SyntaxToken, AstToken, SyntaxKind::{self, *}};

        #(#tokens)*
    };

    format_src(&ast.to_string())
}
