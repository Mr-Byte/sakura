#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
#[non_exhaustive]
pub enum SyntaxKind {
    EOF,
    WhiteSpace,
    Comment,

    SourceFile,
}

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
        unsafe { std::mem::transmute(value) }
    }
}

impl From<SyntaxKind> for u16 {
    #[inline]
    fn from(value: SyntaxKind) -> Self {
        value as u16
    }
}
