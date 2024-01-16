use super::{
    arrays::*,
    branching::*,
    enums::eval_enum_variant,
    functions::*,
    objects::*,
    operations::{equals::*, gt::*, lt::*, *},
    types::{eval_enum, eval_struct},
    variables::*,
};

use crate::{
    ast::*, errors::RuntimeError, io_context::IoContext, resolved::value::Value, scopes::Scopes,
};

pub fn eval_stmts(
    stmts: &Vec<Stmt>,
    scopes: &mut Scopes,
    current_scope: usize,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    // TODO: There's probably a neater way of implementing this function.

    let mut last_result = Value::Void;

    for stmt in stmts.iter() {
        last_result = eval_stmt(stmt, scopes, current_scope, ctx)?;

        if let Value::Void = last_result {
            continue;
        } else {
            return Ok(last_result);
        }
    }

    Ok(last_result)
}

fn eval_stmt(
    stmt: &Stmt,
    scopes: &mut Scopes,
    current_scope: usize,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    match stmt {
        Stmt::Assign(ident, expr) => {
            let result = eval_expr(expr, scopes, current_scope, ctx)?;
            eval_assign(ident, &result, scopes, current_scope, ctx)
        }
        Stmt::Break(expr) => eval_break(expr, scopes, current_scope, ctx),
        Stmt::Declare(ident, expr, declared_type, is_mutable) => eval_declare(
            ident,
            eval_expr(&expr, scopes, current_scope, ctx)?,
            declared_type.clone(),
            is_mutable.clone(),
            scopes,
            current_scope,
        ),
        Stmt::Enum(ident, type_params, variants) => eval_enum(
            ident,
            type_params.clone(),
            variants.clone(),
            scopes,
            current_scope,
        ),
        Stmt::Expr(expr) => eval_expr(expr, scopes, current_scope, ctx),
        Stmt::For(ident, expr, block) => eval_for(ident, &expr, block, scopes, current_scope, ctx),
        Stmt::Func(func) => eval_func_declare(func.clone(), scopes, current_scope),
        Stmt::IfElseChain(chain) => eval_if_else_chain(chain, scopes, current_scope, ctx),
        Stmt::Match(the_match) => eval_match(the_match, scopes, current_scope, ctx),
        Stmt::Struct(ident, type_params, fields) => eval_struct(
            ident,
            type_params.clone(),
            fields.clone(),
            scopes,
            current_scope,
        ),
        Stmt::Wiles(expr, block) => eval_wiles(&expr, block, scopes, current_scope, ctx),
    }
}

pub fn eval_expr(
    expr: &Expr,
    scopes: &mut Scopes,
    current_scope: usize,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    match expr {
        Expr::Array(elems) => eval_array(elems, scopes, current_scope, ctx),
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

        Expr::Object(object) => eval_object(object, scopes, current_scope, ctx),
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
