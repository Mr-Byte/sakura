use token::Token;
use tokenizer::Tokenizer;

mod cursor;
mod token;
mod tokenizer;

pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut tokenizer = Tokenizer::new(input);

    std::iter::from_fn(move || tokenizer.next_token())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tokenizes_interpolated_string() {
        let input = r#""${test}""#;
        let _tokens = tokenize(input).collect::<Vec<_>>();
    }
}
