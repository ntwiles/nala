use super::{
    arrays::*,
    branching::*,
    functions::*,
    objects::*,
    operations::{equals::*, gt::*, lt::*, *},
    variables::*,
};

use crate::{
    ast::{terms::*, *},
    errors::NalaRuntimeError,
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

pub fn interpret_block(
    block: &Block,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut dyn IoContext,
) -> Result<Value, NalaRuntimeError> {
    if let Block::NalaBlock(stmts) = block {
        interpret_stmts(stmts, scopes, current_scope, context)
    } else {
        // TODO: If we accept a Block as a param, probably all variants should be valid arguments.
        panic!("Do not pass Rust blocks to interpret_block")
    }
}

pub fn interpret_stmts(
    stmts: &Stmts,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut dyn IoContext,
) -> Result<Value, NalaRuntimeError> {
    match stmts {
        Stmts::Stmts(stmts, stmt) => {
            let result = interpret_stmts(&*stmts, scopes, current_scope, context)?;

            if let Value::Void = result {
                interpret_stmt(stmt, scopes, current_scope, context)
            } else {
                Ok(result)
            }
        }
        Stmts::Stmt(stmt) => interpret_stmt(stmt, scopes, current_scope, context),
    }
}

fn interpret_stmt(
    stmt: &Stmt,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut dyn IoContext,
) -> Result<Value, NalaRuntimeError> {
    match stmt {
        Stmt::Declare(ident, expr, is_mutable) => {
            let result = evaluate_expr(expr, scopes, current_scope, context)?;
            interpret_declare(ident, &result, scopes, current_scope, is_mutable.clone())
        }
        Stmt::Assign(ident, expr) => {
            let result = evaluate_expr(expr, scopes, current_scope, context)?;
            interpret_assign(ident, &result, scopes, current_scope, context)
        }
        Stmt::If(cond, block) => interpret_if(cond, block, scopes, current_scope, context),
        Stmt::For(ident, expr, block) => {
            interpret_for(ident, &expr, block, scopes, current_scope, context)
        }
        Stmt::Wiles(expr, block) => interpret_wiles(&expr, block, scopes, current_scope, context),
        Stmt::Func(func) => interpret_func(func, scopes, current_scope),
        Stmt::Expr(expr) => evaluate_expr(expr, scopes, current_scope, context),
        Stmt::Break(expr) => Ok(Value::Break(Box::new(expr.clone()))),
    }
}

pub fn evaluate_expr(
    expr: &Expr,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut dyn IoContext,
) -> Result<Value, NalaRuntimeError> {
    match expr {
        Expr::Addend(addend) => evaluate_addend(addend, scopes, current_scope, context),
        Expr::Eq(left, right) => {
            let left = evaluate_expr(left, scopes, current_scope, context)?;
            let right = evaluate_addend(right, scopes, current_scope, context)?;

            Ok(evaluate_equals(left, right))
        }
        Expr::Gt(left, right) => {
            let left = evaluate_expr(left, scopes, current_scope, context)?;
            let right = evaluate_addend(right, scopes, current_scope, context)?;

            evaluate_gt(left, right)
        }
        Expr::Lt(left, right) => {
            let left = evaluate_expr(left, scopes, current_scope, context)?;
            let right = evaluate_addend(right, scopes, current_scope, context)?;

            evaluate_lt(left, right)
        }
        Expr::Array(elems) => evaluate_array(elems, scopes, current_scope, context),
        Expr::Object(object) => evaluate_object(object, scopes, current_scope, context),
    }
}

pub fn evaluate_elems(
    elems: &Vec<Expr>,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut dyn IoContext,
) -> Result<Vec<Value>, NalaRuntimeError> {
    let results: Vec<Result<Value, NalaRuntimeError>> = elems
        .iter()
        .map(|e| evaluate_expr(e, scopes, current_scope, context))
        .collect();

    if let Some(Err(err)) = results.iter().find(|r| r.is_err()) {
        Err(err.clone())
    } else {
        Ok(results.into_iter().map(|r| r.unwrap()).collect())
    }
}
