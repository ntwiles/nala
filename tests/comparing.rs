use nala_interpreter::io_context::TestContext;
use test_util::parse_and_interpret;

#[test]
fn it_runs_bool_expression() {
    let mut ctx = TestContext::new();

    let nala = r#"
        const foo = 'hello';
        const bar = 'hello';
        print(foo == bar);
    "#;

    assert!(parse_and_interpret(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["true"]);
}

#[test]
fn it_runs_equals_for_bools() {
    let mut ctx = TestContext::new();

    let nala = r#"
        const result = true == false;
        print(result);

    "#;

    assert!(parse_and_interpret(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["false"]);
}
