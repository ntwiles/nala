use library::{interpreter::interpret_tree, io_context::TestContext, parser};

use std::fs;

#[test]
fn test_run_examples() {
    let test_data = [
        ("array-for", vec!["foo", "7", "bar", "3"]),
        ("array-empty", vec!["This should print."]),
        ("array-index", vec!["5"]),
        ("array-index-expressions", vec!["55"]),
        ("block-parent-scopes", vec!["7", "7"]),
        ("block-shadowing", vec!["bar", "7"]),
        ("bool-branching", vec!["should print"]),
        ("bool-expression", vec!["true", "false"]),
        ("declare-and-multiply", vec!["28"]),
        ("declare-basic", vec!["28"]),
        ("declare-mutable", vec!["7", "8"]),
        ("func-basic", vec!["Functions work!"]),
        ("func-expressions", vec!["foobar"]),
        ("func-return", vec!["Function returns work!"]),
        ("print-expression", vec!["7"]),
        ("print-hello-world", vec!["hello world"]),
        ("print-multiple", vec!["hello world", "7"]),
        ("print-number", vec!["311"]),
        ("print-string-concat-vars", vec!["hello world"]),
        ("print-string-concat", vec!["hello world"]),
        ("string-special-chars", vec!["!@#$%^&*()_+-=;:\""]),
    ];

    for (file, expected) in test_data {
        let file_name = format!("example/{}.nl", file);
        let mut test_context = TestContext::new();
        assert_example_does_not_throw(&file_name, &mut test_context);
        assert_eq!(test_context.get_output(), &expected, "{}", file_name);
    }
}

#[test]
fn test_run_input_examples() {
    let test_data = [
        ("input-basic",
        vec!["Nathan"],
        vec!["Please enter your name:", "Hello, Nathan"]),
        ("input-numbers",
        vec!["31"],
        vec!["Please enter your age:", "Next year your age will be 32"])
    ];

    for (file, inputs, expected) in test_data {
        let file_name = format!("example/{}.nl", file);

        let mut test_context = TestContext::new();
        test_context.mock_inputs(inputs);

        assert_example_does_not_throw(&file_name, &mut test_context);
        assert_eq!(test_context.get_output(), &expected, "{}", file_name);
    }
}

fn assert_example_does_not_throw(path: &str, test_context: &mut TestContext) {
    let code = fs::read_to_string(path).unwrap();
    let result = parser::parse_code(code);

    if let Some(parsed) = result {
        interpret_tree(parsed, test_context);
    }
}