#[derive(Clone, Debug)]
pub struct NalaRuntimeError {
    pub message: String,
}

impl NalaRuntimeError {
    pub fn new(message: String) -> NalaRuntimeError {
        NalaRuntimeError { message }
    }
}
