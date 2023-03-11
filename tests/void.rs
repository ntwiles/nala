use regex::Regex;

use nala_interpreter::io_context::TestContext;
use test_util::{assert_regex_match, parse_and_run, rgx};

#[test]
fn it_converts_type_to_void() {
    let expected_message = rgx!("Cannot declare a variable with a value of type Void");
    let nala = r#"
        const number = 7;
        const test = void(number);
    "#;

    let result = parse_and_run(nala, &mut TestContext::new());
    assert!(result.is_err());
    assert_regex_match!(expected_message, &result.clone().unwrap_err().message)
}
