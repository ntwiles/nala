use nala_interpreter::io_context::TestContext;
use test_util::parse_and_run;

#[test]
fn it_allows_generic_enum_declare() {
    let mut ctx = TestContext::new();

    let nala = r#"
        enum Variants<T> {
            Foo(T),
            Bar,
        }
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), Vec::<String>::new());
}

#[test]
fn it_allows_generic_enum_variant_assign() {
    let mut ctx = TestContext::new();

    let nala = r#"
        const test = Option::Some(1);
        print(test);
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["Some(1)"]);
}

#[test]
fn it_allows_generic_func_declare() {
    let mut ctx = TestContext::new();

    let nala = r#"
        func foo<T>(value: T): Void {
            print(value);
        }

        print('test');
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["test"]);
}

#[test]
fn it_errors_when_returning_wrong_type_for_resolved_generic() {
    let expected_message =
        "Tried to return value `8` of type `Number` where value of type `String` was expected.";

    let nala = r#"
        func foo<T>( arg: T ): T {
            8;
        }
        
        const found = foo('test');
        print(found);
    "#;

    let result = parse_and_run(nala, &mut TestContext::new());

    assert!(result.is_err());
    assert_eq!(expected_message, &result.clone().unwrap_err().message);
}
