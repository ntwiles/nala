use std::error::Error;
use std::fs;

use library::{interpreter::interpret_tree, parser::parse_code, scope::Scope};

#[test]
fn test_run_all_examples() {
    let files = fs::read_dir("data").unwrap();

    for file in files {
        if let Ok(file) = file {
            let file_name = file.path().display().to_string();
            let result = assert_example_does_not_throw(&file_name);
        }
    }
}

fn assert_example_does_not_throw(path: &str) -> Result<(), ()> {
    let code = fs::read_to_string(path).unwrap();
    let parsed = parse_code(code);
    interpret_tree(parsed, &mut Scope::new(None));
    Ok(())
}
