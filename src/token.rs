// Note: for now these are unused in favor of LALRPOP's default lexer generator.
#[derive(Debug)]
pub enum Token {
    Semicolon,
    Symbol(String),
    Str(String),
}
