use library::{interpreter::interpret_tree, io_context::TestContext, parser};

use std::{collections::HashMap, fs};

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct TestData {
    input: Option<Vec<String>>,
    output: Vec<String>,
}

#[test]
fn test_run_examples() {
    let data = fs::read_to_string("tests/data/output.json").unwrap();
    let data: HashMap<String, TestData> = serde_json::from_str(&data).unwrap();

    let files = fs::read_dir("tests/nala/output");

    for file in files.unwrap() {
        let file = file.unwrap();
        let name = file.file_name().into_string().unwrap();
        let name = name[..name.find('.').unwrap()].to_owned();

        let nala_path = format!("tests/nala/output/{}.nl", name);

        let test_data = if let Some(test_data) = data.get(&name) {
            test_data
        } else {
            panic!("Could not find matching test data for file: {}", name);
        };

        let mut test_context = TestContext::new();
        read_and_execute(&nala_path, &mut test_context);
        assert_eq!(
            test_context.get_output(),
            &test_data.output,
            "{}",
            nala_path
        );
    }
}

#[test]
fn test_run_input_examples() {
    let data = fs::read_to_string("tests/data/input.json").unwrap();
    let data: HashMap<String, TestData> = serde_json::from_str(&data).unwrap();

    let files = fs::read_dir("tests/nala/input");

    for file in files.unwrap() {
        let name = file.unwrap().file_name().into_string().unwrap();
        let name = name[..name.find('.').unwrap()].to_owned();

        let test_data = if let Some(test_data) = data.get(&name) {
            test_data
        } else {
            panic!("Could not find matching test data for file: {}", name);
        };

        let mut test_context = TestContext::new();
        test_context.mock_inputs(test_data.input.clone().unwrap());

        let nala_path = format!("tests/nala/input/{}.nl", name);
        read_and_execute(&nala_path, &mut test_context);
        assert_eq!(
            test_context.get_output(),
            &test_data.output,
            "{}",
            nala_path
        );
    }
}

#[test]
#[should_panic]
fn test_run_error_examples() {
    let test_data = ["assign-void", "array-len"];

    for file in test_data {
        let file_name = format!("tests/nala/error/{}.nl", file);
        let mut test_context = TestContext::new();

        read_and_execute(&file_name, &mut test_context);
    }
}

fn read_and_execute(path: &str, test_context: &mut TestContext) {
    let code = if let Ok(code) = fs::read_to_string(path) {
        code
    } else {
        panic!("Could not load nala file! {}", path);
    };

    let result = parser::parse_code(code);

    if let Some(parsed) = result {
        interpret_tree(parsed, test_context);
    }
}
