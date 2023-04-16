use token::Token;
use tokenizer::Tokenizer;

mod cursor;
mod token;
mod tokenizer;

pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut tokenizer = Tokenizer::new(input);

    std::iter::from_fn(move || tokenizer.next_token())
}

#[cfg(test)]
mod test {
    use crate::token::{LiteralKind, TokenKind};

    use super::*;

    #[test]
    fn tokenizes_interpolated_string() {
        let input = r#""${test}""#;
        let tokens = tokenize(input).collect::<Vec<_>>();

        assert_eq!(6, tokens.len());
        assert_eq!(
            TokenKind::Literal {
                kind: LiteralKind::String { terminated: false },
                suffix_start: None
            },
            tokens[0].kind
        );
        assert_eq!(TokenKind::Dollar, tokens[1].kind);
        assert_eq!(TokenKind::OpenBrace, tokens[2].kind);
        assert_eq!(TokenKind::Identifier, tokens[3].kind);
        assert_eq!(TokenKind::CloseBrace, tokens[4].kind);
        assert_eq!(
            TokenKind::Literal {
                kind: LiteralKind::String { terminated: true },
                suffix_start: None
            },
            tokens[5].kind
        );
    }
}
