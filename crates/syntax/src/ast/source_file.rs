use crate::{AstNode, SyntaxKind, SyntaxNode};

/// Represents the top level node of a parsed source file.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceFile {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for SourceFile {
    fn can_cast(_kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        todo!()
    }

    fn cast(_syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn syntax(&self) -> &SyntaxNode {
        todo!()
    }
}
