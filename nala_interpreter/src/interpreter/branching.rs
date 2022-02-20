use super::basic::*;
use std::sync::Arc;

use crate::{
    ast::{terms::*, *},
    errors::NalaRuntimeError,
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

pub fn interpret_if(
    cond: &Expr,
    block: &Block,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut dyn IoContext,
) -> Result<Value, NalaRuntimeError> {
    let result = evaluate_expr(&cond, scopes, current_scope, context)?;

    if let Value::Bool(bool) = result {
        if bool {
            let block_scope = scopes.new_scope(Some(current_scope));
            interpret_block(&block, scopes, block_scope, context)
        } else {
            Ok(Value::Void)
        }
    } else {
        panic!("Cannot use non-boolean expressions inside 'if' conditions.")
    }
}

pub fn interpret_for(
    ident: &String,
    expr: &Expr,
    block: &Block,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut dyn IoContext,
) -> Result<Value, NalaRuntimeError> {
    let result = evaluate_expr(expr, scopes, current_scope, context)?;

    let mut loop_result = Value::Void;

    if let Value::Array(array) = result {
        let array = Arc::clone(&array);
        let array = array.lock().unwrap();

        for (_, item) in array.iter().enumerate() {
            let block_scope = scopes.new_scope(Some(current_scope));
            scopes.add_binding(ident, block_scope, item.clone(), false);

            loop_result = interpret_block(&block, scopes, block_scope, context)?;

            if let Value::Break(expr) = loop_result {
                return evaluate_expr(&*expr, scopes, current_scope, context);
            }
        }

        Ok(loop_result)
    } else {
        panic!(
            "Cannot iterate over values of non-Array types. Found '{}' of type {:?}",
            ident, loop_result
        )
    }
}

pub fn interpret_wiles(
    expr: &Expr,
    block: &Block,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut dyn IoContext,
) -> Result<Value, NalaRuntimeError> {
    loop {
        let result = evaluate_expr(expr, scopes, current_scope, context)?;

        let condition = if let Value::Bool(condition) = result {
            condition
        } else {
            panic!("Wiles condition must resolve to a value of type Bool");
        };

        if condition {
            let result = interpret_block(block, scopes, current_scope, context)?;

            if let Value::Break(expr) = result {
                return evaluate_expr(&*expr, scopes, current_scope, context);
            }
        } else {
            break;
        }
    }

    Ok(Value::Void)
}
