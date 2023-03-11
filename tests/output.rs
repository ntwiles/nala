use nala_interpreter::io_context::TestContext;
use test_util::parse_and_run;

#[test]
fn it_prints_strings_without_quotes() {
    let mut ctx = TestContext::new();

    let nala = r#"
        const test = 'should print';
        print(test);
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["should print"]);
}
