use std::usize;

use crate::{
    ast::{arrays::*, terms::*},
    errors::NalaRuntimeError,
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

use super::basic::*;

pub fn evaluate_index(
    index: &Index,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Result<Term, NalaRuntimeError> {
    match index {
        Index::Index(ident, expr) => {
            let result = evaluate_expr(expr, scopes, current_scope, context);

            if let Err(e) = result {
                return Err(e);
            }

            if let Term::Num(index) = result.unwrap() {
                let array = scopes.get_value(ident, current_scope, context);
                // TODO: Check that this cast is safe first.
                let index = index as usize;
                if let Term::Array(array) = array {
                    Ok(array.get(index).unwrap().clone())
                } else {
                    panic!("Cannot index into a value which is not an array.");
                }
            } else {
                panic!("Cannot index using non-numeric value.");
            }
        }
        Index::SymbolOrTerm(sot) => match sot {
            SymbolOrTerm::Symbol(ident) => Ok(scopes.get_value(ident, current_scope, context)),
            SymbolOrTerm::Term(term) => Ok(term.clone()),
        },
    }
}

pub fn evaluate_array(
    array: &Array,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Result<Term, NalaRuntimeError> {
    let result = evaluate_elems(&array.elems, scopes, current_scope, context);

    if let Err(e) = result {
        return Err(e);
    }

    let terms = result.unwrap();

    if let Some(first) = terms.clone().first() {
        let first_type = first.get_type();

        for term in terms.clone() {
            if term.get_type() != first_type {
                panic!(
                    "Arrays can contain elements of only a single type. Found elements of types `{0}` and `{1}`.",
                    first_type.to_string(),
                    term.get_type().to_string()
                );
            }
        }
    };

    Ok(Term::Array(terms))
}
