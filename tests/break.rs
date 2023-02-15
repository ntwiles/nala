use nala_interpreter::io_context::TestContext;
use test_util::parse_and_interpret;

#[test]
fn it_runs_break_for() {
    let mut ctx = TestContext::new();

    let nala = r#"
        func findNeedle(haystack: Array<String>): Number {
            mut i = 0;

            for word in haystack {
                if (word == 'needle') {
                    break(i);
                }

                i = i + 1;
            }
        
            -1;
        }
        
        const haystack = ['needle', 'foo', 'needle', 'bar'];
        const index = findNeedle(haystack);
        
        print(haystack[index]);
    "#;

    assert!(parse_and_interpret(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["needle"]);
}

#[test]
fn it_runs_break_wiles() {
    let mut ctx = TestContext::new();

    let nala = r#"
        wiles(true) {
            print('foo');
            break(1);
        }
    "#;

    assert!(parse_and_interpret(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["foo"]);
}
