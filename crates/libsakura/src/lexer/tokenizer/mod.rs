use crate::syntax::kinds::SyntaxKind;

use super::token::Token;

mod cursor;
mod symbol;

#[derive(Debug, PartialEq, Eq)]
enum TokenizerMode {
    Default,
    InterpolatedString,
}

pub(crate) struct Tokenizer<'a> {
    cursor: cursor::Cursor<'a>,
    mode_stack: Vec<TokenizerMode>,
}

/// Implement the core functionality of the tokenizer. This includes the core state machine and handling
/// which lexer mode to dispatch when `next_token` is called.
impl Tokenizer<'_> {
    const DEFAULT_MODE_STACK_CAPACITY: usize = 4;

    /// Create a new tokenizer from the given input string.
    pub(crate) fn new(input: &str) -> Tokenizer<'_> {
        let cursor = cursor::Cursor::new(input);
        let mode_stack = Vec::with_capacity(Self::DEFAULT_MODE_STACK_CAPACITY);

        Tokenizer { cursor, mode_stack }
    }

    /// Get the next available token from the input.
    /// If the end of the input has been reached, `None` is returned.
    /// If an error occurs, the `ERROR` token is returned.
    pub(crate) fn next_token(&mut self) -> Option<Token> {
        let kind = match self.mode_stack.last() {
            None | Some(TokenizerMode::Default) => self.next_token_default()?,
            Some(TokenizerMode::InterpolatedString) => self.next_token_interpolated_string(),
        };

        let token = Token { kind, len: self.cursor.consumed_len() };
        self.cursor.reset_len();

        Some(token)
    }

    /// Scan tokens that constitute the primary portion of the language as represented by the `Default` state.
    fn next_token_default(&mut self) -> Option<SyntaxKind> {
        let next_char = self.cursor.bump()?;
        let kind = match next_char {
            // Comments
            '/' => match self.cursor.first() {
                '/' => self.scan_line_comment(),
                '*' => self.scan_block_comment(),
                _ => SyntaxKind::SLASH,
            },
            // Whitespace
            c if char::is_whitespace(c) => self.scan_whitespace(),
            // Identifiers
            c if symbol::is_identifier_start(c) => self.scan_identifier(),
            // Numbers
            c @ '0'..='9' => self.scan_number(c),
            // Strings
            '"' => {
                self.mode_stack.push(TokenizerMode::InterpolatedString);
                self.scan_double_quoted_string()
            }
            '\'' => {
                let terminated = self.scan_single_quoted_string();

                if terminated {
                    self.scan_identifier();
                }

                SyntaxKind::CHAR_LITERAL
            }
            // Symbols
            c => self.scan_symbol(c),
        };

        Some(kind)
    }

    /// Scan tokens that are part of interpolated strings as represented by the `InterpolatedString` state.
    fn next_token_interpolated_string(&mut self) -> SyntaxKind {
        match self.cursor.first() {
            '$' => {
                self.cursor.bump();

                SyntaxKind::DOLLAR
            }
            '{' => {
                self.cursor.bump();
                self.mode_stack.push(TokenizerMode::Default);

                SyntaxKind::LEFT_CURLY
            }
            c if symbol::is_identifier_start(c) => self.scan_identifier(),
            _ => self.scan_double_quoted_string(),
        }
    }
}

/// Implement complex scanners for various tokens.
impl Tokenizer<'_> {
    fn scan_symbol(&mut self, c: char) -> SyntaxKind {
        match c {
            // Symbol tokens
            ',' => SyntaxKind::COMMA,
            '.' => SyntaxKind::DOT,
            '(' => SyntaxKind::LEFT_PAREN,
            ')' => SyntaxKind::RIGHT_PAREN,
            '{' => {
                self.mode_stack.push(TokenizerMode::Default);
                SyntaxKind::LEFT_CURLY
            }
            '}' => {
                let mode = self.mode_stack.pop();
                debug_assert!(matches!(mode, Some(TokenizerMode::Default) | None));

                SyntaxKind::RIGHT_CURLY
            }
            '[' => SyntaxKind::LEFT_BRACKET,
            ']' => SyntaxKind::RIGHT_BRACKET,
            '@' => SyntaxKind::AT,
            ':' => SyntaxKind::COLON,
            '$' => SyntaxKind::DOLLAR,
            '#' => SyntaxKind::HASH,
            '!' => SyntaxKind::BANG,
            '?' => SyntaxKind::QUESTION,
            '=' => SyntaxKind::EQUAL,
            '<' => SyntaxKind::LESS_THAN,
            '>' => SyntaxKind::GREATER_THAN,
            '+' => SyntaxKind::PLUS,
            '-' => SyntaxKind::MINUS,
            '*' => SyntaxKind::STAR,
            '&' => SyntaxKind::AMPERSAND,
            '|' => SyntaxKind::PIPE,
            '^' => SyntaxKind::CARET,
            '~' => SyntaxKind::TILDE,
            '%' => SyntaxKind::PERCENT,
            _ => SyntaxKind::ERROR,
        }
    }

    fn scan_identifier(&mut self) -> SyntaxKind {
        self.cursor.bump_while(symbol::is_identifier_continue);
        SyntaxKind::IDENTIFIER
    }

    fn scan_whitespace(&mut self) -> SyntaxKind {
        self.cursor.bump_while(char::is_whitespace);
        SyntaxKind::WHITESPACE
    }

    fn scan_line_comment(&mut self) -> SyntaxKind {
        self.cursor.bump_while(|c| c != '\n');
        SyntaxKind::LINE_COMMENT
    }

    fn scan_block_comment(&mut self) -> SyntaxKind {
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

        if depth == 0 {
            SyntaxKind::BLOCK_COMMENT
        } else {
            SyntaxKind::ERROR
        }
    }

    fn scan_number(&mut self, first_digit: char) -> SyntaxKind {
        if first_digit == '0' {
            let has_digits = match self.cursor.first() {
                'b' => {
                    self.cursor.bump();
                    self.scan_decimal_digits()
                }
                'o' => {
                    self.cursor.bump();
                    self.scan_decimal_digits()
                }
                'x' => {
                    self.cursor.bump();
                    self.scan_hexadecimal_digits()
                }
                '0'..='9' | '_' | '.' | 'e' | 'E' => {
                    self.scan_decimal_digits();
                    true
                }
                _ => return SyntaxKind::INT_LITERAL,
            };

            if !has_digits {
                return SyntaxKind::INT_LITERAL;
            }
        } else {
            self.scan_decimal_digits();
        }

        match self.cursor.first() {
            '.' if self.cursor.second() != '.'
                && !symbol::is_identifier_start(self.cursor.second()) =>
            {
                self.cursor.bump();
                if self.cursor.first().is_ascii_digit() {
                    self.scan_decimal_digits();
                    match self.cursor.first() {
                        'e' | 'E' => {
                            self.cursor.bump();
                            self.scan_float_exponent();
                        }
                        _ => (),
                    }
                }

                SyntaxKind::FLOAT_LITERAL
            }
            'e' | 'E' => {
                self.cursor.bump();
                self.scan_float_exponent();
                SyntaxKind::FLOAT_LITERAL
            }
            _ => SyntaxKind::INT_LITERAL,
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

    fn scan_float_exponent(&mut self) {
        if matches!(self.cursor.first(), '-' | '+') {
            self.cursor.bump();
        }

        self.scan_decimal_digits();
    }

    fn scan_double_quoted_string(&mut self) -> SyntaxKind {
        let terminated = self.scan_double_quoted_string_literal();

        if terminated {
            SyntaxKind::STRING_LITERAL
        } else {
            SyntaxKind::STRING_LITERAL_FRAGMENT
        }
    }

    fn scan_double_quoted_string_literal(&mut self) -> bool {
        loop {
            let first = self.cursor.first();
            let second = self.cursor.second();

            if first == '$' && (second == '{' || symbol::is_identifier_start(second)) {
                return false;
            }

            let next = self.cursor.bump();
            match next {
                Some(c) => match c {
                    '"' => {
                        let mode = self.mode_stack.pop();
                        debug_assert_eq!(Some(TokenizerMode::InterpolatedString), mode);

                        return true;
                    }
                    '\\' if symbol::is_reserved_string_symbol(self.cursor.first()) => {
                        self.cursor.bump();
                    }
                    _ => (),
                },
                None => return false,
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
