use super::basic::*;

use crate::{
    ast::{terms::*, *},
    errors::NalaRuntimeError,
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

pub fn interpret_declare(
    ident: &String,
    term: &Term,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    is_mutable: bool,
) -> Result<Term, NalaRuntimeError> {
    if scopes.binding_exists_local(&ident, current_scope) {
        panic!("Binding for {} already exists in local scope.", ident);
    } else {
        if let Term::Void = term {
            return Err(NalaRuntimeError {
                message: "Cannot declare a variable with a value of type Void.".to_string(),
            });
        }

        scopes.add_binding(&ident, current_scope, term.clone(), is_mutable);
    }

    Ok(Term::Void)
}

pub fn interpret_assign(
    variable: &SymbolOrIndex,
    term: &Term,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Result<Term, NalaRuntimeError> {
    match variable {
        SymbolOrIndex::Index(ident, index_expr) => {
            if scopes.binding_exists(&ident, current_scope, context) {
                let index_result = evaluate_expr(&index_expr, scopes, current_scope, context)?;

                if let Term::Void = term {
                    panic!("Cannot assign a value of type Void.");
                }

                let index = if let Term::Num(index) = index_result {
                    index
                } else {
                    panic!("Index does not resolve to a Number.");
                };

                let array = scopes.get_value(&ident, current_scope, context);

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
            if scopes.binding_exists(&ident, current_scope, context) {
                if let Term::Void = term {
                    panic!("Cannot assign a value of type Void.");
                }

                let existing = scopes.get_value(ident, current_scope, context);

                let existing_type = existing.get_type();
                let term_type = term.get_type();

                if existing_type == term_type {
                    scopes.mutate_value(&ident, current_scope, term.clone());
                } else {
                    return Err(NalaRuntimeError {
                        message: format!(
                            "Cannot assign a value of type {0} where {1} is expected.",
                            term_type, existing_type
                        ),
                    });
                }
            } else {
                panic!("Unknown identifier `{}`", ident);
            }
        }
    }

    Ok(Term::Void)
}
