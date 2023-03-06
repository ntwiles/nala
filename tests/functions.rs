use nala_interpreter::io_context::TestContext;
use regex::Regex;
use test_util::{assert_regex_match, parse_and_run, rgx};

#[test]
fn it_runs_func_args() {
    let mut ctx = TestContext::new();

    let nala = r#"
        func add(a: Number, b: Number): Number {
            a + b;
        }
        
        const sum = add(5, 7);
        
        if (sum == 12) { print('works'); }
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["works"]);
}

#[test]
fn it_runs_func_basic() {
    let mut ctx = TestContext::new();

    let nala = r#"
        func printMessage(): Void {
            print('Functions work!');
        }
        
        printMessage();
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["Functions work!"]);
}

#[test]
fn it_runs_func_expressions() {
    let mut ctx = TestContext::new();

    let nala = r#"
        func foo(): String {
            'foo';
        }
        
        func bar(): String {
            'bar';
        }
        
        print(foo() + bar());
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["foobar"]);
}

#[test]
fn it_runs_func_first_class() {
    let mut ctx = TestContext::new();

    let nala = r#"
        func foo(message: String): Void {
            print(message);
        }
        
        const bar = foo;
        bar('This should print.');
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["This should print."]);
}

#[test]
fn it_runs_func_return() {
    let mut ctx = TestContext::new();

    let nala = r#"
        func getMessage(): String {
            'Function returns work!';
        }
        
        print(getMessage());
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["Function returns work!"]);
}

#[test]
fn it_errors_when_passing_primitive_when_nested_is_expected() {
    let expected_message =
        rgx!("Type `Bool` does not support type arguments. Type `Bool<String>` is invalid.");

    let nala = r#"
        func bad(arg: Bool<String>): Void {
            print('break');
        }
        
        bad(false);
    "#;

    let result = parse_and_run(nala, &mut TestContext::new());

    assert!(result.is_err());
    assert_regex_match!(expected_message, &result.clone().unwrap_err().message)
}

#[test]
fn it_prints_function_type_correctly() {
    let mut ctx = TestContext::new();

    let nala = r#"
        func addFoo(input: String): String {
            input + ' foo';
        }

        print(addFoo);
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["Func<String, String>"]);
}

#[test]
fn it_supports_closures_in_scope() {
    let mut ctx = TestContext::new();

    let nala = r#"
        func closureTest(): Void {
            const message = 'closures work!';
        
            func innerFunc(): Void {
                print(message);
            }
        
            innerFunc();
        }

        closureTest();
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["closures work!"]);
}

#[test]
fn it_supports_closures_out_of_scope() {
    let mut ctx = TestContext::new();

    let nala = r#"
        func closureTest(): Func<Void> {
            const message = 'closures REALLY work!';
        
            func innerFunc(): Void {
                print(message);
            }
        
            innerFunc;
        }

        const funcToCall = closureTest();
        funcToCall();
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["closures REALLY work!"]);
}

#[test]
fn it_accepts_return_of_correct_type() {
    let mut ctx = TestContext::new();

    let nala = r#"
        func returnString(): String {
            'this returns a string.';
        }

        print(returnString());
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["this returns a string."]);
}

#[test]
fn it_errors_on_return_of_wrong_type() {
    let mut ctx = TestContext::new();

    let nala = r#"
        func returnString(): Number {
            'this returns a string.';
        }

        print(returnString());
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_err());
}
