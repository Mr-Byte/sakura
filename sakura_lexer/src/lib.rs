use token::Token;
use tokenizer::Tokenizer;

mod cursor;
mod token;
mod tokenizer;

pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut tokenizer = Tokenizer::new(input);

    std::iter::from_fn(move || tokenizer.next_token())
}
