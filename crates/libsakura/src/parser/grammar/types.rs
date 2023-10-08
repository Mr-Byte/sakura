use super::generics;
use crate::parser::grammar::{delimited_list, name};
use crate::parser::parser::Parser;
use crate::syntax::{SyntaxKind, TokenSet};
use crate::T;

pub(super) const TYPE_FIRST_KIND_SET: TokenSet = TokenSet::new(&[
    T!["box"],
    T!["fn"],
    T!["struct"],
    T!["enum"],
    T!["trait"],
    T!["&"],
    T!["_"],
    T!["["],
    SyntaxKind::IDENTIFIER,
]);

pub(super) const TYPE_RECOVERY_SET: TokenSet = TokenSet::new(&[T!["]"], T![","], T!["}"]]);

pub(super) fn type_(parser: &mut Parser) {
    match parser.current() {
        T!["struct"] => struct_type(parser),
        T!["enum"] => todo!(),
        T!["trait"] => todo!(),
        T!["fn"] => todo!(),
        T!["box"] => todo!(),
        T!["&"] => todo!(),
        T!["_"] => todo!(),
        T!["["] => todo!(),
        SyntaxKind::IDENTIFIER => named_type(parser),
        _ => {
            parser.error_recover("expected a type", TYPE_RECOVERY_SET);
        }
    }
}

// TODO: Make this not suck shit
fn named_type(parser: &mut Parser) {
    let marker = parser.start_node();

    name(parser);
    generics::optional_generic_argument_list(parser);

    marker.complete(parser, SyntaxKind::NAMED_TYPE);
}

fn struct_type(parser: &mut Parser) {
    let marker = parser.start_node();

    parser.bump(T!["struct"]);
    struct_field_list(parser);

    marker.complete(parser, SyntaxKind::STRUCT_TYPE);
}

const STRUCT_FIELD_START_SET: TokenSet = TokenSet::new(&[SyntaxKind::IDENTIFIER]);

fn struct_field_list(parser: &mut Parser) {
    // TODO: Overhaul this
    let marker = parser.start_node();

    delimited_list(parser, T!["{"], T!["}"], T![","], STRUCT_FIELD_START_SET, |parser| {
        struct_field(parser)
    });

    marker.complete(parser, SyntaxKind::STRUCT_FIELD_LIST);
}

fn struct_field(parser: &mut Parser) -> bool {
    let marker = parser.start_node();

    name(parser);

    if !parser.eat(T![":"]) {
        parser.error("expected ':'")
    }

    type_(parser);

    marker.complete(parser, SyntaxKind::STRUCT_FIELD);

    true
}
