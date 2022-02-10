// TODO: This is only used inside tests/ , see if theres a way to keep this over there.
use std::fs;

use crate::{
    ast::terms::Term, errors::NalaRuntimeError, interpreter::interpret_tree,
    io_context::TestContext, parser,
};

pub fn read_and_execute(
    path: &str,
    test_context: &mut TestContext,
) -> Result<Term, NalaRuntimeError> {
    let code = if let Ok(code) = fs::read_to_string(path) {
        code
    } else {
        panic!("Could not load nala file! {}", path);
    };

    let lines = code.trim_start().trim_end().split("\n");
    let output: String = lines
        .enumerate()
        .map(|(i, l)| format!("{i} | {l}"))
        .collect::<Vec<String>>()
        .join("\n");

    println!("\n{output}\n");

    let result = parser::parse_code(code);

    match result {
        Ok(parsed) => interpret_tree(parsed, test_context),
        Err(_) => panic!("Could not parse nala file! {}", path),
    }
}
