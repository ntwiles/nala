pub trait IoContext {
    fn print(self: &mut Self, message: &str);
    fn read(self: &mut Self) -> String;
}

pub struct ConsoleContext;

impl IoContext for ConsoleContext {
    fn print(self: &mut Self, message: &str) {
        println!("{}", message);
    }

    fn read(self: &mut Self) -> String {
        let mut line: String = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line
    }
}

pub struct TestContext {
    outputs: Vec<String>,
    inputs: Vec<String>,
}

impl TestContext {
    pub fn new() -> TestContext {
        TestContext {
            outputs: vec![],
            inputs: vec![],
        }
    }

    pub fn get_output(self: &mut Self) -> &Vec<String> {
        &self.outputs
    }

    pub fn mock_inputs(self: &mut Self, inputs: Vec<&str>) {
        self.inputs = inputs.iter().map(|s| s.to_string()).collect()
    }
}

impl IoContext for TestContext {
    fn print(self: &mut Self, message: &str) {
        self.outputs.push(message.to_owned())
    }

    fn read(self: &mut Self) -> String {
        self.inputs.pop().unwrap()
    }
}
