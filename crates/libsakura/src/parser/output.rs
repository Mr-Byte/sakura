use crate::syntax::SyntaxKind;

/// `OutputEvent` is packed to try and store as much information as possible in 32-bits while
/// still being easily represented as an enum. This representation does restrict `ErrorIndex` to
/// 16-bit, but that should be more than enough for the typical AST. If this is not enough, then
/// the ErrorIndex variant could be changed to include an additional u8 to store the index.
enum OutputEvent {
    Enter { kind: SyntaxKind },
    Token { kind: SyntaxKind, input_token_count: u8 },
    Exit,
    // NOTE: This does restrict the total number of errors in an AST to 16-bits
    ErrorIndex(u16),
}

// Assertion to ensure that OutputEvent stays 32-bits in length.
const __OUTPUT_EVENT_SIZE_ASSERT: () = assert!(std::mem::size_of::<OutputEvent>() == 4);

pub enum ParserOutputStep<'a> {
    Enter { kind: SyntaxKind },
    Token { kind: SyntaxKind, input_token_count: u8 },
    Exit,
    Error { message: &'a str },
}

#[derive(Default)]
pub struct ParserOutput {
    events: Vec<OutputEvent>,
    errors: Vec<String>,
}

impl ParserOutput {
    pub fn iter(&self) -> impl Iterator<Item = ParserOutputStep<'_>> {
        self.events.iter().map(|event| match event {
            OutputEvent::Enter { kind } => ParserOutputStep::Enter { kind: *kind },
            OutputEvent::Token { kind, input_token_count } => {
                ParserOutputStep::Token { kind: *kind, input_token_count: *input_token_count }
            }
            OutputEvent::Exit => ParserOutputStep::Exit,
            OutputEvent::ErrorIndex(index) => {
                let message = &self.errors[*index as usize];
                ParserOutputStep::Error { message }
            }
        })
    }

    pub(in crate::parser) fn append_token(&mut self, kind: SyntaxKind, input_token_count: u8) {
        self.events.push(OutputEvent::Token { kind, input_token_count });
    }

    pub(in crate::parser) fn enter_node(&mut self, kind: SyntaxKind) {
        self.events.push(OutputEvent::Enter { kind });
    }

    pub(in crate::parser) fn exit_node(&mut self) {
        self.events.push(OutputEvent::Exit);
    }

    pub(in crate::parser) fn append_error(&mut self, message: String) {
        let index = self.errors.len();

        self.errors.push(message);
        self.events.push(OutputEvent::ErrorIndex(index as u16));
    }
}
