use crate::{
    ast::{
        terms::*,
        types::{PrimitiveInterface::*, PrimitiveType},
    },
    interpreter::evaluate_if_symbol,
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

use super::errors::*;

pub fn evaluate_gt(
    left: Term,
    right: Term,
    scopes: &mut Scopes,
    current_scope: ScopeId,
    context: &mut dyn IoContext,
) -> Term {
    check_operator_implemented(left.get_type(), ">".to_string(), ICompare, context);
    check_operator_implemented(right.get_type(), ">".to_string(), ICompare, context);

    let left = evaluate_if_symbol(left, scopes, current_scope, context);
    let right = evaluate_if_symbol(right, scopes, current_scope, context);

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
