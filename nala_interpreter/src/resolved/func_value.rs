use crate::ast::FuncVariant;

use crate::types::type_variant::TypeVariant;

#[derive(Debug, Clone)]
pub struct FuncValue {
    pub block: Box<FuncVariant>,
    pub params: Vec<Param>,
    pub return_type: TypeVariant,
    pub type_params: Option<String>,
    pub closure_scope: usize,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub ident: String,
    pub param_type: TypeVariant,
}
