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
    ast::{terms::*, *},
    errors::RuntimeError,
    io_context::IoContext,
    scopes::Scopes,
};

pub fn eval_block(
    block: &Block,
    scopes: &mut Scopes,
    current_scope: usize,
    enclosing_scope: Option<usize>,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    if let Block::NalaBlock(stmts) = block {
        eval_stmts(stmts, scopes, current_scope, enclosing_scope, ctx)
    } else {
        // TODO: If we accept a Block as a param, probably all variants should be valid arguments.
        panic!("Do not pass Rust blocks to eval_block")
    }
}

pub fn eval_stmts(
    stmts: &Stmts,
    scopes: &mut Scopes,
    current_scope: usize,
    enclosing_scope: Option<usize>,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    match stmts {
        Stmts::Stmts(stmts, stmt) => {
            let result = eval_stmts(&*stmts, scopes, current_scope, enclosing_scope, ctx)?;

            if let Value::Void = result {
                eval_stmt(stmt, scopes, current_scope, enclosing_scope, ctx)
            } else {
                Ok(result)
            }
        }
        Stmts::Stmt(stmt) => eval_stmt(stmt, scopes, current_scope, enclosing_scope, ctx),
    }
}

fn eval_stmt(
    stmt: &Stmt,
    scopes: &mut Scopes,
    current_scope: usize,
    enclosing_scope: Option<usize>,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    match stmt {
        Stmt::Assign(ident, expr) => {
            let result = eval_expr(expr, scopes, current_scope, enclosing_scope, ctx)?;
            eval_assign(ident, &result, scopes, current_scope, enclosing_scope, ctx)
        }
        Stmt::Break(expr) => eval_break(expr, scopes, current_scope, enclosing_scope, ctx),
        Stmt::Declare(ident, expr, declared_type, is_mutable) => eval_declare(
            ident,
            &expr,
            declared_type.clone(),
            is_mutable.clone(),
            scopes,
            current_scope,
            enclosing_scope,
            ctx,
        ),
        Stmt::Enum(ident, type_args, variants) => eval_enum(
            ident,
            type_args.clone(),
            variants.clone(),
            scopes,
            current_scope,
        ),
        Stmt::Expr(expr) => eval_expr(expr, scopes, current_scope, enclosing_scope, ctx),
        Stmt::For(ident, expr, block) => eval_for(
            ident,
            &expr,
            block,
            scopes,
            current_scope,
            enclosing_scope,
            ctx,
        ),
        Stmt::Func(func) => eval_func(func.clone(), scopes, current_scope), // TODO: Do we need this clone?
        Stmt::IfElseChain(chain) => {
            eval_if_else_chain(chain, scopes, current_scope, enclosing_scope, ctx)
        }
        Stmt::Match(the_match) => {
            eval_match(the_match, scopes, current_scope, enclosing_scope, ctx)
        }
        Stmt::Struct(ident, type_args, fields) => eval_struct(
            ident,
            type_args.clone(),
            fields.clone(),
            scopes,
            current_scope,
        ),
        Stmt::Wiles(expr, block) => {
            eval_wiles(&expr, block, scopes, current_scope, enclosing_scope, ctx)
        }
    }
}

pub fn eval_expr(
    expr: &Expr,
    scopes: &mut Scopes,
    current_scope: usize,
    enclosing_scope: Option<usize>,
    ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    match expr {
        Expr::Array(elems) => eval_array(elems, scopes, current_scope, enclosing_scope, ctx),
        Expr::EnumVariant(variant) => {
            eval_enum_variant(variant, scopes, current_scope, enclosing_scope, ctx)
        }
        Expr::Eq(left, right) => {
            let left = eval_expr(left, scopes, current_scope, enclosing_scope, ctx)?;
            let right = eval_enum_variant(right, scopes, current_scope, enclosing_scope, ctx)?;

            eval_equals(left, right, scopes, current_scope)
        }
        Expr::Gt(left, right) => {
            let left = eval_expr(left, scopes, current_scope, enclosing_scope, ctx)?;
            let right = eval_addend(right, scopes, current_scope, enclosing_scope, ctx)?;

            eval_gt(left, right, scopes, current_scope)
        }
        Expr::Lt(left, right) => {
            let left = eval_expr(left, scopes, current_scope, enclosing_scope, ctx)?;
            let right = eval_addend(right, scopes, current_scope, enclosing_scope, ctx)?;

            eval_lt(left, right, scopes, current_scope)
        }

        Expr::Object(object) => eval_object(object, scopes, current_scope, enclosing_scope, ctx),
    }
}

pub fn eval_elems(
    elems: &Vec<Expr>,
    scopes: &mut Scopes,
    current_scope: usize,
    enclosing_scope: Option<usize>,
    ctx: &mut dyn IoContext,
) -> Result<Vec<Value>, RuntimeError> {
    let results: Vec<Result<Value, RuntimeError>> = elems
        .iter()
        .map(|e| eval_expr(e, scopes, current_scope, enclosing_scope, ctx))
        .collect();

    if let Some(Err(err)) = results.iter().find(|r| r.is_err()) {
        Err(err.clone())
    } else {
        Ok(results.into_iter().map(|r| r.unwrap()).collect())
    }
}
