use super::{patterns::Pattern, Expr, Line};

#[derive(Clone, Debug)]
pub struct IfElseChain {
    pub cond: Expr,
    pub block: Vec<Line>,
    pub else_ifs: Vec<ElseIf>,
    pub else_block: Option<Else>,
}

#[derive(Clone, Debug)]
pub struct ElseIf {
    pub cond: Expr,
    pub block: Vec<Line>,
}

#[derive(Clone, Debug)]
pub struct Else {
    pub block: Vec<Line>,
}

#[derive(Clone, Debug)]
pub struct Match {
    pub expr: Expr,
    pub cases: Vec<MatchCase>,
}

#[derive(Clone, Debug)]
pub struct MatchCase {
    pub pattern: Pattern,
    pub block: Vec<Line>,
}
