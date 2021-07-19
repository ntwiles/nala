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
    Read(String),
    Declare(String, Expr, bool),
    Assign(String, Expr),
    If(Expr, Box<Block>),
}

pub enum Expr {
    Equal(Box<Expr>, Addend),
    Addend(Addend),
    Array(Vec<Expr>),
}

pub enum Addend {
    Add(Box<Addend>, Factor),
    Sub(Box<Addend>, Factor),
    Factor(Factor),
}

pub enum Factor {
    Mult(Box<Factor>, Term),
    Div(Box<Factor>, Term),
    Term(Term),
}

#[derive(Clone)]
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
