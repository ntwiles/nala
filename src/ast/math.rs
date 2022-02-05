use super::{funcs::*, terms::*};

#[derive(Debug, Clone)]
pub enum Addend {
    Add(Box<Addend>, Factor),
    Sub(Box<Addend>, Factor),
    Factor(Factor),
}

#[derive(Debug, Clone)]
pub enum Factor {
    Mult(Box<Factor>, Term),
    Div(Box<Factor>, Term),
    Call(Call),
}

#[derive(Debug)]
pub enum OpKind {
    Add,
    Sub,
    Mult,
    Div,
}
