use nala_interpreter::io_context::TestContext;
use test_util::parse_and_run;

#[test]
fn it_runs_block_parent_scopes() {
    let mut ctx = TestContext::new();

    let nala = r#"  
        const foo = 7;

        if (true) {
            print(foo);
        }

        print(foo);
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["7", "7"]);
}

#[test]
fn it_runs_block_shadowing() {
    let mut ctx = TestContext::new();

    let nala = r#"
        const foo = 7;

        if (true) {
            const foo = 'bar';
            print(foo);
        }

        print(foo);
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["bar", "7"]);
}
