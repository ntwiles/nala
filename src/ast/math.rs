use super::{funcs::*, terms::*};

#[derive(Debug, Clone)]
pub enum Addend {
    Add(Box<Addend>, Factor),
    Sub(Box<Addend>, Factor),
    Factor(Factor),
}

#[derive(Debug, Clone)]
pub enum Factor {
    Mult(Box<Factor>, SymbolOrTerm),
    Div(Box<Factor>, SymbolOrTerm),
    Call(Call),
}
