use nala_interpreter::io_context::TestContext;
use test_util::parse_and_run;

#[test]
fn it_supports_closures_in_scope() {
    let mut ctx = TestContext::new();

    let nala = r#"
        func closureTest(): Void {
            const message = 'closures work!';
        
            func innerFunc(): Void {
                print(message);
            }
        
            innerFunc();
        }

        closureTest();
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["closures work!"]);
}

#[test]
fn it_supports_closures_out_of_scope() {
    let mut ctx = TestContext::new();

    let nala = r#"
        func closureTest(): Func<Void> {
            const message = 'closures REALLY work!';

            func innerFunc(): Void {
                print(message);
            }

            innerFunc;
        }

        const funcToCall = closureTest();
        funcToCall();
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["closures REALLY work!"]);
}

#[test]
fn it_allows_passing_enclosed_variables_to_variants() {
    let mut ctx = TestContext::new();

    let nala = r#"
        enum Option<T> {
            Some(T),
            None,
        }

        func outer(): Func<Void> {
            const message = 'print me';

            func inner(): Void {
                print(Option::Some(message));
            }

            inner;
        }

        const inner = outer();
        inner();
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["Some('print me')"]);
}
