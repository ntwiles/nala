use nala_interpreter::io_context::TestContext;
use test_util::parse_and_interpret;

#[test]
fn it_runs_wiles_basic() {
    let mut test_context = TestContext::new();

    let nala = r#"
        mut i = 0;
        const letters = [ 'h', 'e', 'l', 'l', 'o'];

        wiles (i < 4) {
            print(letters[i]);
            i = i + 1;
        }
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["h", "e", "l", "l"]);
}

#[test]
fn it_runs_array_for() {
    let mut test_context = TestContext::new();

    let nala = r#"
        const secret = 52;
        const attempts = [0, 1, 2];

        const values = [ 'foo', 'bar', 'baz', 'qux' ];

        for value in values {
            print(value);
        }
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["foo", "bar", "baz", "qux"]);
}

#[test]
fn it_runs_array_empty() {
    let mut test_context = TestContext::new();

    let nala = r#"
        const empty = [];

        for value in empty {
            print('This should not print');
        }
        
        print('This should print.');
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["This should print."]);
}
