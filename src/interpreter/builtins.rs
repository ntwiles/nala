use super::{basic::*, io::*};

use crate::{
    ast::*,
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

pub fn evaluate_builtin(
    builtin: &Builtin,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Term {
    match builtin {
        Builtin::Read => evaluate_read(context),
        Builtin::ReadNum => evaluate_readnum(context),
        Builtin::Len(expr) => evaluate_len(expr, scopes, current_scope, context),
        Builtin::Floor(expr) => evaluate_floor(expr, scopes, current_scope, context),
        Builtin::Term(term) => {
            if let Term::Symbol(ident) = term {
                scopes.get_value(ident, current_scope)
            } else {
                term.clone()
            }
        }
    }
}

pub fn evaluate_floor(
    expr: &Expr,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Term {
    let value = evaluate_expr(expr, scopes, current_scope, context);

    if let Term::Num(num) = value {
        Term::Num(num.floor())
    } else {
        panic!("Can only pass values of type Num into floor().");
    }
}

fn evaluate_len(
    expr: &Expr,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Term {
    let value = evaluate_expr(expr, scopes, current_scope, context);

    if let Term::Array(array) = value {
        Term::Num(array.len() as f32)
    } else {
        panic!("Can only pass values of type Array into len().");
    }
}
