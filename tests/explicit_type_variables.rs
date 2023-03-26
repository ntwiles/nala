use nala_interpreter::io_context::TestContext;
use regex::Regex;
use test_util::{assert_regex_match, parse_and_run, rgx};

#[test]
fn it_handles_declare_with_explicit_type() {
    let mut test_context = TestContext::new();

    let nala = r#"
        func printNumber(num: Number): Void {
            print(num);
        }

        const foo: Number = 7;
        printNumber(foo);
    "#;

    let result = parse_and_run(nala, &mut test_context);

    assert!(result.is_ok());
    assert_eq!(test_context.get_output(), vec!["7"]);
}

#[test]
fn it_handles_declare_with_explicit_generic_type() {
    let mut test_context = TestContext::new();

    let nala = r#"
        enum Foo<T> {
            Bar(T),
            Baz,
        }

        func printFoo(foo: Foo<Number>): Void {
            print(foo);
        }

        const foo: Foo<Number> = Foo::Bar(7);
        printFoo(foo);
    "#;

    let result = parse_and_run(nala, &mut test_context);

    assert!(result.is_ok());
    assert_eq!(test_context.get_output(), vec!["Bar(7)"]);
}

#[test]
fn it_handles_declare_with_unfit_value() {
    let expected_message = rgx!("Tried to declare variable `foo` with explicit type `String` but value `7` does not fit that type.");

    let nala = r#"
        const foo: String = 7;
    "#;

    let result = parse_and_run(nala, &mut TestContext::new());

    assert!(result.is_err());
    assert_regex_match!(expected_message, &result.clone().unwrap_err().message)
}

#[test]

fn it_errors_when_given_wrong_array_type() {
    let expected_message = "Tried to declare variable `foo` with explicit type `Array<String>` but value `[7, ]` does not fit that type.";

    let nala = r#"
        const foo: Array<String> = [7];
    "#;

    let result = parse_and_run(nala, &mut TestContext::new());

    assert!(result.is_err());
    assert_eq!(expected_message, &result.clone().unwrap_err().message)
}

#[test]
fn it_handles_declare_with_unfit_value_for_generic_enum() {
    let expected_message = "Tried to declare variable `foo` with explicit type `Foo<String>` but value `Bar(7)` does not fit that type.";

    let nala = r#"
        enum Foo<T> {
            Bar(T),
            Baz,
        }

        const foo: Foo<String> = Foo::Bar(7);
    "#;

    let result = parse_and_run(nala, &mut TestContext::new());

    assert!(result.is_err());
    assert_eq!(expected_message, &result.clone().unwrap_err().message)
}

#[test]
fn it_handles_declare_with_unfit_value_for_generic_struct() {
    let expected_message = "Tried to declare variable `foo` with explicit type `Foo<String>` but value `{ value: 7,  }` does not fit that type.";

    let nala = r#"
        struct Foo<T> {
            value: T,
        }

        const foo: Foo<String> = { value: 7 };
    "#;

    let result = parse_and_run(nala, &mut TestContext::new());

    assert!(result.is_err());
    assert_eq!(expected_message, &result.clone().unwrap_err().message)
}
