use nala_interpreter::io_context::TestContext;
use regex::Regex;
use test_util::{assert_regex_match, parse_and_run, rgx};

#[test]
fn it_runs_wiles_basic() {
    let mut ctx = TestContext::new();

    let nala = r#"
        mut i = 0;
        const letters = [ 'h', 'e', 'l', 'l', 'o'];

        wiles (i < 4) {
            print(letters[i]);
            i = i + 1;
        }
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["h", "e", "l", "l"]);
}

#[test]
fn it_runs_array_for() {
    let mut ctx = TestContext::new();

    let nala = r#"
        const secret = 52;
        const attempts = [0, 1, 2];

        const values = [ 'foo', 'bar', 'baz', 'qux' ];

        for value in values {
            print(value);
        }
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["foo", "bar", "baz", "qux"]);
}

#[test]
fn it_runs_array_empty() {
    let mut ctx = TestContext::new();

    let nala = r#"
        const empty: Array<String> = [];

        for value in empty {
            print('This should not print');
        }
        
        print('This should print.');
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["This should print."]);
}

#[test]
fn it_errors_on_for_on_non_array() {
    let expected_message =
        rgx!("Cannot iterate over values of non-Array types. Found '7' of type `Number`");

    let mut ctx = TestContext::new();

    let nala = r#"
        const nonArray = 7;

        for value in nonArray {
            print('This should not print');
        }
        
        print('This should print.');
    "#;

    let result = parse_and_run(nala, &mut ctx);
    assert!(result.is_err());

    assert_regex_match!(expected_message, &result.clone().unwrap_err().message)
}
