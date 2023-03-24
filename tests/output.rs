use regex::Regex;

use nala_interpreter::io_context::TestContext;
use test_util::{assert_regex_match, parse_and_run, rgx};

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

#[test]
fn it_prints_strings_with_quotes_in_erros() {
    let expected_error = rgx!(
        "Passed value `'should print'` of type `String` to function where `Array<T>` was expected"
    );

    let nala = r#"
        const test = 'should print';
        len(test);
    "#;

    let result = parse_and_run(nala, &mut TestContext::new());

    assert!(result.is_err());
    assert_regex_match!(expected_error, &result.clone().unwrap_err().message)
}
