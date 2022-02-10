use super::basic::*;

use crate::{
    ast::{terms::*, *},
    errors::NalaRuntimeError,
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

pub fn interpret_if(
    cond: &Expr,
    block: &Block,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Result<Term, NalaRuntimeError> {
    let result = evaluate_expr(&cond, scopes, current_scope, context);

    if let Err(e) = result {
        return Err(e);
    }

    if let Term::Bool(bool) = result.unwrap() {
        if bool {
            let block_scope = scopes.new_scope(Some(current_scope));
            interpret_block(&block, scopes, block_scope, context)
        } else {
            Ok(Term::Void)
        }
    } else {
        panic!("Cannot use non-boolean expressions inside 'if' conditions.")
    }
}

pub fn interpret_for(
    ident: &String,
    expr: &Expr,
    block: &Block,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Result<Term, NalaRuntimeError> {
    let result = evaluate_expr(expr, scopes, current_scope, context);

    if result.is_err() {
        return result;
    }

    let mut loop_result = Term::Void;

    if let Term::Array(array) = result.unwrap() {
        for (_, item) in array.iter().enumerate() {
            let block_scope = scopes.new_scope(Some(current_scope));
            scopes.add_binding(ident, block_scope, item.clone(), false);

            let result = interpret_block(&block, scopes, block_scope, context);

            if let Err(e) = result {
                return Err(e);
            }

            loop_result = result.unwrap();

            if let Term::Break(expr) = loop_result {
                return evaluate_expr(&*expr, scopes, current_scope, context);
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

pub fn interpret_wiles(
    expr: &Expr,
    block: &Block,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Result<Term, NalaRuntimeError> {
    loop {
        let result = evaluate_expr(expr, scopes, current_scope, context);

        if result.is_err() {
            return result;
        }

        let condition = if let Term::Bool(condition) = result.unwrap() {
            condition
        } else {
            panic!("Wiles condition must resolve to a value of type Bool");
        };

        if condition {
            let result = interpret_block(block, scopes, current_scope, context);

            if let Err(e) = result {
                return Err(e);
            }

            if let Term::Break(expr) = result.unwrap() {
                return evaluate_expr(&*expr, scopes, current_scope, context);
            }
        } else {
            break;
        }
    }

    Ok(Term::Void)
}
