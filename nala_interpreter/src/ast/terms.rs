#[derive(Debug, Clone)]
pub enum SymbolOrTerm {
    Symbol(String),
    Term(Term),
}

#[derive(Debug, Clone)]
pub enum Term {
    Identifier(String),
    Literal(Literal),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f32),
    String(String),
}
