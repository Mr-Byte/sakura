use crate::cursor::Cursor;
use crate::token::Base;
use crate::token::LiteralKind;
use crate::token::Token;
use crate::token::TokenKind;
use crate::token::TokenKind::*;

// TODO: Handle raw strings (with multiple lines?)

#[derive(Debug, PartialEq, Eq)]
enum TokenizerMode {
    Main,
    DoubleQuoteStringPart,
}

pub(crate) struct Tokenizer<'a> {
    cursor: Cursor<'a>,
    mode_stack: Vec<TokenizerMode>,
}

impl Tokenizer<'_> {
    pub(crate) fn new(input: &str) -> Tokenizer<'_> {
        let cursor = Cursor::new(input);
        let mut mode_stack = Vec::with_capacity(4);
        mode_stack.push(TokenizerMode::Main);

        Tokenizer { cursor, mode_stack }
    }

    pub(crate) fn next_token(&mut self) -> Option<Token> {
        let kind = match self.mode_stack.last() {
            Some(TokenizerMode::Main) => self.scan_main()?,
            Some(TokenizerMode::DoubleQuoteStringPart) => self.scan_double_quoted_string_part(),
            None => unreachable!("ICE: Underran tokenizer mode stack, this is a bug!"),
        };

        let token = Token { kind, len: self.cursor.consumed_len() };
        self.cursor.reset_len();

        Some(token)
    }

    fn scan_main(&mut self) -> Option<TokenKind> {
        let next_char = self.cursor.bump()?;
        let kind = match next_char {
            '/' => match self.cursor.first() {
                '/' => self.scan_line_comment(),
                '*' => self.scan_block_comment(),
                _ => Slash,
            },

            c if char::is_whitespace(c) => self.scan_whitespace(),
            c if is_identifier_start(c) => self.scan_identifier(),
            c @ '0'..='9' => {
                let literal_kind = self.scan_number(c);
                let suffix_start = self.cursor.consumed_len();
                // NOTE: Lex any trailing idenfitiers as a suffix
                self.scan_identifier();

                TokenKind::Literal { kind: literal_kind, suffix_start }
            }

            // Symbol tokens
            ',' => Comma,
            '.' => Dot,
            '(' => OpenParen,
            ')' => CloseParen,
            '{' => {
                self.mode_stack.push(TokenizerMode::Main);
                OpenBrace
            }
            '}' => {
                let mode = self.mode_stack.pop();
                debug_assert_eq!(Some(TokenizerMode::Main), mode);

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

            // Strings
            '"' => {
                self.mode_stack.push(TokenizerMode::DoubleQuoteStringPart);
                self.scan_double_quoted_string_literal()
            }
            '\'' => {
                let terminated = self.scan_single_quoted_string();
                let suffix_start = self.cursor.consumed_len();

                if terminated {
                    self.scan_identifier();
                }

                let kind = LiteralKind::Char { terminated };

                Literal { kind, suffix_start }
            }
            _ => Unknown,
        };

        Some(kind)
    }

    fn scan_double_quoted_string_part(&mut self) -> TokenKind {
        match self.cursor.first() {
            '$' => {
                self.cursor.bump();

                return TokenKind::Dollar;
            }
            '{' => {
                self.cursor.bump();
                self.mode_stack.push(TokenizerMode::Main);

                return TokenKind::OpenBrace;
            }
            _ => self.scan_double_quoted_string_literal(),
        }
    }

    fn scan_double_quoted_string_literal(&mut self) -> TokenKind {
        let terminated = self.scan_double_quoted_string();
        let suffix_start = self.cursor.consumed_len();

        if terminated {
            self.scan_identifier();
        }

        let kind = LiteralKind::String { terminated };

        Literal { kind, suffix_start }
    }

    fn scan_identifier(&mut self) -> TokenKind {
        self.cursor.bump_while(is_identifier_continue);
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
            '.' if self.cursor.second() != '.' && !is_identifier_start(self.cursor.second()) => {
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

    fn scan_double_quoted_string(&mut self) -> bool {
        loop {
            if self.cursor.first() == '$' {
                return false;
            }

            let next = self.cursor.bump();
            match next {
                Some(c) => match c {
                    '"' => {
                        let mode = self.mode_stack.pop();
                        debug_assert_eq!(Some(TokenizerMode::DoubleQuoteStringPart), mode);

                        return true;
                    }
                    '\\' if is_reserved_string_symbol(self.cursor.first()) => {
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

fn is_identifier_start(c: char) -> bool {
    c == '_' || unicode_xid::UnicodeXID::is_xid_start(c)
}

fn is_identifier_continue(c: char) -> bool {
    unicode_xid::UnicodeXID::is_xid_continue(c)
}

fn is_reserved_string_symbol(c: char) -> bool {
    c == '\\' || c == '"' || c == '$'
}
