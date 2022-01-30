use crate::{
    ast::*,
    scope::{ScopeId, Scopes},
};

pub fn evaluate_lt(left: Term, right: Term, scopes: &mut Scopes, current_scope: ScopeId) -> Term {
    match left {
        Term::Num(left) => num_lt(left, right, scopes, current_scope),
        Term::String(left) => string_lt(left, right, scopes, current_scope),
        Term::Symbol(left) => {
            let left = scopes.get_value(&left, current_scope);
            evaluate_lt(left, right, scopes, current_scope)
        }
        Term::Bool(left) => bool_lt(left, right, scopes, current_scope),
        left => panic!(
            "Cannot perform comparisons against values of type {}.",
            left.get_type().to_string()
        ),
    }
}

fn num_lt(left: f32, right: Term, scopes: &mut Scopes, current_scope: ScopeId) -> Term {
    match right {
        Term::Num(right) => Term::Bool(left < right),
        Term::Symbol(right) => {
            let right = scopes.get_value(&right, current_scope);
            evaluate_lt(Term::Num(left), right, scopes, current_scope)
        }
        right => panic!(
            "Cannot perform comparisons between types Num and {}.",
            right.get_type().to_string()
        ),
    }
}

fn string_lt(left: String, right: Term, scopes: &mut Scopes, current_scope: ScopeId) -> Term {
    match right {
        Term::String(right) => Term::Bool(left < right),
        Term::Symbol(right) => {
            let right = scopes.get_value(&right, current_scope);
            evaluate_lt(Term::String(left), right, scopes, current_scope)
        }
        right => panic!(
            "Cannot perform comparisons between types String and {}.",
            right.get_type().to_string()
        ),
    }
}

fn bool_lt(left: bool, right: Term, scopes: &mut Scopes, current_scope: ScopeId) -> Term {
    match right {
        Term::Symbol(right) => {
            let right = scopes.get_value(&right, current_scope);
            evaluate_lt(Term::Bool(left), right, scopes, current_scope)
        }
        Term::Bool(right) => Term::Bool(left < right),
        right => panic!(
            "Cannot perform comparisons between types Bool and {}.",
            right.get_type().to_string()
        ),
    }
}
