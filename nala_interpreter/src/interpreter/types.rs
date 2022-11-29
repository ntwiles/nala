use crate::{
    ast::{terms::Value, types::StructLiteralField},
    errors::NalaRuntimeError,
    scope::{ScopeId, Scopes},
    types::struct_field::StructField,
};

pub fn eval_struct(
    ident: &str,
    fields: Vec<StructLiteralField>,
    scopes: &mut Scopes,
    current_scope: ScopeId,
) -> Result<Value, NalaRuntimeError> {
    let fields = fields
        .into_iter()
        .map(|f| StructField::from_literal(f, scopes, current_scope))
        .collect();

    scopes
        .add_struct_binding(&ident, current_scope, fields)
        .map(|_| Value::Void)
}
