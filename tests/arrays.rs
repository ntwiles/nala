use nala_interpreter::io_context::TestContext;
use regex::Regex;
use test_util::{assert_regex_match, parse_and_interpret, rgx};

#[test]
fn it_runs_array_index_assign() {
    let mut test_context = TestContext::new();

    let nala = r#"
        mut array = ['one', 'two', 'three', 'four', 'five'];
        print(array[2]);
        array[2] = 'foo';
        print(array[2]);
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["three", "foo"]);
}

#[test]
fn it_runs_array_index_expressions() {
    let mut test_context = TestContext::new();

    let nala = r#"
        const foo = ['hello', 'goodbye'];
        const bar = ['world', 'domination'];
        print(foo[0] + ' ' + bar[0]);
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["hello world"]);
}

#[test]
fn it_runs_array_index() {
    let mut test_context = TestContext::new();

    let nala = r#"
        const array = ['foo', 'bar'];
        print(array[1]);
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["bar"]);
}

#[test]
fn it_runs_array_len() {
    let mut test_context = TestContext::new();

    let nala = r#"
        const array = [0, 1, 2, 3, 4];
        const length = len(array);
        print(length);
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["5"]);
}

#[test]
fn it_runs_array_slice() {
    let mut test_context = TestContext::new();

    let nala = r#"
        const array = [ 'what', 'will', 'this', 'thing', 'print'];

        const left = slice(array, 0, 3);
        const right = slice(array, 3, len(array));

        print(left[2]);
        print(right[0]);
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["this", "thing"]);
}

#[test]
fn it_allows_assign_to_index_place_expression() {
    let mut test_context = TestContext::new();

    let nala = r#"
        const letters = [ 'h', 'e', 'l', 'l', 'o'];
        letters[0] = 'j';
        print(letters[0]);
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["j"]);
}

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

// #[test]
// fn it_errors_when_passing_number_arg_to_len() {
//     let expected_message =
//         rgx!("Passed value `7` of type `Number` to function where `Array<Number>` was expected.");

//     let nala = r#"
//         const num = 7;
//         const length = len(num);
//         print(length);
//     "#;

//     let result = parse_and_interpret(nala, &mut TestContext::new());

//     assert!(result.is_err());
//     assert_regex_match!(expected_message, &result.clone().unwrap_err().message)
// }

#[test]
fn it_errors_when_declaring_array_multiple_types() {
    let expected_message = rgx!("Arrays can contain elements of only a single type.");

    let nala = "const bad = [0, '1'];";
    let result = parse_and_interpret(nala, &mut TestContext::new());

    assert!(result.is_err());
    assert_regex_match!(expected_message, &result.clone().unwrap_err().message)
}
