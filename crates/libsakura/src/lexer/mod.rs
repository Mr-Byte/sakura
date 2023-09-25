use crate::syntax::SyntaxKind;
use converter::TokenConverter;
use std::ops::Range;

mod converter;
pub mod tokenizer;

/// A borrowed string that has been lexed into tokens.
#[derive(Debug)]
pub struct LexedStr<'a> {
    text: &'a str,
    kinds: Vec<SyntaxKind>,
    tokens: Vec<Range<usize>>,
    errors: Vec<LexerError>,
}

impl<'a> LexedStr<'a> {
    pub fn new(text: &str) -> LexedStr {
        TokenConverter::convert(text)
    }

    pub fn as_str(&self) -> &'a str {
        self.text
    }

    pub fn len(&self) -> usize {
        self.kinds.len() - 1
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn kind(&self, index: usize) -> SyntaxKind {
        assert!(index < self.len());

        self.kinds[index]
    }

    pub fn text(&self, index: usize) -> &'a str {
        assert!(index < self.len());

        &self.text[self.tokens[index].clone()]
    }

    pub fn error(&self, index: usize) -> Option<&str> {
        assert!(index < self.len());

        let err = self.errors.binary_search_by_key(&index, |err| err.index).ok()?;

        Some(&self.errors[err].message)
    }

    pub fn errors(&self) -> impl Iterator<Item = (usize, &str)> + '_ {
        self.errors.iter().map(|err| (err.index, err.message.as_str()))
    }
}

#[derive(Debug)]
pub struct LexerError {
    pub message: String,
    pub index: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::T;

    #[test]
    fn test_lexer_basic() {
        let input = "fn main() { 5.0e10 }";
        let expected = vec![
            SyntaxKind::IDENTIFIER,
            SyntaxKind::WHITESPACE,
            SyntaxKind::IDENTIFIER,
            T!["("],
            T![")"],
            SyntaxKind::WHITESPACE,
            T!["{"],
            SyntaxKind::WHITESPACE,
            SyntaxKind::FLOAT_LITERAL,
            SyntaxKind::WHITESPACE,
            T!["}"],
            SyntaxKind::EOF,
        ];

        let lexed = LexedStr::new(input);

        assert_eq!(expected, lexed.kinds);
    }

    #[test]
    fn test_lexer_compound_literal() {
        let input = "0o8i32";
        let expected = vec![
            SyntaxKind::INT_LITERAL_PREFIX,
            SyntaxKind::INT_LITERAL,
            SyntaxKind::IDENTIFIER,
            SyntaxKind::EOF,
        ];

        let lexed = LexedStr::new(input);

        assert_eq!(expected, lexed.kinds);
    }

    #[test]
    fn test_lexer_unterminated_string() {
        let input = r#""test"#;
        let expected_kinds = vec![SyntaxKind::STRING_LITERAL, SyntaxKind::EOF];
        let expected_errors =
            vec![(0, r#"Missing trailing " required to terminate a string literal."#)];

        let lexed = LexedStr::new(input);

        assert_eq!(expected_kinds, lexed.kinds);
        assert_eq!(expected_errors, lexed.errors().collect::<Vec<_>>());
    }
}
