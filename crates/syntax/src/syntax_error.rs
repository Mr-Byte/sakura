use std::fmt;

use rowan::{TextRange, TextSize};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SyntaxError {
    message: String,
    text_range: TextRange,
}

impl SyntaxError {
    pub fn new(message: impl Into<String>, range: TextRange) -> Self {
        Self { message: message.into(), text_range: range }
    }

    pub fn new_at_offset(message: impl Into<String>, offset: TextSize) -> Self {
        Self { message: message.into(), text_range: TextRange::empty(offset) }
    }

    pub fn range(&self) -> TextRange {
        self.text_range
    }

    pub fn with_range(mut self, range: TextRange) -> Self {
        self.text_range = range;
        self
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.message.fmt(f)
    }
}
