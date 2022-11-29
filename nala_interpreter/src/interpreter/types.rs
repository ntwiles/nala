use crate::{
    ast::{terms::Value, types::StructField},
    errors::NalaRuntimeError,
    scope::{ScopeId, Scopes},
};

pub fn eval_struct(
    ident: &str,
    fields: Vec<StructField>,
    scopes: &mut Scopes,
    current_scope: ScopeId,
) -> Result<Value, NalaRuntimeError> {
    scopes
        .add_struct_binding(&ident, current_scope, fields)
        .map(|_| Value::Void)
}
