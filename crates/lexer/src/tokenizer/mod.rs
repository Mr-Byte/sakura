use crate::token::Base;
use crate::token::LiteralKind;
use crate::token::Token;
use crate::token::TokenKind;
use crate::token::TokenKind::*;

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

    pub(crate) fn new(input: &str) -> Tokenizer<'_> {
        let cursor = cursor::Cursor::new(input);
        let mut mode_stack = Vec::with_capacity(Self::DEFAULT_MODE_STACK_CAPACITY);
        mode_stack.push(TokenizerMode::Default);

        Tokenizer { cursor, mode_stack }
    }

    pub(crate) fn next_token(&mut self) -> Option<Token> {
        let kind = match self.mode_stack.last() {
            Some(TokenizerMode::Default) => self.next_token_default()?,
            Some(TokenizerMode::InterpolatedString) => self.next_token_interpolated_string(),
            None => {
                unreachable!("ICE: Underflow of the tokenizer mode stack occurred. This is a bug.")
            }
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
                // NOTE: Lex any trailing idenfitiers as a suffix
                self.scan_identifier();

                let suffix_start = if self.cursor.consumed_len() > suffix_start {
                    Some(suffix_start)
                } else {
                    None
                };

                TokenKind::Literal { kind: literal_kind, suffix_start }
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

                return TokenKind::Dollar;
            }
            '{' => {
                self.cursor.bump();
                self.mode_stack.push(TokenizerMode::Default);

                return TokenKind::OpenBrace;
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
                debug_assert_eq!(Some(TokenizerMode::Default), mode);

                CloseBrace
            }
            '[' => OpenBracket,
            ']' => CloseBracket,
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
        let terminated = self.scan_double_quoted_string_literal();
        let suffix_start = self.cursor.consumed_len();

        if terminated {
            self.scan_identifier();
        }

        let kind = LiteralKind::String { terminated };
        let suffix_start =
            if self.cursor.consumed_len() > suffix_start { Some(suffix_start) } else { None };

        Literal { kind, suffix_start }
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
