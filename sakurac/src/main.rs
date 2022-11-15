use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for line in io::stdin().lock().lines() {
        let line = line?;
        let tokens = sakura_lexer::tokenize(&line).collect::<Vec<_>>();

        println!("{:#?}", tokens);
    }

    Ok(())
}
