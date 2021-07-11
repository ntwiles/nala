pub enum Stmt {
    Print(Expr),
}

pub enum Expr {
    Literal(Literal),
}

// TODO: Remove me.
impl Expr {
    pub fn to_string(self) -> String {
        let Expr::Literal(literal) = self;
        literal.to_string()
    }
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
