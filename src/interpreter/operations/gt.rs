use crate::{
    ast::{terms::*, types::*},
    scope::{ScopeId, Scopes},
};

use super::errors::panic_oper_not_impl;

pub fn evaluate_gt(left: Term, right: Term, scopes: &mut Scopes, current_scope: ScopeId) -> Term {
    let left_can_compare = left
        .get_type()
        .implements_interface(PrimitiveInterface::ICompare);

    let right_can_compare = right
        .get_type()
        .implements_interface(PrimitiveInterface::ICompare);

    if !left_can_compare || !right_can_compare {
        panic!("Can't compare!")
    }

    let left = if let Term::Symbol(symbol) = left {
        scopes.get_value(&symbol, current_scope)
    } else {
        left
    };

    match left {
        Term::Num(left) => match right {
            Term::Num(right) => Term::Bool(left > right),
            Term::Symbol(right) => {
                let right = scopes.get_value(&right, current_scope);
                evaluate_gt(Term::Num(left), right, scopes, current_scope)
            }
            right => panic_oper_not_impl!(">", PrimitiveType::Number, right.get_type()),
        },
        Term::String(left) => match right {
            Term::String(right) => Term::Bool(left > right),
            Term::Symbol(right) => {
                let right = scopes.get_value(&right, current_scope);
                evaluate_gt(Term::String(left), right, scopes, current_scope)
            }
            right => panic_oper_not_impl!(">", PrimitiveType::String, right.get_type()),
        },
        left => panic_oper_not_impl!(">", left.get_type()),
    }
}
