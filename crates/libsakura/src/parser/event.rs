use crate::parser::output::ParserOutput;
use crate::syntax::SyntaxKind;

pub enum Event {
    Start { kind: SyntaxKind, forward_parent: Option<usize> },
    Finish,
    Token { kind: SyntaxKind, raw_token_count: u8 },
    Error { message: String },
}

impl Event {
    pub(in crate::parser) fn tombstone() -> Self {
        Event::Start { kind: SyntaxKind::TOMBSTONE, forward_parent: None }
    }
}

pub(in crate::parser) fn process(mut events: Vec<Event>) -> ParserOutput {
    let mut output = ParserOutput::default();
    let mut forward_parents = Vec::new();

    for index in 0..events.len() {
        let event = std::mem::replace(&mut events[index], Event::tombstone());

        match event {
            Event::Start { kind, forward_parent } => {
                forward_parents.push(kind);

                let mut current_index = index;
                let mut current_forward_parent = forward_parent;

                while let Some(forward_parent) = current_forward_parent {
                    current_index += forward_parent;

                    let forward_parent =
                        std::mem::replace(&mut events[current_index], Event::tombstone());
                    let Event::Start { kind, forward_parent } = forward_parent else {
                        unreachable!();
                    };

                    forward_parents.push(kind);
                    current_forward_parent = forward_parent;
                }

                for kind in forward_parents.drain(..).rev() {
                    output.enter_node(kind);
                }
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
