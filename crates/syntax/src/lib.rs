mod syntax_error;
mod syntax_kind;
mod syntax_node;

use std::{marker::PhantomData, sync::Arc};

// Re-export rowan types.
pub use rowan::{
    api::Preorder, Direction, GreenNode, NodeOrToken, SyntaxText, TextRange, TextSize,
    TokenAtOffset, WalkEvent,
};

pub use crate::{
    syntax_error::SyntaxError,
    syntax_kind::SyntaxKind,
    syntax_node::{
        PreorderWithTokens, SakuraLang, SyntaxElement, SyntaxElementChildren, SyntaxNode,
        SyntaxNodeChildren, SyntaxToken,
    },
};

pub struct Parse<T> {
    green_node: GreenNode,
    errors: Arc<[SyntaxError]>,
    _ty: PhantomData<fn() -> T>,
}

impl<T> Clone for Parse<T> {
    fn clone(&self) -> Parse<T> {
        Parse { green_node: self.green_node.clone(), errors: self.errors.clone(), _ty: PhantomData }
    }
}

impl<T> Parse<T> {
    pub(crate) fn new(green_node: GreenNode, errors: Vec<SyntaxError>) -> Parse<T> {
        Parse { green_node, errors: errors.into(), _ty: PhantomData }
    }

    pub fn syntax_node(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node.clone())
    }

    pub fn errors(&self) -> &[SyntaxError] {
        &self.errors
    }
}
