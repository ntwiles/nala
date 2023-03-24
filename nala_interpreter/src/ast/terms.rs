#[derive(Debug, Clone)]
pub enum SymbolOrTerm {
    Symbol(String),
    Term(Term),
}

#[derive(Debug, Clone)]
pub enum Term {
    Identifier(String),
    ValueLiteral(ValueLiteral),
}

#[derive(Debug, Clone)]
pub enum ValueLiteral {
    Number(f32),
    String(String),
}
