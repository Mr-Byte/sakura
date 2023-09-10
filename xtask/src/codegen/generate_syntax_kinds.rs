use anyhow::Result;
use proc_macro2::{Punct, Spacing};
use quote::{format_ident, quote};

use super::{ast_src::KindsSource, fmt::format_src};

pub(crate) fn generate_kinds(kinds: KindsSource<'_>) -> Result<String> {
    let (single_char_token_values, single_char_token_names): (Vec<_>, Vec<_>) = kinds
        .punctuation
        .iter()
        .filter(|(token, _)| token.len() == 1)
        .filter_map(|(token, name)| {
            token.chars().nth(0).map(|char| (char, format_ident!("{}", name)))
        })
        .unzip();

    let punctuation_values = kinds.punctuation.iter().flat_map(|(token, _name)| {
        if "{}[]()".contains(token) {
            token.chars().nth(0).map(|c| quote! { #c })
        } else {
            let cs = token.chars().map(|c| Punct::new(c, Spacing::Joint));
            Some(quote! { #(#cs)* })
        }
    });
    let punctuation =
        kinds.punctuation.iter().map(|(_, name)| format_ident!("{}", name)).collect::<Vec<_>>();

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
            #(#literals,)*
            #(#tokens,)*
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
            #([#punctuation_values] => { $crate::SyntaxKind::#punctuation };)*
            [IDENTIFIER] => { $crate::SyntaxKind::IDENTIFIER };
        }
    };

    format_src(&ast.to_string())
}
