use super::{arrays::*, objects::*, terms::*, types::*, *};

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
pub enum Call {
    Call(String, Vec<Expr>),
    MemberAccess(MemberAccess),
    Index(Index),
    Term(Term),
}
