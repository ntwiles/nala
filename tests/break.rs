use nala_interpreter::io_context::TestContext;
use test_util::parse_and_interpret;

#[test]
fn it_runs_break_for() {
    let mut test_context = TestContext::new();

    let nala = r#"
        func findNeedle(haystack: Array<String>): Number {
            for word in haystack {
                if (word == 'needle') {
                    break(word);
                }
            }
        
            '';
        }
        
        const haystack = ['needle', 'foo', 'needle', 'bar'];
        const found = findNeedle(haystack);
        
        print(found);
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["needle"]);
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
