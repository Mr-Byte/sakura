use std::io::{self, BufRead};

use sakura_lexer::Lexer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for line in io::stdin().lock().lines() {
        let line = line?;
        let tokens = Lexer::tokenize(&line).collect::<Vec<_>>();

        println!("{:#?}", tokens);
    }

    Ok(())
}
