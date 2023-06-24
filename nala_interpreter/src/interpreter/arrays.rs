use std::{
    sync::{Arc, Mutex},
    usize,
};

use crate::{
    ast::{arrays::*, Expr},
    errors::RuntimeError,
    io_context::IoContext,
    resolved::value::Value,
    scopes::Scopes,
    types::{fit::fits_type, inference::infer_type},
};

use super::basic::*;

pub fn eval_index(
    array: &Value,
    index_expr: &Expr,
    scopes: &mut Scopes,
    current_scope: usize,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    let index = eval_expr(index_expr, scopes, current_scope, ctx)?;

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
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    let values = eval_elems(&array.elems, scopes, current_scope, ctx)?;

    if let Some(first) = values.clone().first() {
        let first_type = infer_type(&first, scopes, current_scope)?;

        for value in values.clone() {
            let second_type = infer_type(&value, scopes, current_scope)?;

            // TODO: This partially works. In the case of generic values like Option, if the first
            // value is a Some, we'll be able to infer a concrete type for the generic, and later
            // None values in the array will fit that type. However if the first type is None, we
            // won't be able to infer a type and this will fail.
            //
            // NOTE: There's a related issue mentioned in inference.rs:77 that involves type inference
            // for the array as a whole. If we get that working, maybe we can leverage that here to
            // be lazy and just compare each value here to that inferred type.
            if !fits_type(&value, &first_type, scopes, current_scope)? {
                return Err(RuntimeError::new(
                    &format!("Arrays can contain elements of only a single type. Found elements of types `{first_type}` and `{second_type}`.",
                )));
            }
        }
    };

    Ok(Value::Array(Arc::new(Mutex::new(values))))
}
