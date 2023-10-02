mod generated_kinds;
mod token_set;

pub use generated_kinds::*;
pub(crate) use token_set::*;

impl From<u16> for SyntaxKind {
    #[inline]
    fn from(kind: u16) -> SyntaxKind {
        assert!(kind <= (SyntaxKind::__LAST as u16));
        unsafe { std::mem::transmute::<u16, SyntaxKind>(kind) }
    }
}

impl From<SyntaxKind> for u16 {
    #[inline]
    fn from(kind: SyntaxKind) -> u16 {
        kind as u16
    }
}

impl SyntaxKind {
    #[inline]
    pub fn is_trivia(self) -> bool {
        matches!(
            self,
            SyntaxKind::WHITESPACE | SyntaxKind::LINE_COMMENT | SyntaxKind::BLOCK_COMMENT
        )
    }
}
