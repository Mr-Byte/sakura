use crate::parser::parser::Parser;
use crate::syntax::SyntaxKind;

pub(in crate::parser) fn expression(parser: &mut Parser) {
    todo!()
}

pub(in crate::parser) fn expression_block(parser: &mut Parser) {
    while !parser.at(SyntaxKind::EOF) {
        // TODO: Add expression block parsing!
        parser.bump_any();
    }
}
