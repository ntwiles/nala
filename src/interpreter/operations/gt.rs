use crate::ast::{
    terms::*,
    types::{PrimitiveInterface::*, PrimitiveType},
};

use super::errors::*;

pub fn evaluate_gt(left: Term, right: Term) -> Term {
    let result = check_operator_implemented_both(
        left.get_type(),
        right.get_type(),
        ">".to_string(),
        ICompare,
    );

    if let Err(err) = result {
        return err;
    }

    match left {
        Term::Num(left) => match right {
            Term::Num(right) => Term::Bool(left > right),
            right => panic_oper_not_impl!(">", PrimitiveType::Number, right.get_type()),
        },
        Term::String(left) => match right {
            Term::String(right) => Term::Bool(left > right),
            right => panic_oper_not_impl!(">", PrimitiveType::String, right.get_type()),
        },
        left => panic_oper_not_impl!(">", left.get_type()),
    }
}
