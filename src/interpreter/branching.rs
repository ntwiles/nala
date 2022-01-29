use super::basic::*;

use crate::{
    ast::*,
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

// Todo: Consider returning something other than Void.
pub fn interpret_if(
    cond: &Expr,
    block: &Block,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Term {
    let resolved = evaluate_expr(&cond, scopes, current_scope, context);

    if let Term::Bool(bool) = resolved {
        let mut result = Term::Void;
        if bool {
            let block_scope = scopes.new_scope(Some(current_scope));
            result = interpret_block(&block, scopes, block_scope, context)
        }

        result
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
) -> Term {
    let resolved = evaluate_expr(expr, scopes, current_scope, context);

    if let Term::Array(array) = resolved {
        let mut result = Term::Void;

        for (_, item) in array.iter().enumerate() {
            let block_scope = scopes.new_scope(Some(current_scope));
            scopes.add_binding(ident, block_scope, item.clone(), false);
            result = interpret_block(&block, scopes, block_scope, context);

            if let Term::Break(expr) = result {
                return evaluate_expr(&*expr, scopes, current_scope, context);
            }
        }

        result
    } else {
        panic!(
            "Cannot iterate over values of non-Array types. Found '{}' of type {:?}",
            ident, resolved
        )
    }
}

pub fn interpret_wiles(
    expr: &Expr,
    block: &Block,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Term {
    loop {
        let condition = evaluate_expr(expr, scopes, current_scope, context);
        let condition = if let Term::Bool(condition) = condition {
            condition
        } else {
            panic!("Wiles condition must resolve to a value of type Bool");
        };

        if condition {
            interpret_block(block, scopes, current_scope, context);
        } else {
            break;
        }
    }

    Term::Void
}
