use std::error::Error;
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

pub fn main(path: &str) -> Result<(), Box<dyn Error>> {
    let result = fs::read_to_string(path);

    match result {
        Ok(code) => {
            let parsed = parse_code(code);

            let mut context = ConsoleContext {};
            interpret_tree(parsed, &mut context);
        
            Ok(())
        }
        Err(err) => Err(Box::new(err))
    }
}