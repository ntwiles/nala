use crate::{
    ast::{terms::Value, types::StructLiteralField},
    errors::NalaRuntimeError,
    scope::Scopes,
    types::struct_field::StructField,
};

pub fn eval_struct(
    ident: &str,
    fields: Vec<StructLiteralField>,
    scopes: &mut Scopes,
    current_scope: usize,
) -> Result<Value, NalaRuntimeError> {
    let fields = fields
        .into_iter()
        .map(|f| StructField::from_literal(f, scopes, current_scope))
        .collect();

    scopes
        .add_struct_binding(&ident, current_scope, fields)
        .map(|_| Value::Void)
}
