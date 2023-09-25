#[derive(Debug, Eq, PartialEq)]
pub struct Token {
    kind: TokenKind,
    len: usize,
}

impl Token {
    pub(crate) fn new(kind: TokenKind, len: usize) -> Token {
        Token { kind, len }
    }

    #[inline]
    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
    /// ,
    Comma,
    /// .
    Dot,
    /// (
    OpenParen,
    /// )
    CloseParen,
    /// {
    OpenBrace,
    /// }
    CloseBrace,
    /// [
    OpenBracket,
    /// ]
    CloseBracket,
    /// @,
    At,
    /// :
    Colon,
    /// $
    Dollar,
    /// #
    Hash,
    /// !
    Bang,
    /// ?
    Question,
    /// =
    Eq,
    /// <
    Lt,
    /// >
    Gt,
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Star,
    /// /
    Slash,
    /// &
    And,
    /// |
    Or,
    /// ^
    Caret,
    /// ~
    Tilde,
    /// %
    Percent,

    LineComment,
    BlockComment {
        terminated: bool,
    },
    Whitespace,
    Identifier,
    Literal {
        kind: LiteralKind,
        suffix_start: Option<usize>,
    },
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LiteralKind {
    Int { base: Base, empty: bool },
    Float { base: Base, empty_exponent: bool },
    Char { terminated: bool },
    String { terminated: bool },
    StringPart { terminated: bool },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Base {
    Binary,
    Octal,
    Hexadecimal,
    Decimal,
}
