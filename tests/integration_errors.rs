use library::{io_context::TestContext, test_util::read_and_execute};

fn test_path(name: &str) -> String {
    format!("tests/nala/integration_errors/{}.nl", name)
}

#[test]
#[should_panic(expected = "Cannot index using non-numeric value.")]
fn it_errors_when_indexing_array_with_string() {
    let file_name = test_path("array-index-string");
    let mut test_context = TestContext::new();
    read_and_execute(&file_name, &mut test_context);
}

#[test]
#[should_panic(expected = "Passed value `7` of type Number to func `len`")]
fn it_errors_when_passing_number_arg_to_len() {
    let file_name = test_path("array-len");
    let mut test_context = TestContext::new();
    read_and_execute(&file_name, &mut test_context);
}

#[test]
#[should_panic(expected = "Arrays can contain elements of only a single type.")]
fn it_errors_when_declaring_array_multiple_types() {
    let file_name = test_path("array-multiple-types");
    let mut test_context = TestContext::new();
    read_and_execute(&file_name, &mut test_context);
}

#[test]
#[should_panic(expected = "Cannot assign a value of type String where Number is expected.")]
fn it_errors_when_assigning_wrong_type() {
    let file_name = test_path("assign-types");
    let mut test_context = TestContext::new();

    read_and_execute(&file_name, &mut test_context);
}

#[test]
#[should_panic(expected = "Cannot declare a variable with a value of type Void.")]
fn it_errors_when_assigning_type_void() {
    let file_name = test_path("assign-void");
    let mut test_context = TestContext::new();

    read_and_execute(&file_name, &mut test_context);
}

#[test]
#[should_panic(expected = "Type `Bool` does not support nesting. Type `Bool<String>` is invalid.")]
fn it_errors_when_passing_primitive_when_nested_is_expected() {
    let file_name = test_path("nested-types");
    let mut test_context = TestContext::new();

    read_and_execute(&file_name, &mut test_context);
}
