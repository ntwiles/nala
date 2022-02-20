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

// TODO: Implement this as a Vec<MemberAccess> instead of a linked list.
// This should remain as a linked list in the grammar.
#[derive(Debug, Clone)]
pub enum MemberAccess {
    MemberAccesses(Box<MemberAccess>, String),
    MemberAccess(String, String),
    Index(Index),
}
