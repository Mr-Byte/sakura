use crate::parser::output::ParserOutput;
use crate::syntax::SyntaxKind;

pub enum Event {
    Start {
        kind: SyntaxKind,
        // NOTE: Does this need forward parent?
    },
    Finish,
    Token {
        kind: SyntaxKind,
        raw_token_count: u8,
    },
    Error {
        message: String,
    },
}

impl Event {
    pub(crate) fn tombstone() -> Self {
        Event::Start { kind: SyntaxKind::TOMBSTONE }
    }
}

pub(super) fn process(mut events: Vec<Event>) -> ParserOutput {
    let mut output = ParserOutput::default();

    for index in 0..events.len() {
        let event = std::mem::replace(&mut events[index], Event::tombstone());

        match event {
            Event::Start { kind } => {
                output.enter_node(kind);
            }
            Event::Finish => {
                output.exit_node();
            }
            Event::Token { kind, raw_token_count } => {
                output.append_token(kind, raw_token_count);
            }
            Event::Error { message } => {
                output.append_error(message);
            }
        }
    }

    output
}
