pub trait IoContext {
    fn print(self: &mut Self, message: &str);
    // fn read();
}

pub struct ConsoleContext;

impl IoContext for ConsoleContext {
    fn print(self: &mut Self, message: &str) {
        println!("{}", message);
    }

    // fn read() {

    // }
}

pub struct TestContext {
    output: Vec<String>,
}

impl TestContext {
    pub fn new() -> TestContext {
        TestContext { output: vec![] }
    }

    pub fn get_output(self: &mut Self) -> &Vec<String> {
        &self.output
    }
}

impl IoContext for TestContext {
    fn print(self: &mut Self, message: &str) {
        self.output.push(message.to_owned())
    }
}
