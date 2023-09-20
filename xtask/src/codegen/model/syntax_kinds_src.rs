pub(crate) struct SyntaxKindsSrc<'a> {
    pub(crate) punctuation: &'a [(&'a str, &'a str)],
    pub(crate) keywords: &'a [&'a str],
    pub(crate) literals: &'a [&'a str],
    pub(crate) tokens: &'a [&'a str],
    pub(crate) nodes: &'a [&'a str],
}

pub(crate) const SYNTAX_KINDS_SRC: SyntaxKindsSrc<'static> = SyntaxKindsSrc {
    punctuation: &[
        ("(", "LEFT_PAREN"),
        (")", "RIGHT_PAREN"),
        ("{", "LEFT_CURLY"),
        ("}", "RIGHT_CURLY"),
        ("[", "LEFT_BRACKET"),
        ("]", "RIGHT_BRACKET"),
        (",", "COMMA"),
        (".", "DOT"),
        (":", "COLON"),
        ("=", "EQUAL"),
        ("+", "PLUS"),
        ("-", "MINUS"),
        ("*", "STAR"),
        ("/", "SLASH"),
        ("&", "AMPERSAND"),
        ("|", "PIPE"),
        ("^", "CARET"),
        ("~", "TILDE"),
        ("%", "PERCENT"),
        ("@", "AT"),
        ("#", "HASH"),
        ("!", "BANG"),
        ("?", "QUESTION"),
        ("$", "DOLLAR"),
        ("**", "DOUBLE_STAR"),
        ("..", "DOUBLE_DOT"),
        ("..=", "DOUBLE_DOT_EQUAL"),
        ("==", "DOUBLE_EQUAL"),
        ("&&", "DOUBLE_AMPERSAND"),
        ("||", "DOUBLE_PIPE"),
        ("=>", "FAT_ARROW"),
        ("!=", "NOT_EQUAL"),
        ("<", "LESS_THAN"),
        (">", "GREATER_THAN"),
        ("<=", "LESS_THAN_EQUAL"),
        (">=", "GREATER_THAN_EQUAL"),
        ("+=", "PLUS_EQUAL"),
        ("-=", "MINUS_EQUAL"),
        ("|=", "PIPE_EQUAL"),
        ("&=", "AMPERSAND_EQUAL"),
        ("^=", "CARET_EQUAL"),
        ("/=", "SLASH_EQUAL"),
        ("*=", "STAR_EQUAL"),
        ("%=", "PERCENT_EQUAL"),
        ("<<", "SHIFT_LEFT"),
        (">>", "SHIFT_RIGHT"),
        ("<<=", "SHIFT_LEFT_EQUAL"),
        (">>=", "SHIFT_RIGHT_EQUAL"),
    ],
    keywords: &["type", "struct", "enum", "trait", "let", "mut", "fn", "true", "false"],
    literals: &[
        "INT_LITERAL",
        "FLOAT_LITERAL",
        "CHAR_LITERAL",
        "BYTE_LITERAL",
        "STRING_LITERAL",
        "STRING_LITERAL_FRAGMENT",
    ],
    tokens: &["IDENTIFIER", "WHITESPACE", "ERROR", "LINE_COMMENT", "BLOCK_COMMENT"],
    nodes: &[
        "SOURCE_FILE",
        "NAME",
        "LITERAL",
        "ITEM",
        "TYPE_DEFINITION",
        "TYPE_LIST",
        "STRUCT_TYPE",
        "TRAIT_TYPE",
        "ENUM_TYPE",
        "ENUM_VARIANT",
        "ENUM_VARIANT_BODY_TYPE_LIST",
        "ENUM_VARIANT_BODY_EXPR",
        "ENUM_VARIANT_LIST",
        "STRUCT_FIELD",
        "STRUCT_FIELD_DEFINITION_LIST",
        "BINARY_EXPR",
        "INTERPOLATED_STRING",
        "INTERPOLATED_STRING_PARTS",
        "INTERPOLATED_STRING_SLOT",
    ],
};
