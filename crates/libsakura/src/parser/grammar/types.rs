use crate::parser::grammar::{delimited_list, name};
use crate::parser::parser::Parser;
use crate::syntax::{SyntaxKind, TokenSet};
use crate::T;

pub(in crate::parser) fn type_body(parser: &mut Parser) {
    match parser.current() {
        T!["struct"] => struct_type(parser),
        _ => {
            parser.error_and_bump("expected a type");
        }
    }
}

fn struct_type(parser: &mut Parser) {
    let marker = parser.start_node();

    parser.bump(T!["struct"]);
    struct_field_list(parser);

    marker.complete(parser, SyntaxKind::STRUCT_TYPE);
}

fn struct_field_list(parser: &mut Parser) {
    let marker = parser.start_node();

    delimited_list(parser, T!["{"], T!["}"], T![","], struct_field);

    marker.complete(parser, SyntaxKind::STRUCT_FIELD_LIST);
}

fn struct_field(parser: &mut Parser) {
    let marker = parser.start_node();

    name(parser);
    parser.expect(T![":"]);
    name(parser);

    marker.complete(parser, SyntaxKind::STRUCT_FIELD);
}

pub(in crate::parser) fn generic_parameter_list(parser: &mut Parser) {
    let marker = parser.start_node();

    delimited_list(parser, T!["["], T!["]"], T![","], name);

    marker.complete(parser, SyntaxKind::GENERIC_PARAMETER_LIST);
}

pub(in crate::parser) fn constraint_list(parser: &mut Parser) {
    let marker = parser.start_node();

    while !parser.at(T!["="]) {
        constraint(parser);
    }

    marker.complete(parser, SyntaxKind::CONSTRAINT_LIST);
}

const CONSTRAINT_END: TokenSet = TokenSet::new(&[T!["="], T!["where"]]);

fn constraint(parser: &mut Parser) {
    let marker = parser.start_node();

    parser.expect(T!["where"]);
    name(parser);
    parser.expect(T![":"]);

    let inner_marker = parser.start_node();
    loop {
        // TODO: Add support for types as constraints, not just names.
        named_type(parser);

        if parser.nth_at(0, T![","]) && parser.nth_at_token_set(1, CONSTRAINT_END) {
            parser.expect(T![","]);
            break;
        }

        if parser.at_token_set(CONSTRAINT_END) {
            break;
        }

        parser.expect(T![","]);
    }
    inner_marker.complete(parser, SyntaxKind::NAMED_TYPE_LIST);

    marker.complete(parser, SyntaxKind::CONSTRAINT);
}

pub(in crate::parser) fn named_type(parser: &mut Parser) {
    let marker = parser.start_node();

    if parser.at(T!["box"]) {
        parser.bump(T!["box"]);
    }

    name(parser);

    if parser.at(T!["["]) {
        let marker = parser.start_node();
        delimited_list(parser, T!["["], T!["]"], T![","], named_type);
        marker.complete(parser, SyntaxKind::GENERIC_ARGUMENT_LIST);
    }

    marker.complete(parser, SyntaxKind::NAMED_TYPE);
}
