mod common;

use common::check;
use expect_test::expect;
use libsakura::parser::EntryPoint;

#[test]
fn parses_simple_struct_declaration() {
    check(
        EntryPoint::SourceFile,
        r#"
            type Foo = struct {
                bar: i32,
            }
            "#,
        expect![[r#"
            SOURCE_FILE
              WHITESPACE "\n            "
              TYPE_DECLARATION
                TYPE_KW "type"
                WHITESPACE " "
                NAME
                  IDENTIFIER "Foo"
                WHITESPACE " "
                EQUAL "="
                WHITESPACE " "
                STRUCT_TYPE
                  STRUCT_KW "struct"
                  WHITESPACE " "
                  STRUCT_FIELD_LIST
                    LEFT_CURLY "{"
                    WHITESPACE "\n                "
                    STRUCT_FIELD
                      NAME
                        IDENTIFIER "bar"
                      COLON ":"
                      WHITESPACE " "
                      NAMED_TYPE
                        NAME
                          IDENTIFIER "i32"
                    COMMA ","
                    WHITESPACE "\n            "
                    RIGHT_CURLY "}"
              WHITESPACE "\n            "
        "#]],
    );
}

#[test]
fn parses_simple_struct_declaration_with_malformed_fields_using_keywords() {
    check(
        EntryPoint::SourceFile,
        r#"
            type Foo = struct {
                where: i32,
            }
            "#,
        expect![[r#"
            SOURCE_FILE
              WHITESPACE "\n            "
              TYPE_DECLARATION
                TYPE_KW "type"
                WHITESPACE " "
                NAME
                  IDENTIFIER "Foo"
                WHITESPACE " "
                EQUAL "="
                WHITESPACE " "
                STRUCT_TYPE
                  STRUCT_KW "struct"
                  WHITESPACE " "
                  STRUCT_FIELD_LIST
                    LEFT_CURLY "{"
                    WHITESPACE "\n                "
                    STRUCT_FIELD
                      ERROR
                        WHERE_KW "where"
                      COLON ":"
                      WHITESPACE " "
                      NAMED_TYPE
                        NAME
                          IDENTIFIER "i32"
                    COMMA ","
                    WHITESPACE "\n            "
                    RIGHT_CURLY "}"
              WHITESPACE "\n            "
            error 49: expected a name
        "#]],
    );
}

#[test]
fn parses_malformed_simple_struct_declaration_with_missing_colon() {
    check(
        EntryPoint::SourceFile,
        r#"
            type Foo = struct {
                bar i32,
            }
            "#,
        expect![[r#"
            SOURCE_FILE
              WHITESPACE "\n            "
              TYPE_DECLARATION
                TYPE_KW "type"
                WHITESPACE " "
                NAME
                  IDENTIFIER "Foo"
                WHITESPACE " "
                EQUAL "="
                WHITESPACE " "
                STRUCT_TYPE
                  STRUCT_KW "struct"
                  WHITESPACE " "
                  STRUCT_FIELD_LIST
                    LEFT_CURLY "{"
                    WHITESPACE "\n                "
                    STRUCT_FIELD
                      NAME
                        IDENTIFIER "bar"
                      WHITESPACE " "
                      NAMED_TYPE
                        NAME
                          IDENTIFIER "i32"
                    COMMA ","
                    WHITESPACE "\n            "
                    RIGHT_CURLY "}"
              WHITESPACE "\n            "
            error 52: expected ':'
        "#]],
    );
}

#[test]
fn parses_malformed_simple_struct_declaration_with_missing_comma() {
    check(
        EntryPoint::SourceFile,
        r#"
            type Foo = struct {
                bar: i32
                foo: i32
            }
            "#,
        expect![[r#"
            SOURCE_FILE
              WHITESPACE "\n            "
              TYPE_DECLARATION
                TYPE_KW "type"
                WHITESPACE " "
                NAME
                  IDENTIFIER "Foo"
                WHITESPACE " "
                EQUAL "="
                WHITESPACE " "
                STRUCT_TYPE
                  STRUCT_KW "struct"
                  WHITESPACE " "
                  STRUCT_FIELD_LIST
                    LEFT_CURLY "{"
                    WHITESPACE "\n                "
                    STRUCT_FIELD
                      NAME
                        IDENTIFIER "bar"
                      COLON ":"
                      WHITESPACE " "
                      NAMED_TYPE
                        NAME
                          IDENTIFIER "i32"
                    WHITESPACE "\n                "
                    STRUCT_FIELD
                      NAME
                        IDENTIFIER "foo"
                      COLON ":"
                      WHITESPACE " "
                      NAMED_TYPE
                        NAME
                          IDENTIFIER "i32"
                    WHITESPACE "\n            "
                    RIGHT_CURLY "}"
              WHITESPACE "\n            "
            error 57: expected COMMA
        "#]],
    );
}

#[test]
fn parses_malformed_simple_struct_declaration_with_multiple_missing_comma() {
    check(
        EntryPoint::SourceFile,
        r#"
            type Foo = struct {
                bar: i32
                foo: i32
                baz: i32
            }
            "#,
        expect![[r#"
            SOURCE_FILE
              WHITESPACE "\n            "
              TYPE_DECLARATION
                TYPE_KW "type"
                WHITESPACE " "
                NAME
                  IDENTIFIER "Foo"
                WHITESPACE " "
                EQUAL "="
                WHITESPACE " "
                STRUCT_TYPE
                  STRUCT_KW "struct"
                  WHITESPACE " "
                  STRUCT_FIELD_LIST
                    LEFT_CURLY "{"
                    WHITESPACE "\n                "
                    STRUCT_FIELD
                      NAME
                        IDENTIFIER "bar"
                      COLON ":"
                      WHITESPACE " "
                      NAMED_TYPE
                        NAME
                          IDENTIFIER "i32"
                    WHITESPACE "\n                "
                    STRUCT_FIELD
                      NAME
                        IDENTIFIER "foo"
                      COLON ":"
                      WHITESPACE " "
                      NAMED_TYPE
                        NAME
                          IDENTIFIER "i32"
                    WHITESPACE "\n                "
                    STRUCT_FIELD
                      NAME
                        IDENTIFIER "baz"
                      COLON ":"
                      WHITESPACE " "
                      NAMED_TYPE
                        NAME
                          IDENTIFIER "i32"
                    WHITESPACE "\n            "
                    RIGHT_CURLY "}"
              WHITESPACE "\n            "
            error 57: expected COMMA
            error 82: expected COMMA
        "#]],
    );
}

#[test]
fn parses_malformed_simple_struct_declaration_with_missing_right_curly() {
    check(
        EntryPoint::SourceFile,
        r#"
            type Foo = struct {
                bar: i32,
                foo: i32
            "#,
        expect![[r#"
            SOURCE_FILE
              WHITESPACE "\n            "
              TYPE_DECLARATION
                TYPE_KW "type"
                WHITESPACE " "
                NAME
                  IDENTIFIER "Foo"
                WHITESPACE " "
                EQUAL "="
                WHITESPACE " "
                STRUCT_TYPE
                  STRUCT_KW "struct"
                  WHITESPACE " "
                  STRUCT_FIELD_LIST
                    LEFT_CURLY "{"
                    WHITESPACE "\n                "
                    STRUCT_FIELD
                      NAME
                        IDENTIFIER "bar"
                      COLON ":"
                      WHITESPACE " "
                      NAMED_TYPE
                        NAME
                          IDENTIFIER "i32"
                    COMMA ","
                    WHITESPACE "\n                "
                    STRUCT_FIELD
                      NAME
                        IDENTIFIER "foo"
                      COLON ":"
                      WHITESPACE " "
                      NAMED_TYPE
                        NAME
                          IDENTIFIER "i32"
              WHITESPACE "\n            "
            error 83: expected RIGHT_CURLY
        "#]],
    );
}

#[test]
fn parses_new_type_declaration_with_generic_args() {
    check(
        EntryPoint::SourceFile,
        r#"
            type Foo = Bar[i32]
            "#,
        expect![[r#"
            SOURCE_FILE
              WHITESPACE "\n            "
              TYPE_DECLARATION
                TYPE_KW "type"
                WHITESPACE " "
                NAME
                  IDENTIFIER "Foo"
                WHITESPACE " "
                EQUAL "="
                WHITESPACE " "
                NAMED_TYPE
                  NAME
                    IDENTIFIER "Bar"
                  GENERIC_ARGUMENT_LIST
                    LEFT_BRACKET "["
                    TYPE_ARGUMENT
                      NAMED_TYPE
                        NAME
                          IDENTIFIER "i32"
                    RIGHT_BRACKET "]"
              WHITESPACE "\n            "
        "#]],
    );
}

#[test]
fn parses_new_type_declaration_with_struct_generic_args() {
    check(
        EntryPoint::SourceFile,
        r#"
            type Foo = Bar[struct { bar: i32 }]
            "#,
        expect![[r#"
            SOURCE_FILE
              WHITESPACE "\n            "
              TYPE_DECLARATION
                TYPE_KW "type"
                WHITESPACE " "
                NAME
                  IDENTIFIER "Foo"
                WHITESPACE " "
                EQUAL "="
                WHITESPACE " "
                NAMED_TYPE
                  NAME
                    IDENTIFIER "Bar"
                  GENERIC_ARGUMENT_LIST
                    LEFT_BRACKET "["
                    TYPE_ARGUMENT
                      STRUCT_TYPE
                        STRUCT_KW "struct"
                        WHITESPACE " "
                        STRUCT_FIELD_LIST
                          LEFT_CURLY "{"
                          WHITESPACE " "
                          STRUCT_FIELD
                            NAME
                              IDENTIFIER "bar"
                            COLON ":"
                            WHITESPACE " "
                            NAMED_TYPE
                              NAME
                                IDENTIFIER "i32"
                          WHITESPACE " "
                          RIGHT_CURLY "}"
                    RIGHT_BRACKET "]"
              WHITESPACE "\n            "
        "#]],
    );
}

#[test]
fn parses_new_type_declaration_with_malformed_generic_args() {
    check(
        EntryPoint::SourceFile,
        r#"
            type Foo = Bar[i32,
            "#,
        expect![[r#"
            SOURCE_FILE
              WHITESPACE "\n            "
              TYPE_DECLARATION
                TYPE_KW "type"
                WHITESPACE " "
                NAME
                  IDENTIFIER "Foo"
                WHITESPACE " "
                EQUAL "="
                WHITESPACE " "
                NAMED_TYPE
                  NAME
                    IDENTIFIER "Bar"
                  GENERIC_ARGUMENT_LIST
                    LEFT_BRACKET "["
                    TYPE_ARGUMENT
                      NAMED_TYPE
                        NAME
                          IDENTIFIER "i32"
                    COMMA ","
              WHITESPACE "\n            "
            error 32: expected RIGHT_BRACKET
        "#]],
    );
}

#[test]
fn parses_new_type_declaration_with_generic_params_and_args() {
    check(
        EntryPoint::SourceFile,
        r#"
            type Foo[T] = Bar[T]
            "#,
        expect![[r#"
            SOURCE_FILE
              WHITESPACE "\n            "
              TYPE_DECLARATION
                TYPE_KW "type"
                WHITESPACE " "
                NAME
                  IDENTIFIER "Foo"
                GENERIC_PARAMETER_LIST
                  LEFT_BRACKET "["
                  TYPE_PARAMETER
                    NAME
                      IDENTIFIER "T"
                  RIGHT_BRACKET "]"
                WHITESPACE " "
                EQUAL "="
                WHITESPACE " "
                NAMED_TYPE
                  NAME
                    IDENTIFIER "Bar"
                  GENERIC_ARGUMENT_LIST
                    LEFT_BRACKET "["
                    TYPE_ARGUMENT
                      NAMED_TYPE
                        NAME
                          IDENTIFIER "T"
                    RIGHT_BRACKET "]"
              WHITESPACE "\n            "
        "#]],
    );
}

#[test]
fn parses_new_type_declaration_with_generic_params_constraints_and_args() {
    check(
        EntryPoint::SourceFile,
        r#"
            type Foo[T] where T: Baz + Qux = Bar[T]
            "#,
        expect![[r#"
            SOURCE_FILE
              WHITESPACE "\n            "
              TYPE_DECLARATION
                TYPE_KW "type"
                WHITESPACE " "
                NAME
                  IDENTIFIER "Foo"
                GENERIC_PARAMETER_LIST
                  LEFT_BRACKET "["
                  TYPE_PARAMETER
                    NAME
                      IDENTIFIER "T"
                  RIGHT_BRACKET "]"
                WHITESPACE " "
                CONSTRAINT_LIST
                  CONSTRAINT
                    WHERE_KW "where"
                    WHITESPACE " "
                    NAME
                      IDENTIFIER "T"
                    COLON ":"
                    WHITESPACE " "
                    TYPE_BOUND_LIST
                      TYPE_BOUND
                        NAMED_TYPE
                          NAME
                            IDENTIFIER "Baz"
                      WHITESPACE " "
                      PLUS "+"
                      WHITESPACE " "
                      TYPE_BOUND
                        NAMED_TYPE
                          NAME
                            IDENTIFIER "Qux"
                WHITESPACE " "
                EQUAL "="
                WHITESPACE " "
                NAMED_TYPE
                  NAME
                    IDENTIFIER "Bar"
                  GENERIC_ARGUMENT_LIST
                    LEFT_BRACKET "["
                    TYPE_ARGUMENT
                      NAMED_TYPE
                        NAME
                          IDENTIFIER "T"
                    RIGHT_BRACKET "]"
              WHITESPACE "\n            "
        "#]],
    );
}

#[test]
fn parses_new_type_declaration_with_generic_params_constraints_and_args_with_malformed_where_clause(
) {
    check(
        EntryPoint::SourceFile,
        r#"
            type Foo[T] where T Baz + Qux = Bar[T]
            "#,
        expect![[r#"
            SOURCE_FILE
              WHITESPACE "\n            "
              TYPE_DECLARATION
                TYPE_KW "type"
                WHITESPACE " "
                NAME
                  IDENTIFIER "Foo"
                GENERIC_PARAMETER_LIST
                  LEFT_BRACKET "["
                  TYPE_PARAMETER
                    NAME
                      IDENTIFIER "T"
                  RIGHT_BRACKET "]"
                WHITESPACE " "
                CONSTRAINT_LIST
                  CONSTRAINT
                    WHERE_KW "where"
                    WHITESPACE " "
                    NAME
                      IDENTIFIER "T"
                    WHITESPACE " "
                    TYPE_BOUND_LIST
                      TYPE_BOUND
                        NAMED_TYPE
                          NAME
                            IDENTIFIER "Baz"
                      WHITESPACE " "
                      PLUS "+"
                      WHITESPACE " "
                      TYPE_BOUND
                        NAMED_TYPE
                          NAME
                            IDENTIFIER "Qux"
                WHITESPACE " "
                EQUAL "="
                WHITESPACE " "
                NAMED_TYPE
                  NAME
                    IDENTIFIER "Bar"
                  GENERIC_ARGUMENT_LIST
                    LEFT_BRACKET "["
                    TYPE_ARGUMENT
                      NAMED_TYPE
                        NAME
                          IDENTIFIER "T"
                    RIGHT_BRACKET "]"
              WHITESPACE "\n            "
            error 32: expected ':'
        "#]],
    );
}

#[test]
fn parses_new_type_declaration_with_malformed_generic_params_and_args() {
    check(
        EntryPoint::SourceFile,
        r#"
            type Foo[T = Bar[T]
            "#,
        expect![[r#"
            SOURCE_FILE
              WHITESPACE "\n            "
              TYPE_DECLARATION
                TYPE_KW "type"
                WHITESPACE " "
                NAME
                  IDENTIFIER "Foo"
                GENERIC_PARAMETER_LIST
                  LEFT_BRACKET "["
                  TYPE_PARAMETER
                    NAME
                      IDENTIFIER "T"
                    WHITESPACE " "
                    EQUAL "="
                    WHITESPACE " "
                    NAMED_TYPE
                      NAME
                        IDENTIFIER "Bar"
                      GENERIC_ARGUMENT_LIST
                        LEFT_BRACKET "["
                        TYPE_ARGUMENT
                          NAMED_TYPE
                            NAME
                              IDENTIFIER "T"
                        RIGHT_BRACKET "]"
              WHITESPACE "\n            "
            error 32: expected RIGHT_BRACKET
            error 32: expected '='
        "#]],
    );
}

#[test]
fn parses_new_type_declaration_with_malformed_generic_params_constraints_and_args() {
    check(
        EntryPoint::SourceFile,
        r#"
            type Foo[T where T: Baz + Qux = Bar[T]
            "#,
        expect![[r#"
            SOURCE_FILE
              WHITESPACE "\n            "
              TYPE_DECLARATION
                TYPE_KW "type"
                WHITESPACE " "
                NAME
                  IDENTIFIER "Foo"
                GENERIC_PARAMETER_LIST
                  LEFT_BRACKET "["
                  TYPE_PARAMETER
                    NAME
                      IDENTIFIER "T"
                WHITESPACE " "
                CONSTRAINT_LIST
                  CONSTRAINT
                    WHERE_KW "where"
                    WHITESPACE " "
                    NAME
                      IDENTIFIER "T"
                    COLON ":"
                    WHITESPACE " "
                    TYPE_BOUND_LIST
                      TYPE_BOUND
                        NAMED_TYPE
                          NAME
                            IDENTIFIER "Baz"
                      WHITESPACE " "
                      PLUS "+"
                      WHITESPACE " "
                      TYPE_BOUND
                        NAMED_TYPE
                          NAME
                            IDENTIFIER "Qux"
                WHITESPACE " "
                EQUAL "="
                WHITESPACE " "
                NAMED_TYPE
                  NAME
                    IDENTIFIER "Bar"
                  GENERIC_ARGUMENT_LIST
                    LEFT_BRACKET "["
                    TYPE_ARGUMENT
                      NAMED_TYPE
                        NAME
                          IDENTIFIER "T"
                    RIGHT_BRACKET "]"
              WHITESPACE "\n            "
            error 23: expected RIGHT_BRACKET
        "#]],
    );
}