//! This module defines a concrete syntax tree (CST), as used by the Sakura compiler and APIs.
//!
//! The design is heavily based on [rust-analyzer](https://github.com/rust-lang/rust-analyzer/blob/36a70b7435c48837018c71576d7bb4e8f763f501/crates/syntax/src/syntax_node.rs).
//!
//! This is mostly a wrapper around the `rowan` crate.

use rowan::{GreenNodeBuilder, Language};

use crate::{Parse, SyntaxError, SyntaxKind, TextSize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SakuraLanguage {}

impl Language for SakuraLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        SyntaxKind::from(raw.0)
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.into())
    }
}

pub type SyntaxNode = rowan::SyntaxNode<SakuraLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<SakuraLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<SakuraLanguage>;
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<SakuraLanguage>;
pub type SyntaElementChildren = rowan::SyntaxElementChildren<SakuraLanguage>;
pub type PreorderWithTokens = rowan::api::PreorderWithTokens<SakuraLanguage>;

#[derive(Default)]
pub struct SyntaxTreeBuilder {
    errors: Vec<SyntaxError>,
    inner: GreenNodeBuilder<'static>,
}

impl SyntaxTreeBuilder {
    pub fn finish(self) -> Parse<SyntaxNode> {
        let green = self.inner.finish();

        Parse::new(green, self.errors)
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
