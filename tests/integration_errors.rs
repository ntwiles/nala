use library::{
    io_context::TestContext,
    test_util::read_and_execute,
    util::{assert_regex_match, regex},
};

fn test_path(name: &str) -> String {
    format!("tests/nala/integration_errors/{}.nl", name)
}

#[test]
fn it_errors_when_indexing_array_with_string() {
    let expected_message = regex!("Cannot index using non-numeric value.");
    let result = read_and_execute(&test_path("array-index-string"), &mut TestContext::new());
    assert!(matches!(result.clone(), Err(_)));
    assert_regex_match!(expected_message, &result.clone().unwrap_err().message)
}

#[test]
fn it_errors_when_passing_number_arg_to_len() {
    let expected_message = regex!(
        "Passed value `7` of type `Number` to func `len` where `Array<Number>` was expected."
    );
    let result = read_and_execute(&test_path("array-len"), &mut TestContext::new());
    assert!(matches!(result.clone(), Err(_)));
    assert_regex_match!(expected_message, &result.clone().unwrap_err().message)
}

#[test]
fn it_errors_when_declaring_array_multiple_types() {
    let expected_message = regex!("Arrays can contain elements of only a single type.");
    let result = read_and_execute(&test_path("array-multiple-types"), &mut TestContext::new());
    assert!(matches!(result.clone(), Err(_)));
    assert_regex_match!(expected_message, &result.clone().unwrap_err().message)
}

#[test]
fn it_errors_when_assigning_wrong_type() {
    let expected_message = regex!("Cannot assign a value of type String where Number is expected.");
    let result = read_and_execute(&test_path("assign-types"), &mut TestContext::new());
    assert!(matches!(result.clone(), Err(_)));
    assert_regex_match!(expected_message, &result.clone().unwrap_err().message)
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
