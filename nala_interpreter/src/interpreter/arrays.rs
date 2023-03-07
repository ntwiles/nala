use std::{
    sync::{Arc, Mutex},
    usize,
};

use crate::{
    ast::{arrays::*, terms::Value, Expr},
    errors::RuntimeError,
    io_context::IoContext,
    scopes::Scopes,
    types::inference::infer_type,
};

use super::basic::*;

pub fn eval_index(
    array: &Value,
    index_expr: &Expr,
    scopes: &mut Scopes,
    current_scope: usize,
    enclosing_scope: Option<usize>,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    let index = eval_expr(index_expr, scopes, current_scope, enclosing_scope, ctx)?;

    if let Value::Num(index) = index {
        if let Value::Array(array) = array {
            let array = Arc::clone(&array);
            let array = array.lock().unwrap();
            Ok(array.get(index as usize).unwrap().clone())
        } else {
            Err(RuntimeError::new(
                "Cannot index into a value which is not an array.",
            ))
        }
    } else {
        Err(RuntimeError::new("Cannot index using non-numeric value."))
    }
}

pub fn eval_array(
    array: &Array,
    scopes: &mut Scopes,
    current_scope: usize,
    enclosing_scope: Option<usize>,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    let values = eval_elems(&array.elems, scopes, current_scope, enclosing_scope, ctx)?;

    if let Some(first) = values.clone().first() {
        let first_type = infer_type(&first, scopes, current_scope)?;

        // TODO: This calls infer_type() on the same value twice. This is inefficient.
        for value in values.clone() {
            if infer_type(&value, scopes, current_scope)? != first_type {
                let second_type = infer_type(&value, scopes, current_scope)?;

                return Err(RuntimeError::new(
                    &format!("Arrays can contain elements of only a single type. Found elements of types `{first_type}` and `{second_type}`.",
                )));
            }
        }
    };

    Ok(Value::Array(Arc::new(Mutex::new(values))))
}
