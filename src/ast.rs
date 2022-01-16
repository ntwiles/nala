pub enum Program {
    Block(Block),
    Stmts(Stmts),
}

#[derive(Debug, Clone)]
pub struct Block {
    pub stmts: Stmts,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Print(Expr),
    Declare(String, Expr, bool),
    Assign(String, Expr),
    If(Expr, Box<Block>),
    For(String, Expr, Box<Block>),
    Func(String, Box<Block>),
    Expr(Expr)
}

#[derive(Debug, Clone)]
pub enum Stmts {
    Stmts(Box<Stmts>, Stmt),
    Stmt(Stmt),
}

#[derive(Debug, Clone)]
pub struct Array {
    pub elems: Box<Elems>,
}

#[derive(Debug, Clone)]
pub enum Elems {
    Elems(Box<Elems>, Expr),
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Eq(Box<Expr>, Addend),
    Gt(Box<Expr>, Addend),
    Lt(Box<Expr>, Addend),
    Addend(Addend),
    Array(Array),
    Index(String, Box<Expr>),
    Read,
    ReadNum,
    Call(String)
}

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
    Term(Term),
}

#[derive(Debug, Clone)]
pub enum Term {
    Bool(bool),
    Symbol(String),
    String(String),
    Num(f32),
    Array(Vec<Term>),
    Func(Box<Block>),
    Void,
}

#[derive(Debug)]
pub enum OpKind {
    Add,
    Sub,
    Mult,
    Div,
}

impl Term {
    pub fn to_string(&self) -> String {
        match self {
            Term::Symbol(_) => {
                panic!("Cannot know string representation of un-evaluated symbol.")
            }
            Term::String(t) => t.to_owned(),
            Term::Num(n) => n.to_string(),
            Term::Bool(b) => b.to_string(),
            Term::Array(a) => String::from(format!("[{}]", a.len())),
            Term::Func(_) => String::from("<Func>"),
            Term::Void => String::from("<Void>"),
        }
    }
}
