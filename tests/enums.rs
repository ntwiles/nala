use nala_interpreter::io_context::TestContext;
use test_util::parse_and_run;

#[test]
fn it_runs_enum_declare_flat() {
    let mut ctx = TestContext::new();

    let nala = r#"
        enum Foo {
            Bar,
            Baz,
        }
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
}

#[test]
fn it_runs_enum_declare_composite() {
    let mut ctx = TestContext::new();

    let nala = r#"
        enum Foo {
            Bar(Number),
            Baz(String),
        }
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
}

#[test]
fn it_runs_enum_variant_assign() {
    let mut ctx = TestContext::new();

    let nala = r#"
        enum Boolean {
            True,
            False,
        }

        const test = Boolean::True;
        print(test);
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["Boolean::True"]);
}

#[test]
fn it_runs_enum_variant_compare() {
    let mut ctx = TestContext::new();

    let nala = r#"
        enum Boolean {
            True,
            False,
        }

        const test = Boolean::True;
        print(test == Boolean::True);
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["true"]);
}

#[test]
fn it_runs_enum_variant_with_data() {
    let mut ctx = TestContext::new();

    let nala = r#"
        enum Option {
            Some(String),
            None,
        }

        const test = Option::Some('hello');
        print(test);
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["Option::Some(hello)"]);
}
