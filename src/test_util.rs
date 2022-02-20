// TODO: This is only used inside tests/ , see if theres a way to keep this over there.

use crate::{
    ast::terms::Term, errors::NalaRuntimeError, interpreter::interpret_tree,
    io_context::TestContext, parser,
};

pub fn parse_and_interpret(
    nala: &str,
    test_context: &mut TestContext,
) -> Result<Term, NalaRuntimeError> {
    let lines = nala.trim_start().trim_end().split("\n");
    let output: String = lines
        .enumerate()
        .map(|(i, l)| format!("{i} | {l}"))
        .collect::<Vec<String>>()
        .join("\n");

    println!("\n{output}\n");

    let result = parser::parse_code(nala.to_owned());

    match result {
        Ok(parsed) => interpret_tree(parsed, test_context),
        Err(_) => panic!("Could not parse nala!"),
    }
}
