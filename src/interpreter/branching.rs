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
        if bool {
            let block_scope = scopes.new_scope(Some(current_scope));
            interpret_block(&block, scopes, block_scope, context);
        }
    } else {
        panic!("Cannot use non-boolean expressions inside 'if' conditions.")
    }

    Term::Void
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
        for (_, item) in array.iter().enumerate() {
            let block_scope = scopes.new_scope(Some(current_scope));
            scopes.add_binding(ident, block_scope, item.clone(), false);
            interpret_block(&block, scopes, block_scope, context);
        }
    } else {
        panic!(
            "Cannot iterate over values of non-Array types. Found '{}' of type {:?}",
            ident, resolved
        )
    }

    Term::Void
}
