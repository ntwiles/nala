use nala_interpreter::io_context::TestContext;
use regex::Regex;
use test_util::{assert_regex_match, parse_and_run, rgx};

#[test]
fn it_infers_type_of_number() {
    let nala = r#"
        const foo = 1;
    "#;

    let result = parse_and_run(nala, &mut TestContext::new());

    assert!(result.is_ok());
}

#[test]
fn it_infers_type_of_generic_if_possible() {
    let nala = r#"
        const foo = Option::Some(1);
    "#;

    let result = parse_and_run(nala, &mut TestContext::new());

    assert!(result.is_ok());
}

#[test]
fn it_errors_on_empty_array() {
    let expected_message = "Cannot infer type of an empty array.";

    let nala = r#"
        const empty = [];
    "#;

    let result = parse_and_run(nala, &mut TestContext::new());

    assert!(result.is_err());
    assert_eq!(expected_message, &result.clone().unwrap_err().message)
}

#[test]
fn it_errors_if_no_info_for_inference() {
    let expected_message = rgx!("Not enough information to infer type of generic enum variant.");

    let nala = r#"
        const foo = Option::None;
    "#;

    let result = parse_and_run(nala, &mut TestContext::new());
    assert!(result.is_err());
    assert_regex_match!(expected_message, &result.clone().unwrap_err().message)
}

#[test]
fn it_errors_if_not_enough_info_for_inference() {
    let expected_message = rgx!("Can't assign value of type `What<T>` because it's generic. Try declaring the type explicitly.");

    let nala = r#"
        enum What<T> {
            This(T),
            That(Number),
            TheOther,
        }

        const foo = What::That(7);
    "#;

    let result = parse_and_run(nala, &mut TestContext::new());
    assert!(result.is_err());
    assert_regex_match!(expected_message, &result.clone().unwrap_err().message)
}

// TODO: Create test for the below case; passing MyEnum::Bar(7) should throw an error as there is not
// enough information for type inference, but it doesn't.

// enum MyEnum<T> {
//     Foo(T),
//     Bar(Number),
//     Baz
// }

// func foo<T>(param: MyEnum<T> ): Void {
//     print(MyEnum::Foo(param));
// }

// foo(MyEnum::Bar(7));
