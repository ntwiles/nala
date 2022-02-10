use crate::ast::{terms::*, types::*};

use super::errors::panic_oper_not_impl;

pub fn evaluate_equals(left: Term, right: Term) -> Term {
    match left {
        Term::Num(left) => num_equals(left, right),
        Term::String(left) => string_equals(left, right),
        Term::Bool(left) => bool_equals(left, right),
        Term::Variant(left) => variant_equals(left, right),
        other => panic_oper_not_impl!("==", other.get_type().to_string()),
    }
}

fn num_equals(left: f32, right: Term) -> Term {
    if let Term::Num(right) = right {
        Term::Bool(left == right)
    } else {
        panic_oper_not_impl!("==", PrimitiveType::Number, right.get_type())
    }
}

fn string_equals(left: String, right: Term) -> Term {
    if let Term::String(right) = right {
        Term::Bool(left == right)
    } else {
        panic_oper_not_impl!("==", PrimitiveType::String, right.get_type())
    }
}

fn bool_equals(left: bool, right: Term) -> Term {
    if let Term::Bool(right) = right {
        Term::Bool(left == right)
    } else {
        panic_oper_not_impl!("==", PrimitiveType::Bool, right.get_type())
    }
}

fn variant_equals(left: String, right: Term) -> Term {
    if let Term::Variant(right) = right {
        Term::Bool(left == right)
    } else {
        panic_oper_not_impl!("==", PrimitiveType::Variant, right.get_type())
    }
}
