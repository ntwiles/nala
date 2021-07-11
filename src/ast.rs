pub enum Stmt {
    Print(Literal),
    Assign,
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
