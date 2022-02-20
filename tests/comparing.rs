use nala_interpreter::io_context::TestContext;
use test_util::parse_and_interpret;

#[test]
fn it_runs_bool_expression() {
    let mut test_context = TestContext::new();

    let nala = r#"
        const foo = 'hello';
        const bar = 'hello';
        print(foo == bar);
        print(1 == 7);
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["true", "false"]);
}

#[test]
fn it_runs_equals_for_bools() {
    let mut test_context = TestContext::new();

    let nala = r#"
        const result = true == false;
        print(result);
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["false"]);
}
