use crate::parser::grammar::{error_block, generics, name_recovery, types};
use crate::parser::marker::Marker;
use crate::parser::parser::Parser;
use crate::syntax::{SyntaxKind, TokenSet};
use crate::T;

const ITEM_RECOVERY_SET: TokenSet =
    TokenSet::new(&[T!["type"], T!["fn"], T!["const"], T!["extend"], T!["export"], T!["import"]]);

pub(super) fn module(parser: &mut Parser<'_>) {
    while !parser.at(SyntaxKind::EOF) {
        item(parser);
    }
}

fn item(parser: &mut Parser<'_>) {
    let marker = parser.start_node();
    let Err(marker) = optional_item(parser, marker) else {
        return;
    };

    marker.abandon(parser);
    parser.error("expected an item");
}

fn optional_export(parser: &mut Parser<'_>) -> bool {
    if !parser.at(T!["export"]) {
        return false;
    }

    let marker = parser.start_node();
    parser.bump(T!["export"]);
    marker.complete(parser, SyntaxKind::EXPORT);

    true
}

fn optional_item(parser: &mut Parser<'_>, marker: Marker) -> Result<(), Marker> {
    let is_exported = optional_export(parser);

    match parser.current() {
        T!["type"] => type_declaration(parser, marker),
        _ if is_exported => {
            parser.error_and_bump("expected an item");
            marker.complete(parser, SyntaxKind::ERROR);
        }
        T!["{"] => {
            error_block(parser, "expected an item");
            return Err(marker);
        }
        _ => {
            parser.error_and_bump("expected an item");
            return Err(marker);
        }
    }

    Ok(())
}

fn type_declaration(parser: &mut Parser, marker: Marker) {
    parser.bump(T!["type"]);

    name_recovery(parser, ITEM_RECOVERY_SET);
    generics::optional_generic_parameter_list(parser);

    match parser.current() {
        T!["where"] => {
            generics::optional_constraint_list(parser);
            match parser.current() {
                T!["="] => {
                    parser.bump(T!["="]);
                    types::type_(parser);
                }
                _ => parser.error("expected '='"),
            }
        }
        T!["="] => {
            parser.bump(T!["="]);
            types::type_(parser);
        }
        _ => parser.error("expected '='"),
    }

    marker.complete(parser, SyntaxKind::TYPE_DECLARATION);
}
