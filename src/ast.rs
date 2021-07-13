pub enum Program {
    Stmt(Stmt),
    Stmts(Box<Program>, Stmt),
}

pub enum Stmt {
    Print(Expr),
    Declare(String, Expr),
}

pub enum Expr {
    Add(Box<Expr>, Factor),
    Sub(Box<Expr>, Factor),
    Factor(Factor),
}

pub enum Factor {
    Mult(Box<Factor>, Term),
    Div(Box<Factor>, Term),
    Term(Term),
}

#[derive(Clone)]
pub enum Term {
    Symbol(String),
    String(String),
    Num(f32),
}

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
        }
    }
}
