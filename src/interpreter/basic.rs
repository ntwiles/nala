use super::{arrays::*, branching::*, builtins::*, functions::*, operations::*, variables::*};

use crate::{
    ast::*,
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

pub fn interpret_block(
    block: &Block,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Term {
    match block {
        Block::NalaBlock(stmts) => interpret_stmts(stmts, scopes, current_scope, context),
        Block::RustBlock(params, func) => {
            invoke_builtin(*func, params, scopes, current_scope, context)
        }
    }
}

pub fn interpret_stmts(
    stmts: &Stmts,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Term {
    match stmts {
        Stmts::Stmts(stmts, stmt) => {
            let result = interpret_stmts(&*stmts, scopes, current_scope, context);

            if let Term::Void = result {
                interpret_stmt(stmt, scopes, current_scope, context)
            } else {
                result
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
) -> Term {
    match stmt {
        Stmt::Declare(ident, expr, is_mutable) => {
            let term = evaluate_expr(expr, scopes, current_scope, context);
            interpret_declare(ident, &term, scopes, current_scope, is_mutable.clone())
        }
        Stmt::Assign(ident, expr) => {
            let term = evaluate_expr(expr, scopes, current_scope, context);
            interpret_assign(ident, &term, scopes, current_scope, context)
        }
        Stmt::If(cond, block) => interpret_if(cond, block, scopes, current_scope, context),
        Stmt::For(ident, expr, block) => {
            interpret_for(ident, &expr, block, scopes, current_scope, context)
        }
        Stmt::Wiles(expr, block) => interpret_wiles(&expr, block, scopes, current_scope, context),
        Stmt::Func(ident, params, block) => {
            interpret_func(ident, params, block, scopes, current_scope)
        }
        Stmt::Expr(expr) => evaluate_expr(expr, scopes, current_scope, context),
        Stmt::Break(expr) => Term::Break(Box::new(expr.clone())),
        Stmt::Enum(ident, kinds) => Term::Void,
    }
}

pub fn evaluate_expr(
    expr: &Expr,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Term {
    match expr {
        Expr::Eq(left, right) => {
            let left = evaluate_expr(left, scopes, current_scope, context);
            let right = evaluate_addend(right, scopes, current_scope, context);
            evaluate_equals(left, right, scopes, current_scope)
        }
        Expr::Gt(left, right) => {
            let left = evaluate_expr(left, scopes, current_scope, context);
            let right = evaluate_addend(right, scopes, current_scope, context);
            evaluate_gt(left, right, scopes, current_scope)
        }
        Expr::Lt(left, right) => {
            let left = evaluate_expr(left, scopes, current_scope, context);
            let right = evaluate_addend(right, scopes, current_scope, context);
            evaluate_lt(left, right, scopes, current_scope)
        }
        Expr::Addend(addend) => evaluate_addend(addend, scopes, current_scope, context),
        Expr::Array(elems) => evaluate_array(elems, scopes, current_scope, context),
    }
}

pub fn evaluate_elems(
    elems: &Elems,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Vec<Term> {
    match elems {
        Elems::Elems(elems, expr) => {
            let mut elems = evaluate_elems(elems, scopes, current_scope, context);
            elems.push(evaluate_expr(&expr, scopes, current_scope, context));
            elems
        }
        Elems::Expr(expr) => vec![evaluate_expr(&expr, scopes, current_scope, context)],
        Elems::Empty => vec![],
    }
}
