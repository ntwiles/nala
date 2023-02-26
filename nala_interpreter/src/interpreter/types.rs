use crate::{
    ast::{terms::Value, types::StructLiteralField, VariantDeclare},
    errors::RuntimeError,
    scopes::{type_binding::TypeBinding, Scopes},
    types::struct_field::StructField,
};

pub fn eval_struct(
    ident: &str,
    fields: Vec<StructLiteralField>,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<Value, RuntimeError> {
    let binding = TypeBinding::Struct(
        fields
            .into_iter()
            .map(|f| StructField::from_literal(f, scopes, current_scope))
            .collect(),
    );

    scopes
        .add_type_binding(&ident, current_scope, binding)
        .map(|_| Value::Void)
}

pub fn eval_enum(
    ident: &str,
    variants: Vec<VariantDeclare>,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<Value, RuntimeError> {
    let binding = TypeBinding::Enum(variants);

    scopes
        .add_type_binding(&ident, current_scope, binding)
        .map(|_| Value::Void)
}
