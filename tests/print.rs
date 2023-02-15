use nala_interpreter::io_context::TestContext;
use test_util::parse_and_interpret;

#[test]
fn it_runs_print_expression() {
    let mut ctx = TestContext::new();

    let nala = "print(5 + 10 * 2 / 4 - 3);";

    assert!(parse_and_interpret(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["7"]);
}

#[test]
fn it_runs_print_hello_world() {
    let mut ctx = TestContext::new();

    let nala = "print('hello world');";

    assert!(parse_and_interpret(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["hello world"]);
}

#[test]
fn it_runs_print_multiple() {
    let mut ctx = TestContext::new();

    let nala = r#"
        print('hello world'); 
        print(10 * 2 / 4 + 5 - 3);
    "#;

    assert!(parse_and_interpret(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["hello world", "7"]);
}

#[test]
fn it_runs_print_number() {
    let mut ctx = TestContext::new();

    let nala = "print(311);";

    assert!(parse_and_interpret(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["311"]);
}

#[test]
fn it_runs_print_string_concat_vars() {
    let mut ctx = TestContext::new();

    let nala = r#"
        const foo = 'hello ';
        const bar = 'world';
        print(foo + bar);
    "#;

    assert!(parse_and_interpret(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["hello world"]);
}

#[test]
fn it_runs_print_string_concat() {
    let mut ctx = TestContext::new();

    let nala = "print('hello ' + 'world');";

    assert!(parse_and_interpret(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["hello world"]);
}
