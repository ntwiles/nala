use nala_interpreter::io_context::TestContext;
use test_util::parse_and_run;

#[test]
fn it_runs_basic_struct_declare() {
    let mut ctx = TestContext::new();

    let nala = r#"
        struct Foo {
            bar: String,
            baz: Number,
        }

        const foo: Foo = {
            bar: 'bar',
            baz: 7
        };

        print(foo.bar);
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["bar"]);
}

#[test]
fn it_runs_nested_struct_declare() {
    let mut ctx = TestContext::new();

    let nala = r#"
        struct Foo {
            bar: String,
            nested: {
                baz: Number,
            }
        }

        const foo: Foo = {
            bar: 'bar',
            nested: {
                baz: 7
            }
        };

        print(foo.nested.baz);
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["7"]);
}
