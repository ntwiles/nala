#[derive(Debug, Clone)]
pub enum Pattern {
    Enum(String, String, Capture),
}

#[derive(Debug, Clone)]
pub enum Capture {
    Capture,
    NoCapture,
}
