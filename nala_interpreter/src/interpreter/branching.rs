use super::basic::*;
use std::sync::Arc;

use crate::{
    ast::{
        branching::{Else, ElseIf, IfElseChain, Match, MatchCase},
        *,
    },
    errors::RuntimeError,
    io_context::IoContext,
    resolved::value::Value,
    scopes::Scopes,
    types::inference::infer_type,
};

pub fn eval_if_else_chain(
    chain: &IfElseChain,
    scopes: &mut Scopes,
    current_scope: usize,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    let IfElseChain {
        cond,
        block,
        else_ifs,
        else_block,
    } = chain;

    if eval_cond(cond, scopes, current_scope, ctx)? {
        let block_scope = scopes.new_scope(Some(current_scope));
        return eval_lines(&block, scopes, block_scope, ctx);
    }

    for else_if in else_ifs.iter() {
        let ElseIf { cond, block } = else_if;

        if eval_cond(cond, scopes, current_scope, ctx)? {
            let block_scope = scopes.new_scope(Some(current_scope));
            return eval_lines(block, scopes, block_scope, ctx);
        }
    }

    if let Some(else_block) = else_block {
        let Else { block } = else_block;
        let block_scope = scopes.new_scope(Some(current_scope));
        return eval_lines(&block, scopes, block_scope, ctx);
    }

    Ok(Value::Void)
}

fn eval_cond(
    cond: &Expr,
    scopes: &mut Scopes,
    current_scope: usize,
    ctx: &mut dyn IoContext,
) -> Result<bool, RuntimeError> {
    if let Value::Bool(cond) = eval_expr(cond, scopes, current_scope, ctx)? {
        Ok(cond)
    } else {
        Err(RuntimeError::new(
            "Cannot use non-boolean expressions inside 'if' conditions.",
        ))
    }
}

pub fn eval_for(
    ident: &String,
    expr: &Expr,
    block: &Vec<Line>,
    scopes: &mut Scopes,
    current_scope: usize,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    let result = eval_expr(expr, scopes, current_scope, ctx)?;

    let mut loop_result = Value::Void;

    if let Value::Array(array) = result {
        let array = Arc::clone(&array);
        let array = array.lock().unwrap();

        for (_, item) in array.iter().enumerate() {
            let block_scope = scopes.new_scope(Some(current_scope));

            scopes.add_binding(ident, item.clone(), None, block_scope, false)?;

            loop_result = eval_lines(&block, scopes, block_scope, ctx)?;

            if let Value::Break(value) = loop_result {
                return Ok(*value);
            }
        }

        Ok(loop_result)
    } else {
        Err(RuntimeError::new(&format!(
            "Cannot iterate over values of non-Array types. Found '{result}' of type `{}`",
            infer_type(&result, scopes, current_scope)?
        )))
    }
}

pub fn eval_wiles(
    expr: &Expr,
    block: &Vec<Line>,
    scopes: &mut Scopes,
    current_scope: usize,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    loop {
        let result = eval_expr(expr, scopes, current_scope, ctx)?;

        let condition = if let Value::Bool(condition) = result {
            condition
        } else {
            Err(RuntimeError::new(
                "Wiles condition must resolve to a value of type Bool",
            ))?
        };

        if condition {
            let result = eval_lines(block, scopes, current_scope, ctx)?;

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
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    let val = eval_expr(expr, scopes, current_scope, ctx)?;
    Ok(Value::Break(Box::new(val)))
}

pub fn eval_match(
    the_match: &Match,
    scopes: &mut Scopes,
    current_scope: usize,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    let Match { expr, cases } = the_match;

    let expr = eval_expr(expr, scopes, current_scope, ctx)?;

    // TODO: Throw error if not all cases are covered.

    for case in cases.iter() {
        let MatchCase { pattern, block } = case;

        if let Some(bindings) = pattern.matches(&expr) {
            let block_scope = scopes.new_scope(Some(current_scope));

            for (ident, value) in bindings.iter() {
                scopes.add_binding(ident, value.clone(), None, block_scope, false)?;
            }

            return eval_lines(&block, scopes, block_scope, ctx);
        }
    }

    Ok(Value::Void)
}
