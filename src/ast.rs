pub enum Stmt {
    Print(Expr),
}

pub enum Expr {
    Term(Term),
    Oper(Box<Expr>, OpKind, Term),
}

pub enum OpKind {
    Add,
}

pub enum Term {
    String(String),
    Num(i32),
}

impl Term {
    pub fn to_string(self) -> String {
        match self {
            Term::String(t) => t,
            Term::Num(n) => n.to_string(),
        }
    }
}
