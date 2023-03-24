use super::{terms::*, types::type_literal_variant::TypeLiteralVariant, *};

#[derive(Debug, Clone)]
pub struct FuncDeclare {
    pub ident: String,
    pub params: Vec<ParamDeclare>,
    pub return_type: TypeLiteralVariant,
    pub type_params: Option<String>,
    pub block: Box<FuncVariant>,
}

#[derive(Debug, Clone)]
pub struct ParamDeclare {
    pub ident: String,
    pub param_type: TypeLiteralVariant,
}

#[derive(Debug, Clone)]
pub enum Call {
    Call(PlaceExpression, Option<TypeLiteralVariant>, Vec<Expr>),
    PlaceExpression(PlaceExpression),
    Value(Value),
}
