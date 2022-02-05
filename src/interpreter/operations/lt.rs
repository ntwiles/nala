use crate::{
    ast::{types::*, *},
    scope::{ScopeId, Scopes},
};

use super::errors::panic_oper_not_impl;

pub fn evaluate_lt(left: Term, right: Term, scopes: &mut Scopes, current_scope: ScopeId) -> Term {
    match left {
        Term::Num(left) => num_lt(left, right, scopes, current_scope),
        Term::String(left) => string_lt(left, right, scopes, current_scope),
        Term::Symbol(left) => {
            let left = scopes.get_value(&left, current_scope);
            evaluate_lt(left, right, scopes, current_scope)
        }
        Term::Bool(left) => bool_lt(left, right, scopes, current_scope),
        left => panic_oper_not_impl!("<", left.get_type()),
    }
}

fn num_lt(left: f32, right: Term, scopes: &mut Scopes, current_scope: ScopeId) -> Term {
    match right {
        Term::Num(right) => Term::Bool(left < right),
        Term::Symbol(right) => {
            let right = scopes.get_value(&right, current_scope);
            evaluate_lt(Term::Num(left), right, scopes, current_scope)
        }
        right => panic_oper_not_impl!("<", PrimitiveType::Number, right.get_type()),
    }
}

fn string_lt(left: String, right: Term, scopes: &mut Scopes, current_scope: ScopeId) -> Term {
    match right {
        Term::String(right) => Term::Bool(left < right),
        Term::Symbol(right) => {
            let right = scopes.get_value(&right, current_scope);
            evaluate_lt(Term::String(left), right, scopes, current_scope)
        }
        right => panic_oper_not_impl!("<", PrimitiveType::String, right.get_type()),
    }
}

fn bool_lt(left: bool, right: Term, scopes: &mut Scopes, current_scope: ScopeId) -> Term {
    match right {
        Term::Symbol(right) => {
            let right = scopes.get_value(&right, current_scope);
            evaluate_lt(Term::Bool(left), right, scopes, current_scope)
        }
        Term::Bool(right) => Term::Bool(left < right),
        right => panic_oper_not_impl!("<", PrimitiveType::Bool, right.get_type()),
    }
}
