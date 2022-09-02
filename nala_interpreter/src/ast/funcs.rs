use super::{terms::*, types::*, *};

#[derive(Debug, Clone)]
pub struct Func {
    pub ident: String,
    pub params: Vec<Param>,
    pub block: Box<Block>,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub ident: String,
    pub param_type: TypeVariant,
}

#[derive(Debug, Clone)]
pub enum Invocation {
    Invocation(PlaceExpression, Vec<Expr>),
    PlaceExpression(PlaceExpression),
    Value(Value),
}
