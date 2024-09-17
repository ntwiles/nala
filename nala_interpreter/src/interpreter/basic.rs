use super::{
    branching::*,
    enums::eval_enum_variant,
    functions::*,
    operations::{equals::*, gt::*, lt::*, *},
    types::{eval_enum, eval_struct},
    variables::*,
};

use crate::{
    ast::*, errors::RuntimeError, io_context::IoContext, resolved::value::Value, scopes::Scopes,
};

pub fn eval_lines(
    lines: &Vec<Line>,
    scopes: &mut Scopes,
    current_scope: usize,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    // TODO: There's probably a neater way of implementing this function.

    let mut last_result = Value::Void;

    for line in lines.iter() {
        last_result = eval_line(line, scopes, current_scope, ctx)?;

        if let Value::Void = last_result {
            continue;
        } else {
            return Ok(last_result);
        }
    }

    Ok(last_result)
}

fn eval_line(
    line: &Line,
    scopes: &mut Scopes,
    current_scope: usize,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    match line {
        Line::Assign(ident, expr) => {
            let result = eval_expr(expr, scopes, current_scope, ctx)?;
            eval_assign(ident, &result, scopes, current_scope, ctx)
        }
        Line::Break(expr) => eval_break(expr, scopes, current_scope, ctx),
        Line::Declare(ident, expr, declared_type, is_mutable) => eval_declare(
            ident,
            eval_expr(&expr, scopes, current_scope, ctx)?,
            declared_type.clone(),
            is_mutable.clone(),
            scopes,
            current_scope,
        ),
        Line::Enum(ident, type_params, variants) => eval_enum(
            ident,
            type_params.clone(),
            variants.clone(),
            scopes,
            current_scope,
        ),
        Line::Expr(expr) => eval_expr(expr, scopes, current_scope, ctx),
        Line::For(ident, expr, block) => eval_for(ident, &expr, block, scopes, current_scope, ctx),
        Line::Func(func) => eval_func_declare(func.clone(), scopes, current_scope),
        Line::IfElseChain(chain) => eval_if_else_chain(chain, scopes, current_scope, ctx),
        Line::Match(the_match) => eval_match(the_match, scopes, current_scope, ctx),
        Line::Struct(ident, type_params, fields) => eval_struct(
            ident,
            type_params.clone(),
            fields.clone(),
            scopes,
            current_scope,
        ),
        Line::Wiles(expr, block) => eval_wiles(&expr, block, scopes, current_scope, ctx),
    }
}

pub fn eval_expr(
    expr: &Expr,
    scopes: &mut Scopes,
    current_scope: usize,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    match expr {
        Expr::EnumVariant(variant) => eval_enum_variant(variant, scopes, current_scope, ctx),
        Expr::Eq(left, right) => {
            let left = eval_expr(left, scopes, current_scope, ctx)?;
            let right = eval_enum_variant(right, scopes, current_scope, ctx)?;

            eval_equals(left, right, scopes, current_scope)
        }
        Expr::Gt(left, right) => {
            let left = eval_expr(left, scopes, current_scope, ctx)?;
            let right = eval_addend(right, scopes, current_scope, ctx)?;

            eval_gt(left, right, scopes, current_scope)
        }
        Expr::Lt(left, right) => {
            let left = eval_expr(left, scopes, current_scope, ctx)?;
            let right = eval_addend(right, scopes, current_scope, ctx)?;

            eval_lt(left, right, scopes, current_scope)
        }
    }
}

pub fn eval_elems(
    elems: &Vec<Expr>,
    scopes: &mut Scopes,
    current_scope: usize,
    ctx: &mut dyn IoContext,
) -> Result<Vec<Value>, RuntimeError> {
    let results: Vec<Result<Value, RuntimeError>> = elems
        .iter()
        .map(|e| eval_expr(e, scopes, current_scope, ctx))
        .collect();

    if let Some(Err(err)) = results.iter().find(|r| r.is_err()) {
        Err(err.clone())
    } else {
        Ok(results.into_iter().map(|r| r.unwrap()).collect())
    }
}
