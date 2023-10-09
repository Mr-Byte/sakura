use crate::parser::parser::Parser;
use crate::syntax::SyntaxKind;
use crate::T;

pub (in crate::parser) fn expression_block(parser: &mut Parser) {
    while !parser.at(T!["{"]) && !parser.at(SyntaxKind::EOF) {

        // TODO: Add expression block parsing!
        parser.bump_any();
    }
}