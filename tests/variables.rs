use nala_interpreter::io_context::TestContext;
use test_util::parse_and_interpret;

#[test]
fn it_runs_declare_and_multiply() {
    let mut test_context = TestContext::new();

    let nala = r#"
        const foo = 4;
        print(7 * foo);
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["28"]);
}

#[test]
fn it_runs_declare_basic() {
    let mut test_context = TestContext::new();

    let nala = r#"
        const foo = 4 * 7;
        print(foo);
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["28"]);
}

#[test]
fn it_runs_declare_mutable() {
    let mut test_context = TestContext::new();

    let nala = r#"
        mut mutable = 7;
        print(mutable);
        mutable = 8;
        print(mutable);
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["7", "8"]);
}

#[test]
fn it_runs_string_special_chars() {
    let mut test_context = TestContext::new();

    let nala = r#"
        print('!@#$%^&*()_+-=;:"');
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["!@#$%^&*()_+-=;:\""]);
}
