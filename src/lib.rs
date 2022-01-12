use std::fs;

mod ast;
pub mod interpreter;
pub mod io_context;
#[allow(dead_code)]
mod lexer;
pub mod parser;
pub mod scope;

use interpreter::*;
use io_context::{ConsoleContext, IoContext} ;
use parser::*;

pub fn main(path: &str) -> () {
    let result = fs::read_to_string(path);
    let mut context = ConsoleContext {};

    match result {
        Ok(code) => {
            let parsed = parse_code(code);
            interpret_tree(parsed, &mut context);
        }
        Err(err) => {
            context.print(&format!("Error loading nala file: {}", err))
        }
    }
}