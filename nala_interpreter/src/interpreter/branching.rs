use super::basic::*;
use std::sync::Arc;

use crate::{
    ast::{
        branching::{Else, ElseIf, IfElseChain},
        terms::*,
        *,
    },
    errors::NalaRuntimeError,
    io_context::IoContext,
    scopes::Scopes,
};

pub fn eval_if_else_chain(
    chain: &IfElseChain,
    scopes: &mut Scopes,
    current_scope: usize,
    enclosing_scope: Option<usize>,
    ctx: &mut dyn IoContext,
) -> Result<Value, NalaRuntimeError> {
    let IfElseChain {
        cond,
        block,
        else_ifs,
        else_block,
    } = chain;

    if eval_cond(cond, scopes, current_scope, enclosing_scope, ctx)? {
        let block_scope = scopes.new_scope(Some(current_scope));
        return eval_block(&block, scopes, block_scope, ctx);
    }

    for else_if in else_ifs.iter() {
        let ElseIf { cond, block } = else_if;

        if eval_cond(cond, scopes, current_scope, enclosing_scope, ctx)? {
            let block_scope = scopes.new_scope(Some(current_scope));
            return eval_block(&block, scopes, block_scope, ctx);
        }
    }

    if let Some(else_block) = else_block {
        let Else { block } = else_block;
        let block_scope = scopes.new_scope(Some(current_scope));
        return eval_block(&block, scopes, block_scope, ctx);
    }

    Ok(Value::Void)
}

fn eval_cond(
    cond: &Expr,
    scopes: &mut Scopes,
    current_scope: usize,
    enclosing_scope: Option<usize>,
    ctx: &mut dyn IoContext,
) -> Result<bool, NalaRuntimeError> {
    if let Value::Bool(cond) = eval_expr(cond, scopes, current_scope, enclosing_scope, ctx)? {
        Ok(cond)
    } else {
        Err(NalaRuntimeError {
            message: "Cannot use non-boolean expressions inside 'if' conditions.".to_owned(),
        })
    }
}

pub fn eval_for(
    ident: &String,
    expr: &Expr,
    block: &Block,
    scopes: &mut Scopes,
    current_scope: usize,
    enclosing_scope: Option<usize>,
    ctx: &mut dyn IoContext,
) -> Result<Value, NalaRuntimeError> {
    let result = eval_expr(expr, scopes, current_scope, enclosing_scope, ctx)?;

    let mut loop_result = Value::Void;

    if let Value::Array(array) = result {
        let array = Arc::clone(&array);
        let array = array.lock().unwrap();

        for (_, item) in array.iter().enumerate() {
            let block_scope = scopes.new_scope(Some(current_scope));

            scopes.add_binding(ident, block_scope, item.clone(), false)?;

            loop_result = eval_block(&block, scopes, block_scope, ctx)?;

            if let Value::Break(value) = loop_result {
                return Ok(*value);
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
    current_scope: usize,
    enclosing_scope: Option<usize>,
    ctx: &mut dyn IoContext,
) -> Result<Value, NalaRuntimeError> {
    loop {
        let result = eval_expr(expr, scopes, current_scope, enclosing_scope, ctx)?;

        let condition = if let Value::Bool(condition) = result {
            condition
        } else {
            panic!("Wiles condition must resolve to a value of type Bool");
        };

        if condition {
            let result = eval_block(block, scopes, current_scope, ctx)?;

            if let Value::Break(value) = result {
                return Ok(*value);
            }
        } else {
            break;
        }
    }

    Ok(Value::Void)
}

pub fn eval_break(
    expr: &Expr,
    scopes: &mut Scopes,
    current_scope: usize,
    enclosing_scope: Option<usize>,
    ctx: &mut dyn IoContext,
) -> Result<Value, NalaRuntimeError> {
    let val = eval_expr(expr, scopes, current_scope, enclosing_scope, ctx)?;
    Ok(Value::Break(Box::new(val)))
}
