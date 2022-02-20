use super::*;

#[derive(Debug, Clone)]
pub enum Unwrap {
    Literal(Box<Expr>, Pattern),
    Symbol(Box<Expr>, String),
}

#[derive(Debug, Clone)]
pub enum IsPattern {
    Literal(Box<Expr>, Pattern),
    Symbol(Box<Expr>, String),
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Enum(String, String, Box<Option<Pattern>>),
    Capture(Capture),
}

#[derive(Debug, Clone)]
pub enum Capture {
    Capture,
    NoCapture,
}
