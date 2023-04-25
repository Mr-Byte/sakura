#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum SyntaxKind {
    EOF,
    Semicolon,
    Comma,
    LeftParen,
    RightParen,
    LeftCurly,
    RightCurly,
    LeftBracket,
    RightBracket,
    LeftAngle,
    RightAngle,
    At,
    Pound,
    Tilde,
    Question,
    Dollar,
    Amp,
    Pipe,
    Plus,
    Star,
    Slash,
    Caret,
    Percent,
    Underscore,
    Dot,
    DoubleDot,
    DoubleDotEq,
    Colon,
    DoubleColon,
    Eq,
    DoubleEqual,
    FatArrow,
    Bang,
    NotEq,
    Minus,
    ThinArrow,
    LtEq,
    GtEq,
    PusEq,
    MinusEq,
    PipeEq,
    AmpEq,
    CaretEq,
    SlashEq,
    StarEq,
    PercentEq,
    DoubleAmp,
    DoublePipe,
    ShiftLeft,
    ShiftRight,
    ShiftLeftEq,
    ShiftRightEq,
    WhiteSpace,
    Comment,
    SourceFile,
    #[doc(hidden)]
    __LAST,
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
