use std::{
    sync::{Arc, Mutex},
    usize,
};

use crate::{
    ast::{arrays::*, terms::*},
    errors::NalaRuntimeError,
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

use super::basic::*;

pub fn evaluate_index(
    index: &Index,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut dyn IoContext,
) -> Result<Value, NalaRuntimeError> {
    match index {
        Index::Index(ident, expr) => {
            let index = evaluate_expr(expr, scopes, current_scope, context)?;

            if let Value::Num(index) = index {
                let array = scopes.get_value(ident, current_scope, context)?;

                if let Value::Array(array) = array {
                    let array = Arc::clone(&array);
                    let array = array.lock().unwrap();
                    Ok(array.get(index as usize).unwrap().clone())
                } else {
                    panic!("Cannot index into a value which is not an array.");
                }
            } else {
                Err(NalaRuntimeError {
                    message: "Cannot index using non-numeric value.".to_owned(),
                })
            }
        }
    }
}

pub fn evaluate_array(
    array: &Array,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut dyn IoContext,
) -> Result<Value, NalaRuntimeError> {
    let values = evaluate_elems(&array.elems, scopes, current_scope, context)?;

    if let Some(first) = values.clone().first() {
        let first_type = first.get_type();

        for value in values.clone() {
            if value.get_type() != first_type {
                let message = format!("Arrays can contain elements of only a single type. Found elements of types `{0}` and `{1}`.",
                first_type,
                value.get_type());

                return Err(NalaRuntimeError { message });
            }
        }
    };

    Ok(Value::Array(Arc::new(Mutex::new(values))))
}
