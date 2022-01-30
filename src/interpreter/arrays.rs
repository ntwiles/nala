use std::usize;

use crate::{
    ast::*,
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

use super::basic::*;

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
                let array = scopes.get_value(ident, current_scope);
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
        Index::Term(term) => {
            if let Term::Symbol(ident) = term {
                scopes.get_value(ident, current_scope)
            } else {
                term.clone()
            }
        }
    }
}

pub fn evaluate_array(
    array: &Array,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut impl IoContext,
) -> Term {
    println!("Evaluating array.");
    let terms = evaluate_elems(&array.elems, scopes, current_scope, context);
    println!("About to return array.");
    Term::Array(terms)
}
