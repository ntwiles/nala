use regex::Regex;

use nala_interpreter::io_context::TestContext;
use test_util::{assert_regex_match, parse_and_run, rgx};

#[test]
fn it_prints_strings_without_quotes() {
    let mut ctx = TestContext::new();

    let nala = r#"
        const test = 'should print';
        print(test);
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["should print"]);
}

#[test]
fn it_prints_strings_with_quotes_in_errors() {
    let expected_error = rgx!(
        "Passed value `'should print'` of type `String` to function where `Array<T>` was expected"
    );

    let nala = r#"
        const test = 'should print';
        len(test);
    "#;

    let result = parse_and_run(nala, &mut TestContext::new());

    assert!(result.is_err());
    assert_regex_match!(expected_error, &result.clone().unwrap_err().message)
}

#[test]
fn it_prints_strings_in_enum_data_with_quotes_in_errors() {
    let expected_error = "Passed value `Some('test')` of type `Option<String>` to function where `Option<Number>` was expected.";

    let nala = r#"
        func foo(param: Option<Number> ): Option<Number> {
            param;
        }
        
        foo(Option::Some('test'));
    "#;

    let result = parse_and_run(nala, &mut TestContext::new());

    assert!(result.is_err());
    assert_eq!(expected_error, &result.clone().unwrap_err().message);
}

#[test]
fn it_prints_object_entries_in_alphabetical_order() {
    let mut ctx = TestContext::new();

    let nala = r#"
        const array = [
            { b: 'bar', a: 'foo', c: 'fred' },
            { b: 'baz', c: 'thud', a: 'qux' }
        ];

        print(array);
    "#;

    let result = parse_and_run(nala, &mut ctx);

    let expected_output =
        "[{ a: 'foo', b: 'bar', c: 'fred', }, { a: 'qux', b: 'baz', c: 'thud', }, ]";

    assert!(result.is_ok());
    assert_eq!(ctx.get_output(), vec![expected_output]);
}

#[test]
fn it_prints_struct_entries_in_alphabetical_order() {
    let expected_error =
        "Arrays can contain elements of only a single type. Found elements of types `{ a: String, b: String, c: String, }` and `{ a: Number, b: String, c: String, }`.";

    let nala = r#"
        const array = [
            { b: 'bar', a: 'foo', c: 'fred' },
            { b: 'baz', c: 'thud', a: 7 }
        ];

        print(array);
    "#;

    let result = parse_and_run(nala, &mut TestContext::new());

    assert!(result.is_err());
    assert_eq!(expected_error, &result.clone().unwrap_err().message);
}
