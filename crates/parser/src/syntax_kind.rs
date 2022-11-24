mod generated;

pub use generated::*;

impl SyntaxKind {
    // TODO: Implement utility functions for checking syntax kinds.
    #[inline]
    pub fn is_trivia(self) -> bool {
        matches!(self, SyntaxKind::WhiteSpace | SyntaxKind::Comment)
    }
}

impl From<u16> for SyntaxKind {
    #[inline]
    fn from(value: u16) -> Self {
        assert!(value < SyntaxKind::__LAST as u16);
        unsafe { std::mem::transmute(value) }
    }
}

impl From<SyntaxKind> for u16 {
    #[inline]
    fn from(value: SyntaxKind) -> Self {
        value as u16
    }
}
