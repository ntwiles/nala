// TODO: Improve this to work with above style of testing. Maybe should throw status can be embedded in json files.
// TODO: We're not asserting what is thrown in these tests.
#[test]
#[should_panic]
fn test_run_runtime_error_examples() {
    let test_data = ["assign-types", "nested-types"];

    for file in test_data {
        let file_name = format!("tests/nala/error/runtime/{}.nl", file);
        let mut test_context = TestContext::new();

        read_and_execute(&file_name, &mut test_context);
    }
}

#[test]
#[should_panic(expected = "Could not parse nala file!")]
fn test_run_parse_error_examples() {
        let file_name = format!("tests/nala/error/parse/array-len.nl", file);
        let mut test_context = TestContext::new();
        read_and_execute(&file_name, &mut test_context);
    }
}