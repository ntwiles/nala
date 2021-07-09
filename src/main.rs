use std::env;
use std::error::Error;
use std::fs;

mod lexer;
mod token;

use lexer::Lexer;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];
    let code = fs::read_to_string(path).unwrap();

    println!("Lexing code: {}", code);

    let tokens = Lexer::lex_code(code);

    println!("{:?}", tokens);

    Ok(())
}
