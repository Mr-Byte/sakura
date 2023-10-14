use crate::parser::grammar::types::{type_, TYPE_FIRST_KIND_SET};
use crate::parser::parser::Parser;
use crate::syntax::SyntaxKind;
use crate::T;

pub(super) fn parse(parser: &mut Parser) {
    let marker = parser.start_node();

    parser.bump(T!["union"]);

    marker.complete(parser, SyntaxKind::UNION_TYPE);
}
