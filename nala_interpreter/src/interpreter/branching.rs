use super::basic::*;
use std::sync::Arc;

use crate::{
    ast::{terms::*, *},
    errors::NalaRuntimeError,
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

pub fn eval_if(
    cond: &Expr,
    block: &Block,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    ctx: &mut dyn IoContext,
) -> Result<Value, NalaRuntimeError> {
    let result = eval_expr(&cond, scopes, current_scope, ctx)?;

    if let Value::Bool(bool) = result {
        if bool {
            let block_scope = scopes.new_scope(Some(current_scope));
            eval_block(&block, scopes, block_scope, ctx)
        } else {
            Ok(Value::Void)
        }
    } else {
        panic!("Cannot use non-boolean expressions inside 'if' conditions.")
    }
}

pub fn eval_for(
    ident: &String,
    expr: &Expr,
    block: &Block,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    ctx: &mut dyn IoContext,
) -> Result<Value, NalaRuntimeError> {
    let result = eval_expr(expr, scopes, current_scope, ctx)?;

    let mut loop_result = Value::Void;

    if let Value::Array(array) = result {
        let array = Arc::clone(&array);
        let array = array.lock().unwrap();

        for (_, item) in array.iter().enumerate() {
            let block_scope = scopes.new_scope(Some(current_scope));
            scopes.add_binding(ident, block_scope, item.clone(), false);

            loop_result = eval_block(&block, scopes, block_scope, ctx)?;

            if let Value::Break(expr) = loop_result {
                return eval_expr(&*expr, scopes, current_scope, ctx);
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

pub fn eval_wiles(
    expr: &Expr,
    block: &Block,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    ctx: &mut dyn IoContext,
) -> Result<Value, NalaRuntimeError> {
    loop {
        let result = eval_expr(expr, scopes, current_scope, ctx)?;

        let condition = if let Value::Bool(condition) = result {
            condition
        } else {
            panic!("Wiles condition must resolve to a value of type Bool");
        };

        if condition {
            let result = eval_block(block, scopes, current_scope, ctx)?;

            if let Value::Break(expr) = result {
                return eval_expr(&*expr, scopes, current_scope, ctx);
            }
        } else {
            break;
        }
    }

    Ok(Value::Void)
}
