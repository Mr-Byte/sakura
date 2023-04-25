#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub len: usize,
}

impl Token {
    pub fn new(kind: TokenKind, len: usize) -> Token {
        Token { kind, len }
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
    // TODO: Figure out how to handle interpolated strings.
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Base {
    Binary,
    Octal,
    Hexadecimal,
    Decimal,
}
