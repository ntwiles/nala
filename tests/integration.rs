extern crate test_util;

use nala_interpreter::io_context::TestContext;
use test_util::parse_and_interpret;

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
fn it_runs_block_parent_scopes() {
    let mut test_context = TestContext::new();

    let nala = r#"  
        const foo = 7;

        if (true) {
            print(foo);
        }

        print(foo);
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["7", "7"]);
}

#[test]
fn it_runs_block_shadowing() {
    let mut test_context = TestContext::new();

    let nala = r#"
        const foo = 7;

        if (true) {
            const foo = 'bar';
            print(foo);
        }

        print(foo);
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["bar", "7"]);
}

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
fn it_runs_bool_expression() {
    let mut test_context = TestContext::new();

    let nala = r#"
        const foo = 'hello';
        const bar = 'hello';
        print(foo == bar);
        print(1 == 7);
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["true", "false"]);
}

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

#[test]
fn it_runs_declare_and_multiply() {
    let mut test_context = TestContext::new();

    let nala = r#"
        const foo = 4;
        print(7 * foo);
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["28"]);
}

#[test]
fn it_runs_declare_basic() {
    let mut test_context = TestContext::new();

    let nala = r#"
        const foo = 4 * 7;
        print(foo);
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["28"]);
}

#[test]
fn it_runs_declare_mutable() {
    let mut test_context = TestContext::new();

    let nala = r#"
        mut mutable = 7;
        print(mutable);
        mutable = 8;
        print(mutable);
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["7", "8"]);
}

#[test]
fn it_runs_enum_basic() {
    let mut test_context = TestContext::new();

    let nala = r#"
        enum FooKind {
            Bar,
            Baz
        }
        
        const kind = FooKind::Bar;
        print(kind);
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["FooKind::Bar"]);
}

#[test]
fn it_runs_enum_compare() {
    let mut test_context = TestContext::new();

    let nala = r#"
        enum FooKind {
            Bar,
            Baz
        }
        
        const kind = FooKind::Bar;
        
        if (kind is FooKind::Baz) {
            print('should not print');
        }
        
        if (kind is FooKind::Bar) {
            print('should print');
        }
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["should print"]);
}

#[test]
fn it_runs_enum_declare() {
    let mut test_context = TestContext::new();

    let nala = r#"
        enum FooKind {
            Bar,
            Baz
        }
        
        print('success');
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["success"]);
}

#[test]
fn it_runs_func_args() {
    let mut test_context = TestContext::new();

    let nala = r#"
        func add(a: Number, b: Number) {
            a + b;
        }
        
        print(add(5, 7));
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["12"]);
}

#[test]
fn it_runs_func_basic() {
    let mut test_context = TestContext::new();

    let nala = r#"
        func printMessage() {
            print('Functions work!');
        }
        
        printMessage();
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["Functions work!"]);
}

#[test]
fn it_runs_func_expressions() {
    let mut test_context = TestContext::new();

    let nala = r#"
        func foo() {
            'foo';
        }
        
        func bar() {
            'bar';
        }
        
        print(foo() + bar());
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["foobar"]);
}

#[test]
fn it_runs_func_first_class() {
    let mut test_context = TestContext::new();

    let nala = r#"
        func foo(message: String) {
            print(message);
        }
        
        const bar = foo;
        bar('This should print.');
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["This should print."]);
}

#[test]
fn it_runs_func_return() {
    let mut test_context = TestContext::new();

    let nala = r#"
        func getMessage() {
            'Function returns work!';
        }
        
        print(getMessage());
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["Function returns work!"]);
}

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

#[test]
fn it_runs_num_floor() {
    let mut test_context = TestContext::new();

    let nala = r#"
        const ratio = 6.7;
        print(floor(ratio));
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["6"]);
}

#[test]
fn it_runs_print_expression() {
    let mut test_context = TestContext::new();

    let nala = "print(5 + 10 * 2 / 4 - 3);";

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["7"]);
}

#[test]
fn it_runs_print_hello_world() {
    let mut test_context = TestContext::new();

    let nala = "print('hello world');";

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["hello world"]);
}

#[test]
fn it_runs_print_multiple() {
    let mut test_context = TestContext::new();

    let nala = r#"
        print('hello world'); 
        print(10 * 2 / 4 + 5 - 3);
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["hello world", "7"]);
}

#[test]
fn it_runs_print_number() {
    let mut test_context = TestContext::new();

    let nala = "print(311);";

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["311"]);
}

#[test]
fn it_runs_print_string_concat_vars() {
    let mut test_context = TestContext::new();

    let nala = r#"
        const foo = 'hello ';
        const bar = 'world';
        print(foo + bar);
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["hello world"]);
}

#[test]
fn it_runs_print_string_concat() {
    let mut test_context = TestContext::new();

    let nala = "print('hello ' + 'world');";

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["hello world"]);
}

#[test]
fn it_runs_string_special_chars() {
    let mut test_context = TestContext::new();

    let nala = r#"
        print('!@#$%^&*()_+-=;:"');
    "#;

    assert!(parse_and_interpret(nala, &mut test_context).is_ok());
    assert_eq!(test_context.get_output(), vec!["!@#$%^&*()_+-=;:\""]);
}

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
