extern crate lalrpop_util;
extern crate regex;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::fs;

pub mod ast;
mod builtin_types;
mod builtins;
pub mod errors;
pub mod interpreter;
pub mod io_context;
#[allow(dead_code)]
mod lexer;
pub mod parser;
pub mod resolved;
pub mod scopes;
pub mod types;
pub mod utils;

use interpreter::*;
use io_context::ConsoleContext;
use parser::*;

pub fn main(path: &str) -> () {
    let code = fs::read_to_string(path);
    let mut ctx = ConsoleContext {};

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
            format!("Nala Parse Error:\n  file:\n    {path} \n  message:\n    {message}")
        );

        return;
    }

    match eval_program(parse_result.unwrap(), &mut ctx) {
        Ok(_) => println!("Execution completed."),
        Err(e) => println!("Nala Runtime Error: {}", e.message),
    }
}
