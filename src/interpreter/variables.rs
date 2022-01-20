use super::basic::*;

use crate::{
    ast::*,
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

pub fn interpret_declare(
    ident: &String,
    expr: &Expr,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
    is_mutable: bool,
) -> Term {
    if scopes.binding_exists_local(&ident, current_scope) {
        panic!("Binding for {} already exists in local scope.", ident);
    } else {
        let value = evaluate_expr(&expr, scopes, current_scope, context);

        if let Term::Void = value {
            panic!("Cannot assign Void.");
        }

        scopes.add_binding(&ident, current_scope, value, is_mutable);
    }

    Term::Void
}

pub fn interpret_assign(
    ident: &String,
    expr: &Expr,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Term {
    println!("Interpreting assignment!");

    if scopes.binding_exists(&ident, current_scope) {
        let value = evaluate_expr(&expr, scopes, current_scope, context);

        if let Term::Void = value {
            panic!("Cannot assign Void.");
        }

        scopes.mutate_value(&ident, current_scope, value);
    } else {
        panic!("Unknown identifier `{}`", ident);
    }

    Term::Void
}
