use crate::parser::grammar::error_block;
use crate::parser::parser::Parser;
use crate::syntax::SyntaxKind;
use crate::T;

pub(super) fn parse(parser: &mut Parser) {
    let marker = parser.start_node();

    parser.bump(T!["trait"]);

    if !parser.expect(T!["{"]) {
        marker.abandon(parser);
        return;
    }

    trait_items(parser);

    marker.complete(parser, SyntaxKind::TRAIT_TYPE);
}

fn trait_items(parser: &mut Parser) {
    assert!(parser.at(T!["{"]));

    parser.bump(T!["{"]);

    while !parser.at(T!["}"]) && !parser.at(SyntaxKind::EOF) {
        if parser.at(T!["{"]) {
            error_block(parser, "expected a trait item");
            continue;
        }

        trait_item(parser);
    }

    parser.expect(T!["}"]);
}

fn trait_item(parser: &mut Parser) {
    todo!()
}
