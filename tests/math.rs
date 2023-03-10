use nala_interpreter::io_context::TestContext;
use test_util::parse_and_run;

#[test]
fn it_runs_num_floor() {
    let mut ctx = TestContext::new();

    let nala = r#"
        const ratio = 6.7;
        print(floor(ratio));
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["6"]);
}

// TODO: Cover operator not implemented errors.
