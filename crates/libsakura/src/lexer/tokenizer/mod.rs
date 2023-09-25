//! Tokenizer provides the underlying tokenization logic for the lexer. It returns an iterator of
//! [`Token`] that can be used by consumers to parse and analyze a Sakura source file at the lexical
//! level. Many of the tokens returned include additional context about possible lexer errors.
//! These tokens do not represent the final [`SyntaxKind`](crate::syntax::SyntaxKind) produced by
//! the conversion from tokens to syntax.

mod cursor;
mod symbol;
mod token;

pub use token::{Base, LiteralKind, Token, TokenKind};

/// Using the provided input, this function will produce an iterator of [`Token`] representing the
/// lexed tokens of the input.
pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut tokenizer = Tokenizer::new(input);

    std::iter::from_fn(move || tokenizer.next_token())
}

#[derive(Debug, PartialEq, Eq)]
enum TokenizerMode {
    Default,
    InterpolatedString,
}

struct Tokenizer<'a> {
    cursor: cursor::Cursor<'a>,
    mode_stack: Vec<TokenizerMode>,
}

use TokenKind::*;

/// Implement the core functionality of the tokenizer. This includes the core state machine and handling
/// which lexer mode to dispatch when `next_token` is called.
impl Tokenizer<'_> {
    const DEFAULT_MODE_STACK_CAPACITY: usize = 4;

    fn new(input: &str) -> Tokenizer<'_> {
        let cursor = cursor::Cursor::new(input);
        let mode_stack = Vec::with_capacity(Self::DEFAULT_MODE_STACK_CAPACITY);

        Tokenizer { cursor, mode_stack }
    }

    fn next_token(&mut self) -> Option<Token> {
        let kind = match self.mode_stack.last() {
            None | Some(TokenizerMode::Default) => self.next_token_default()?,
            Some(TokenizerMode::InterpolatedString) => self.next_token_interpolated_string(),
        };

        let token = Token { kind, len: self.cursor.consumed_len() };
        self.cursor.reset_len();

        Some(token)
    }

    /// Scan tokens that constitute the primary portion of the language as represented by the `Default` state.
    fn next_token_default(&mut self) -> Option<TokenKind> {
        let next_char = self.cursor.bump()?;
        let kind = match next_char {
            // Comments
            '/' => match self.cursor.first() {
                '/' => self.scan_line_comment(),
                '*' => self.scan_block_comment(),
                _ => Slash,
            },
            // Whitespace
            c if char::is_whitespace(c) => self.scan_whitespace(),
            // Identifiers
            c if symbol::is_identifier_start(c) => self.scan_identifier(),
            // Numbers
            c @ '0'..='9' => {
                let literal_kind = self.scan_number(c);
                let suffix_start = self.cursor.consumed_len();
                // NOTE: Lex any trailing identifiers as a suffix
                self.scan_identifier();

                let suffix_start = if self.cursor.consumed_len() > suffix_start {
                    Some(suffix_start)
                } else {
                    None
                };

                Literal { kind: literal_kind, suffix_start }
            }
            // Strings
            '"' => {
                self.mode_stack.push(TokenizerMode::InterpolatedString);
                self.scan_double_quoted_string()
            }
            '\'' => {
                let terminated = self.scan_single_quoted_string();
                let suffix_start = self.cursor.consumed_len();

                if terminated {
                    self.scan_identifier();
                }

                let kind = LiteralKind::Char { terminated };
                let suffix_start = if self.cursor.consumed_len() > suffix_start {
                    Some(suffix_start)
                } else {
                    None
                };

                Literal { kind, suffix_start }
            }
            // Symbols
            c => self.scan_symbol(c),
        };

        Some(kind)
    }

    /// Scan tokens that are part of interpolated strings as represented by the `InterpolatedString` state.
    fn next_token_interpolated_string(&mut self) -> TokenKind {
        match self.cursor.first() {
            '$' => {
                self.cursor.bump();

                Dollar
            }
            '{' => {
                self.cursor.bump();
                self.mode_stack.push(TokenizerMode::Default);

                OpenBrace
            }
            c if symbol::is_identifier_start(c) => self.scan_identifier(),
            _ => self.scan_double_quoted_string(),
        }
    }
}

/// Implement complex scanners for various tokens.
impl Tokenizer<'_> {
    fn scan_symbol(&mut self, c: char) -> TokenKind {
        match c {
            // Symbol tokens
            ',' => Comma,
            '.' => Dot,
            '(' => OpenParen,
            ')' => CloseParen,
            '{' => {
                self.mode_stack.push(TokenizerMode::Default);
                OpenBrace
            }
            '}' => {
                let mode = self.mode_stack.pop();
                debug_assert!(matches!(mode, Some(TokenizerMode::Default) | None));

                CloseBrace
            }
            '[' => OpenBracket,
            ']' => CloseBracket,
            '@' => At,
            ':' => Colon,
            '$' => Dollar,
            '#' => Hash,
            '!' => Bang,
            '?' => Question,
            '=' => Eq,
            '<' => Lt,
            '>' => Gt,
            '+' => Plus,
            '-' => Minus,
            '*' => Star,
            '&' => And,
            '|' => Or,
            '^' => Caret,
            '~' => Tilde,
            '%' => Percent,
            _ => Unknown,
        }
    }

    fn scan_identifier(&mut self) -> TokenKind {
        self.cursor.bump_while(symbol::is_identifier_continue);
        Identifier
    }

    fn scan_whitespace(&mut self) -> TokenKind {
        self.cursor.bump_while(char::is_whitespace);
        Whitespace
    }

    fn scan_line_comment(&mut self) -> TokenKind {
        self.cursor.bump_while(|c| c != '\n');
        LineComment
    }

    fn scan_block_comment(&mut self) -> TokenKind {
        self.cursor.bump();

        let mut depth: usize = 1;
        while let Some(c) = self.cursor.bump() {
            match c {
                '/' if self.cursor.first() == '*' => {
                    self.cursor.bump();
                    depth += 1;
                }
                '*' if self.cursor.first() == '/' => {
                    self.cursor.bump();
                    depth -= 1;

                    if depth == 0 {
                        break;
                    }
                }

                _ => (),
            }
        }

        BlockComment { terminated: depth == 0 }
    }

    fn scan_number(&mut self, first_digit: char) -> LiteralKind {
        let mut base = Base::Decimal;

        if first_digit == '0' {
            let has_digits = match self.cursor.first() {
                'b' => {
                    base = Base::Binary;
                    self.cursor.bump();
                    self.scan_decimal_digits()
                }
                'o' => {
                    base = Base::Octal;
                    self.cursor.bump();
                    self.scan_decimal_digits()
                }
                'x' => {
                    base = Base::Hexadecimal;
                    self.cursor.bump();
                    self.scan_hexadecimal_digits()
                }
                '0'..='9' | '_' | '.' | 'e' | 'E' => {
                    self.scan_decimal_digits();
                    true
                }
                _ => return LiteralKind::Int { base, empty: false },
            };

            if !has_digits {
                return LiteralKind::Int { base, empty: true };
            }
        } else {
            self.scan_decimal_digits();
        }

        match self.cursor.first() {
            '.' if self.cursor.second() != '.'
                && !symbol::is_identifier_start(self.cursor.second()) =>
            {
                self.cursor.bump();
                let mut empty_exponent = false;
                if self.cursor.first().is_ascii_digit() {
                    self.scan_decimal_digits();
                    match self.cursor.first() {
                        'e' | 'E' => {
                            self.cursor.bump();
                            empty_exponent = !self.scan_float_exponent();
                        }
                        _ => (),
                    }
                }

                LiteralKind::Float { base, empty_exponent }
            }
            'e' | 'E' => {
                self.cursor.bump();
                let empty_exponent = !self.scan_float_exponent();
                LiteralKind::Float { base, empty_exponent }
            }
            _ => LiteralKind::Int { base, empty: false },
        }
    }

    fn scan_decimal_digits(&mut self) -> bool {
        let mut has_digits = false;

        loop {
            match self.cursor.first() {
                '_' => {
                    self.cursor.bump();
                }
                '0'..='9' => {
                    self.cursor.bump();
                    has_digits = true;
                }
                _ => break,
            }
        }

        has_digits
    }

    fn scan_hexadecimal_digits(&mut self) -> bool {
        let mut has_digits = false;

        loop {
            match self.cursor.first() {
                '_' => {
                    self.cursor.bump();
                }
                '0'..='9' | 'a'..='f' | 'A'..='F' => {
                    self.cursor.bump();
                    has_digits = true;
                }
                _ => break,
            }
        }

        has_digits
    }

    fn scan_float_exponent(&mut self) -> bool {
        if matches!(self.cursor.first(), '-' | '+') {
            self.cursor.bump();
        }

        self.scan_decimal_digits()
    }

    fn scan_double_quoted_string(&mut self) -> TokenKind {
        let (terminated, slot_after) = self.scan_double_quoted_string_literal();
        let suffix_start = self.cursor.consumed_len();

        if terminated {
            self.scan_identifier();
        }

        let kind = if slot_after {
            LiteralKind::StringPart { terminated }
        } else {
            LiteralKind::String { terminated }
        };
        let suffix_start =
            if self.cursor.consumed_len() > suffix_start { Some(suffix_start) } else { None };

        Literal { kind, suffix_start }
    }

    fn scan_double_quoted_string_literal(&mut self) -> (bool, bool) {
        loop {
            let first = self.cursor.first();
            let second = self.cursor.second();

            if first == '$' && (second == '{' || symbol::is_identifier_start(second)) {
                return (true, true);
            }

            let next = self.cursor.bump();
            match next {
                Some(c) => match c {
                    '"' => {
                        let mode = self.mode_stack.pop();
                        debug_assert_eq!(Some(TokenizerMode::InterpolatedString), mode);

                        return (true, false);
                    }
                    '\\' if symbol::is_reserved_string_symbol(self.cursor.first()) => {
                        self.cursor.bump();
                    }
                    _ => (),
                },
                None => return (false, false),
            }
        }
    }

    fn scan_single_quoted_string(&mut self) -> bool {
        while let Some(c) = self.cursor.bump() {
            match c {
                '\'' => return true,
                '\\' if self.cursor.first() == '\\' || self.cursor.first() == '\'' => {
                    self.cursor.bump();
                }
                _ => (),
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn tokenizes_basic_double_quoted_string() {
        let input = r#""test""#;
        let tokens = tokenize(input).collect::<Vec<_>>();
        let expected =
            Literal { kind: LiteralKind::String { terminated: true }, suffix_start: None };

        assert_eq!(1, tokens.len());
        assert_eq!(expected, tokens[0].kind);
    }

    #[test]
    fn tokenizes_interpolated_string_with_identifier() {
        let input = r#""$test""#;
        let tokens = tokenize(input).collect::<Vec<_>>();
        let expected =
            Literal { kind: LiteralKind::StringPart { terminated: true }, suffix_start: None };

        assert_eq!(4, tokens.len());
        assert_eq!(expected, tokens[0].kind);
        assert_eq!(Dollar, tokens[1].kind);
        assert_eq!(Identifier, tokens[2].kind);
        assert_eq!(
            Literal { kind: LiteralKind::String { terminated: true }, suffix_start: None },
            tokens[3].kind
        );
    }

    #[test]
    fn tokenizes_interpolated_string_with_expression() {
        let input = r#""${test}""#;
        let tokens = tokenize(input).collect::<Vec<_>>();
        let open_expected =
            Literal { kind: LiteralKind::StringPart { terminated: true }, suffix_start: None };
        let close_expected =
            Literal { kind: LiteralKind::String { terminated: true }, suffix_start: None };

        assert_eq!(6, tokens.len());
        assert_eq!(open_expected, tokens[0].kind);
        assert_eq!(Dollar, tokens[1].kind);
        assert_eq!(OpenBrace, tokens[2].kind);
        assert_eq!(Identifier, tokens[3].kind);
        assert_eq!(CloseBrace, tokens[4].kind);
        assert_eq!(close_expected, tokens[5].kind);
    }

    #[test]
    fn tokenizes_identifiers() {
        let input = "test";
        let tokens = tokenize(input).collect::<Vec<_>>();

        assert_eq!(1, tokens.len());
        assert_eq!(Identifier, tokens[0].kind);
        assert_eq!("test", &input[..tokens[0].len]);
    }

    #[test]
    fn tokenizes_decimal_integer_numbers() {
        let input = "123";
        let tokens = tokenize(input).collect::<Vec<_>>();
        let expected = Literal {
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
        let expected = Literal {
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
        let expected = Literal {
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
        let expected = Literal {
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
        symbols.insert(",", Comma);
        symbols.insert(":", Colon);
        symbols.insert("(", OpenParen);
        symbols.insert(")", CloseParen);
        symbols.insert("{", OpenBrace);
        symbols.insert("}", CloseBrace);
        symbols.insert("[", OpenBracket);
        symbols.insert("]", CloseBracket);
        symbols.insert("+", Plus);
        symbols.insert("-", Minus);
        symbols.insert("*", Star);
        symbols.insert("/", Slash);
        symbols.insert("%", Percent);
        symbols.insert("<", Lt);
        symbols.insert(">", Gt);
        symbols.insert("=", Eq);
        symbols.insert("!", Bang);
        symbols.insert("?", Question);
        symbols.insert("@", At);
        symbols.insert("$", Dollar);
        symbols.insert("#", Hash);
        symbols.insert("^", Caret);
        symbols.insert("&", And);
        symbols.insert("|", Or);

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
            Identifier,
            Whitespace,
            Identifier,
            Whitespace,
            Eq,
            Whitespace,
            Literal { kind: LiteralKind::StringPart { terminated: true }, suffix_start: None },
            Dollar,
            OpenBrace,
            Identifier,
            Plus,
            Literal {
                kind: LiteralKind::Int { base: Base::Decimal, empty: false },
                suffix_start: None,
            },
            CloseBrace,
            Literal { kind: LiteralKind::StringPart { terminated: true }, suffix_start: None },
            Dollar,
            Identifier,
            Literal { kind: LiteralKind::String { terminated: true }, suffix_start: None },
        ];

        for (expected, actual) in expected_kinds.iter().zip(tokens.iter()) {
            assert_eq!(expected, &actual.kind);
        }
    }
}
