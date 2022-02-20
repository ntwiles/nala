use nala_interpreter::io_context::TestContext;
use test_util::parse_and_interpret;

#[test]
fn it_runs_bool_branching() {
    let mut test_context = TestContext::new();

    let nala = r#"
        if (true) {
            print('should print');
        }
        
        if (false) {
            print('should not print');
        }
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["should print"]);
}
