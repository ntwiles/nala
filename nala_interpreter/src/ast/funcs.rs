use super::{terms::ValueLiteral, types::type_literal_variant::TypeVariantLiteral, *};

#[derive(Debug, Clone)]
pub struct FuncDeclare {
    pub ident: String,
    pub params: Vec<ParamDeclare>,
    pub return_type: TypeVariantLiteral,
    pub type_param: Option<String>,
    pub block: Box<FuncVariant>,
}

#[derive(Debug, Clone)]
pub struct ParamDeclare {
    pub ident: String,
    pub param_type: TypeVariantLiteral,
}

#[derive(Debug, Clone)]
pub enum Call {
    Call(PlaceExpression, Option<TypeVariantLiteral>, Vec<Expr>),
    PlaceExpression(PlaceExpression),
    ValueLiteral(ValueLiteral),
}
