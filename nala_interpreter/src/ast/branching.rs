use super::{patterns::Pattern, Block, Expr, Stmts};

#[derive(Clone, Debug)]
pub struct IfElseChain {
    pub cond: Expr,
    pub block: Stmts,
    pub else_ifs: Vec<ElseIf>,
    pub else_block: Option<Else>,
}

#[derive(Clone, Debug)]
pub struct ElseIf {
    pub cond: Expr,
    pub block: Stmts,
}

#[derive(Clone, Debug)]
pub struct Else {
    pub block: Stmts,
}

#[derive(Clone, Debug)]
pub struct Match {
    pub expr: Expr,
    pub cases: Vec<MatchCase>,
}

#[derive(Clone, Debug)]
pub struct MatchCase {
    pub pattern: Pattern,
    pub block: Stmts,
}
