/// Implementation based on the [cursor module](https://github.com/rust-lang/rust/blob/master/compiler/rustc_lexer/src/cursor.rs) from the Rust compiler's lexer.
use std::str::Chars;

pub struct Cursor<'a> {
    initial_len: usize,
    chars: Chars<'a>,
}

pub const EOF: char = '\0';

impl Cursor<'_> {
    pub fn new(input: &str) -> Cursor<'_> {
        Cursor { initial_len: input.len(), chars: input.chars() }
    }

    pub fn bump(&mut self) -> Option<char> {
        self.chars.next()
    }

    pub fn bump_while(&mut self, predicate: impl Fn(char) -> bool) -> bool {
        let mut consumed_input = false;

        while predicate(self.first()) && !self.is_eof() {
            self.bump();
            consumed_input = true;
        }

        consumed_input
    }

    pub fn first(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF)
    }

    pub fn second(&self) -> char {
        self.chars.clone().nth(1).unwrap_or(EOF)
    }

    pub fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    pub fn consumed_len(&self) -> usize {
        self.initial_len - self.chars.as_str().len()
    }

    pub fn reset_len(&mut self) {
        self.initial_len = self.chars.as_str().len()
    }
}
