use crate::{
    ast::{
        terms::*,
        types::{PrimitiveInterface::*, PrimitiveType},
    },
    errors::*,
};

use super::errors::*;

pub fn evaluate_lt(left: Term, right: Term) -> Result<Term, NalaRuntimeError> {
    check_operator_implemented_both(left.get_type(), right.get_type(), ">".to_string(), ICompare)?;

    match left {
        Term::Num(left) => Ok(num_lt(left, right)),
        Term::String(left) => Ok(string_lt(left, right)),
        Term::Bool(left) => Ok(bool_lt(left, right)),
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
