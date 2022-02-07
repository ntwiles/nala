use std::usize;

use crate::{
    ast::{arrays::*, terms::*},
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

use super::{basic::*, evaluate_if_symbol};

pub fn evaluate_index(
    index: &Index,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Term {
    match index {
        Index::Index(ident, expr) => {
            let index = evaluate_expr(expr, scopes, current_scope, context);

            if let Term::Num(index) = index {
                let array = scopes.get_value(ident, current_scope, context);
                // TODO: Check that this cast is safe first.
                let index = index as usize;
                if let Term::Array(array) = array {
                    array.get(index).unwrap().clone()
                } else {
                    panic!("Cannot index into a value which is not an array.");
                }
            } else {
                panic!("Cannot index using non-numeric value.");
            }
        }
        Index::Term(term) => evaluate_if_symbol(term.clone(), scopes, current_scope, context),
    }
}

pub fn evaluate_array(
    array: &Array,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Term {
    let terms = evaluate_elems(&array.elems, scopes, current_scope, context);

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

    Term::Array(terms)
}
