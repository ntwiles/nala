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
            let result = evaluate_expr(expr, scopes, current_scope, context)?;

            if let Term::Num(index) = result {
                let array = scopes.get_value(ident, current_scope, context);
                // TODO: Check that this cast is safe first.
                let index = index as usize;
                if let Term::Array(array) = array {
                    Ok(array.get(index).unwrap().clone())
                } else {
                    panic!("Cannot index into a value which is not an array.");
                }
            } else {
                Err(NalaRuntimeError {
                    message: "Cannot index using non-numeric value.".to_owned(),
                })
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
    let terms = evaluate_elems(&array.elems, scopes, current_scope, context)?;

    if let Some(first) = terms.clone().first() {
        let first_type = first.get_type();

        for term in terms.clone() {
            if term.get_type() != first_type {
                let message = format!("Arrays can contain elements of only a single type. Found elements of types `{0}` and `{1}`.",
                first_type.to_string(),
                term.get_type().to_string());

                return Err(NalaRuntimeError { message });
            }
        }
    };

    Ok(Term::Array(terms))
}
