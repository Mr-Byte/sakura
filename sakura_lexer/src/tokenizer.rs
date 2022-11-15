use crate::cursor::Cursor;
use crate::token::Base;
use crate::token::LiteralKind;
use crate::token::Token;
use crate::token::TokenKind;
use crate::token::TokenKind::*;

pub(crate) struct Tokenizer<'a>(Cursor<'a>);

impl Tokenizer<'_> {
    pub(crate) fn new<'a>(input: &'a str) -> Tokenizer<'a> {
        Tokenizer(Cursor::new(input))
    }

    pub(crate) fn next_token(&mut self) -> Option<Token> {
        let next_char = self.0.bump()?;

        let token_kind = match next_char {
            '/' => match self.0.first() {
                '/' => self.scan_line_comment(),
                '*' => self.scan_block_comment(),
                _ => Slash,
            },

            c if char::is_whitespace(c) => self.scan_whitespace(),
            c if is_identifier_start(c) => self.scan_identifier(),
            c @ '0'..='9' => {
                let literal_kind = self.scan_number(c);
                let suffix_start = self.0.consumed_len();
                // NOTE: Lex any trailing idenfitiers as a suffix
                self.scan_identifier();

                TokenKind::Literal {
                    kind: literal_kind,
                    suffix_start,
                }
            }

            // Symbol tokens
            ',' => Comma,
            '.' => Dot,
            '(' => OpenParen,
            ')' => CloseParen,
            '{' => OpenBrace,
            '}' => CloseBrace,
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
                let terminated = self.scan_double_quoted_string();
                let suffix_start = self.0.consumed_len();

                if terminated {
                    self.scan_identifier();
                }

                let kind = LiteralKind::String { terminated };

                Literal { kind, suffix_start }
            }
            '\'' => {
                let terminated = self.scan_single_quoted_string();
                let suffix_start = self.0.consumed_len();

                if terminated {
                    self.scan_identifier();
                }

                let kind = LiteralKind::Char { terminated };

                Literal { kind, suffix_start }
            }
            _ => Unknown,
        };

        let token = Token {
            kind: token_kind,
            len: self.0.consumed_len(),
        };
        self.0.reset_len();

        Some(token)
    }

    pub(self) fn scan_identifier(&mut self) -> TokenKind {
        self.0.bump_while(is_identifier_continue);
        Identifier
    }

    pub(self) fn scan_whitespace(&mut self) -> TokenKind {
        self.0.bump_while(char::is_whitespace);
        Whitespace
    }

    pub(self) fn scan_line_comment(&mut self) -> TokenKind {
        self.0.bump_while(|c| c != '\n');
        LineComment
    }

    pub(self) fn scan_block_comment(&mut self) -> TokenKind {
        self.0.bump();

        let mut depth: usize = 1;
        while let Some(c) = self.0.bump() {
            match c {
                '/' if self.0.first() == '*' => {
                    self.0.bump();
                    depth += 1;
                }
                '*' if self.0.first() == '/' => {
                    self.0.bump();
                    depth -= 1;

                    if depth == 0 {
                        break;
                    }
                }

                _ => (),
            }
        }

        BlockComment {
            terminated: depth == 0,
        }
    }

    pub(self) fn scan_number(&mut self, first_digit: char) -> LiteralKind {
        let mut base = Base::Decimal;

        if first_digit == '0' {
            let has_digits = match self.0.first() {
                'b' => {
                    base = Base::Binary;
                    self.0.bump();
                    self.scan_decimal_digits()
                }
                'o' => {
                    base = Base::Octal;
                    self.0.bump();
                    self.scan_decimal_digits()
                }
                'x' => {
                    base = Base::Hexadecimal;
                    self.0.bump();
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

        match self.0.first() {
            '.' if self.0.second() != '.' && !is_identifier_start(self.0.second()) => {
                self.0.bump();
                let mut empty_exponent = false;
                if self.0.first().is_digit(10) {
                    self.scan_decimal_digits();
                    match self.0.first() {
                        'e' | 'E' => {
                            self.0.bump();
                            empty_exponent = !self.scan_float_exponent();
                        }
                        _ => (),
                    }
                }

                LiteralKind::Float {
                    base,
                    empty_exponent,
                }
            }
            'e' | 'E' => {
                self.0.bump();
                let empty_exponent = !self.scan_float_exponent();
                LiteralKind::Float {
                    base,
                    empty_exponent,
                }
            }
            _ => LiteralKind::Int { base, empty: false },
        }
    }

    pub(self) fn scan_decimal_digits(&mut self) -> bool {
        let mut has_digits = false;

        loop {
            match self.0.first() {
                '_' => {
                    self.0.bump();
                }
                '0'..='9' => {
                    self.0.bump();
                    has_digits = true;
                }
                _ => break,
            }
        }

        has_digits
    }

    pub(self) fn scan_hexadecimal_digits(&mut self) -> bool {
        let mut has_digits = false;

        loop {
            match self.0.first() {
                '_' => {
                    self.0.bump();
                }
                '0'..='9' | 'a'..='f' | 'A'..='F' => {
                    self.0.bump();
                    has_digits = true;
                }
                _ => break,
            }
        }

        has_digits
    }

    pub(self) fn scan_float_exponent(&mut self) -> bool {
        if matches!(self.0.first(), '-' | '+') {
            self.0.bump();
        }

        self.scan_decimal_digits()
    }

    pub(self) fn scan_double_quoted_string(&mut self) -> bool {
        while let Some(c) = self.0.bump() {
            match c {
                '"' => return true,
                '\\' if self.0.first() == '\\' || self.0.first() == '"' => {
                    self.0.bump();
                }
                _ => (),
            }
        }

        false
    }

    pub(self) fn scan_single_quoted_string(&mut self) -> bool {
        while let Some(c) = self.0.bump() {
            match c {
                '\'' => return true,
                '\\' if self.0.first() == '\\' || self.0.first() == '\'' => {
                    self.0.bump();
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
