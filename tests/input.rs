use nala_interpreter::io_context::TestContext;
use test_util::parse_and_interpret;

#[test]
fn it_runs_input_basic() {
    let input = vec!["Nathan"];
    let output = vec!["Please enter your name:", "Hello, Nathan"];

    let mut test_context = TestContext::new();
    test_context.mock_inputs(input);

    let nala = r#"
        print('Please enter your name:');
        const input = read();
        print('Hello, ' + input);
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), output);
}

#[test]
fn it_runs_input_numbers() {
    let input = vec!["31"];
    let output = vec![
        "Please enter your age:",
        "Next year your age will be:",
        "32",
    ];

    let mut test_context = TestContext::new();
    test_context.mock_inputs(input);

    let nala = r#"
        print('Please enter your age:');
        const input = readnum();
        const result = input + 1;
        print('Next year your age will be:');
        print(result);
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), output);
}
