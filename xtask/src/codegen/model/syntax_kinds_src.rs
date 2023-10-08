pub(crate) struct SyntaxKindsSrc<'src> {
    pub(crate) punctuation: &'src [(&'src str, &'src str)],
    pub(crate) keywords: &'src [&'src str],
    pub(crate) literals: &'src [&'src str],
    pub(crate) tokens: &'src [&'src str],
    pub(crate) nodes: &'src [&'src str],
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
        ("_", "UNDERSCORE"),
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
    keywords: &[
        "type", "struct", "enum", "trait", "val", "var", "const", "using", "extend", "import",
        "export", "fn", "true", "false", "where", "box",
    ],
    literals: &[
        "INT_LITERAL",
        "FLOAT_LITERAL",
        "CHAR_LITERAL",
        "BYTE_LITERAL",
        "STRING_LITERAL",
        "STRING_LITERAL_FRAGMENT",
    ],
    tokens: &[
        "IDENTIFIER",
        "WHITESPACE",
        "ERROR",
        "LINE_COMMENT",
        "BLOCK_COMMENT",
        "INT_LITERAL_PREFIX",
    ],
    nodes: &[
        "SOURCE_FILE",
        "NAME",
        "LITERAL",
        "EXPORT",
        "ITEM",
        "TYPE_DECLARATION",
        "NAMED_TYPE",
        "STRUCT_TYPE",
        "TRAIT_TYPE",
        "ENUM_TYPE",
        "ENUM_VARIANT",
        "ENUM_VARIANT_BODY_TYPE_LIST",
        "ENUM_VARIANT_BODY_EXPR",
        "ENUM_VARIANT_LIST",
        "STRUCT_FIELD",
        "STRUCT_FIELD_LIST",
        "TYPE_LIST",
        "GENERIC_PARAMETER_LIST",
        "TYPE_PARAMETER",
        "GENERIC_ARGUMENT_LIST",
        "TYPE_ARGUMENT",
        "CONSTRAINT",
        "CONSTRAINT_LIST",
        "TYPE_BOUND_LIST",
        "TYPE_BOUND",
        "BINARY_EXPR",
        "INTERPOLATED_STRING",
        "INTERPOLATED_STRING_PARTS",
        "INTERPOLATED_STRING_SLOT",
    ],
};
