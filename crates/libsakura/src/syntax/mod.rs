pub mod ast;
mod kinds;

use crate::parse::Parse;
pub use kinds::*;
use rowan::{GreenNode, GreenNodeBuilder, Language, TextSize};

/// A node in the immutable tree. It has other nodes and tokens as children.
pub type SyntaxNode = rowan::SyntaxNode<SakuraLanguage>;

/// A leaf node in the AST.
pub type SyntaxToken = rowan::SyntaxToken<SakuraLanguage>;

/// A `SyntaxNode` or a `SyntaxToken`.
pub type SyntaxElement = rowan::SyntaxElement<SakuraLanguage>;

/// Children of a `SyntaxNode`.
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<SakuraLanguage>;

/// A wrapper around `SyntaxNodePtr`.
pub type SyntaxNodePtr = rowan::ast::SyntaxNodePtr<SakuraLanguage>;

/// A language implementation for use in `Rowan`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SakuraLanguage {}

impl Language for SakuraLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= (SyntaxKind::__LAST as u16));
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.into())
    }
}

#[derive(Default)]
pub struct SyntaxTreeBuilder {
    errors: Vec<SyntaxError>,
    inner: GreenNodeBuilder<'static>,
}

impl SyntaxTreeBuilder {
    pub(crate) fn finish_raw(self) -> (GreenNode, Vec<SyntaxError>) {
        let green = self.inner.finish();
        let errors = self.errors;

        (green, errors)
    }

    pub fn finish(self) -> Parse<SyntaxNode> {
        let (green, errors) = self.finish_raw();

        Parse::new(green, errors)
    }

    pub fn token(&mut self, kind: SyntaxKind, text: &str) {
        let kind = SakuraLanguage::kind_to_raw(kind);
        self.inner.token(kind, text);
    }

    pub fn start_node(&mut self, kind: SyntaxKind) {
        let kind = SakuraLanguage::kind_to_raw(kind);
        self.inner.start_node(kind);
    }

    pub fn finish_node(&mut self) {
        self.inner.finish_node();
    }

    pub fn error(&mut self, _error: String, _text_pos: TextSize) {
        self.errors.push(SyntaxError);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxError;
