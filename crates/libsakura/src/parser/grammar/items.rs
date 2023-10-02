use crate::parser::parser::Parser;
use crate::syntax::{SyntaxKind, TokenSet};
use crate::T;

pub(super) const ITEM_RECOVERY_SET: TokenSet =
    TokenSet::new(&[T!["type"], T!["fn"], T!["const"], T!["import"], T!["export"], T!["extend"]]);

pub(super) fn module(parser: &mut Parser<'_>) {
    while !(parser.at(SyntaxKind::EOF)) {}
}
