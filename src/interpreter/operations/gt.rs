use crate::{
    ast::{types::*, *},
    scope::{ScopeId, Scopes},
};

use super::errors::panic_oper_not_impl;

pub fn evaluate_gt(left: Term, right: Term, scopes: &mut Scopes, current_scope: ScopeId) -> Term {
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
        Term::Symbol(left) => {
            let left = scopes.get_value(&left, current_scope);
            evaluate_gt(left, right, scopes, current_scope)
        }
        Term::Bool(left) => match right {
            Term::Symbol(right) => {
                let right = scopes.get_value(&right, current_scope);
                evaluate_gt(Term::Bool(left), right, scopes, current_scope)
            }
            Term::Bool(right) => Term::Bool(left > right),
            right => panic_oper_not_impl!(">", PrimitiveType::Bool, right.get_type()),
        },
        left => panic_oper_not_impl!(">", left.get_type()),
    }
}
