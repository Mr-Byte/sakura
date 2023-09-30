use crate::codegen::{err::CodegenError, model::SyntaxKindsSrc};
use convert_case::{Case, Casing};
use quote::{format_ident, quote};

use super::fmt::format_src;

pub(crate) fn generate(kinds: SyntaxKindsSrc<'_>) -> Result<String, CodegenError> {
    let (single_char_token_values, single_char_token_names) = get_single_char_tokens(&kinds);
    let (punctuation_values, punctuation) = get_punctuation(&kinds);
    let (keyword_values, keywords) = get_keywords(&kinds);

    let literals = kinds.literals.iter().map(|name| format_ident!("{}", name)).collect::<Vec<_>>();
    let tokens = kinds.tokens.iter().map(|name| format_ident!("{}", name)).collect::<Vec<_>>();
    let nodes = kinds.nodes.iter().map(|name| format_ident!("{}", name)).collect::<Vec<_>>();

    let ast = quote! {
        #![allow(dead_code, bad_style, missing_docs, unreachable_pub, clippy::manual_non_exhaustive, clippy::upper_case_acronyms)]
        #[non_exhaustive]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(u16)]
        pub enum SyntaxKind {
            // NOTE: Temporary kind used in parsing, but not present in the final tree.
            #[doc(hidden)]
            TOMBSTONE,
            #[doc(hidden)]
            EOF,
            #(#punctuation,)*
            #(#keywords,)*
            #(#literals,)*
            #(#tokens,)*

            //NOTE: Used by TokenSet to ensure it can't be used to store nodes.
            #[doc(hidden)]
            __TOKEN_SENTINEL,

            #(#nodes,)*

            // NOTE: Kind used to help enable u16 casting.
            #[doc(hidden)]
            __LAST,
        }

        use self::SyntaxKind::*;

        impl SyntaxKind {
            pub fn is_punctuation(self) -> bool {
                matches!(self, #(#punctuation)|*)
            }

            pub fn is_literal(self) -> bool {
                matches!(self, #(#literals)|*)
            }

            pub fn is_keyword(self) -> bool {
                matches!(self, #(#keywords)|*)
            }

            pub fn from_keyword(ident: &str) -> Option<SyntaxKind> {
                let kw = match ident {
                    #(#keyword_values => #keywords,)*
                    _ => return None,
                };

                Some(kw)
            }

            pub fn from_char(c: char) -> Option<SyntaxKind> {
                let tok = match c {
                    #(#single_char_token_values => #single_char_token_names,)*
                    _ => return None,
                };
                Some(tok)
            }
        }

        #[macro_export]
        macro_rules! T {
            #([#punctuation_values] => { $crate::syntax::SyntaxKind::#punctuation };)*
            #([#keyword_values] => { $crate::syntax::SyntaxKind::#keywords };)*
            ["identifier"] => { $crate::syntax::SyntaxKind::IDENTIFIER };
        }
    };

    let result = format_src(&ast.to_string())?;
    Ok(result)
}

fn get_punctuation<'a>(kinds: &'a SyntaxKindsSrc<'_>) -> (Vec<&'a str>, Vec<proc_macro2::Ident>) {
    let punctuation_values =
        kinds.punctuation.iter().map(|(token, _name)| *token).collect::<Vec<_>>();
    let punctuation =
        kinds.punctuation.iter().map(|(_, name)| format_ident!("{}", name)).collect::<Vec<_>>();

    (punctuation_values, punctuation)
}

fn get_single_char_tokens(kinds: &SyntaxKindsSrc<'_>) -> (Vec<char>, Vec<proc_macro2::Ident>) {
    kinds
        .punctuation
        .iter()
        .filter(|(token, _)| token.len() == 1)
        .filter_map(|(token, name)| {
            token.chars().next().map(|char| (char, format_ident!("{}", name)))
        })
        .unzip()
}

fn get_keywords<'a>(kinds: &'a SyntaxKindsSrc<'_>) -> (&'a [&'a str], Vec<proc_macro2::Ident>) {
    let keyword_values = kinds.keywords;
    let keywords = kinds
        .keywords
        .iter()
        .map(|name| format_ident!("{}_KW", name.to_case(Case::UpperSnake)))
        .collect::<Vec<_>>();

    (keyword_values, keywords)
}
