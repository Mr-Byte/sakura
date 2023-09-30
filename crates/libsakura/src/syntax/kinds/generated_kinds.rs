//! Code generated by `cargo xtask codegen`; DO NOT EDIT.

#![allow(
    dead_code,
    bad_style,
    missing_docs,
    unreachable_pub,
    clippy::manual_non_exhaustive,
    clippy::upper_case_acronyms
)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum SyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc(hidden)]
    EOF,
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_CURLY,
    RIGHT_CURLY,
    LEFT_BRACKET,
    RIGHT_BRACKET,
    COMMA,
    DOT,
    COLON,
    EQUAL,
    PLUS,
    MINUS,
    STAR,
    SLASH,
    AMPERSAND,
    PIPE,
    CARET,
    TILDE,
    PERCENT,
    AT,
    HASH,
    BANG,
    QUESTION,
    DOLLAR,
    DOUBLE_STAR,
    DOUBLE_DOT,
    DOUBLE_DOT_EQUAL,
    DOUBLE_EQUAL,
    DOUBLE_AMPERSAND,
    DOUBLE_PIPE,
    FAT_ARROW,
    NOT_EQUAL,
    LESS_THAN,
    GREATER_THAN,
    LESS_THAN_EQUAL,
    GREATER_THAN_EQUAL,
    PLUS_EQUAL,
    MINUS_EQUAL,
    PIPE_EQUAL,
    AMPERSAND_EQUAL,
    CARET_EQUAL,
    SLASH_EQUAL,
    STAR_EQUAL,
    PERCENT_EQUAL,
    SHIFT_LEFT,
    SHIFT_RIGHT,
    SHIFT_LEFT_EQUAL,
    SHIFT_RIGHT_EQUAL,
    TYPE_KW,
    STRUCT_KW,
    ENUM_KW,
    TRAIT_KW,
    LET_KW,
    MUT_KW,
    FN_KW,
    TRUE_KW,
    FALSE_KW,
    INT_LITERAL,
    FLOAT_LITERAL,
    CHAR_LITERAL,
    BYTE_LITERAL,
    STRING_LITERAL,
    STRING_LITERAL_FRAGMENT,
    IDENTIFIER,
    WHITESPACE,
    ERROR,
    LINE_COMMENT,
    BLOCK_COMMENT,
    INT_LITERAL_PREFIX,
    #[doc(hidden)]
    __TOKEN_SENTINEL,
    SOURCE_FILE,
    NAME,
    LITERAL,
    ITEM,
    TYPE_DEFINITION,
    TYPE_LIST,
    STRUCT_TYPE,
    TRAIT_TYPE,
    ENUM_TYPE,
    ENUM_VARIANT,
    ENUM_VARIANT_BODY_TYPE_LIST,
    ENUM_VARIANT_BODY_EXPR,
    ENUM_VARIANT_LIST,
    STRUCT_FIELD,
    STRUCT_FIELD_DEFINITION_LIST,
    BINARY_EXPR,
    INTERPOLATED_STRING,
    INTERPOLATED_STRING_PARTS,
    INTERPOLATED_STRING_SLOT,
    #[doc(hidden)]
    __LAST,
}

use self::SyntaxKind::*;

impl SyntaxKind {
    pub fn is_punctuation(self) -> bool {
        matches!(
            self,
            LEFT_PAREN
                | RIGHT_PAREN
                | LEFT_CURLY
                | RIGHT_CURLY
                | LEFT_BRACKET
                | RIGHT_BRACKET
                | COMMA
                | DOT
                | COLON
                | EQUAL
                | PLUS
                | MINUS
                | STAR
                | SLASH
                | AMPERSAND
                | PIPE
                | CARET
                | TILDE
                | PERCENT
                | AT
                | HASH
                | BANG
                | QUESTION
                | DOLLAR
                | DOUBLE_STAR
                | DOUBLE_DOT
                | DOUBLE_DOT_EQUAL
                | DOUBLE_EQUAL
                | DOUBLE_AMPERSAND
                | DOUBLE_PIPE
                | FAT_ARROW
                | NOT_EQUAL
                | LESS_THAN
                | GREATER_THAN
                | LESS_THAN_EQUAL
                | GREATER_THAN_EQUAL
                | PLUS_EQUAL
                | MINUS_EQUAL
                | PIPE_EQUAL
                | AMPERSAND_EQUAL
                | CARET_EQUAL
                | SLASH_EQUAL
                | STAR_EQUAL
                | PERCENT_EQUAL
                | SHIFT_LEFT
                | SHIFT_RIGHT
                | SHIFT_LEFT_EQUAL
                | SHIFT_RIGHT_EQUAL
        )
    }

    pub fn is_literal(self) -> bool {
        matches!(
            self,
            INT_LITERAL
                | FLOAT_LITERAL
                | CHAR_LITERAL
                | BYTE_LITERAL
                | STRING_LITERAL
                | STRING_LITERAL_FRAGMENT
        )
    }

    pub fn is_keyword(self) -> bool {
        matches!(
            self,
            TYPE_KW | STRUCT_KW | ENUM_KW | TRAIT_KW | LET_KW | MUT_KW | FN_KW | TRUE_KW | FALSE_KW
        )
    }

    pub fn from_keyword(ident: &str) -> Option<SyntaxKind> {
        let kw = match ident {
            "type" => TYPE_KW,
            "struct" => STRUCT_KW,
            "enum" => ENUM_KW,
            "trait" => TRAIT_KW,
            "let" => LET_KW,
            "mut" => MUT_KW,
            "fn" => FN_KW,
            "true" => TRUE_KW,
            "false" => FALSE_KW,
            _ => return None,
        };

        Some(kw)
    }

    pub fn from_char(c: char) -> Option<SyntaxKind> {
        let tok = match c {
            '(' => LEFT_PAREN,
            ')' => RIGHT_PAREN,
            '{' => LEFT_CURLY,
            '}' => RIGHT_CURLY,
            '[' => LEFT_BRACKET,
            ']' => RIGHT_BRACKET,
            ',' => COMMA,
            '.' => DOT,
            ':' => COLON,
            '=' => EQUAL,
            '+' => PLUS,
            '-' => MINUS,
            '*' => STAR,
            '/' => SLASH,
            '&' => AMPERSAND,
            '|' => PIPE,
            '^' => CARET,
            '~' => TILDE,
            '%' => PERCENT,
            '@' => AT,
            '#' => HASH,
            '!' => BANG,
            '?' => QUESTION,
            '$' => DOLLAR,
            '<' => LESS_THAN,
            '>' => GREATER_THAN,
            _ => return None,
        };

        Some(tok)
    }
}

#[macro_export]
macro_rules! T { ["("] => { $ crate :: syntax :: SyntaxKind :: LEFT_PAREN } ; [")"] => { $ crate :: syntax :: SyntaxKind :: RIGHT_PAREN } ; ["{"] => { $ crate :: syntax :: SyntaxKind :: LEFT_CURLY } ; ["}"] => { $ crate :: syntax :: SyntaxKind :: RIGHT_CURLY } ; ["["] => { $ crate :: syntax :: SyntaxKind :: LEFT_BRACKET } ; ["]"] => { $ crate :: syntax :: SyntaxKind :: RIGHT_BRACKET } ; [","] => { $ crate :: syntax :: SyntaxKind :: COMMA } ; ["."] => { $ crate :: syntax :: SyntaxKind :: DOT } ; [":"] => { $ crate :: syntax :: SyntaxKind :: COLON } ; ["="] => { $ crate :: syntax :: SyntaxKind :: EQUAL } ; ["+"] => { $ crate :: syntax :: SyntaxKind :: PLUS } ; ["-"] => { $ crate :: syntax :: SyntaxKind :: MINUS } ; ["*"] => { $ crate :: syntax :: SyntaxKind :: STAR } ; ["/"] => { $ crate :: syntax :: SyntaxKind :: SLASH } ; ["&"] => { $ crate :: syntax :: SyntaxKind :: AMPERSAND } ; ["|"] => { $ crate :: syntax :: SyntaxKind :: PIPE } ; ["^"] => { $ crate :: syntax :: SyntaxKind :: CARET } ; ["~"] => { $ crate :: syntax :: SyntaxKind :: TILDE } ; ["%"] => { $ crate :: syntax :: SyntaxKind :: PERCENT } ; ["@"] => { $ crate :: syntax :: SyntaxKind :: AT } ; ["#"] => { $ crate :: syntax :: SyntaxKind :: HASH } ; ["!"] => { $ crate :: syntax :: SyntaxKind :: BANG } ; ["?"] => { $ crate :: syntax :: SyntaxKind :: QUESTION } ; ["$"] => { $ crate :: syntax :: SyntaxKind :: DOLLAR } ; ["**"] => { $ crate :: syntax :: SyntaxKind :: DOUBLE_STAR } ; [".."] => { $ crate :: syntax :: SyntaxKind :: DOUBLE_DOT } ; ["..="] => { $ crate :: syntax :: SyntaxKind :: DOUBLE_DOT_EQUAL } ; ["=="] => { $ crate :: syntax :: SyntaxKind :: DOUBLE_EQUAL } ; ["&&"] => { $ crate :: syntax :: SyntaxKind :: DOUBLE_AMPERSAND } ; ["||"] => { $ crate :: syntax :: SyntaxKind :: DOUBLE_PIPE } ; ["=>"] => { $ crate :: syntax :: SyntaxKind :: FAT_ARROW } ; ["!="] => { $ crate :: syntax :: SyntaxKind :: NOT_EQUAL } ; ["<"] => { $ crate :: syntax :: SyntaxKind :: LESS_THAN } ; [">"] => { $ crate :: syntax :: SyntaxKind :: GREATER_THAN } ; ["<="] => { $ crate :: syntax :: SyntaxKind :: LESS_THAN_EQUAL } ; [">="] => { $ crate :: syntax :: SyntaxKind :: GREATER_THAN_EQUAL } ; ["+="] => { $ crate :: syntax :: SyntaxKind :: PLUS_EQUAL } ; ["-="] => { $ crate :: syntax :: SyntaxKind :: MINUS_EQUAL } ; ["|="] => { $ crate :: syntax :: SyntaxKind :: PIPE_EQUAL } ; ["&="] => { $ crate :: syntax :: SyntaxKind :: AMPERSAND_EQUAL } ; ["^="] => { $ crate :: syntax :: SyntaxKind :: CARET_EQUAL } ; ["/="] => { $ crate :: syntax :: SyntaxKind :: SLASH_EQUAL } ; ["*="] => { $ crate :: syntax :: SyntaxKind :: STAR_EQUAL } ; ["%="] => { $ crate :: syntax :: SyntaxKind :: PERCENT_EQUAL } ; ["<<"] => { $ crate :: syntax :: SyntaxKind :: SHIFT_LEFT } ; [">>"] => { $ crate :: syntax :: SyntaxKind :: SHIFT_RIGHT } ; ["<<="] => { $ crate :: syntax :: SyntaxKind :: SHIFT_LEFT_EQUAL } ; [">>="] => { $ crate :: syntax :: SyntaxKind :: SHIFT_RIGHT_EQUAL } ; ["type"] => { $ crate :: syntax :: SyntaxKind :: TYPE_KW } ; ["struct"] => { $ crate :: syntax :: SyntaxKind :: STRUCT_KW } ; ["enum"] => { $ crate :: syntax :: SyntaxKind :: ENUM_KW } ; ["trait"] => { $ crate :: syntax :: SyntaxKind :: TRAIT_KW } ; ["let"] => { $ crate :: syntax :: SyntaxKind :: LET_KW } ; ["mut"] => { $ crate :: syntax :: SyntaxKind :: MUT_KW } ; ["fn"] => { $ crate :: syntax :: SyntaxKind :: FN_KW } ; ["true"] => { $ crate :: syntax :: SyntaxKind :: TRUE_KW } ; ["false"] => { $ crate :: syntax :: SyntaxKind :: FALSE_KW } ; ["identifier"] => { $ crate :: syntax :: SyntaxKind :: IDENTIFIER } ; }
