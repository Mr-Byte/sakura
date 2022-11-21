mod parsing;
mod syntax_error;
mod syntax_node;

pub mod ast;

use std::{marker::PhantomData, sync::Arc};

pub use crate::{
    ast::AstNode,
    syntax_error::SyntaxError,
    syntax_node::{
        PreorderWithTokens, SakuraLanguage, SyntaElementChildren, SyntaxElement, SyntaxNode,
        SyntaxNodeChildren, SyntaxToken, SyntaxTreeBuilder,
    },
};

pub use parser::SyntaxKind;

pub use rowan::{
    api::Preorder, Direction, GreenNode, NodeOrToken, SyntaxText, TextRange, TextSize,
    TokenAtOffset, WalkEvent,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Parse<T> {
    green: GreenNode,
    errors: Arc<Vec<SyntaxError>>,
    _type: PhantomData<fn() -> T>,
}

impl<T> Parse<T> {
    fn new(green: GreenNode, errors: Vec<SyntaxError>) -> Self {
        Self {
            green,
            errors: Arc::new(errors),
            _type: PhantomData,
        }
    }

    pub fn syntax_node(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green.clone())
    }

    pub fn errors(&self) -> &[SyntaxError] {
        self.errors.as_ref()
    }
}

impl<T: AstNode> Parse<T> {
    pub fn to_syntax(self) -> Parse<SyntaxNode> {
        Parse {
            green: self.green,
            errors: self.errors,
            _type: PhantomData,
        }
    }

    pub fn tree(&self) -> T {
        T::cast(self.syntax_node()).expect("failed to unwrap node, unexpected type encountered")
    }

    pub fn ok(self) -> Result<T, Arc<Vec<SyntaxError>>> {
        todo!()
    }
}

impl Parse<SyntaxNode> {
    pub fn cast<N: AstNode>(self) -> Option<Parse<N>> {
        if !N::can_cast(self.syntax_node().kind()) {
            return None;
        }

        Some(Parse {
            green: self.green,
            errors: self.errors,
            _type: PhantomData,
        })
    }
}

// TODO: Implement Parse<SourceFile> when reparsing is implemented!
pub use ast::SourceFile;

impl SourceFile {
    pub fn parse(text: &str) -> Parse<SourceFile> {
        let (green, errors) = parsing::parse_text(text);
        let root = SyntaxNode::new_root(green.clone());

        // TODO: Validate AST and add additional errors.

        assert_eq!(root.kind(), SyntaxKind::SourceFile);
        Parse::new(green, errors)
    }
}
