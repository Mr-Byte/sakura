use crate::lexer::LexedStr;
use crate::parser;
use crate::parser::tree_builder::build_tree;
use crate::syntax::{ast::AstNode, GreenNode, SyntaxNode};
use std::{marker::PhantomData, sync::Arc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parse<T> {
    green: GreenNode,
    errors: Arc<[SyntaxError]>,
    _ty: PhantomData<fn() -> T>,
}

impl<T> Parse<T> {
    pub(crate) fn new(green: GreenNode, errors: impl IntoIterator<Item = SyntaxError>) -> Parse<T> {
        Parse { green, errors: errors.into_iter().collect(), _ty: PhantomData }
    }

    pub fn syntax_node(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green.clone())
    }

    pub fn errors(&self) -> &[SyntaxError] {
        &self.errors
    }
}

impl<T: AstNode> Parse<T> {
    pub fn to_syntax(self) -> Parse<SyntaxNode> {
        Parse { green: self.green, errors: self.errors, _ty: PhantomData }
    }

    pub fn tree(&self) -> T {
        T::cast(self.syntax_node()).unwrap()
    }

    pub fn ok(self) -> Result<T, Arc<[SyntaxError]>> {
        if self.errors.is_empty() {
            Ok(self.tree())
        } else {
            Err(self.errors)
        }
    }
}

impl Parse<SyntaxNode> {
    pub fn cast<N: AstNode>(self) -> Option<Parse<N>> {
        N::cast(self.syntax_node()).map(|_| Parse {
            green: self.green,
            errors: self.errors,
            _ty: PhantomData,
        })
    }
}

pub use crate::syntax::ast::SourceFile;
use crate::syntax::SyntaxError;

impl SourceFile {
    pub fn parse(text: &str) -> Parse<SourceFile> {
        let (node, errors) = parse_text(text);

        Parse::new(node, errors)
    }
}

pub(crate) fn parse_text(text: &str) -> (GreenNode, Vec<SyntaxError>) {
    let lexed = LexedStr::new(text);
    let parser_input = lexed.as_input();
    let parser_output = parser::EntryPoint::SourceFile.parse(&parser_input);

    build_tree(lexed, parser_output)
}
