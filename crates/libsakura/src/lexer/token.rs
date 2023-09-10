use crate::syntax::kinds::SyntaxKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token {
    pub kind: SyntaxKind,
    pub len: usize,
}

impl Token {
    pub fn new(kind: SyntaxKind, len: usize) -> Token {
        Token { kind, len }
    }
}
