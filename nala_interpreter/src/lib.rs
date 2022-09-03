extern crate lalrpop_util;
extern crate regex;
extern crate reqwest;

use std::fs;

pub mod ast;
mod builtins;
pub mod errors;
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

    let code = match code {
        Ok(code) => code,
        Err(err) => {
            println!("Error loading nala file: {}", err);
            return;
        }
    };

    let parse_result = parse_code(code);

    if parse_result.is_err() {
        let message = parse_result.unwrap_err();
        println!(
            "{}",
            format!(
                "Nala Parse Error:\n  file:\n    {0} \n  message:\n    {1}",
                path, message
            )
        );

        return;
    }

    match interpret_tree(parse_result.unwrap(), &mut context) {
        Ok(_) => println!("Execution completed."),
        Err(e) => println!("Nala Runtime Error: {0}", e.message),
    }
}
