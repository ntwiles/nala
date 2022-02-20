use super::*;

#[derive(Debug, Clone)]
pub struct Array {
    pub elems: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub enum Index {
    Index(String, Box<Expr>),
}
