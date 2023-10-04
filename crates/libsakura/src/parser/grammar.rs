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
    item: fn(&mut Parser<'_>),
) {
    parser.expect(start);

    loop {
        if parser.at(end) || parser.at(SyntaxKind::EOF) {
            break;
        }

        item(parser);

        if parser.nth_at(0, delimiter) && parser.nth_at(1, end) {
            parser.expect(delimiter);
            break;
        }

        if !parser.at(end) {
            parser.expect(delimiter);
        }
    }

    parser.expect(end);
}
