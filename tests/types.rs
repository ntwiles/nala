// use nala_interpreter::io_context::TestContext;
// use test_util::parse_and_interpret;

// #[test]
// fn it_runs_enum_basic() {
//     let mut test_context = TestContext::new();

//     let nala = r#"
//         enum FooKind {
//             Bar,
//             Baz
//         }

//         const kind = FooKind::Bar;
//         print(kind);
//     "#;

//     assert!(parse_and_interpret(nala, &mut test_context).is_ok());
//     assert_eq!(test_context.get_output(), vec!["FooKind::Bar"]);
// }

// #[test]
// fn it_runs_enum_compare() {
//     let mut test_context = TestContext::new();

//     let nala = r#"
//         enum FooKind {
//             Bar,
//             Baz
//         }

//         const kind = FooKind::Bar;

//         if (kind is FooKind::Baz) {
//             print('should not print');
//         }

//         if (kind is FooKind::Bar) {
//             print('should print');
//         }
//     "#;

//     assert!(parse_and_interpret(nala, &mut test_context).is_ok());
//     assert_eq!(test_context.get_output(), vec!["should print"]);
// }

// #[test]
// fn it_runs_enum_declare() {
//     let mut test_context = TestContext::new();

//     let nala = r#"
//         enum FooKind {
//             Bar,
//             Baz
//         }

//         print('success');
//     "#;

//     assert!(parse_and_interpret(nala, &mut test_context).is_ok());
//     assert_eq!(test_context.get_output(), vec!["success"]);
// }
