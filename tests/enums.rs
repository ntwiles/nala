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
    assert_eq!(ctx.get_output(), vec!["True"]);
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
        const test = Option::Some('hello');
        print(test);
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["Some('hello')"]);
}

// TODO: This is failing right now with a parse error.
// #[test]
// fn it_runs_enum_variant_with_explicit_builtin_type_no_data() {
//     let mut ctx = TestContext::new();

//     let nala = r#"
//         const isTrue: Boolean = Boolean::True;
//         print(isTrue);
//     "#;

//     assert!(parse_and_run(nala, &mut ctx).is_ok());
//     assert_eq!(ctx.get_output(), vec!["true"]);
// }

#[test]
fn it_runs_enum_variant_with_explicit_custom_type_no_data() {
    let mut ctx = TestContext::new();

    let nala = r#"
        enum Foo {
            Bar,
            Baz
        }

        const test: Foo = Foo::Bar;
        print(test);
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["Bar"]);
}
