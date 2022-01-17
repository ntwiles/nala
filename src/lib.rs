use std::fs;

mod ast;
pub mod interpreter;
pub mod io_context;
#[allow(dead_code)]
mod lexer;
pub mod parser;
pub mod scope;

use interpreter::*;
use io_context::ConsoleContext;

use parser::*;

pub fn main(path: &str) -> () {
    let code = fs::read_to_string(path);
    let mut context = ConsoleContext {};

    let result = match code {
        Ok(code) => parse_code(code),
        Err(err) => {
            println!("Error loading nala file: {}", err);
            return;
        },
    };

    if let Some(parsed) = result {
        interpret_tree(parsed, &mut context);
    } else {
        println!("{}", format!("Error parsing nala file: {}", path))
    }
}