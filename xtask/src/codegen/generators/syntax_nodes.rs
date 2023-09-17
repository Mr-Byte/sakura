use crate::codegen::{
    err::CodegenError,
    model::{GrammarSrc, SyntaxKindsSrc},
};
use convert_case::{Case, Casing};
use quote::{format_ident, quote};
use std::collections::HashSet;

use super::fmt::format_src;

pub(crate) fn generate(
    kinds: SyntaxKindsSrc<'_>,
    grammar: &GrammarSrc,
) -> Result<String, CodegenError> {
    let nodes = get_nodes(grammar);
    let enums = get_enums(grammar);

    let node_names = grammar.nodes.iter().map(|it| &it.name);
    let defined_nodes: HashSet<_> = node_names.collect();

    for node in kinds
        .nodes
        .iter()
        .map(|kind| kind.to_case(Case::Pascal))
        .filter(|name| !defined_nodes.iter().any(|&it| it == name))
    {
        eprintln!("Warning: node {} not defined in ast source", node);

        drop(node);
    }

    let ast = quote! {
        use crate::T;
        use crate::syntax::{SyntaxNode, SyntaxToken, SyntaxKind::{self, *}};
        use crate::syntax::ast::{AstNode, AstChildren};
        use super::support;

        #(#nodes)*
        #(#enums)*
    };

    let result = format_src(&ast.to_string())?;
    Ok(result)
}

fn get_nodes(
    grammar: &GrammarSrc,
) -> impl std::iter::Iterator<Item = proc_macro2::TokenStream> + '_ {
    grammar.nodes.iter().map(|node| {
        let name = format_ident!("{}", node.name);
        let kind = format_ident!("{}", &node.name.to_case(Case::UpperSnake));
        let traits = node.traits.iter().map(|trait_name| {
            let trait_name = format_ident!("{}", trait_name);
            quote!(impl ast::#trait_name for #name {})
        });

        let methods = node.fields.iter().map(|field| {
            let method_name = field.method_name();
            let ty = field.ty();

            if field.is_many() {
                quote! {
                    pub fn #method_name(&self) -> AstChildren<#ty> {
                        support::children(&self.syntax)
                    }
                }
            } else if let Some(token_kind) = field.token_kind() {
                quote! {
                    pub fn #method_name(&self) -> Option<#ty> {
                        support::token(&self.syntax, #token_kind)
                    }
                }
            } else {
                quote! {
                    pub fn #method_name(&self) -> Option<#ty> {
                        support::child(&self.syntax)
                    }
                }
            }
        });

        quote! {
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            pub struct #name {
                pub(crate) syntax: SyntaxNode,
            }

            #(#traits)*

            impl #name {
                #(#methods)*
            }

            impl AstNode for #name {
                fn can_cast(kind: SyntaxKind) -> bool {
                    kind == #kind
                }

                fn cast(syntax: SyntaxNode) -> Option<Self> {
                    if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
                }

                fn syntax(&self) -> &SyntaxNode { &self.syntax }
            }
        }
    })
}

fn get_enums(
    grammar: &GrammarSrc,
) -> impl std::iter::Iterator<Item = proc_macro2::TokenStream> + '_ {
    grammar.enums.iter().map(|en| {
        let variants: Vec<_> = en.variants.iter().map(|var| format_ident!("{}", var)).collect();
        let name = format_ident!("{}", en.name);
        let kinds: Vec<_> = variants
            .iter()
            .map(|name| format_ident!("{}", name.to_string().to_case(Case::UpperSnake)))
            .collect();
        let traits = en.traits.iter().map(|trait_name| {
            let trait_name = format_ident!("{}", trait_name);
            quote!(impl ast::#trait_name for #name {})
        });

        let ast_node = quote! {
                impl AstNode for #name {
                    fn can_cast(kind: SyntaxKind) -> bool {
                        matches!(kind, #(#kinds)|*)
                    }
                    fn cast(syntax: SyntaxNode) -> Option<Self> {
                        let res = match syntax.kind() {
                            #(
                            #kinds => #name::#variants(#variants { syntax }),
                            )*
                            _ => return None,
                        };
                        Some(res)
                    }
                    fn syntax(&self) -> &SyntaxNode {
                        match self {
                            #(
                            #name::#variants(it) => it.syntax(),
                            )*
                        }
                    }
                }
        };

        quote! {
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            pub enum #name {
                #(#variants(#variants),)*
            }

            #(#traits)*

            #(
                impl From<#variants> for #name {
                    fn from(node: #variants) -> #name {
                        #name::#variants(node)
                    }
                }
            )*
            #ast_node
        }
    })
}
