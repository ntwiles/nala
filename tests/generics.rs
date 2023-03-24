use nala_interpreter::io_context::TestContext;
use test_util::parse_and_run;

#[test]
fn it_allows_generic_enum_declare() {
    let mut ctx = TestContext::new();

    let nala = r#"
        enum Variants<T> {
            Foo(T),
            Bar,
        }
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), Vec::<String>::new());
}

#[test]
fn it_allows_generic_enum_variant_assign() {
    let mut ctx = TestContext::new();

    let nala = r#"
        const test = Option::Some(1);
        print(test);
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["Some(1)"]);
}

#[test]
fn it_allows_generic_func_declare() {
    let mut ctx = TestContext::new();

    let nala = r#"
        func foo<T>(value: T): Void {
            print(value);
        }

        print('test');
    "#;

    assert!(parse_and_run(nala, &mut ctx).is_ok());
    assert_eq!(ctx.get_output(), vec!["test"]);
}

// TODO: In the following example, T will be inferred to be type String (the type of 'Howard'). However,
// we return a type Option<Number>, and no type checking is done to discover that error. This should be fixed.
// func find<T>(haystack: Array<String>, needle: T ): T {
//     for item in haystack {
//         if (item == needle) {
//             break(Option::Some(7));
//         }
//     }

//     Option::None;
// }

// const found = find([ 'Jimmy', 'Kim', 'Howard', 'Lalo' ], 'Howard');
// print(found);
