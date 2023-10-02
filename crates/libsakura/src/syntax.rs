pub mod ast;
mod builder;
mod error;
mod kinds;

pub use builder::*;
pub use error::*;
pub use kinds::*;

use rowan::Language;

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
