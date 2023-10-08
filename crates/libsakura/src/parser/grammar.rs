mod generics;
mod items;
mod types;

use super::parser::Parser;
use crate::syntax::{SyntaxKind, TokenSet};

pub(in crate::parser) mod entry {
    use super::*;

    pub(in crate::parser) mod top {
        use super::*;
        use crate::parser::grammar::items::module;

        pub(in crate::parser) fn source_file(parser: &mut Parser<'_>) {
            let marker = parser.start_node();

            module(parser);

            marker.complete(parser, SyntaxKind::SOURCE_FILE);
        }
    }
}

pub(in crate::parser) fn name(parser: &mut Parser<'_>) {
    name_recovery(parser, TokenSet::EMPTY);
}

pub(in crate::parser) fn name_recovery(parser: &mut Parser<'_>, recovery_set: TokenSet) {
    if !parser.at(SyntaxKind::IDENTIFIER) {
        parser.error_recover("expected a name", recovery_set);
        return;
    }

    let marker = parser.start_node();
    parser.bump(SyntaxKind::IDENTIFIER);
    marker.complete(parser, SyntaxKind::NAME);
}

pub(in crate::parser) fn delimited_list(
    parser: &mut Parser<'_>,
    start: SyntaxKind,
    end: SyntaxKind,
    delimiter: SyntaxKind,
    first_kind_set: TokenSet,
    mut item: impl FnMut(&mut Parser<'_>) -> bool,
) {
    parser.bump(start);

    while !parser.at(end) && !parser.at(SyntaxKind::EOF) {
        if !item(parser) {
            break;
        }

        if parser.at(delimiter) {
            parser.bump(delimiter);
            continue;
        }

        if !parser.at_token_set(first_kind_set) {
            break;
        }

        parser.error(format!("expected {delimiter:?}"));
    }

    parser.expect(end);
}
