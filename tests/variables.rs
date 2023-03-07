use nala_interpreter::io_context::TestContext;
use regex::Regex;
use test_util::{assert_regex_match, parse_and_run, rgx};

#[test]
fn it_runs_declare_and_multiply() {
    let mut test_context = TestContext::new();

    let nala = r#"
        const foo = 4;
        print(7 * foo);
    "#;

    assert!(parse_and_run(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["28"]);
}

#[test]
fn it_runs_declare_basic() {
    let mut test_context = TestContext::new();

    let nala = r#"
        const foo = 4 * 7;
        print(foo);
    "#;

    assert!(parse_and_run(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["28"]);
}

#[test]
fn it_runs_declare_mutable() {
    let mut test_context = TestContext::new();

    let nala = r#"
        mut mutable = 7;
        print(mutable);
        mutable = 8;
        print(mutable);
    "#;

    assert!(parse_and_run(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["7", "8"]);
}

#[test]
fn it_runs_string_special_chars() {
    let mut test_context = TestContext::new();

    let nala = r#"
        print('!@#$%^&*()_+-=;:"');
    "#;

    assert!(parse_and_run(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["!@#$%^&*()_+-=;:\""]);
}

#[test]
fn it_errors_when_assigning_wrong_type() {
    let expected_message = rgx!("Cannot assign a value of type String where Number is expected.");

    let nala = r#"
        mut num = 7;
        num = 'hello';
    "#;

    let result = parse_and_run(nala, &mut TestContext::new());

    assert!(result.is_err());
    assert_regex_match!(expected_message, &result.clone().unwrap_err().message)
}

#[test]
fn it_errors_when_assigning_type_void() {
    let expected_message = rgx!("Cannot declare a variable with a value of type Void.");

    let nala = r#"
        func returnVoid(): Void {
            const void = 'void';
        }
        
        const void = returnVoid();
    "#;

    let result = parse_and_run(nala, &mut TestContext::new());

    assert!(result.is_err());
    assert_regex_match!(expected_message, &result.clone().unwrap_err().message)
}

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
    // TODO: Shouldn't this print "Foo<Number>::Bar(7)"?
    assert_eq!(test_context.get_output(), vec!["Foo::Bar(7)"]);
}

#[test]
fn it_handles_declare_with_unfit_value() {
    let expected_message = rgx!("Tried to declare variable `foo` with explicit type `String` but value does not fit that type.");

    let nala = r#"
        const foo: String = 7;
    "#;

    let result = parse_and_run(nala, &mut TestContext::new());

    assert!(result.is_err());
    assert_regex_match!(expected_message, &result.clone().unwrap_err().message)
}

// TODO: This is failing on .is_err() because we're not strict enough in checking type fit.
// #[test]
// fn it_handles_declare_with_unfit_value_for_generic() {
//     let expected_message = rgx!("Tried to declare variable `foo` with explicit type `String` but value does not fit that type.");

//     let nala = r#"
//         enum Foo<T> {
//             Bar(T),
//             Baz,
//         }

//         const foo: Foo<String> = Foo::Bar(7);
//     "#;

//     let result = parse_and_run(nala, &mut TestContext::new());

//     assert!(result.is_err());
//     assert_regex_match!(expected_message, &result.clone().unwrap_err().message)
// }
