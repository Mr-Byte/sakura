use std::collections::BTreeSet;

use convert_case::{Case, Casing};
use quote::{format_ident, quote};
use ungrammar::{Grammar, Rule};

#[derive(Default, Debug)]
pub(crate) struct GrammarSrc {
    pub(crate) tokens: Vec<String>,
    pub(crate) nodes: Vec<NodeSrc>,
    pub(crate) enums: Vec<EnumSrc>,
}

#[derive(Debug)]
pub(crate) struct NodeSrc {
    pub(crate) name: String,
    pub(crate) traits: Vec<String>,
    pub(crate) fields: Vec<Field>,
}

#[derive(Debug, Clone)]
pub(crate) struct EnumSrc {
    pub(crate) name: String,
    pub(crate) traits: Vec<String>,
    pub(crate) variants: Vec<String>,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Field {
    Token(String),
    Node { name: String, ty: String, cardinality: Cardinality },
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Cardinality {
    Optional,
    Many,
}

impl NodeSrc {
    fn remove_field(&mut self, to_remove: Vec<usize>) {
        to_remove.into_iter().rev().for_each(|idx| {
            self.fields.remove(idx);
        });
    }
}

impl Field {
    pub(crate) fn is_many(&self) -> bool {
        matches!(self, Field::Node { cardinality: Cardinality::Many, .. })
    }

    pub(crate) fn token_kind(&self) -> Option<proc_macro2::TokenStream> {
        match self {
            Field::Token(token) => Some(quote! { T![#token] }),
            _ => None,
        }
    }

    pub(crate) fn method_name(&self) -> proc_macro2::Ident {
        match self {
            Field::Token(name) => {
                // TODO: Add Sakura tokens
                let name = match name.as_str() {
                    "!" => "bang",
                    "#" => "hash",
                    "{" => "left_curly",
                    "}" => "right_curly",
                    "(" => "left_paren",
                    ")" => "right_paren",
                    "[" => "left_bracket",
                    "]" => "right_bracket",
                    "," => "comma",
                    "@" => "at",
                    "$" => "dollar",
                    "&" => "ampersand",
                    "|" => "pipe",
                    "=" => "equal",
                    ":" => "colon",
                    _ => name,
                };

                format_ident!("{}_token", name.to_case(Case::Snake))
            }
            Field::Node { name, .. } => {
                if name == "type" {
                    format_ident!("ty")
                } else {
                    format_ident!("{}", name)
                }
            }
        }
    }

    pub(crate) fn ty(&self) -> proc_macro2::Ident {
        match self {
            Field::Token(_) => format_ident!("SyntaxToken"),
            Field::Node { ty, .. } => format_ident!("{}", ty),
        }
    }
}

const TOKENS: &[&str] = &[
    "Whitespace",
    "LineComment",
    "BlockComment",
    "StringLiteral",
    "StringLiteralFragment",
    "IntLiteral",
    "FloatLiteral",
    "Identifier",
];

pub(crate) fn lower(grammar: Grammar) -> GrammarSrc {
    let tokens = TOKENS.iter().cloned().map(ToOwned::to_owned).collect::<Vec<_>>();
    let mut grammar_src = GrammarSrc { tokens, ..Default::default() };
    let nodes = grammar.iter().collect::<Vec<_>>();

    for &node in &nodes {
        let name = grammar[node].name.clone();
        let rule = &grammar[node].rule;

        if let Some(variants) = lower_enum(&grammar, rule) {
            let enum_src = EnumSrc { name, traits: Vec::new(), variants };
            grammar_src.enums.push(enum_src);

            continue;
        };

        let mut fields = Vec::new();
        lower_rule(&mut fields, &grammar, None, rule);

        grammar_src.nodes.push(NodeSrc { name, traits: Vec::new(), fields });
    }

    deduplicate_fields(&mut grammar_src);
    extract_enums(&mut grammar_src);
    extract_struct_traits(&mut grammar_src);
    extract_enum_traits(&mut grammar_src);

    grammar_src
}

fn lower_enum(grammar: &Grammar, rule: &Rule) -> Option<Vec<String>> {
    let Rule::Alt(alternatives) = rule else {
        return None;
    };

    let mut variants = Vec::new();

    for alternative in alternatives {
        let Rule::Node(node) = alternative else {
            return None;
        };

        variants.push(grammar[*node].name.clone());
    }

    Some(variants)
}

fn lower_rule(acc: &mut Vec<Field>, grammar: &Grammar, label: Option<&String>, rule: &Rule) {
    if lower_comma_list(acc, grammar, label, rule) {
        return;
    }

    match rule {
        Rule::Node(node) => {
            let ty = grammar[*node].name.clone();
            let name = label.cloned().unwrap_or_else(|| ty.to_case(Case::Snake));
            let field = Field::Node { name, ty, cardinality: Cardinality::Optional };

            acc.push(field);
        }
        Rule::Token(token) => {
            assert!(label.is_none());

            let name = grammar[*token].name.clone();
            let field = Field::Token(name);

            acc.push(field);
        }
        Rule::Rep(inner) => {
            if let Rule::Node(node) = &**inner {
                let ty = grammar[*node].name.clone();
                let name =
                    label.cloned().unwrap_or_else(|| format!("{}s", ty.to_case(Case::Snake)));
                let field = Field::Node { name, ty, cardinality: Cardinality::Many };

                acc.push(field);
                return;
            }

            panic!("uhandled rule: {rule:?}")
        }
        Rule::Labeled { label: l, rule } => {
            assert!(label.is_none());

            // TODO: Look into this!
            // let manually_implemented = matches!(
            //     l.as_str(),
            //     "lhs"
            //         | "rhs"
            //         | "then_branch"
            //         | "else_branch"
            //         | "start"
            //         | "end"
            //         | "op"
            //         | "index"
            //         | "base"
            //         | "value"
            //         | "trait"
            //         | "self_ty"
            // );

            // if manually_implemented {
            //     return;
            // }

            lower_rule(acc, grammar, Some(l), rule);
        }
        Rule::Seq(rules) | Rule::Alt(rules) => {
            for rule in rules {
                lower_rule(acc, grammar, label, rule)
            }
        }
        Rule::Opt(rule) => lower_rule(acc, grammar, label, rule),
    }
}

fn lower_comma_list(
    acc: &mut Vec<Field>,
    grammar: &Grammar,
    label: Option<&String>,
    rule: &Rule,
) -> bool {
    let Rule::Seq(rule) = rule else {
        return false;
    };

    let [Rule::Node(node), Rule::Rep(repeat), Rule::Opt(trailing_comma)] = rule.as_slice() else {
        return false;
    };

    let Rule::Seq(ref repeat) = **repeat else {
        return false;
    };

    let [comma, Rule::Node(repeat_node)] = repeat.as_slice() else {
        return false;
    };

    if comma != &**trailing_comma || node != repeat_node {
        return false;
    }

    let ty = grammar[*node].name.clone();
    let name = label.cloned().unwrap_or_else(|| format!("{}s", ty.to_case(Case::Snake)));
    let field = Field::Node { name, ty, cardinality: Cardinality::Many };

    acc.push(field);

    true
}

fn deduplicate_fields(ast: &mut GrammarSrc) {
    for node in &mut ast.nodes {
        let mut outer = 0;
        'outer: while outer < node.fields.len() {
            let outer_field = &node.fields[outer];

            for inner in 0..outer {
                let inner_field = &node.fields[inner];

                if outer_field == inner_field {
                    node.fields.remove(outer);
                    continue 'outer;
                }
            }

            outer += 1;
        }
    }
}

fn extract_enums(ast: &mut GrammarSrc) {
    for node in &mut ast.nodes {
        for enm in &ast.enums {
            let mut to_remove = Vec::new();

            for (index, field) in node.fields.iter().enumerate() {
                let ty = field.ty().to_string();
                if enm.variants.iter().any(|variant| variant == &ty) {
                    to_remove.push(index);
                }
            }

            if to_remove.len() == enm.variants.len() {
                node.remove_field(to_remove);

                let ty = enm.name.clone();
                let name = ty.to_case(Case::Snake);

                node.fields.push(Field::Node { name, ty, cardinality: Cardinality::Optional });
            }
        }
    }
}

fn extract_struct_traits(grammar: &mut GrammarSrc) {
    let traits: &[(&str, &[&str])] = &[];

    for node in &mut grammar.nodes {
        for (name, methods) in traits {
            extract_struct_trait(node, name, methods);
        }
    }
}

fn extract_struct_trait(node: &mut NodeSrc, trait_name: &str, methods: &[&str]) {
    let mut to_remove = Vec::new();
    for (i, field) in node.fields.iter().enumerate() {
        let method_name = field.method_name().to_string();
        if methods.iter().any(|&it| it == method_name) {
            to_remove.push(i);
        }
    }
    if to_remove.len() == methods.len() {
        node.traits.push(trait_name.to_string());
        node.remove_field(to_remove);
    }
}

fn extract_enum_traits(ast: &mut GrammarSrc) {
    let enums = ast.enums.clone();
    for enm in &mut ast.enums {
        // if enm.name == "Stmt" {
        //     continue;
        // }
        let nodes = &ast.nodes;

        let variant_traits = enm.variants.iter().map(|variant| {
            nodes
                .iter()
                .find_map(|node| {
                    if &node.name != variant {
                        return None;
                    }

                    Some(node.traits.iter().cloned().collect::<BTreeSet<_>>())
                })
                .unwrap_or_else(|| {
                    enums
                        .iter()
                        .find_map(|node| {
                            if &node.name != variant {
                                return None;
                            }

                            Some(node.traits.iter().cloned().collect::<BTreeSet<_>>())
                        })
                        .unwrap_or_else(|| {
                            let name = &enm.name;

                            panic!(
                                "Could not find a struct `{variant}` for enum `{name}::{variant}`",
                            );
                        })
                })
        });

        enm.traits = variant_traits
            .reduce(|acc, traits| acc.intersection(&traits).cloned().collect())
            .unwrap_or_default()
            .into_iter()
            .collect();
    }
}
