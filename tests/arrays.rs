use nala_interpreter::io_context::TestContext;
use test_util::parse_and_interpret;

#[test]
fn it_runs_array_index_assign() {
    let mut test_context = TestContext::new();

    let nala = r#"
        mut array = [0, 1, 7, 3, 4];
        print(array[2]);
        array[2] = 2;
        print(array[2]);
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["7", "2"]);
}

#[test]
fn it_runs_array_index_expressions() {
    let mut test_context = TestContext::new();

    let nala = r#"
        const foo = [12, 34];
        const bar = [43, 21];

        print(foo[0] + bar[0]);
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["55"]);
}

#[test]
fn it_runs_array_index() {
    let mut test_context = TestContext::new();

    let nala = r#"
        const array = [5 + 6, 2 + 3];
        print(array[1]);
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["5"]);
}

#[test]
fn it_runs_array_len() {
    let mut test_context = TestContext::new();

    let nala = r#"
        const array = [0, 1, 2, 3, 4];
        const length = len(array);
        print(length);
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["5"]);
}

#[test]
fn it_runs_array_slice() {
    let mut test_context = TestContext::new();

    let nala = r#"
        const array = [ 0, 1, 2, 3, 4, 5];

        const left = slice(array, 0, 3);
        const right = slice(array, 3, len(array));

        print(left[2]);
        print(right[0]);
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["2", "3"]);
}

#[test]
fn it_allows_assign_to_index_place_expression() {
    let mut test_context = TestContext::new();

    let nala = r#"
        const letters = [ 'h', 'e', 'l', 'l', 'o'];
        letters[0] = 'j';
        print(letters[0]);
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["j"]);
}
