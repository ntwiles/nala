use super::*;

#[derive(Debug, Clone)]
pub struct Array {
    pub elems: Vec<Expr>,
}

// TODO: Single variant enums should just be structs.
#[derive(Debug, Clone)]
pub enum Index {
    Index(String, Box<Expr>),
}
