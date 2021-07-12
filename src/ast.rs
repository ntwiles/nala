pub enum Stmt {
    Print(Expr),
}

pub enum Expr {
    Add(Box<Expr>, Factor),
    Sub(Box<Expr>, Factor),
    Factor(Factor),
}

pub enum Factor {
    Mult(Box<Factor>, Term),
    Term(Term),
}

pub enum Term {
    String(String),
    Num(f32),
}

pub enum OpKind {
    Add,
    Sub,
    Mult,
}

impl Term {
    pub fn to_string(self) -> String {
        match self {
            Term::String(t) => t,
            Term::Num(n) => n.to_string(),
        }
    }
}
