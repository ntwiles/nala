use nala_interpreter::io_context::TestContext;
use test_util::parse_and_run;

#[test]
fn it_allows_generic_enum_declare() {
    let mut ctx = TestContext::new();

    let nala = r#"
        enum Option<T> {
            Some(T),
            None,
        }
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), Vec::<String>::new());
}

#[test]
fn it_allows_generic_enum_variant_assign() {
    let mut ctx = TestContext::new();

    let nala = r#"
        enum Option<T> {
            Some(T),
            None,
        }

        const test = Option::Some(1);
        print(test);
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["Some(1)"]);
}
