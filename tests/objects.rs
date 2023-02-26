use nala_interpreter::io_context::TestContext;
use regex::Regex;
use test_util::{assert_regex_match, parse_and_run, rgx};

#[test]
fn it_errors_when_doing_member_access_on_non_object() {
    let expected_message = rgx!("Cannot access member `field` of non-Object `7`.");

    let nala = r#"
        const object = {
            number: 7
        };

        const bad = object.number.field;
    "#;

    let result = parse_and_run(nala, &mut TestContext::new());

    assert!(result.is_err());
    assert_regex_match!(expected_message, &result.clone().unwrap_err().message)
}
