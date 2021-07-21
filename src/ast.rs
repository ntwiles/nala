pub enum Program {
    Block(Block),
    Stmts(Stmts),
}

pub struct Block {
    pub stmts: Stmts,
}

pub enum Stmts {
    Stmts(Box<Stmts>, Stmt),
    Stmt(Stmt),
}

pub enum Stmt {
    Print(Expr),
    Declare(String, Expr, bool),
    Assign(String, Expr),
    If(Expr, Box<Block>),
    For(String, Expr, Box<Block>),
}

#[derive(Debug)]
pub struct Array {
    pub elems: Box<Elems>,
}

#[derive(Debug)]
pub enum Elems {
    Elems(Box<Elems>, Expr),
    Expr(Expr),
}

#[derive(Debug)]
pub enum Expr {
    Eq(Box<Expr>, Addend),
    Gt(Box<Expr>, Addend),
    Lt(Box<Expr>, Addend),
    Addend(Addend),
    Array(Array),
    Index(String, Box<Expr>),
    Read,
    ReadNum,
}

#[derive(Debug)]
pub enum Addend {
    Add(Box<Addend>, Factor),
    Sub(Box<Addend>, Factor),
    Factor(Factor),
}

#[derive(Debug)]
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
        }
    }
}
