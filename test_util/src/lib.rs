extern crate nala_interpreter;
extern crate regex;

use nala_interpreter::{
    ast::terms::Value, errors::RuntimeError, interpreter::eval_tree, io_context::TestContext,
    parser,
};

pub fn parse_and_run(nala: &str, test_context: &mut TestContext) -> Result<Value, RuntimeError> {
    let lines = nala.trim_start().trim_end().split("\n");
    let output: String = lines
        .enumerate()
        .map(|(i, l)| format!("{i} | {l}"))
        .collect::<Vec<String>>()
        .join("\n");

    println!("\n{output}\n");

    let result = parser::parse_code(nala.to_owned());

    match result {
        Ok(parsed) => eval_tree(parsed, test_context),
        Err(_) => panic!("Could not parse nala!"),
    }
}

#[macro_export]
macro_rules! rgx {
    ($pattern:literal) => {
        Regex::new($pattern).unwrap()
    };
}

#[macro_export]
macro_rules! assert_regex_match {
    ($re:ident, $str:expr) => {
        if !$re.is_match($str) {
            panic!(
                "\nString does not match regex. \n  String: {0} \n  Regex: {1}",
                $str, $re,
            )
        }
    };
}
