use crate::parser::expressions;
use crate::parser::grammar::{error_block, name};
use crate::parser::parser::Parser;
use crate::syntax::SyntaxKind;
use crate::T;

pub(super) fn parse(parser: &mut Parser) {
    let marker = parser.start_node();

    parser.bump(T!["enum"]);

    if !parser.at(T!["{"]) {
        parser.error("expected an enum definition");
    } else {
        enum_variant_list(parser);
    }

    marker.complete(parser, SyntaxKind::ENUM_TYPE);
}

// TODO: Rename to enum_member_list and enum_member
fn enum_variant_list(parser: &mut Parser) {
    assert!(parser.at(T!["{"]));

    let marker = parser.start_node();
    parser.bump(T!["{"]);

    while !parser.at(T!["}"]) && !parser.at(SyntaxKind::EOF) {
        if parser.at(T!["{"]) {
            error_block(parser, "expected enum member");
            continue;
        }

        enum_variant(parser);

        if !parser.at(T!["}"]) {
            parser.expect(T![","]);
        }
    }

    marker.complete(parser, SyntaxKind::ENUM_VARIANT_LIST);
}

fn enum_variant(parser: &mut Parser) {
    let marker = parser.start_node();

    if !parser.at(SyntaxKind::IDENTIFIER) {
        marker.abandon(parser);
        parser.error_and_bump("expected an enum variant");
        return;
    }

    name(parser);

    if parser.at(T!["="]) {
        parser.bump(T!["="]);
        enum_variant_expression(parser);
    }

    marker.complete(parser, SyntaxKind::ENUM_VARIANT);
}

fn enum_variant_expression(parser: &mut Parser) {
    let marker = parser.start_node();

    expressions::expression(parser);

    marker.complete(parser, SyntaxKind::ENUM_VARIANT_EXPRESSION);
}
