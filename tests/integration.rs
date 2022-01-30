use library::{interpreter::interpret_tree, io_context::TestContext, parser};

use std::{collections::HashMap, fs};

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct TestData {
    input: Option<Vec<String>>,
    output: Vec<String>,
}

// TODO: Now that tests are improved, output and input tests don't need to be split up in this way.
// We should categorize these differently.
#[test]
fn test_run_output_examples() {
    test_run_examples("output");
}

#[test]
fn test_run_input_examples() {
    test_run_examples("input");
}

fn test_run_examples(cat: &str) {
    let data = fs::read_to_string(format!("tests/data/{}.json", cat)).unwrap();
    let data: HashMap<String, TestData> = serde_json::from_str(&data).unwrap();

    let files = fs::read_dir(format!("tests/nala/{}", cat));

    for file in files.unwrap() {
        let name = file.unwrap().file_name().into_string().unwrap();
        let name = name[..name.find('.').unwrap()].to_owned();

        let test_data = if let Some(test_data) = data.get(&name) {
            test_data
        } else {
            panic!("Could not find matching test data for file: {}", name);
        };

        let mut test_context = TestContext::new();

        if let Some(input) = test_data.input.clone() {
            test_context.mock_inputs(input);
        }

        let nala_path = format!("tests/nala/{0}/{1}.nl", cat, name);

        println!("Testing: {}", nala_path);
        read_and_execute(&nala_path, &mut test_context);
        assert_eq!(
            test_context.get_output(),
            &test_data.output,
            "{}",
            nala_path
        );
    }
}

// TODO: Improve this to work with above style of testing. Maybe should throw status can be embedded in json files.
#[test]
#[should_panic(expected = "Could not parse nala file!")]
fn test_run_parse_error_examples() {
    let test_data = ["assign-void", "array-len", "for-return-void"];

    for file in test_data {
        let file_name = format!("tests/nala/error/parse/{}.nl", file);
        let mut test_context = TestContext::new();

        read_and_execute(&file_name, &mut test_context);
    }
}

// TODO: Improve this to work with above style of testing. Maybe should throw status can be embedded in json files.
// TODO: We're not asserting what is thrown in these tests.
#[test]
#[should_panic]
fn test_run_runtime_error_examples() {
    let test_data = ["assign-types", "nested-types"];

    for file in test_data {
        let file_name = format!("tests/nala/error/runtime/{}.nl", file);
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
    } else {
        panic!("Could not parse nala file! {}", path)
    }
}
