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

#[test]
fn it_runs_single_elif_branching() {
    let mut test_context = TestContext::new();

    let nala = r#"
        if (false) {
            print('should not print');
        } elif (true) {
            print('should print');
        }
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["should print"]);
}

#[test]
fn it_runs_multiple_elif_branching() {
    let mut test_context = TestContext::new();

    let nala = r#"
        if (false) {
            print('should not print');
        } elif (false) {
            print('should not print either');
        } elif (true) {
            print('should print');
        }
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["should print"]);
}

#[test]
fn it_runs_if_else_branching() {
    let mut test_context = TestContext::new();

    let nala = r#"
        if (false) {
            print('should not print');
        } else {
            print('should print');
        }
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["should print"]);
}
