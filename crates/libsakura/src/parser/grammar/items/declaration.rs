use crate::parser::grammar::name;
use crate::parser::grammar::types;
use crate::parser::marker::Marker;
use crate::parser::parser::Parser;
use crate::syntax::SyntaxKind;
use crate::T;

pub(in crate::parser) fn type_declaration(parser: &mut Parser, marker: Marker) {
    parser.bump(T!["type"]);

    name(parser);

    if parser.at(T!["["]) {
        types::generic_parameter_list(parser);
    }

    if parser.at(T!["where"]) {
        types::constraint_list(parser);
    }

    parser.expect(T!["="]);

    types::type_body(parser);

    marker.complete(parser, SyntaxKind::TYPE_DECLARATION);
}
