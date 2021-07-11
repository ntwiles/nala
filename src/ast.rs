pub enum Stmt {
    Print(Expr),
}

pub enum Expr {
    Literal(Literal),
    Oper(Literal, OpKind, Literal),
}

pub enum OpKind {
    Add,
}

pub enum Literal {
    String(String),
    Num(i32),
}

impl Literal {
    pub fn to_string(self) -> String {
        match self {
            Literal::String(t) => t,
            Literal::Num(n) => n.to_string(),
        }
    }
}
