use nala_interpreter::io_context::TestContext;
use test_util::parse_and_run;

#[test]
fn it_runs_bool() {
    let mut ctx = TestContext::new();

    let nala = r#"
        if (true) {
            print('should print');
        }
        
        if (false) {
            print('should not print');
        }
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["should print"]);
}

#[test]
fn it_runs_single_elif() {
    let mut ctx = TestContext::new();

    let nala = r#"
        if (false) {
            print('should not print');
        } elif (true) {
            print('should print');
        }
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["should print"]);
}

#[test]
fn it_runs_multiple_elif() {
    let mut ctx = TestContext::new();

    let nala = r#"
        if (false) {
            print('should not print');
        } elif (false) {
            print('should not print either');
        } elif (true) {
            print('should print');
        }
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["should print"]);
}

#[test]
fn it_runs_if_else() {
    let mut ctx = TestContext::new();

    let nala = r#"
        if (false) {
            print('should not print');
        } else {
            print('should print');
        }
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["should print"]);
}

#[test]
fn it_runs_if_elif_else() {
    let mut ctx = TestContext::new();

    let nala = r#"
        if (false) {
            print('should not print');
        } elif (false) {
            print('also should not');
        } else {
            print('should print');
        }
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["should print"]);
}
