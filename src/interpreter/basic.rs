use super::{
    arrays::*,
    branching::*,
    enums::*,
    functions::*,
    operations::{equals::*, gt::*, lt::*, *},
    variables::*,
};

use crate::{
    ast::{arrays::*, terms::*, *},
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

pub fn interpret_block(
    block: &Block,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Result<Term, Term> {
    if let Block::NalaBlock(stmts) = block {
        interpret_stmts(stmts, scopes, current_scope, context)
    } else {
        // TODO: If we accept a Block as a param, probably all variants should
        // be valid arguments.
        panic!("Do not pass Rust blocks to interpret_block")
    }
}

pub fn interpret_stmts(
    stmts: &Stmts,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Result<Term, Term> {
    match stmts {
        Stmts::Stmts(stmts, stmt) => {
            let result = interpret_stmts(&*stmts, scopes, current_scope, context);

            if let Err(e) = result {
                return Err(e);
            }

            let result = result.unwrap();

            if let Term::Void = result {
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
    context: &mut impl IoContext,
) -> Result<Term, Term> {
    match stmt {
        Stmt::Declare(ident, expr, is_mutable) => {
            let result = evaluate_expr(expr, scopes, current_scope, context);

            if result.is_err() {
                return result;
            }

            interpret_declare(
                ident,
                &result.unwrap(),
                scopes,
                current_scope,
                is_mutable.clone(),
            )
        }
        Stmt::Assign(ident, expr) => {
            let result = evaluate_expr(expr, scopes, current_scope, context);

            if result.is_err() {
                return result;
            }

            interpret_assign(ident, &result.unwrap(), scopes, current_scope, context)
        }
        Stmt::If(cond, block) => interpret_if(cond, block, scopes, current_scope, context),
        Stmt::For(ident, expr, block) => {
            interpret_for(ident, &expr, block, scopes, current_scope, context)
        }
        Stmt::Wiles(expr, block) => interpret_wiles(&expr, block, scopes, current_scope, context),
        Stmt::Func(func) => interpret_func(func, scopes, current_scope),
        Stmt::Expr(expr) => evaluate_expr(expr, scopes, current_scope, context),
        Stmt::Break(expr) => Ok(Term::Break(Box::new(expr.clone()))),
        Stmt::Enum(ident, kinds) => interpret_enum(ident, kinds, scopes, current_scope),
    }
}

pub fn evaluate_expr(
    expr: &Expr,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Result<Term, Term> {
    match expr {
        Expr::Eq(left, right) => {
            let left = evaluate_expr(left, scopes, current_scope, context);
            let right = evaluate_kind(right, scopes, current_scope, context);

            if left.is_err() {
                return left;
            }
            if right.is_err() {
                return right;
            }

            Ok(evaluate_equals(left.unwrap(), right.unwrap()))
        }
        Expr::Gt(left, right) => {
            let left = evaluate_expr(left, scopes, current_scope, context);
            let right = evaluate_addend(right, scopes, current_scope, context);

            if let Err(e) = left {
                return Err(e);
            }

            if let Err(e) = right {
                return Err(e);
            }

            Ok(evaluate_gt(left.unwrap(), right.unwrap()))
        }
        Expr::Lt(left, right) => {
            let left = evaluate_expr(left, scopes, current_scope, context);
            let right = evaluate_addend(right, scopes, current_scope, context);

            if let Err(e) = left {
                return Err(e);
            }

            if let Err(e) = right {
                return Err(e);
            }

            Ok(evaluate_lt(left.unwrap(), right.unwrap()))
        }
        Expr::Array(elems) => evaluate_array(elems, scopes, current_scope, context),
        Expr::KindValue(kind) => evaluate_kind(kind, scopes, current_scope, context),
    }
}

pub fn evaluate_elems(
    elems: &Elems,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Result<Vec<Term>, Term> {
    match elems {
        Elems::Elems(elems, expr) => {
            let elems_result = evaluate_elems(elems, scopes, current_scope, context);
            let result = evaluate_expr(&expr, scopes, current_scope, context);

            if elems_result.is_err() {
                return elems_result;
            }

            if let Err(e) = result {
                return Err(e);
            }

            let mut elems = elems_result.unwrap();

            elems.push(result.unwrap());
            Ok(elems)
        }
        Elems::Expr(expr) => {
            let result = evaluate_expr(&expr, scopes, current_scope, context);

            if let Err(e) = result {
                return Err(e);
            }

            Ok(vec![result.unwrap()])
        }
        Elems::Empty => Ok(vec![]),
    }
}
