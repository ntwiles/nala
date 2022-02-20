use super::{terms::*, *};

#[derive(Debug, Clone)]
pub struct Array {
    pub elems: Box<Elems>,
}

#[derive(Debug, Clone)]
pub enum Elems {
    Elems(Box<Elems>, Expr),
    Expr(Expr),
    Empty,
}

#[derive(Debug, Clone)]
pub enum Index {
    Index(String, Box<Expr>),
    SymbolOrTerm(SymbolOrTerm),
}
