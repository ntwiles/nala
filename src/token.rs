#[derive(Debug)]
pub enum Token {
    Semicolon,
    Symbol(String),
    Str(String),
}
