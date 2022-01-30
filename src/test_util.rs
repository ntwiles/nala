use std::fs;

use crate::{interpreter::interpret_tree, io_context::TestContext, parser};

pub fn read_and_execute(path: &str, test_context: &mut TestContext) {
    let code = if let Ok(code) = fs::read_to_string(path) {
        code
    } else {
        panic!("Could not load nala file! {}", path);
    };
    let result = parser::parse_code(code);
    match result {
        Ok(parsed) => interpret_tree(parsed, test_context),
        Err(_) => panic!("Could not parse nala file! {}", path),
    }
}
