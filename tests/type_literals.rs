use nala_interpreter::io_context::TestContext;
use test_util::parse_and_run;

#[test]
fn it_errors_when_providing_type_arg_to_flat_type() {
    let expected_message =
        "Type `Bool` does not support type arguments. Type `Bool<String>` is invalid.";

    let nala = r#"
        func bad(arg: Bool<String>): Void {
            print('break');
        }

        bad(false);
    "#;

    let result = parse_and_run(nala, &mut TestContext::new());

    assert!(result.is_err());
    assert_eq!(expected_message, &result.clone().unwrap_err().message)
}
