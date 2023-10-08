use crate::parser::grammar::types::{type_, TYPE_FIRST_KIND_SET};
use crate::parser::grammar::{delimited_list, name};
use crate::parser::marker::Marker;
use crate::parser::parser::Parser;
use crate::syntax::{SyntaxKind, TokenSet};
use crate::T;

const GENERIC_PARAM_SET: TokenSet = TokenSet::new(&[SyntaxKind::IDENTIFIER]);

pub(in crate::parser) fn optional_generic_parameter_list(parser: &mut Parser) {
    if parser.at(T!["["]) {
        generic_parameter_list(parser);
    }
}

fn generic_parameter_list(parser: &mut Parser) {
    let marker = parser.start_node();

    delimited_list(parser, T!["["], T!["]"], T![","], GENERIC_PARAM_SET, |parser| {
        let marker = parser.start_node();
        generic_parameter(parser, marker)
    });

    marker.complete(parser, SyntaxKind::GENERIC_PARAMETER_LIST);
}

fn generic_parameter(parser: &mut Parser, marker: Marker) -> bool {
    match parser.current() {
        SyntaxKind::IDENTIFIER => generic_type_parameter(parser, marker),
        _ => {
            marker.abandon(parser);
            parser.error_and_bump("expected a generic parameter");
            return false;
        }
    }

    true
}

fn generic_type_parameter(parser: &mut Parser, marker: Marker) {
    assert!(parser.at(SyntaxKind::IDENTIFIER));

    name(parser);

    if parser.at(T!["="]) {
        parser.bump(T!["="]);
        type_(parser);
    }

    marker.complete(parser, SyntaxKind::TYPE_PARAMETER);
}

pub(in crate::parser) fn optional_constraint_list(parser: &mut Parser) {
    if parser.at(T!["where"]) {
        constraint_list(parser);
    }
}

// TODO: Improve constraint list parsing.
fn constraint_list(parser: &mut Parser) {
    let marker = parser.start_node();

    while parser.at(T!["where"]) {
        constraint(parser);
    }

    marker.complete(parser, SyntaxKind::CONSTRAINT_LIST);
}

fn constraint(parser: &mut Parser) {
    let marker = parser.start_node();

    parser.bump(T!["where"]);
    name(parser);

    if !parser.eat(T![":"]) {
        parser.error("expected ':'");
    }

    type_bound_list(parser);
    marker.complete(parser, SyntaxKind::CONSTRAINT);
}

fn type_bound_list(parser: &mut Parser) {
    let marker = parser.start_node();

    while type_bound(parser) {
        if !parser.eat(T!["+"]) {
            break;
        }
    }

    marker.complete(parser, SyntaxKind::TYPE_BOUND_LIST);
}

fn type_bound(parser: &mut Parser) -> bool {
    let marker = parser.start_node();

    if !parser.at_token_set(TYPE_FIRST_KIND_SET) {
        marker.abandon(parser);
        parser.error_and_bump("expected a type bound");
        return false;
    }

    type_(parser);
    marker.complete(parser, SyntaxKind::TYPE_BOUND);

    true
}

const GENERIC_ARGUMENT_SET: TokenSet = TokenSet::new(&[SyntaxKind::IDENTIFIER]);

pub(in crate::parser) fn optional_generic_argument_list(parser: &mut Parser) {
    if parser.at(T!["["]) {
        generic_argument_list(parser);
    }
}

fn generic_argument_list(parser: &mut Parser) {
    let marker = parser.start_node();

    delimited_list(parser, T!["["], T!["]"], T![","], GENERIC_ARGUMENT_SET, |parser| {
        let marker = parser.start_node();
        type_argument(parser, marker)
    });

    marker.complete(parser, SyntaxKind::GENERIC_ARGUMENT_LIST);
}

fn type_argument(parser: &mut Parser, marker: Marker) -> bool {
    if !parser.at_token_set(TYPE_FIRST_KIND_SET) {
        marker.abandon(parser);
        parser.error_and_bump("expected a generic argument");
        return false;
    }

    type_(parser);
    marker.complete(parser, SyntaxKind::TYPE_ARGUMENT);

    true
}
