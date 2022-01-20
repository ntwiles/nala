use super::basic::*;

use crate::{
    ast::*,
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

pub fn interpret_print(
    expr: &Expr,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Term {
    let result = evaluate_expr(&expr, scopes, current_scope, context);

    if let Term::Symbol(ident) = result {
        context.print(&scopes.get_value(&ident, current_scope).to_string());
    } else {
        context.print(&result.to_string());
    }

    Term::Void
}

pub fn evaluate_read(context: &mut impl IoContext) -> Term {
    let input = context.read();
    Term::String(input.trim().to_string())
}

pub fn evaluate_readnum(context: &mut impl IoContext) -> Term {
    let mut input = context.read();
    input = input.trim().to_string();
    let result = input.parse::<f32>();
    match result {
        Ok(num) => Term::Num(num),
        Err(_) => panic!("Could not parse input '{}' as type Num.", input),
    }
}
