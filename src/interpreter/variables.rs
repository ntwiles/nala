use super::basic::*;

use crate::{
    ast::*,
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

pub fn interpret_declare(
    ident: &String,
    term: &Term,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    is_mutable: bool,
) -> Term {
    if scopes.binding_exists_local(&ident, current_scope) {
        panic!("Binding for {} already exists in local scope.", ident);
    } else {
        if let Term::Void = term {
            panic!("Cannot assign Void.");
        }

        scopes.add_binding(&ident, current_scope, term.clone(), is_mutable);
    }

    Term::Void
}

pub fn interpret_assign(
    variable: &SymbolOrIndex,
    term: &Term,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Term {
    match variable {
        SymbolOrIndex::Index(ident, index_expr) => {
            if scopes.binding_exists(&ident, current_scope) {
                let index = evaluate_expr(&index_expr, scopes, current_scope, context);

                if let Term::Void = term {
                    panic!("Cannot assign Void.");
                }

                let index = if let Term::Num(index) = index {
                    index
                } else {
                    panic!("Index does not resolve to a Number.");
                };

                let array = scopes.get_value(&ident, current_scope);

                if let Term::Array(mut array) = array {
                    // TODO: This doesn't work with bad input.
                    array[index as usize] = term.clone();

                    scopes.mutate_value(&ident, current_scope, Term::Array(array));
                } else {
                    panic!("Trying to index into a non-Array.")
                }
            }
        }
        SymbolOrIndex::Symbol(ident) => {
            if scopes.binding_exists(&ident, current_scope) {
                if let Term::Void = term {
                    panic!("Cannot assign Void.");
                }

                scopes.mutate_value(&ident, current_scope, term.clone());
            } else {
                panic!("Unknown identifier `{}`", ident);
            }
        }
    }

    Term::Void
}
