use library::{io_context::TestContext, test_util::read_and_execute};

fn test_path(name: &str) -> String {
    format!("tests/nala/integration_errors/{}.nl", name)
}

#[test]
#[should_panic(expected = "Cannot index using non-numeric value.")]
fn it_errors_when_indexing_array_with_string() {
    read_and_execute(&test_path("array-index-string"), &mut TestContext::new());
}

#[test]
#[should_panic(expected = "Wrong value type passed to builtin func len")]
fn it_errors_when_passing_number_arg_to_len() {
    read_and_execute(&test_path("array-len"), &mut TestContext::new());
}

#[test]
#[should_panic(expected = "Arrays can contain elements of only a single type.")]
fn it_errors_when_declaring_array_multiple_types() {
    read_and_execute(&test_path("array-multiple-types"), &mut TestContext::new());
}

#[test]
#[should_panic(expected = "Cannot assign a value of type String where Number is expected.")]
fn it_errors_when_assigning_wrong_type() {
    read_and_execute(&test_path("assign-types"), &mut TestContext::new());
}

#[test]
#[should_panic(expected = "Cannot declare a variable with a value of type Void.")]
fn it_errors_when_assigning_type_void() {
    read_and_execute(&test_path("assign-void"), &mut TestContext::new());
}

#[test]
#[should_panic(expected = "Type `Bool` does not support nesting. Type `Bool<String>` is invalid.")]
fn it_errors_when_passing_primitive_when_nested_is_expected() {
    read_and_execute(&test_path("nested-types"), &mut TestContext::new());
}
