use super::{
    arrays::*,
    branching::*,
    functions::*,
    objects::*,
    operations::{equals::*, gt::*, lt::*, *},
    types::eval_struct,
    variables::*,
};

use crate::{
    ast::{terms::*, *},
    errors::NalaRuntimeError,
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

pub fn eval_block(
    block: &Block,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    ctx: &mut dyn IoContext,
) -> Result<Value, NalaRuntimeError> {
    if let Block::NalaBlock(stmts) = block {
        eval_stmts(stmts, scopes, current_scope, ctx)
    } else {
        // TODO: If we accept a Block as a param, probably all variants should be valid arguments.
        panic!("Do not pass Rust blocks to eval_block")
    }
}

pub fn eval_stmts(
    stmts: &Stmts,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    ctx: &mut dyn IoContext,
) -> Result<Value, NalaRuntimeError> {
    match stmts {
        Stmts::Stmts(stmts, stmt) => {
            let result = eval_stmts(&*stmts, scopes, current_scope, ctx)?;

            if let Value::Void = result {
                eval_stmt(stmt, scopes, current_scope, ctx)
            } else {
                Ok(result)
            }
        }
        Stmts::Stmt(stmt) => eval_stmt(stmt, scopes, current_scope, ctx),
    }
}

fn eval_stmt(
    stmt: &Stmt,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    ctx: &mut dyn IoContext,
) -> Result<Value, NalaRuntimeError> {
    match stmt {
        Stmt::Declare(ident, expr, is_mutable) => {
            let result = eval_expr(expr, scopes, current_scope, ctx)?;
            eval_declare(ident, &result, scopes, current_scope, is_mutable.clone())
        }
        Stmt::Assign(ident, expr) => {
            let result = eval_expr(expr, scopes, current_scope, ctx)?;
            eval_assign(ident, &result, scopes, current_scope, ctx)
        }
        Stmt::If(cond, block) => eval_if(cond, block, scopes, current_scope, ctx),
        Stmt::For(ident, expr, block) => eval_for(ident, &expr, block, scopes, current_scope, ctx),
        Stmt::Wiles(expr, block) => eval_wiles(&expr, block, scopes, current_scope, ctx),
        Stmt::Func(func) => eval_func(func, scopes, current_scope),
        Stmt::Expr(expr) => eval_expr(expr, scopes, current_scope, ctx),
        Stmt::Break(expr) => Ok(Value::Break(Box::new(expr.clone()))),
        Stmt::Struct(ident, fields) => eval_struct(ident, fields.clone(), scopes, current_scope),
    }
}

pub fn eval_expr(
    expr: &Expr,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    ctx: &mut dyn IoContext,
) -> Result<Value, NalaRuntimeError> {
    match expr {
        Expr::Addend(addend) => eval_addend(addend, scopes, current_scope, ctx),
        Expr::Eq(left, right) => {
            let left = eval_expr(left, scopes, current_scope, ctx)?;
            let right = eval_addend(right, scopes, current_scope, ctx)?;

            Ok(eval_equals(left, right, scopes, current_scope))
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
        Expr::Array(elems) => eval_array(elems, scopes, current_scope, ctx),
        Expr::Object(object) => eval_object(object, scopes, current_scope, ctx),
    }
}

pub fn eval_elems(
    elems: &Vec<Expr>,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    ctx: &mut dyn IoContext,
) -> Result<Vec<Value>, NalaRuntimeError> {
    let results: Vec<Result<Value, NalaRuntimeError>> = elems
        .iter()
        .map(|e| eval_expr(e, scopes, current_scope, ctx))
        .collect();

    if let Some(Err(err)) = results.iter().find(|r| r.is_err()) {
        Err(err.clone())
    } else {
        Ok(results.into_iter().map(|r| r.unwrap()).collect())
    }
}
