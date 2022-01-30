use crate::{
    ast::*,
    scope::{ScopeId, Scopes},
};

use super::errors::panic_oper_not_impl;

pub fn evaluate_equals(
    left: Term,
    right: Term,
    scopes: &mut Scopes,
    current_scope: ScopeId,
) -> Term {
    match left {
        Term::Num(left) => num_equals(left, right, scopes, current_scope),
        Term::String(left) => string_equals(left, right, scopes, current_scope),
        Term::Symbol(left) => {
            let left = scopes.get_value(&left, current_scope);
            evaluate_equals(left, right, scopes, current_scope)
        }
        Term::Bool(left) => bool_equals(left, right, scopes, current_scope),
        Term::Kind(left) => kind_equals(left, right),
        other => panic_oper_not_impl!("==", other.get_type().to_string()),
    }
}

fn num_equals(left: f32, right: Term, scopes: &mut Scopes, current_scope: ScopeId) -> Term {
    if let Term::Num(right) = right {
        Term::Bool(left == right)
    } else if let Term::Symbol(right) = right {
        let right = scopes.get_value(&right, current_scope);
        evaluate_equals(Term::Num(left), right, scopes, current_scope)
    } else {
        panic_oper_not_impl!("==", PrimitiveType::Number, right.get_type())
    }
}

fn string_equals(left: String, right: Term, scopes: &mut Scopes, current_scope: ScopeId) -> Term {
    if let Term::String(right) = right {
        Term::Bool(left == right)
    } else if let Term::Symbol(right) = right {
        let right = scopes.get_value(&right, current_scope);
        evaluate_equals(Term::String(left), right, scopes, current_scope)
    } else {
        panic_oper_not_impl!("==", PrimitiveType::String, right.get_type())
    }
}

fn bool_equals(left: bool, right: Term, scopes: &mut Scopes, current_scope: ScopeId) -> Term {
    if let Term::Bool(right) = right {
        Term::Bool(left == right)
    } else if let Term::Symbol(right) = right {
        let right = scopes.get_value(&right, current_scope);
        evaluate_equals(Term::Bool(left), right, scopes, current_scope)
    } else {
        panic_oper_not_impl!("==", PrimitiveType::Bool, right.get_type())
    }
}

fn kind_equals(left: String, right: Term) -> Term {
    if let Term::Kind(right) = right {
        Term::Bool(left == right)
    } else {
        panic_oper_not_impl!("==", PrimitiveType::Kind, right.get_type())
    }
}
