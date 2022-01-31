use library::{io_context::TestContext, test_util::read_and_execute};

fn test_path(name: &str) -> String {
    format!("tests/nala/integration/{}.nl", name)
}

#[test]
fn it_runs_array_empty() {
    let mut test_context = TestContext::new();
    read_and_execute(&test_path("array-empty"), &mut test_context);
    assert_eq!(test_context.get_output(), vec!["This should print."]);
}

#[test]
fn it_runs_array_for() {
    let mut test_context = TestContext::new();
    read_and_execute(&test_path("array-for"), &mut test_context);
    assert_eq!(test_context.get_output(), vec!["foo", "bar", "baz", "qux"]);
}

#[test]
fn it_runs_array_index_assign() {
    let mut test_context = TestContext::new();
    read_and_execute(&test_path("array-index-assign"), &mut test_context);
    assert_eq!(test_context.get_output(), vec!["7", "2"]);
}

#[test]
fn it_runs_array_index_expressions() {
    let mut test_context = TestContext::new();
    read_and_execute(&test_path("array-index-expressions"), &mut test_context);
    assert_eq!(test_context.get_output(), vec!["55"]);
}

#[test]
fn it_runs_array_index() {
    let mut test_context = TestContext::new();
    read_and_execute(&test_path("array-index"), &mut test_context);
    assert_eq!(test_context.get_output(), vec!["5"]);
}

#[test]
fn it_runs_array_len() {
    let mut test_context = TestContext::new();
    read_and_execute(&test_path("array-len"), &mut test_context);
    assert_eq!(test_context.get_output(), vec!["5"]);
}

#[test]
fn it_runs_array_slice() {
    let mut test_context = TestContext::new();
    read_and_execute(&test_path("array-slice"), &mut test_context);
    assert_eq!(test_context.get_output(), vec!["2", "3"]);
}

#[test]
fn it_runs_block_parent_scopes() {
    let mut test_context = TestContext::new();
    read_and_execute(&test_path("block-parent-scopes"), &mut test_context);
    assert_eq!(test_context.get_output(), vec!["7", "7"]);
}

#[test]
fn it_runs_block_shadowing() {
    let mut test_context = TestContext::new();
    read_and_execute(&test_path("block-shadowing"), &mut test_context);
    assert_eq!(test_context.get_output(), vec!["bar", "7"]);
}

#[test]
fn it_runs_bool_branching() {
    let mut test_context = TestContext::new();
    read_and_execute(&test_path("bool-branching"), &mut test_context);
    assert_eq!(test_context.get_output(), vec!["should print"]);
}

#[test]
fn it_runs_bool_expression() {
    let mut test_context = TestContext::new();
    read_and_execute(&test_path("bool-expression"), &mut test_context);
    assert_eq!(test_context.get_output(), vec!["true", "false"]);
}

#[test]
fn it_runs_break_for() {
    let mut test_context = TestContext::new();
    read_and_execute(&test_path("break-for"), &mut test_context);
    assert_eq!(test_context.get_output(), vec!["2"]);
}

#[test]
fn it_runs_break_wiles() {
    let mut test_context = TestContext::new();
    read_and_execute(&test_path("break-wiles"), &mut test_context);
    assert_eq!(test_context.get_output(), vec!["foo"]);
}

#[test]
fn it_runs_declare_and_multiply() {
    let mut test_context = TestContext::new();
    read_and_execute(&test_path("declare-and-multiply"), &mut test_context);
    assert_eq!(test_context.get_output(), vec!["28"]);
}

#[test]
fn it_runs_declare_basic() {
    let mut test_context = TestContext::new();
    read_and_execute(&test_path("declare-basic"), &mut test_context);
    assert_eq!(test_context.get_output(), vec!["28"]);
}

#[test]
fn it_runs_declare_mutable() {
    let mut test_context = TestContext::new();
    read_and_execute(&test_path("declare-mutable"), &mut test_context);
    assert_eq!(test_context.get_output(), vec!["7", "8"]);
}

#[test]
fn it_runs_enum_basic() {
    let mut test_context = TestContext::new();
    read_and_execute(&test_path("enum-basic"), &mut test_context);
    assert_eq!(test_context.get_output(), vec!["FooKind::Bar"]);
}

#[test]
fn it_runs_enum_compare() {
    let mut test_context = TestContext::new();
    read_and_execute(&test_path("enum-compare"), &mut test_context);
    assert_eq!(test_context.get_output(), vec!["should print"]);
}

#[test]
fn it_runs_enum_declare() {
    let mut test_context = TestContext::new();
    read_and_execute(&test_path("enum-declare"), &mut test_context);
    assert_eq!(test_context.get_output(), vec!["success"]);
}

#[test]
fn it_runs_func_args() {
    let mut test_context = TestContext::new();
    read_and_execute(&test_path("func-args"), &mut test_context);
    assert_eq!(test_context.get_output(), vec!["The total is 12"]);
}

#[test]
fn it_runs_func_basic() {
    let mut test_context = TestContext::new();
    read_and_execute(&test_path("func-basic"), &mut test_context);
    assert_eq!(test_context.get_output(), vec!["Functions work!"]);
}

#[test]
fn it_runs_func_expressions() {
    let mut test_context = TestContext::new();
    read_and_execute(&test_path("func-expressions"), &mut test_context);
    assert_eq!(test_context.get_output(), vec!["foobar"]);
}

#[test]
fn it_runs_func_first_class() {
    let mut test_context = TestContext::new();
    read_and_execute(&test_path("func-first-class"), &mut test_context);
    assert_eq!(test_context.get_output(), vec!["This should print."]);
}

#[test]
fn it_runs_func_return() {
    let mut test_context = TestContext::new();
    read_and_execute(&test_path("func-return"), &mut test_context);
    assert_eq!(test_context.get_output(), vec!["Function returns work!"]);
}

#[test]
fn it_runs_input_basic() {
    let input = vec!["Nathan"];
    let output = vec!["Please enter your name:", "Hello, Nathan"];

    let mut test_context = TestContext::new();
    test_context.mock_inputs(input);

    read_and_execute(&test_path("input-basic"), &mut test_context);
    assert_eq!(test_context.get_output(), output);
}

#[test]
fn it_runs_input_numbers() {
    let input = vec!["31"];
    let output = vec!["Please enter your age:", "Next year your age will be 32"];

    let mut test_context = TestContext::new();
    test_context.mock_inputs(input);

    read_and_execute(&test_path("input-numbers"), &mut test_context);
    assert_eq!(test_context.get_output(), output);
}

#[test]
fn it_runs_num_floor() {
    let mut test_context = TestContext::new();
    read_and_execute(&test_path("num-floor"), &mut test_context);
    assert_eq!(test_context.get_output(), vec!["6"]);
}

#[test]
fn it_runs_print_expression() {
    let mut test_context = TestContext::new();
    read_and_execute(&test_path("print-expression"), &mut test_context);
    assert_eq!(test_context.get_output(), vec!["7"]);
}

#[test]
fn it_runs_print_hello_world() {
    let mut test_context = TestContext::new();
    read_and_execute(&test_path("print-hello-world"), &mut test_context);
    assert_eq!(test_context.get_output(), vec!["hello world"]);
}

#[test]
fn it_runs_print_multiple() {
    let mut test_context = TestContext::new();
    read_and_execute(&test_path("print-multiple"), &mut test_context);
    assert_eq!(test_context.get_output(), vec!["hello world", "7"]);
}

#[test]
fn it_runs_print_number() {
    let mut test_context = TestContext::new();
    read_and_execute(&test_path("print-number"), &mut test_context);
    assert_eq!(test_context.get_output(), vec!["311"]);
}

#[test]
fn it_runs_print_string_concat_vars() {
    let mut test_context = TestContext::new();
    read_and_execute(&test_path("print-string-concat-vars"), &mut test_context);
    assert_eq!(test_context.get_output(), vec!["hello world"]);
}

#[test]
fn it_runs_print_string_concat() {
    let mut test_context = TestContext::new();
    read_and_execute(&test_path("print-string-concat"), &mut test_context);
    assert_eq!(test_context.get_output(), vec!["hello world"]);
}

#[test]
fn it_runs_string_special_chars() {
    let mut test_context = TestContext::new();
    read_and_execute(&test_path("string-special-chars"), &mut test_context);
    assert_eq!(test_context.get_output(), vec!["!@#$%^&*()_+-=;:\""]);
}

#[test]
fn it_runs_wiles_basic() {
    let mut test_context = TestContext::new();
    read_and_execute(&test_path("wiles-basic"), &mut test_context);
    assert_eq!(test_context.get_output(), vec!["h", "e", "l", "l"]);
}
