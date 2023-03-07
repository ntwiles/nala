use nala_interpreter::io_context::TestContext;
use test_util::parse_and_run;

#[test]
fn it_infers_type_of_primitive() {
    let nala = r#"
        const foo = 1;
    "#;

    let result = parse_and_run(nala, &mut TestContext::new());

    assert!(result.is_ok());
}

#[test]
fn it_infers_type_of_generic_if_possible() {
    let nala = r#"
        enum Option<T> {
            Some(T),
            None,
        }

        const foo = Option::Some(1);
    "#;

    let result = parse_and_run(nala, &mut TestContext::new());

    assert!(result.is_ok());
}

// TODO: This are failing now on .is_err(), fix this.

// #[test]
// fn it_errors_if_no_info_for_inference() {
//     let nala = r#"
//         enum Option<T> {
//             Some(T),
//             None,
//         }

//         const foo = Option::None;
//     "#;

//     let result = parse_and_run(nala, &mut TestContext::new());

//     assert!(result.is_err());
// }

// #[test]
// fn it_errors_if_not_enough_info_for_inference() {
//     let nala = r#"
//         enum Option<T> {
//             This(T),
//             That(Number),
//             TheOther,
//         }

//         const foo = Option::That(7);
//     "#;

//     let result = parse_and_run(nala, &mut TestContext::new());

//     assert!(result.is_ok());
// }
