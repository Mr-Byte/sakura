use token::Token;
use tokenizer::Tokenizer;

mod token;
mod tokenizer;

/// Tokenize the provided input into an iterator of tokens.
pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut tokenizer = Tokenizer::new(input);

    std::iter::from_fn(move || tokenizer.next_token())
}

#[cfg(test)]
mod test {
    use self::token::{Base, LiteralKind, TokenKind};
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn tokenizes_basic_double_quoted_string() {
        let input = r#""test""#;
        let tokens = tokenize(input).collect::<Vec<_>>();
        let expected = TokenKind::Literal {
            kind: LiteralKind::String { terminated: true, slot_after: false },
            suffix_start: None,
        };

        assert_eq!(1, tokens.len());
        assert_eq!(expected, tokens[0].kind);
    }

    #[test]
    fn tokenizes_interpolated_string_with_identifier() {
        let input = r#""$test""#;
        let tokens = tokenize(input).collect::<Vec<_>>();
        let expected = TokenKind::Literal {
            kind: LiteralKind::String { terminated: true, slot_after: true },
            suffix_start: None,
        };

        assert_eq!(4, tokens.len());
        assert_eq!(expected, tokens[0].kind);
        assert_eq!(TokenKind::Dollar, tokens[1].kind);
        assert_eq!(TokenKind::Identifier, tokens[2].kind);
        assert_eq!(
            TokenKind::Literal {
                kind: LiteralKind::String { terminated: true, slot_after: false },
                suffix_start: None
            },
            tokens[3].kind
        );
    }

    #[test]
    fn tokenizes_interpolated_string_with_expression() {
        let input = r#""${test}""#;
        let tokens = tokenize(input).collect::<Vec<_>>();
        let open_expected = TokenKind::Literal {
            kind: LiteralKind::String { terminated: true, slot_after: true },
            suffix_start: None,
        };
        let close_expected = TokenKind::Literal {
            kind: LiteralKind::String { terminated: true, slot_after: false },
            suffix_start: None,
        };

        assert_eq!(6, tokens.len());
        assert_eq!(open_expected, tokens[0].kind);
        assert_eq!(TokenKind::Dollar, tokens[1].kind);
        assert_eq!(TokenKind::OpenBrace, tokens[2].kind);
        assert_eq!(TokenKind::Identifier, tokens[3].kind);
        assert_eq!(TokenKind::CloseBrace, tokens[4].kind);
        assert_eq!(close_expected, tokens[5].kind);
    }

    #[test]
    fn tokenizes_identifiers() {
        let input = "test";
        let tokens = tokenize(input).collect::<Vec<_>>();

        assert_eq!(1, tokens.len());
        assert_eq!(TokenKind::Identifier, tokens[0].kind);
        assert_eq!("test", &input[..tokens[0].len]);
    }

    #[test]
    fn tokenizes_decimal_integer_numbers() {
        let input = "123";
        let tokens = tokenize(input).collect::<Vec<_>>();
        let expected = TokenKind::Literal {
            kind: LiteralKind::Int { base: Base::Decimal, empty: false },
            suffix_start: None,
        };

        assert_eq!(1, tokens.len());
        assert_eq!(expected, tokens[0].kind);
        assert_eq!("123", &input[..tokens[0].len]);
    }

    #[test]
    fn tokenizes_hexadecimal_integer_numbers() {
        let input = "0x0123456789ABCDEFabcdef";
        let tokens = tokenize(input).collect::<Vec<_>>();
        let expected = TokenKind::Literal {
            kind: LiteralKind::Int { base: Base::Hexadecimal, empty: false },
            suffix_start: None,
        };

        assert_eq!(1, tokens.len());
        assert_eq!(expected, tokens[0].kind);
        assert_eq!("0x0123456789ABCDEFabcdef", &input[..tokens[0].len]);
    }

    #[test]
    fn tokenizes_octal_integer_numbers() {
        let input = "0o01234567";
        let tokens = tokenize(input).collect::<Vec<_>>();
        let expected = TokenKind::Literal {
            kind: LiteralKind::Int { base: Base::Octal, empty: false },
            suffix_start: None,
        };

        assert_eq!(1, tokens.len());
        assert_eq!(expected, tokens[0].kind);
        assert_eq!("0o01234567", &input[..tokens[0].len]);
    }

    #[test]
    fn tokenizes_binary_integer_numbers() {
        let input = "0b01010101010101010101010101010101";
        let tokens = tokenize(input).collect::<Vec<_>>();
        let expected = TokenKind::Literal {
            kind: LiteralKind::Int { base: Base::Binary, empty: false },
            suffix_start: None,
        };

        assert_eq!(1, tokens.len());
        assert_eq!(expected, tokens[0].kind);
        assert_eq!("0b01010101010101010101010101010101", &input[..tokens[0].len]);
    }

    #[test]
    fn tokenizes_symbols() {
        let mut symbols = HashMap::new();
        symbols.insert(",", TokenKind::Comma);
        symbols.insert(":", TokenKind::Colon);
        symbols.insert("(", TokenKind::OpenParen);
        symbols.insert(")", TokenKind::CloseParen);
        symbols.insert("{", TokenKind::OpenBrace);
        symbols.insert("}", TokenKind::CloseBrace);
        symbols.insert("[", TokenKind::OpenBracket);
        symbols.insert("]", TokenKind::CloseBracket);
        symbols.insert("+", TokenKind::Plus);
        symbols.insert("-", TokenKind::Minus);
        symbols.insert("*", TokenKind::Star);
        symbols.insert("/", TokenKind::Slash);
        symbols.insert("%", TokenKind::Percent);
        symbols.insert("<", TokenKind::Lt);
        symbols.insert(">", TokenKind::Gt);
        symbols.insert("=", TokenKind::Eq);
        symbols.insert("!", TokenKind::Bang);
        symbols.insert("?", TokenKind::Question);
        symbols.insert("@", TokenKind::At);
        symbols.insert("$", TokenKind::Dollar);
        symbols.insert("#", TokenKind::Hash);
        symbols.insert("^", TokenKind::Caret);
        symbols.insert("&", TokenKind::And);
        symbols.insert("|", TokenKind::Or);

        for (symbol, kind) in symbols {
            let tokens = tokenize(symbol).collect::<Vec<_>>();
            assert_eq!(1, tokens.len());
            assert_eq!(kind, tokens[0].kind);
            assert_eq!(symbol.len(), tokens[0].len);
        }
    }

    #[test]
    fn tokenizes_complex_string() {
        let input = r#"let x = "${y+2}, $z""#;
        let tokens = tokenize(input).collect::<Vec<_>>();
        let expected_kinds = vec![
            TokenKind::Identifier,
            TokenKind::Whitespace,
            TokenKind::Identifier,
            TokenKind::Whitespace,
            TokenKind::Eq,
            TokenKind::Whitespace,
            TokenKind::Literal {
                kind: LiteralKind::String { terminated: true, slot_after: true },
                suffix_start: None,
            },
            TokenKind::Dollar,
            TokenKind::OpenBrace,
            TokenKind::Identifier,
            TokenKind::Plus,
            TokenKind::Literal {
                kind: LiteralKind::Int { base: Base::Decimal, empty: false },
                suffix_start: None,
            },
            TokenKind::CloseBrace,
            TokenKind::Literal {
                kind: LiteralKind::String { terminated: true, slot_after: true },
                suffix_start: None,
            },
            TokenKind::Dollar,
            TokenKind::Identifier,
            TokenKind::Literal {
                kind: LiteralKind::String { terminated: true, slot_after: false },
                suffix_start: None,
            },
        ];

        for (expected, actual) in expected_kinds.iter().zip(tokens.iter()) {
            assert_eq!(expected, &actual.kind);
        }
    }
}
