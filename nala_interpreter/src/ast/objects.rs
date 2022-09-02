use super::*;

#[derive(Debug, Clone)]
pub struct Object {
    pub entries: Vec<KeyValuePair>,
}

#[derive(Debug, Clone)]
pub struct KeyValuePair {
    pub key: String,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone)]
pub enum MemberAccess {
    MemberAccesses(Box<MemberAccess>, String),
    MemberAccess(String, String),
}
