use crate::parser::expressions;
use crate::parser::grammar::delimited_list;
use crate::parser::parser::Parser;
use crate::syntax::{SyntaxKind, TokenSet};
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

const ENUM_VARIANT_FIRST_SET: TokenSet = TokenSet::new(&[SyntaxKind::IDENTIFIER]);

fn enum_variant_list(parser: &mut Parser) {
    let marker = parser.start_node();

    delimited_list(parser, T!["{"], T!["}"], T![","], ENUM_VARIANT_FIRST_SET, |parser| {
        enum_variant(parser)
    });

    marker.complete(parser, SyntaxKind::ENUM_VARIANT_LIST);
}

fn enum_variant(parser: &mut Parser) -> bool {
    let marker = parser.start_node();

    if parser.current() != SyntaxKind::IDENTIFIER {
        marker.abandon(parser);
        parser.error_and_bump("expected an enum variant");
        return false;
    }

    parser.bump(SyntaxKind::IDENTIFIER);

    if parser.at(T!["="]) {
        parser.bump(T!["="]);
        enum_variant_expression(parser);
    }

    marker.complete(parser, SyntaxKind::ENUM_VARIANT);

    true
}

fn enum_variant_expression(parser: &mut Parser) {
    let marker = parser.start_node();

    expressions::expression(parser);

    marker.complete(parser, SyntaxKind::ENUM_VARIANT_EXPRESSION);
}
