use super::{Block, Expr};

#[derive(Clone, Debug)]
pub struct IfElseChain {
    pub cond: Expr,
    pub block: Box<Block>,
    pub else_ifs: Vec<ElseIf>,
    pub else_block: Option<Else>,
}

#[derive(Clone, Debug)]
pub struct ElseIf {
    pub cond: Expr,
    pub block: Box<Block>,
}

#[derive(Clone, Debug)]
pub struct Else {
    pub block: Box<Block>,
}
