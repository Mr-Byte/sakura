mod items;

use crate::syntax::SyntaxKind;

use super::parser::Parser;

pub(in crate::parser) mod entry {
    use super::*;

    pub(in crate::parser) mod top {
        use super::*;

        pub(in crate::parser) fn source_file(parser: &mut Parser<'_>) {
            let marker = parser.start_node();

            marker.complete(parser, SyntaxKind::SOURCE_FILE);
        }
    }
}
