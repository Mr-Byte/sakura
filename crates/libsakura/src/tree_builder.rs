use crate::lexer::LexedStr;
use crate::parser::{ParserOutput, ParserOutputStep};
use crate::syntax::{SyntaxError, SyntaxKind, SyntaxTreeBuilder};
use rowan::{GreenNode, TextRange};

pub enum StrStep<'a> {
    Token { kind: SyntaxKind, text: &'a str },
    Enter { kind: SyntaxKind },
    Exit,
    Error { message: &'a str, position: usize },
}

pub(crate) fn build_tree(lexed: LexedStr, output: ParserOutput) -> (GreenNode, Vec<SyntaxError>) {
    let mut builder = SyntaxTreeBuilder::default();

    lexed.with_output(&output, &mut |step| match step {
        StrStep::Token { kind, text } => builder.token(kind, text),
        StrStep::Enter { kind } => builder.start_node(kind),
        StrStep::Exit => builder.finish_node(),
        StrStep::Error { message, position } => builder
            .error(message.to_string(), position.try_into().expect("position should convert")),
    });

    let (node, mut errors) = builder.finish_raw();

    for (index, err) in lexed.errors() {
        let text_range = lexed.text_range(index);
        let text_range = TextRange::new(
            text_range.start.try_into().expect("text_range start should convert"),
            text_range.end.try_into().expect("text_range end should convert"),
        );

        errors.push(SyntaxError::new(err, text_range));
    }

    (node, errors)
}

impl<'a> LexedStr<'a> {
    pub(crate) fn with_output(&self, output: &ParserOutput, sink: &mut dyn FnMut(StrStep<'_>)) {
        let mut builder =
            TreeBuilder { lexed: self, position: 0, state: State::PendingEnter, sink };

        for event in output.iter() {
            match event {
                ParserOutputStep::Token { kind, input_token_count } => {
                    builder.token(kind, input_token_count)
                }
                ParserOutputStep::Enter { kind } => builder.enter(kind),
                ParserOutputStep::Exit => builder.exit(),
                ParserOutputStep::Error { message } => {
                    let text_position = builder.lexed.text_range(builder.position);
                    (builder.sink)(StrStep::Error { message, position: text_position.start })
                }
            }
        }

        match std::mem::replace(&mut builder.state, State::Normal) {
            State::PendingExit => {
                builder.eat_trivia();
                (builder.sink)(StrStep::Exit);
            }
            State::PendingEnter | State::Normal => unreachable!(),
        }
    }
}

enum State {
    PendingEnter,
    Normal,
    PendingExit,
}

struct TreeBuilder<'input, 'sink> {
    lexed: &'input LexedStr<'input>,
    position: usize,
    state: State,
    sink: &'sink mut dyn FnMut(StrStep<'_>),
}

impl TreeBuilder<'_, '_> {
    fn token(&mut self, kind: SyntaxKind, token_count: u8) {
        match std::mem::replace(&mut self.state, State::Normal) {
            State::PendingEnter => unreachable!(),
            State::PendingExit => (self.sink)(StrStep::Exit),
            State::Normal => (),
        }

        self.eat_trivia();
        self.do_token(kind, token_count as usize);
    }

    fn enter(&mut self, kind: SyntaxKind) {
        match std::mem::replace(&mut self.state, State::Normal) {
            State::PendingEnter => {
                (self.sink)(StrStep::Enter { kind });

                return;
            }
            State::PendingExit => (self.sink)(StrStep::Exit),
            State::Normal => (),
        }

        let trivia_count = (self.position..self.lexed.len())
            .take_while(|&position| self.lexed.kind(position).is_trivia())
            .count();
        let leading_trivia = self.position..(self.position + trivia_count);
        let attached_trivia_count = attached_trivia_count(
            kind,
            leading_trivia
                .rev()
                .map(|position| (self.lexed.kind(position), self.lexed.text(position))),
        );

        self.eat_trivia_count(trivia_count - attached_trivia_count);
        (self.sink)(StrStep::Enter { kind });
        self.eat_trivia_count(attached_trivia_count);
    }

    fn exit(&mut self) {
        match std::mem::replace(&mut self.state, State::PendingExit) {
            State::PendingEnter => unreachable!(),
            State::PendingExit => (self.sink)(StrStep::Exit),
            State::Normal => (),
        }
    }

    fn eat_trivia(&mut self) {
        while self.position < self.lexed.len() {
            let kind = self.lexed.kind(self.position);

            if !kind.is_trivia() {
                break;
            }

            self.do_token(kind, 1);
        }
    }

    fn eat_trivia_count(&mut self, count: usize) {
        for _ in 0..count {
            let kind = self.lexed.kind(self.position);
            assert!(kind.is_trivia());

            self.do_token(kind, 1);
        }
    }

    fn do_token(&mut self, kind: SyntaxKind, token_count: usize) {
        let text = &self.lexed.text_from_range(self.position..(self.position + 1));

        self.position += token_count;
        (self.sink)(StrStep::Token { kind, text });
    }
}

fn attached_trivia_count<'a>(
    kind: SyntaxKind,
    trivia: impl Iterator<Item = (SyntaxKind, &'a str)>,
) -> usize {
    match kind {
        // TODO: Add all other top level nodes here
        SyntaxKind::TYPE_DECLARATION => {
            let mut count = 0;
            let mut trivia = trivia.enumerate().peekable();

            // TODO: Revisit this!
            while let Some((index, (kind, text))) = trivia.next() {
                match kind {
                    SyntaxKind::WHITESPACE if text.contains("\n\n") => break,
                    SyntaxKind::LINE_COMMENT | SyntaxKind::BLOCK_COMMENT => {
                        count = index + 1;
                    }
                    _ => (),
                }
            }

            count
        }
        _ => 0,
    }
}
