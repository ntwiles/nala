use super::{funcs::*, terms::*};

#[derive(Debug, Clone)]
pub enum Addition {
    Add(Box<Addition>, Factor),
    Sub(Box<Addition>, Factor),
    Factor(Factor),
}

#[derive(Debug, Clone)]
pub enum Factor {
    Mult(Box<Factor>, Term),
    Div(Box<Factor>, Term),
    Call(Call),
}
