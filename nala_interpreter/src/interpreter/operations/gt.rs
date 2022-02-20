use crate::{
    ast::{
        terms::*,
        types::{PrimitiveInterface::*, *},
    },
    errors::*,
};

use super::errors::*;

pub fn evaluate_gt(left: Term, right: Term) -> Result<Term, NalaRuntimeError> {
    check_operator_implemented_both(left.get_type(), right.get_type(), ">".to_string(), ICompare)?;

    match left {
        Term::Num(left) => match right {
            Term::Num(right) => Ok(Term::Bool(left > right)),
            right => panic_oper_not_impl_for(
                ">",
                &TypeVariant::Type(Type::PrimitiveType(PrimitiveType::Number)),
                &right.get_type(),
            ),
        },
        Term::String(left) => match right {
            Term::String(right) => Ok(Term::Bool(left > right)),
            right => panic_oper_not_impl_for(
                ">",
                &TypeVariant::Type(Type::PrimitiveType(PrimitiveType::String)),
                &right.get_type(),
            ),
        },
        left => panic_oper_not_impl(">", &left.get_type()),
    }
}
