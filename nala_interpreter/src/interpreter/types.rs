use crate::{scope::{Scopes, ScopeId}, io_context::IoContext, errors::NalaRuntimeError, ast::{terms::Value, types::StructField}};



pub fn eval_struct(
    ident: &str,
    fields: &Vec<StructField>,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    ctx: &mut dyn IoContext,
) -> Result<Value, NalaRuntimeError> {
    Ok(Value::Void)
}