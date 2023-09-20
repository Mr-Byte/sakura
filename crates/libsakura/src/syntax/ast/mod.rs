mod generated_nodes;
mod generated_tokens;
mod literal_ext;

use super::{kinds::SyntaxKind, SyntaxNode, SyntaxNodeChildren, SyntaxToken};
use std::marker::PhantomData;

pub use generated_nodes::*;
pub use generated_tokens::*;
pub use literal_ext::*;

pub trait AstNode {
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized;

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized;

    fn syntax(&self) -> &SyntaxNode;

    fn source_string(&self) -> String {
        self.syntax().to_string()
    }

    fn clone_for_update(&self) -> Self
    where
        Self: Sized,
    {
        Self::cast(self.syntax().clone_for_update()).unwrap()
    }

    fn clone_subtree(&self) -> Self
    where
        Self: Sized,
    {
        Self::cast(self.syntax().clone_subtree()).unwrap()
    }
}

/// Like `AstNode`, but wraps tokens rather than interior nodes.
pub trait AstToken {
    fn can_cast(token: SyntaxKind) -> bool
    where
        Self: Sized;

    fn cast(syntax: SyntaxToken) -> Option<Self>
    where
        Self: Sized;

    fn syntax(&self) -> &SyntaxToken;

    fn text(&self) -> &str {
        self.syntax().text()
    }
}

/// An iterator over `SyntaxNode` children of a particular AST type.
#[derive(Debug, Clone)]
pub struct AstChildren<N> {
    inner: SyntaxNodeChildren,
    ph: PhantomData<N>,
}

impl<N> AstChildren<N> {
    fn new(parent: &SyntaxNode) -> Self {
        AstChildren { inner: parent.children(), ph: PhantomData }
    }
}

impl<N: AstNode> Iterator for AstChildren<N> {
    type Item = N;
    fn next(&mut self) -> Option<N> {
        self.inner.find_map(N::cast)
    }
}

mod support {
    use super::{AstChildren, AstNode, SyntaxKind, SyntaxNode, SyntaxToken};

    pub(super) fn child<N: AstNode>(parent: &SyntaxNode) -> Option<N> {
        parent.children().find_map(N::cast)
    }

    pub(super) fn children<N: AstNode>(parent: &SyntaxNode) -> AstChildren<N> {
        AstChildren::new(parent)
    }

    pub(super) fn token(parent: &SyntaxNode, kind: SyntaxKind) -> Option<SyntaxToken> {
        parent.children_with_tokens().filter_map(|it| it.into_token()).find(|it| it.kind() == kind)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        syntax::{
            ast::{Expr, InterpolatedStringSlot},
            SyntaxKind, SyntaxTreeBuilder,
        },
        T,
    };

    #[test]
    fn test_interpolated_string_slot() {
        let mut builder = SyntaxTreeBuilder::default();

        builder.start_node(SyntaxKind::INTERPOLATED_STRING_SLOT);
        builder.token(T!["$"], "$");
        builder.token(T!["{"], "{");
        builder.token(SyntaxKind::WHITESPACE, " ");
        builder.start_node(SyntaxKind::NAME);
        builder.token(T!["identifier"], "test");
        builder.finish_node();
        builder.token(SyntaxKind::WHITESPACE, " ");
        builder.token(T!["}"], "}");
        builder.finish_node();

        let parse = builder.finish();
        let slot = parse.cast::<InterpolatedStringSlot>().unwrap().tree();
        let value = slot.syntax.text().to_string();
        assert_eq!("${ test }", value);

        let expr = slot.expr().expect("expected an expression");
        let Expr::Name(ident) = expr else {
            panic!("expected an identifier");
        };
        let token = ident.identifier_token().expect("expected an identifier token");
        assert_eq!("test", token.text());
    }
}
