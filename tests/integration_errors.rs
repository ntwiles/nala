use nala_interpreter::io_context::TestContext;
use regex::Regex;
use test_util::{assert_regex_match, parse_and_interpret, rgx};

#[test]
fn it_errors_when_indexing_array_with_string() {
    let expected_message = rgx!("Cannot index using non-numeric value.");

    let nala = r#"
        const nums = [0, 1, 2, 3];
        print(nums['0']);
    "#;

    let result = parse_and_interpret(nala, &mut TestContext::new());

    assert!(result.is_err());
    assert_regex_match!(expected_message, &result.clone().unwrap_err().message)
}

#[test]
fn it_errors_when_passing_number_arg_to_len() {
    let expected_message =
        rgx!("Passed value `7` of type `Number` to func `len` where `Array<Number>` was expected.");

    let nala = r#"
        const num = 7;
        const length = len(num);
        print(length);
    "#;

    let result = parse_and_interpret(nala, &mut TestContext::new());

    assert!(result.is_err());
    assert_regex_match!(expected_message, &result.clone().unwrap_err().message)
}

#[test]
fn it_errors_when_declaring_array_multiple_types() {
    let expected_message = rgx!("Arrays can contain elements of only a single type.");

    let nala = "const bad = [0, '1'];";
    let result = parse_and_interpret(nala, &mut TestContext::new());

    assert!(result.is_err());
    assert_regex_match!(expected_message, &result.clone().unwrap_err().message)
}

#[test]
fn it_errors_when_assigning_wrong_type() {
    let expected_message = rgx!("Cannot assign a value of type String where Number is expected.");

    let nala = r#"
        mut num = 7;
        num = 'hello';
    "#;

    let result = parse_and_interpret(nala, &mut TestContext::new());

    assert!(result.is_err());
    assert_regex_match!(expected_message, &result.clone().unwrap_err().message)
}

#[test]
fn it_errors_when_assigning_type_void() {
    let expected_message = rgx!("Cannot declare a variable with a value of type Void.");

    let nala = r#"
        func returnVoid() {
            const void = 'void';
        }
        
        const void = returnVoid();
    "#;

    let result = parse_and_interpret(nala, &mut TestContext::new());

    assert!(result.is_err());
    assert_regex_match!(expected_message, &result.clone().unwrap_err().message)
}

#[test]
fn it_errors_when_passing_primitive_when_nested_is_expected() {
    let expected_message =
        rgx!("Type `Bool` does not support nesting. Type `Bool<String>` is invalid.");

    let nala = r#"
        func bad(arg: Bool<String>) {
            print('break');
        }
        
        bad(false);
    "#;

    let result = parse_and_interpret(nala, &mut TestContext::new());

    assert!(result.is_err());
    assert_regex_match!(expected_message, &result.clone().unwrap_err().message)
}
