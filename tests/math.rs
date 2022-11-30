// use nala_interpreter::io_context::TestContext;
// use test_util::parse_and_interpret;

// #[test]
// fn it_runs_num_floor() {
//     let mut test_context = TestContext::new();

//     let nala = r#"
//         const ratio = 6.7;
//         print(floor(ratio));
//     "#;

//     assert!(parse_and_interpret(nala, &mut test_context).is_ok());
//     assert_eq!(test_context.get_output(), vec!["6"]);
// }
