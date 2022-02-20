use nala_interpreter::io_context::TestContext;
use test_util::parse_and_interpret;

#[test]
fn it_runs_block_parent_scopes() {
    let mut test_context = TestContext::new();

    let nala = r#"  
        const foo = 7;

        if (true) {
            print(foo);
        }

        print(foo);
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["7", "7"]);
}

#[test]
fn it_runs_block_shadowing() {
    let mut test_context = TestContext::new();

    let nala = r#"
        const foo = 7;

        if (true) {
            const foo = 'bar';
            print(foo);
        }

        print(foo);
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["bar", "7"]);
}
