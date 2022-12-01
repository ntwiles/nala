use super::{Block, Expr};

#[derive(Clone, Debug)]
pub struct IfElseChain {
    pub cond: Expr,
    pub block: Box<Block>,
    pub else_ifs: Vec<ElseIf>,
}

#[derive(Clone, Debug)]
pub struct ElseIf {
    pub cond: Expr,
    pub block: Box<Block>,
}
