use crate::{
    ast::{
        terms::*,
        types::{PrimitiveInterface::*, PrimitiveType},
    },
    io_context::IoContext,
};

use super::errors::*;

pub fn evaluate_gt(left: Term, right: Term, context: &mut dyn IoContext) -> Term {
    check_operator_implemented(left.get_type(), ">".to_string(), ICompare, context);
    check_operator_implemented(right.get_type(), ">".to_string(), ICompare, context);

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
