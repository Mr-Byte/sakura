mod declaration;

use crate::parser::marker::Marker;
use crate::parser::parser::Parser;
use crate::syntax::SyntaxKind;
use crate::T;

pub(super) fn module(parser: &mut Parser<'_>) {
    while !(parser.at(SyntaxKind::EOF)) {
        item(parser);
    }
}

fn item(parser: &mut Parser<'_>) {
    let marker = parser.start_node();

    let Err(marker) = optional_item(parser, marker) else {
        return;
    };

    marker.abandon(parser);

    match parser.current() {
        _ => parser.error_and_bump("expected an item"),
    }
}

fn optional_item(parser: &mut Parser<'_>, marker: Marker) -> Result<(), Marker> {
    match parser.current() {
        T!["type"] => declaration::type_declaration(parser, marker),
        _ => {
            parser.error_and_bump("expected an item");
            return Err(marker);
        }
    }

    Ok(())
}
