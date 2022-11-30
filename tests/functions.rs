use nala_interpreter::io_context::TestContext;
use regex::Regex;
use test_util::{assert_regex_match, parse_and_interpret, rgx};

#[test]
fn it_runs_func_args() {
    let mut test_context = TestContext::new();

    let nala = r#"
        func add(a: Number, b: Number): Number {
            a + b;
        }
        
        const sum = add(5, 7);
        
        if (sum == 12) { print('works'); }
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["works"]);
}

#[test]
fn it_runs_func_basic() {
    let mut test_context = TestContext::new();

    let nala = r#"
        func printMessage(): Void {
            print('Functions work!');
        }
        
        printMessage();
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["Functions work!"]);
}

#[test]
fn it_runs_func_expressions() {
    let mut test_context = TestContext::new();

    let nala = r#"
        func foo(): String {
            'foo';
        }
        
        func bar(): String {
            'bar';
        }
        
        print(foo() + bar());
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["foobar"]);
}

#[test]
fn it_runs_func_first_class() {
    let mut test_context = TestContext::new();

    let nala = r#"
        func foo(message: String): Void {
            print(message);
        }
        
        const bar = foo;
        bar('This should print.');
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["This should print."]);
}

#[test]
fn it_runs_func_return() {
    let mut test_context = TestContext::new();

    let nala = r#"
        func getMessage(): String {
            'Function returns work!';
        }
        
        print(getMessage());
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["Function returns work!"]);
}

#[test]
fn it_errors_when_passing_primitive_when_nested_is_expected() {
    let expected_message =
        rgx!("Type `Bool` does not support nesting. Type `Bool<String>` is invalid.");

    let nala = r#"
        func bad(arg: Bool<String>): Void {
            print('break');
        }
        
        bad(false);
    "#;

    let result = parse_and_interpret(nala, &mut TestContext::new());

    assert!(result.is_err());
    assert_regex_match!(expected_message, &result.clone().unwrap_err().message)
}

// #[test]
// fn it_prints_function_type_correctly() {
//     let mut test_context = TestContext::new();

//     let nala = r#"
//         func addFoo(input: String): String {
//             input + ' foo';
//         }

//         print(addFoo);
//     "#;

//     assert!(parse_and_interpret(nala, &mut test_context).is_ok());
//     assert_eq!(test_context.get_output(), vec!["Func<String, String>"]);
// }
