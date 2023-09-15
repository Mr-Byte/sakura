use token::Token;
use tokenizer::Tokenizer;

mod token;
mod tokenizer;

/// Tokenize the provided input into an iterator of tokens.
///
/// If an error occurs during tokenization an `ERROR` token will be returned.
/// The iterator will end on either the end of input or an unrecoverable error.
pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut tokenizer = Tokenizer::new(input);

    std::iter::from_fn(move || tokenizer.next_token())
}

#[cfg(test)]
mod test {
    use crate::syntax::SyntaxKind;

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn tokenizes_basic_double_quoted_string() {
        let input = r#""test""#;
        let tokens = tokenize(input).collect::<Vec<_>>();
        let expected = SyntaxKind::STRING_LITERAL;

        assert_eq!(1, tokens.len());
        assert_eq!(expected, tokens[0].kind);
    }

    #[test]
    fn tokenizes_interpolated_string_with_identifier() {
        let input = r#""$test""#;
        let tokens = tokenize(input).collect::<Vec<_>>();
        let expected = SyntaxKind::STRING_LITERAL_FRAGMENT;

        assert_eq!(4, tokens.len());
        assert_eq!(expected, tokens[0].kind);
        assert_eq!(SyntaxKind::DOLLAR, tokens[1].kind);
        assert_eq!(SyntaxKind::IDENTIFIER, tokens[2].kind);
        assert_eq!(SyntaxKind::STRING_LITERAL, tokens[3].kind);
    }

    #[test]
    fn tokenizes_interpolated_string_with_expression() {
        let input = r#""${test}""#;
        let tokens = tokenize(input).collect::<Vec<_>>();
        let open_expected = SyntaxKind::STRING_LITERAL_FRAGMENT;
        let close_expected = SyntaxKind::STRING_LITERAL;

        assert_eq!(6, tokens.len());
        assert_eq!(open_expected, tokens[0].kind);
        assert_eq!(SyntaxKind::DOLLAR, tokens[1].kind);
        assert_eq!(SyntaxKind::LEFT_CURLY, tokens[2].kind);
        assert_eq!(SyntaxKind::IDENTIFIER, tokens[3].kind);
        assert_eq!(SyntaxKind::RIGHT_CURLY, tokens[4].kind);
        assert_eq!(close_expected, tokens[5].kind);
    }

    #[test]
    fn tokenizes_identifiers() {
        let input = "test";
        let tokens = tokenize(input).collect::<Vec<_>>();

        assert_eq!(1, tokens.len());
        assert_eq!(SyntaxKind::IDENTIFIER, tokens[0].kind);
        assert_eq!("test", &input[..tokens[0].len]);
    }

    #[test]
    fn tokenizes_decimal_integer_numbers() {
        let input = "123";
        let tokens = tokenize(input).collect::<Vec<_>>();
        let expected = SyntaxKind::INT_LITERAL;

        assert_eq!(1, tokens.len());
        assert_eq!(expected, tokens[0].kind);
        assert_eq!("123", &input[..tokens[0].len]);
    }

    #[test]
    fn tokenizes_hexadecimal_integer_numbers() {
        let input = "0x0123456789ABCDEFabcdef";
        let tokens = tokenize(input).collect::<Vec<_>>();
        let expected = SyntaxKind::INT_LITERAL;

        assert_eq!(1, tokens.len());
        assert_eq!(expected, tokens[0].kind);
        assert_eq!("0x0123456789ABCDEFabcdef", &input[..tokens[0].len]);
    }

    #[test]
    fn tokenizes_octal_integer_numbers() {
        let input = "0o01234567";
        let tokens = tokenize(input).collect::<Vec<_>>();
        let expected = SyntaxKind::INT_LITERAL;

        assert_eq!(1, tokens.len());
        assert_eq!(expected, tokens[0].kind);
        assert_eq!("0o01234567", &input[..tokens[0].len]);
    }

    #[test]
    fn tokenizes_binary_integer_numbers() {
        let input = "0b01010101010101010101010101010101";
        let tokens = tokenize(input).collect::<Vec<_>>();
        let expected = SyntaxKind::INT_LITERAL;

        assert_eq!(1, tokens.len());
        assert_eq!(expected, tokens[0].kind);
        assert_eq!("0b01010101010101010101010101010101", &input[..tokens[0].len]);
    }

    #[test]
    fn tokenizes_symbols() {
        let mut symbols = HashMap::new();
        symbols.insert(",", SyntaxKind::COMMA);
        symbols.insert(":", SyntaxKind::COLON);
        symbols.insert("(", SyntaxKind::LEFT_PAREN);
        symbols.insert(")", SyntaxKind::RIGHT_PAREN);
        symbols.insert("{", SyntaxKind::LEFT_CURLY);
        symbols.insert("}", SyntaxKind::RIGHT_CURLY);
        symbols.insert("[", SyntaxKind::LEFT_BRACKET);
        symbols.insert("]", SyntaxKind::RIGHT_BRACKET);
        symbols.insert("+", SyntaxKind::PLUS);
        symbols.insert("-", SyntaxKind::MINUS);
        symbols.insert("*", SyntaxKind::STAR);
        symbols.insert("/", SyntaxKind::SLASH);
        symbols.insert("%", SyntaxKind::PERCENT);
        symbols.insert("<", SyntaxKind::LESS_THAN);
        symbols.insert(">", SyntaxKind::GREATER_THAN);
        symbols.insert("=", SyntaxKind::EQUAL);
        symbols.insert("!", SyntaxKind::BANG);
        symbols.insert("?", SyntaxKind::QUESTION);
        symbols.insert("@", SyntaxKind::AT);
        symbols.insert("$", SyntaxKind::DOLLAR);
        symbols.insert("#", SyntaxKind::HASH);
        symbols.insert("^", SyntaxKind::CARET);
        symbols.insert("&", SyntaxKind::AMPERSAND);
        symbols.insert("|", SyntaxKind::PIPE);

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
            SyntaxKind::IDENTIFIER,
            SyntaxKind::WHITESPACE,
            SyntaxKind::IDENTIFIER,
            SyntaxKind::WHITESPACE,
            SyntaxKind::EQUAL,
            SyntaxKind::WHITESPACE,
            SyntaxKind::STRING_LITERAL_FRAGMENT,
            SyntaxKind::DOLLAR,
            SyntaxKind::LEFT_CURLY,
            SyntaxKind::IDENTIFIER,
            SyntaxKind::PLUS,
            SyntaxKind::INT_LITERAL,
            SyntaxKind::RIGHT_CURLY,
            SyntaxKind::STRING_LITERAL_FRAGMENT,
            SyntaxKind::DOLLAR,
            SyntaxKind::IDENTIFIER,
            SyntaxKind::STRING_LITERAL,
        ];

        for (expected, actual) in expected_kinds.iter().zip(tokens.iter()) {
            assert_eq!(expected, &actual.kind);
        }
    }
}
