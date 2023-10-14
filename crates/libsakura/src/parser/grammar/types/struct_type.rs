use crate::parser::grammar::types::type_;
use crate::parser::grammar::{error_block, name, types};
use crate::parser::parser::Parser;
use crate::syntax::SyntaxKind;
use crate::T;

pub(super) fn parse(parser: &mut Parser) {
    let marker = parser.start_node();

    parser.bump(T!["struct"]);

    if !parser.at(T!["{"]) {
        parser.error("expected '{'");
        marker.abandon(parser);
        return;
    }

    struct_field_list(parser);

    marker.complete(parser, SyntaxKind::STRUCT_TYPE);
}

fn struct_field_list(parser: &mut Parser) {
    assert!(parser.at(T!["{"]));

    let marker = parser.start_node();
    parser.bump(T!["{"]);

    while !parser.at(T!["}"]) && !parser.at(SyntaxKind::EOF) {
        if parser.at(T!["{"]) {
            error_block(parser, "expected a field");
            continue;
        }

        struct_field(parser);

        if !parser.at(T!["}"]) {
            parser.expect(T![","]);
        }
    }

    parser.expect(T!["}"]);
    marker.complete(parser, SyntaxKind::STRUCT_FIELD_LIST);
}

fn struct_field(parser: &mut Parser) {
    let marker = parser.start_node();

    if !parser.at(SyntaxKind::IDENTIFIER) {
        marker.abandon(parser);
        parser.error_and_bump("expected a field declaration");
        return;
    }

    name(parser);

    if !parser.eat(T![":"]) {
        parser.error("expected ':'")
    }

    types::type_(parser);

    if parser.at(T!["delegate"]) {
        optional_delegated_field(parser);
    }

    marker.complete(parser, SyntaxKind::STRUCT_FIELD);
}

fn optional_delegated_field(parser: &mut Parser) {
    assert!(parser.at(T!["delegate"]));

    let marker = parser.start_node();

    parser.bump(T!["delegate"]);
    type_(parser);

    marker.complete(parser, SyntaxKind::DELEGATED_FIELD);
}
