use crate::ast::terms::Value;

#[derive(Clone, Debug)]
pub struct ValueBinding {
    pub value: Value,
    pub is_mutable: bool,
}
