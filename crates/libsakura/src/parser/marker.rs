use crate::parser::event::Event;
use crate::parser::parser::Parser;
use crate::syntax::SyntaxKind;
use drop_bomb::DropBomb;

/// Marker is used to mark the start of a syntax node. It must either be completed or abandoned.
/// If it isn't completed or abandoned, it will panic when dropped.
pub(in crate::parser) struct Marker {
    position: usize,
    drop_bomb: DropBomb,
}

impl Marker {
    pub(in crate::parser) fn new(position: usize) -> Self {
        Self { position, drop_bomb: DropBomb::new("Marker must be either completed or abandoned") }
    }

    pub(in crate::parser) fn position(&self) -> usize {
        self.position
    }

    pub(in crate::parser) fn complete(
        mut self,
        parser: &mut Parser<'_>,
        kind: SyntaxKind,
    ) -> CompletedMarker {
        self.drop_bomb.defuse();

        let index = self.position;
        let Event::Start { kind: start_kind } = &mut parser.events[index] else {
            unreachable!();
        };

        *start_kind = kind;

        parser.push_event(Event::Finish);
        CompletedMarker { position: self.position, kind }
    }

    pub(in crate::parser) fn abandon(mut self, parser: &mut Parser<'_>) {
        self.drop_bomb.defuse();

        let index = self.position;
        let Event::Start { kind } = &mut parser.events[index] else {
            unreachable!();
        };

        *kind = SyntaxKind::TOMBSTONE;
    }
}

pub(in crate::parser) struct CompletedMarker {
    position: usize,
    kind: SyntaxKind,
}

impl CompletedMarker {
    pub(in crate::parser) fn kind(&self) -> SyntaxKind {
        self.kind
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::LexedStr;
    use crate::parser::event::Event;
    use crate::parser::input::ParserInput;
    use crate::parser::parser::Parser;
    use crate::syntax::SyntaxKind;

    #[test]
    #[should_panic]
    fn marker_should_panic_when_dropped() {
        let input = LexedStr::new("test");
        let input: ParserInput = input.into();
        let mut parser = Parser::new(&input);

        drop(parser.start_node());
    }

    #[test]
    fn marker_updates_start_event_on_complete() {
        let input = LexedStr::new("test");
        let input: ParserInput = input.into();
        let mut parser = Parser::new(&input);

        let marker = parser.start_node();
        parser.eat(SyntaxKind::IDENTIFIER);
        let completed = marker.complete(&mut parser, SyntaxKind::TYPE_DEFINITION);
        let finished = parser.finish();

        assert_eq!(completed.kind(), SyntaxKind::TYPE_DEFINITION);
        assert!(matches!(finished[0], Event::Start { kind: SyntaxKind::TYPE_DEFINITION }));
        assert!(matches!(finished[1], Event::Token { kind: SyntaxKind::IDENTIFIER, .. }));
        assert!(matches!(finished[2], Event::Finish));
    }

    #[test]
    fn marker_doesnt_update_start_event_on_abandon() {
        let input = LexedStr::new("test");
        let input: ParserInput = input.into();
        let mut parser = Parser::new(&input);

        let marker = parser.start_node();
        parser.eat(SyntaxKind::IDENTIFIER);
        marker.abandon(&mut parser);
        let finished = parser.finish();

        assert!(matches!(finished[0], Event::Start { kind: SyntaxKind::TOMBSTONE }));
        assert!(matches!(finished[1], Event::Token { kind: SyntaxKind::IDENTIFIER, .. }));
    }
}