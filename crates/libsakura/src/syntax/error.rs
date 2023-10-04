use rowan::{TextRange, TextSize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxError {
    pub message: String,
    pub text_range: TextRange,
}

impl SyntaxError {
    pub(crate) fn new(message: impl Into<String>, text_range: TextRange) -> Self {
        Self { message: message.into(), text_range }
    }

    pub(crate) fn new_at_offset(message: impl Into<String>, offset: TextSize) -> Self {
        Self { message: message.into(), text_range: TextRange::empty(offset) }
    }
}
