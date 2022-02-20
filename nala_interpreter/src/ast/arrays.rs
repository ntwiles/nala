use super::*;

#[derive(Debug, Clone)]
pub struct Array {
    pub elems: Box<Elems>,
}

// TODO: Implement this as a Vec<Expr> instead of a linked list.
// This should remain as a linked list in the grammar.
#[derive(Debug, Clone)]
pub enum Elems {
    Elems(Box<Elems>, Expr),
    Expr(Expr),
    Empty,
}

#[derive(Debug, Clone)]
pub enum Index {
    Index(String, Box<Expr>),
}
