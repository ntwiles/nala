use nala_interpreter::io_context::TestContext;
use test_util::parse_and_interpret;

#[test]
fn it_runs_func_args() {
    let mut test_context = TestContext::new();

    let nala = r#"
        func add(a: Number, b: Number) {
            a + b;
        }
        
        print(add(5, 7));
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["12"]);
}

#[test]
fn it_runs_func_basic() {
    let mut test_context = TestContext::new();

    let nala = r#"
        func printMessage() {
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
        func foo() {
            'foo';
        }
        
        func bar() {
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
        func foo(message: String) {
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
        func getMessage() {
            'Function returns work!';
        }
        
        print(getMessage());
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["Function returns work!"]);
}
