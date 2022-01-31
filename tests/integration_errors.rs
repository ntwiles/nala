use library::{io_context::TestContext, test_util::read_and_execute};

#[test]
#[should_panic(expected = "Passed value `7` of type Number to func `len`")]
fn it_errors_when_passing_number_arg_to_len() {
    let file_name = "tests/nala/integration_errors/array-len.nl";
    let mut test_context = TestContext::new();
    read_and_execute(&file_name, &mut test_context);
}

#[test]
#[should_panic(expected = "Arrays can contain elements of only a single type.")]
fn it_errors_when_declaring_array_multiple_types() {
    let file_name = "tests/nala/integration_errors/array-multiple-types.nl";
    let mut test_context = TestContext::new();
    read_and_execute(&file_name, &mut test_context);
}

#[test]
#[should_panic(expected = "Cannot assign value of type String where Number is expected.")]
fn it_errors_when_assigning_wrong_type() {
    let file_name = "tests/nala/integration_errors/assign-types.nl";
    let mut test_context = TestContext::new();

    read_and_execute(&file_name, &mut test_context);
}

#[test]
#[should_panic(expected = "Cannot assign Void.")]
fn it_errors_when_assigning_type_void() {
    let file_name = "tests/nala/integration_errors/assign-void.nl";
    let mut test_context = TestContext::new();

    read_and_execute(&file_name, &mut test_context);
}

#[test]
#[should_panic(expected = "Type `Bool` does not support nesting. Type `Bool<String>` is invalid.")]
fn it_errors_when_passing_primitive_when_nested_is_expected() {
    let file_name = "tests/nala/integration_errors/nested-types.nl";
    let mut test_context = TestContext::new();

    read_and_execute(&file_name, &mut test_context);
}
