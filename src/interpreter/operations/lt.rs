use crate::{
    ast::{
        terms::*,
        types::{PrimitiveInterface::*, PrimitiveType},
    },
    interpreter::evaluate_if_symbol,
    scope::{ScopeId, Scopes},
};

use super::errors::*;

pub fn evaluate_lt(left: Term, right: Term, scopes: &mut Scopes, current_scope: ScopeId) -> Term {
    check_operator_implemented(left.get_type(), ">".to_string(), ICompare);
    check_operator_implemented(right.get_type(), ">".to_string(), ICompare);

    let left = evaluate_if_symbol(left, scopes, current_scope);
    let right = evaluate_if_symbol(right, scopes, current_scope);

    match left {
        Term::Num(left) => num_lt(left, right),
        Term::String(left) => string_lt(left, right),
        Term::Bool(left) => bool_lt(left, right),
        left => panic_oper_not_impl!("<", left.get_type()),
    }
}

fn num_lt(left: f32, right: Term) -> Term {
    match right {
        Term::Num(right) => Term::Bool(left < right),
        right => panic_oper_not_impl!("<", PrimitiveType::Number, right.get_type()),
    }
}

fn string_lt(left: String, right: Term) -> Term {
    match right {
        Term::String(right) => Term::Bool(left < right),
        right => panic_oper_not_impl!("<", PrimitiveType::String, right.get_type()),
    }
}

fn bool_lt(left: bool, right: Term) -> Term {
    match right {
        Term::Bool(right) => Term::Bool(left < right),
        right => panic_oper_not_impl!("<", PrimitiveType::Bool, right.get_type()),
    }
}
