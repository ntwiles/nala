use nala_interpreter::io_context::TestContext;
use test_util::parse_and_run;

#[test]
fn it_matches_correct_enum_variants() {
    let mut ctx = TestContext::new();

    let nala = r#"
        enum Foo {
            Bar,
            Baz,
        }

        const foo = Foo::Baz;

        match (foo) {
            Foo::Bar => { print('foo is bar'); }
            Foo::Baz => { print('foo is baz'); }
        }
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["foo is baz"]);
}
