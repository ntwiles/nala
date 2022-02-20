use nala_interpreter::io_context::TestContext;
use test_util::parse_and_interpret;

#[test]
fn it_runs_break_for() {
    let mut test_context = TestContext::new();

    let nala = r#"
        func foo() {
            mut i = 0;
            const array = [0, 1, 2];

            for hay in array {
                if (i > 0) {
                    break(i);
                }

                i = i + 1;
            }

            -1;
        }

        print(foo());
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["1"]);
}

#[test]
fn it_runs_break_wiles() {
    let mut test_context = TestContext::new();

    let nala = r#"
        wiles(true) {
            print('foo');
            break(1);
        }
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["foo"]);
}
