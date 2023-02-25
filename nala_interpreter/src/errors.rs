#[derive(Clone, Debug)]
pub struct RuntimeError {
    pub message: String,
}

impl RuntimeError {
    pub fn new(message: &str) -> RuntimeError {
        RuntimeError {
            message: message.to_string(),
        }
    }
}
