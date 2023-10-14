use super::generics;
use crate::parser::grammar::name;
use crate::parser::parser::Parser;
use crate::syntax::{SyntaxKind, TokenSet};
use crate::T;

mod enum_type;
mod struct_type;
mod trait_type;
mod union_type;

pub(super) const TYPE_FIRST_KIND_SET: TokenSet = TokenSet::new(&[
    T!["struct"],
    T!["enum"],
    T!["union"],
    T!["trait"],
    T!["fn"],
    T!["box"],
    T!["ref"],
    T!["_"],
    T!["["],
    SyntaxKind::IDENTIFIER,
]);

pub(super) const TYPE_RECOVERY_SET: TokenSet = TokenSet::new(&[T!["]"], T![","], T!["}"]]);

// Type =
//   StructType
// | EnumType
// | UnionType
// | TraitType
// | FunctionType
// | BoxType
// | ReferenceType
// | InferType
// | SliceType
// | ArrayType
// | NamedType
pub(super) fn type_(parser: &mut Parser) {
    match parser.current() {
        T!["struct"] => struct_type::parse(parser),
        T!["enum"] => enum_type::parse(parser),
        T!["union"] => union_type::parse(parser),
        T!["trait"] => trait_type::parse(parser),
        T!["fn"] => function_type(parser),
        T!["box"] => box_type(parser),
        T!["ref"] => reference_type(parser),
        T!["_"] => infer_type(parser),
        T!["["] => slice_or_array_type(parser),
        SyntaxKind::IDENTIFIER => named_type(parser),
        _ => {
            parser.error_recover("expected a type", TYPE_RECOVERY_SET);
        }
    }
}

// FunctionType = 'fn' GenericArgumentsList? '(' ArgumentsList? ')' ':' Type
fn function_type(parser: &mut Parser) {
    todo!()
}

// BoxType = 'box' Type
fn box_type(parser: &mut Parser) {
    assert!(parser.at(T!["box"]));

    let marker = parser.start_node();
    parser.bump(T!["box"]);
    type_(parser);
    marker.complete(parser, SyntaxKind::BOXED_TYPE);
}

// ReferenceType = 'ref' Type
fn reference_type(parser: &mut Parser) {
    assert!(parser.at(T!["ref"]));

    let marker = parser.start_node();
    parser.bump(T!["ref"]);
    type_(parser);
    marker.complete(parser, SyntaxKind::REFERENCE_TYPE);
}

// InferType = '_'
fn infer_type(parser: &mut Parser) {
    assert!(parser.at(T!["_"]));

    let marker = parser.start_node();
    parser.bump(T!["_"]);
    marker.complete(parser, SyntaxKind::INFERRED_TYPE);
}

// SliceType = '[' Type ']'
// ArrayType = '[' Type ';' Expression ']'
fn slice_or_array_type(parser: &mut Parser) {
    assert!(parser.at(T!["["]));

    let marker = parser.start_node();
    parser.bump(T!["["]);
    type_(parser);

    if !parser.at(T![","]) {
        parser.expect(T!["]"]);
        marker.complete(parser, SyntaxKind::SLICE_TYPE);
        return;
    }

    todo!("implement array type parsing once expression parsing is implemented.");
}

// NamedType = Name GenericArgumentsList?
fn named_type(parser: &mut Parser) {
    let marker = parser.start_node();

    name(parser);
    generics::optional_generic_argument_list(parser);

    marker.complete(parser, SyntaxKind::NAMED_TYPE);
}
