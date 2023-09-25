use crate::lexer::tokenizer::{Base, LiteralKind, Token, TokenKind};
use crate::syntax::SyntaxKind;
use crate::T;
use std::ops::Range;

pub mod tokenizer;

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
}

struct TokenConverter<'a> {
    result: LexedStr<'a>,
    offset: usize,
}

impl<'a> TokenConverter<'a> {
    fn new(text: &'a str) -> TokenConverter<'a> {
        TokenConverter {
            result: LexedStr { text, kinds: Vec::new(), tokens: Vec::new(), errors: Vec::new() },
            offset: 0,
        }
    }

    fn convert(text: &str) -> LexedStr {
        let tokens = tokenizer::tokenize(text);
        let mut converter = TokenConverter::new(text);

        for token in tokens {
            let text = &converter.result.text[converter.offset..][..token.len];

            converter.append_token(token, text);
        }

        converter.result
    }

    fn append_token(&mut self, token: Token, text: &str) {
        if let TokenKind::Literal { kind, suffix_start } = token.kind.clone() {
            self.append_literal(kind, suffix_start, text);
            return;
        }

        let mut err = None::<&str>;
        let syntax_kind = match token.kind {
            TokenKind::Comma => T![","],
            TokenKind::Dot => T!["."],
            TokenKind::OpenParen => T!["("],
            TokenKind::CloseParen => T![")"],
            TokenKind::OpenBrace => T!["{"],
            TokenKind::CloseBrace => T!["}"],
            TokenKind::OpenBracket => T!["["],
            TokenKind::CloseBracket => T!["]"],
            TokenKind::At => T!["@"],
            TokenKind::Colon => T![":"],
            TokenKind::Dollar => T!["$"],
            TokenKind::Hash => T!["#"],
            TokenKind::Bang => T!["!"],
            TokenKind::Question => T!["?"],
            TokenKind::Eq => T!["="],
            TokenKind::Lt => T!["<"],
            TokenKind::Gt => T![">"],
            TokenKind::Plus => T!["+"],
            TokenKind::Minus => T!["-"],
            TokenKind::Star => T!["*"],
            TokenKind::Slash => T!["/"],
            TokenKind::And => T!["&"],
            TokenKind::Or => T!["|"],
            TokenKind::Caret => T!["^"],
            TokenKind::Tilde => T!["~"],
            TokenKind::Percent => T!["%"],
            TokenKind::LineComment => SyntaxKind::LINE_COMMENT,
            TokenKind::BlockComment { terminated } if terminated => {
                err = Some("Missing trailing */ required to terminate a block comment.");
                SyntaxKind::BLOCK_COMMENT
            }
            TokenKind::BlockComment { .. } => SyntaxKind::BLOCK_COMMENT,
            TokenKind::Whitespace => SyntaxKind::WHITESPACE,
            TokenKind::Identifier => SyntaxKind::IDENTIFIER,
            TokenKind::Unknown => SyntaxKind::ERROR,
            _ => unreachable!("Unexpected token kind: {:?}", token.kind),
        };

        self.push(syntax_kind, text, err);
    }

    fn append_literal(&mut self, kind: LiteralKind, suffix_start: Option<usize>, text: &str) {
        let mut err = None::<&str>;
        let (text, kind) = match kind {
            LiteralKind::Int { empty, base } => {
                if empty {
                    // TODO: Write better error message
                    err = Some("Empty integer literal.");
                }

                if base != Base::Decimal {
                    let (prefix, text) = text.split_at(2);

                    self.push(SyntaxKind::INT_LITERAL_PREFIX, prefix, None);

                    (text, SyntaxKind::INT_LITERAL)
                } else {
                    (text, SyntaxKind::INT_LITERAL)
                }
            }
            LiteralKind::Float { empty_exponent, .. } => {
                if empty_exponent {
                    err = Some("Empty exponent in float literal.");
                }

                (text, SyntaxKind::FLOAT_LITERAL)
            }
            LiteralKind::Char { terminated, .. } => {
                if !terminated {
                    err = Some("Missing trailing ' required to terminate a char literal.");
                }

                (text, SyntaxKind::CHAR_LITERAL)
            }
            LiteralKind::String { terminated, .. } => {
                if !terminated {
                    err = Some(r#"Missing trailing " required to terminate a string literal."#);
                }

                (text, SyntaxKind::STRING_LITERAL)
            }
            LiteralKind::StringPart { terminated, .. } => {
                if !terminated {
                    err = Some(r#"Missing trailing " required to terminate a string literal."#);
                }

                (text, SyntaxKind::STRING_LITERAL)
            }
        };

        let Some(suffix_start) = suffix_start else {
            self.push(kind, text, err);
            return;
        };

        let (literal, suffix) = text.split_at(suffix_start);
        self.push(kind, literal, err);
        self.push(SyntaxKind::IDENTIFIER, suffix, None);
    }

    fn push(&mut self, kind: SyntaxKind, text: &str, err: Option<&str>) {
        let token_range = self.offset..self.offset + text.len();
        self.result.kinds.push(kind);
        self.result.tokens.push(token_range.clone());
        self.offset += text.len();

        if let Some(err) = err {
            self.result.errors.push(LexerError { message: err.to_string(), token: token_range });
        }
    }
}

#[derive(Debug)]
pub struct LexerError {
    pub message: String,
    pub token: Range<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let input = "fn main() { 0x5i32 }";
        let _lexed = dbg!(LexedStr::new(input));
    }
}
