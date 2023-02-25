#[derive(Clone, Debug)]
pub struct RuntimeError {
    pub message: String,
}

impl RuntimeError {
    pub fn new(message: String) -> RuntimeError {
        RuntimeError { message }
    }
}
